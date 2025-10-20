#![deny(clippy::all)]

use crate::implementation::implementation;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr};

mod implementation;

#[proc_macro]
pub fn checked(input: TokenStream) -> TokenStream {
    implementation(
        parse_macro_input!(input as Expr),
        "Checked",
        "checked",
    )
    .into()
}

#[proc_macro]
pub fn encapsulating(input: TokenStream) -> TokenStream {
    implementation(
        parse_macro_input!(input as Expr),
        "Encapsulating",
        "encapsulating",
    )
    .into()
}

#[proc_macro]
pub fn saturating(input: TokenStream) -> TokenStream {
    implementation(
        parse_macro_input!(input as Expr),
        "Saturating",
        "saturating",
    )
    .into()
}

#[proc_macro]
pub fn wrapping(input: TokenStream) -> TokenStream {
    implementation(
        parse_macro_input!(input as Expr),
        "Wrapping",
        "wrapping",
    )
    .into()
}
