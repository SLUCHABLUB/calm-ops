#![deny(clippy::pedantic, clippy::arithmetic_side_effects)]
//! # `calm-ops`
//!
//! *Non [`panic`]ing operations made easy.*
//!
//! To avoid panics on integer overflow, Rust provides several functions for integers.
//! These are all named according to their respective semantics on overflow: e.g. `wrapping_add`, `saturating_sub`, `checked_div`.
//! These are however more cumbersome to write and read than the standard operators: `+` `-` `/`.
//! Whilst there are the [`Wrapping`](std::num::Wrapping) and [`Saturating`](std::num::Saturating) wrappers in the standard library,
//! you need to wrap and unwrap your values in order to use them.
//! Some languages, like Zig, provide specific operators like `+%` and `+|` for specifying the overflow semantics.
//! and whilst you could implement this in rust using macros, I've opted for something different in this crate.
//!
//! This crate provides four macros: [`wrapping`], [`saturating`], [`checked`] and [`encapsulating`](macro@encapsulating),
//! that allow you to specify the overflow semantics.
//!
//! ```
//! # use calm_ops::*;
//! # assert!(
//! wrapping!(127_i8 + 1_i8) == -128_i8
//! # );
//!
//! # assert!(
//! saturating!(127_i8 + 1_i8) == 127_i8
//! # );
//!
//! # assert!(
//! checked!(127_i8 + 1_i8) == None
//! # );
//!
//! # assert!(
//! encapsulating!(127_i8 + 1_i8) == 128_i16
//! # );
//! ```
//!
//! They all take an operation expression and evaluates it using the specified sematic.
//! I personally find this more readable than inventing new operators.
//! This is however done in a shallow manner. Therefore,
//!
//! ```should_panic
//! # #![allow(arithmetic_overflow)]
//! # use calm_ops::*;
//! wrapping!((127_i8 + 1_i8) + 1_i8) != -127_i8
//! # ;
//! ```
//!
//! Will still panic.
//!
//! Casting is also supported.
//!
//! ```
//! # use calm_ops::*;
//! # assert!(
//! wrapping!(255_u8 as i8) == -1_i8
//! # );
//!
//! # assert!(
//! saturating!(255_u8 as i8) == 127_i8
//! # );
//!
//! # assert!(
//! checked!(255_u8 as i8) == None
//! # );
//! ```

pub use calm_ops_macros::*;
pub use calm_ops_traits::*;

#[cfg(test)]
mod test;
