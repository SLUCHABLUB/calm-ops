#![deny(clippy::all)]

use crate::implementation::implementation;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr};

mod implementation;

const CHECKED_TRAIT_SUFFIXES: &[&str] = &["Add", "Div", "Mul", "Neg", "Rem", "Shl", "Shr", "Sub"];
const ENCAPSULATING_TRAIT_SUFFIXES: &[&str] = &["Add", "Mul", "Neg", "Sub"];
const SATURATING_TRAIT_SUFFIXES: &[&str] = &["Add", "Mul", "Sub"];
const WRAPPING_TRAIT_SUFFIXES: &[&str] = &["Add", "Mul", "Neg", "Shl", "Shr", "Sub"];

#[proc_macro]
pub fn checked(input: TokenStream) -> TokenStream {
    implementation(
        parse_macro_input!(input as Expr),
        "Checked",
        "checked",
        CHECKED_TRAIT_SUFFIXES,
    )
    .into()
}

#[proc_macro]
pub fn encapsulating(input: TokenStream) -> TokenStream {
    implementation(
        parse_macro_input!(input as Expr),
        "Encapsulating",
        "encapsulating",
        ENCAPSULATING_TRAIT_SUFFIXES,
    )
    .into()
}

#[proc_macro]
pub fn saturating(input: TokenStream) -> TokenStream {
    implementation(
        parse_macro_input!(input as Expr),
        "Saturating",
        "saturating",
        SATURATING_TRAIT_SUFFIXES,
    )
    .into()
}

#[proc_macro]
pub fn wrapping(input: TokenStream) -> TokenStream {
    implementation(
        parse_macro_input!(input as Expr),
        "Wrapping",
        "wrapping",
        WRAPPING_TRAIT_SUFFIXES,
    )
    .into()
}
