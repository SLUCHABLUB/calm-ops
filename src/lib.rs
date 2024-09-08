#![deny(clippy::all, clippy::arithmetic_side_effects)]

use paste::paste;
use std::ops::{Add, Mul, Neg, Sub};

mod implementations;

pub use calm_ops_macros::{checked, encapsulating, saturating, wrapping};

pub use num::traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr, CheckedSub,
};
pub use num::traits::{SaturatingAdd, SaturatingMul, SaturatingSub};
pub use num::traits::{
    WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub,
};

/// Used for types that can be encapsulated in a "larger" type.
/// Any operations[^operations] that may panic due to overflow may not panic
/// due to overflow when the same values, cast to `Self::Super`, are operated.
///
/// # Examples
///
/// Adding two `u8` values may panic if the sum is greater than `u8::MAX`.
/// But casting the values to `u16` first ensures that the operation won't panic.
///
/// ```
///  // This will panic:
///  // let a: u8 = u8::MAX + u8::MAX;
///
///  // But this won't:
///  let b: u16 = u8::MAX as u16 + u8::MAX as u16;
/// ```
///
/// This trait, as well as the `Encapsulating*` traits, exist to make this easier.
/// The `encapsulating` macro also exists as a natural shorthand.
///
/// ```
/// # let b: u16 = u8::MAX as u16 + u8::MAX as u16;
/// #
/// use calm_ops::{EncapsulatingAdd, encapsulating};
///
/// let c: u16 = u8::MAX.encapsulating_add(u8::MAX);
///
/// let d: u16 = encapsulating!(u8::MAX + u8::MAX);
///
/// assert_eq!(b, c);
/// assert_eq!(c, d);
/// ```
///
/// [^operations]: addition, subtraction, multiplication, and negation
pub trait Encapsulate: Sized {
    type Super;

    fn encapsulate(self) -> Self::Super;
}

macro_rules! define_operation {
    ($trait_name: ident, $method:ident, $op:tt) => {
        paste! {
            pub trait [<Encapsulating $trait_name>]: Encapsulate
            where
                Self::Super: $trait_name<Output = Self::Super>
            {
                fn [<encapsulating_ $method>](self, rhs: Self) -> Self::Super {
                    #![allow(
                        clippy::arithmetic_side_effects,
                        reason = "Value is encapsulated in a large enough type",
                    )]
                    self.encapsulate() $op rhs.encapsulate()
                }
            }
        }
    };
}

define_operation!(Add, add, +);
define_operation!(Mul, mul, *);
define_operation!(Sub, sub, -);

pub trait EncapsulatingNeg: Encapsulate
where
    Self::Super: Neg<Output = Self::Super>,
{
    fn encapsulating_neg(self) -> Self::Super {
        #![allow(
            clippy::arithmetic_side_effects,
            reason = "Value is encapsulated in a large enough type"
        )]
        -self.encapsulate()
    }
}
