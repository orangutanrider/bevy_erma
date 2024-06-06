mod collect_until_punct;

use std::str::*;
use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub(crate) fn compile_error_stream(msg: &str) -> TokenStream {
    let Ok(stream) = TokenStream::from_str(&("compile_error!(\"".to_owned() + msg + "\")")) else {
        panic!("Unexpected lex error while trying to create a compile_error! token stream.")
    };

    return stream;
}

/// (Input stream, Processed tokens, Found ident)
pub(crate) fn until_ident(iter: TokenIter) -> (TokenIter, Vec<TokenTree>, Option<Ident>) {
    return collect_until_ident(iter, Vec::new())
}

fn collect_until_ident(mut iter: TokenIter, mut collection: Vec<TokenTree>) -> (TokenIter, Vec<TokenTree>, Option<Ident>) {
    let Some(token) = iter.next() else {
        return (iter, collection, None) // Ident has not been found, and the entire input has been processed.
    };

    match token {
        TokenTree::Ident(ident) => return (iter, collection, Some(ident)),
        _ => { // Non ident
            collection.push(token); // Add to collection
            return collect_until_ident(iter, collection) // And recur.
        },
    }
}

/// Returns true if the the iter contains a "mut" ident. Otherwise, return false.
/// To find the mut ident, it will search through groups recursively.
fn contains_mut_recursive(mut iter: TokenIter) -> bool {
    let token = iter.next();
    let Some(token) = token else {
        return false;
    };

    match token {
        TokenTree::Group(group) => {
            let group = group.stream().into_iter();
            if contains_mut_recursive(group) {
                return true
            }
            return contains_mut_recursive(iter);
        },
        TokenTree::Ident(ident) => {
            let ident = ident.to_string();
            if ident == "mut" {
                return true
            }
            return contains_mut_recursive(iter);
        },
        TokenTree::Punct(_) => {
            return contains_mut_recursive(iter);
        },
        TokenTree::Literal(_) => {
            return contains_mut_recursive(iter);
        },
    }
}