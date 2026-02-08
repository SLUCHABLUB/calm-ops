#![deny(clippy::pedantic)]

use crate::implementation::implementation;
use proc_macro::TokenStream;
use syn::{Expr, parse_macro_input};

mod implementation;

/// Evaluates a binary operation using checked overflow semantics.
///
/// This means that an `Option` is always returned.
///
/// ```
/// # use calm_ops::checked;
/// # assert! (
/// checked!(126_i8 + 1_i8) == Some(127_i8)
/// # );
/// # assert! (
/// checked!(127_i8 + 1_i8) == None
/// # );
///
/// # assert! (
/// checked!(1_u8 - 1_u8) == Some(0_u8)
/// # );
/// # assert! (
/// checked!(0_u8 - 1_u8) == None
/// # );
///
/// # assert! (
/// checked!(2_i8 * 63_i8) == Some(126_i8)
/// # );
/// # assert! (
/// checked!(2_i8 * 64_i8) == None
/// # );
/// # assert! (
/// checked!(-128_i8 * -1_i8) == None
/// # );
///
/// # assert! (
/// checked!(42 / 2) == Some(21)
/// # );
/// # assert! (
/// checked!(42 / 0) == None
/// # );
/// # assert! (
/// checked!(-128_i8 / -1_i8) == None
/// # );
///
/// # assert! (
/// checked!(42 % 5) == Some(2)
/// # );
/// # assert! (
/// checked!(-42 % 5) == Some(-2)
/// # );
/// # assert! (
/// checked!(42 % 0) == None
/// # );
/// # assert! (
/// checked!(-128_i8 % -1_i8) == None
/// # );
///
/// # assert! (
/// checked!(-42_i8) == Some(-42_i8)
/// # );
/// # assert! (
/// checked!(-(-128_i8)) == None
/// # );
///
/// # assert! (
/// checked!(42_u8 as i8) == Some(42_i8)
/// # );
/// # assert! (
/// checked!(128_u8 as i8) == None
/// # );
///
/// // These do not check for overflow, but the value of the right operator.
///
/// # assert! (
/// checked!(255_u8 << 7) == Some(128_u8)
/// # );
/// # assert! (
/// checked!(-1_i8 << 7) == Some(-128_i8)
/// # );
/// # assert! (
/// checked!(1_u8 << 8) == None
/// # );
///
/// # assert! (
/// checked!(255_u8 >> 7) == Some(1_u8)
/// # );
/// # assert! (
/// checked!(-128_i8 >> 7) == Some(-1_i8)
/// # );
/// # assert! (
/// checked!(255_u8 >> 8) == None
/// # );
/// ```
#[proc_macro]
pub fn checked(input: TokenStream) -> TokenStream {
    implementation(parse_macro_input!(input as Expr), "Checked", "checked").into()
}

/// Evaluates a binary operation by first encapsulating in a larger type to avoid overflow.
///
/// Note that the result will be unconditionally encapsulated.
///
/// ```
/// # use calm_ops::encapsulating;
/// # assert! (
/// encapsulating!(126_i8 + 1_i8) == 127_i16
/// # );
/// # assert! (
/// encapsulating!(127_i8 + 1_i8) == 128_i16
/// # );
///
/// // Subtraction is currently only implemented for signed integers.
///
/// # assert! (
/// encapsulating!(-127_i8 - 1_i8) == -128_i16
/// # );
/// # assert! (
/// encapsulating!(-128_i8 - 1_i8) == -129_i16
/// # );
///
/// # assert! (
/// encapsulating!(2_i8 * 63_i8) == 126_i16
/// # );
/// # assert! (
/// encapsulating!(2_i8 * 64_i8) == 128_i16
/// # );
/// # assert! (
/// encapsulating!(-128_i8 * -1_i8) == 128_i16
/// # );
///
/// // Division and modulo are not yet implemented.
///
/// # assert! (
/// encapsulating!(-42_i8) == -42_i16
/// # );
/// # assert! (
/// encapsulating!(-(-128_i8)) == 128_i16
/// # );
/// ```
#[proc_macro]
pub fn encapsulating(input: TokenStream) -> TokenStream {
    implementation(
        parse_macro_input!(input as Expr),
        "Encapsulating",
        "encapsulating",
    )
    .into()
}

/// Evaluates a binary operation using saturating overflow semantics.
///
/// ```
/// # use calm_ops::saturating;
/// # assert! (
/// saturating!(126_i8 + 1_i8) == 127_i8
/// # );
/// # assert! (
/// saturating!(127_i8 + 1_i8) == 127_i8
/// # );
///
/// # assert! (
/// saturating!(1_u8 - 1_u8) == 0_u8
/// # );
/// # assert! (
/// saturating!(0_u8 - 1_u8) == 0_u8
/// # );
///
/// # assert! (
/// saturating!(2_i8 * 63_i8) == 126_i8
/// # );
/// # assert! (
/// saturating!(2_i8 * 64_i8) == 127_i8
/// # );
/// # assert! (
/// saturating!(-128_i8 * -1_i8) == 127_i8
/// # );
///
/// // Division and modulo are not yet implemented.
///
/// # assert! (
/// saturating!(-42_i8) == -42_i8
/// # );
/// # assert! (
/// saturating!(-(-128_i8)) == 127_i8
/// # );
///
/// # assert! (
/// saturating!(42_u8 as i8) == 42_i8
/// # );
/// # assert! (
/// saturating!(128_u8 as i8) == 127_i8
/// # );
///
/// // Shift operations are not yet implemented.
/// ```
#[proc_macro]
pub fn saturating(input: TokenStream) -> TokenStream {
    implementation(
        parse_macro_input!(input as Expr),
        "Saturating",
        "saturating",
    )
    .into()
}

/// Evaluates a binary operation using wrapping overflow semantics.
///
/// This is the semantics of the default operators in release mode.
///
/// ```
/// # use calm_ops::wrapping;
/// # assert! (
/// wrapping!(126_i8 + 1_i8) == 127_i8
/// # );
/// # assert! (
/// wrapping!(127_i8 + 1_i8) == -128_i8
/// # );
///
/// # assert! (
/// wrapping!(1_u8 - 1_u8) == 0_u8
/// # );
/// # assert! (
/// wrapping!(0_u8 - 1_u8) == 255_u8
/// # );
///
/// # assert! (
/// wrapping!(2_i8 * 63_i8) == 126_i8
/// # );
/// # assert! (
/// wrapping!(2_i8 * 64_i8) == -128_i8
/// # );
/// # assert! (
/// wrapping!(-128_i8 * -1_i8) == -128_i8
/// # );
///
/// // Division and modulo are not yet implemented.
///
/// # assert! (
/// wrapping!(-42_i8) == -42_i8
/// # );
/// # assert! (
/// wrapping!(-(-128_i8)) == -128_i8
/// # );
///
/// // Casting is not yes implemented.
///
/// // These do not wrap the result, but the value of the right operator before shifting.
///
/// # assert! (
/// wrapping!(255_u8 << 7) == 128_u8
/// # );
/// # assert! (
/// wrapping!(-1_i8 << 7) == -128_i8
/// # );
/// # assert! (
/// wrapping!(1_u8 << 8) == 1_u8
/// # );
///
/// # assert! (
/// wrapping!(255_u8 >> 7) == 1_u8
/// # );
/// # assert! (
/// wrapping!(-128_i8 >> 7) == -1_i8
/// # );
/// # assert! (
/// wrapping!(255_u8 >> 8) == 255_u8
/// # );
/// ```
#[proc_macro]
pub fn wrapping(input: TokenStream) -> TokenStream {
    implementation(parse_macro_input!(input as Expr), "Wrapping", "wrapping").into()
}
