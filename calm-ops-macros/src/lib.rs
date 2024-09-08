#![deny(clippy::all)]

use proc_macro::TokenStream;
use syn::parse_macro_input;
use crate::wrapping::wrapping_implementation;

mod wrapping;
mod util;

#[proc_macro]
pub fn checked(_input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro]
pub fn encapsulating(_input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro]
pub fn saturating(_input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro]
pub fn wrapping(input: TokenStream) -> TokenStream {
    wrapping_implementation(parse_macro_input!(input as _)).into()
}
