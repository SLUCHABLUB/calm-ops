#![deny(clippy::all, clippy::arithmetic_side_effects)]

mod encapsulating;
mod wrapper;

pub use calm_ops_macros::*;
pub use encapsulating::*;
pub use wrapper::*;