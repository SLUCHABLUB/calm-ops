use std::ops::{Add, Mul, Neg, Sub};
use num::Integer;
use num::rational::Ratio;

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

macro_rules! define {
    ($trait_name: ident, $method:ident, $op:tt) => {
        paste::paste! {
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

define!(Add, add, +);
define!(Mul, mul, *);
define!(Sub, sub, -);

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


macro_rules! impl_unsigned {
    ($sub:ty: $sup:ty) => {
        impl Encapsulate for $sub {
            type Super = $sup;

            fn encapsulate(self) -> Self::Super {
                self.into()
            }
        }

        impl EncapsulatingAdd for $sub {}
        impl EncapsulatingMul for $sub {}
    };
}

macro_rules! impl_signed {
    ($sub:ty: $sup:ty) => {
        impl_unsigned!($sub: $sup);
        impl EncapsulatingSub for $sub {}
        impl EncapsulatingNeg for $sub {}
    };
}

impl_unsigned!(u8: u16);
impl_unsigned!(u16: u32);
impl_unsigned!(u32: u64);

impl_signed!(i8: i16);
impl_signed!(i16: i32);
impl_signed!(i32: i64);

#[cfg(feature = "i128")]
impl_unsigned!(u64: u128);
#[cfg(feature = "i128")]
impl_signed!(i64: i128);

#[cfg(feature = "big-int")]
impl_unsigned!(u128: num::BigUint);
#[cfg(feature = "big-int")]
impl_signed!(i128: num::BigInt);

impl<T: Encapsulate + Clone + Integer> Encapsulate for Ratio<T>
where
    T::Super: Clone + Integer,
{
    type Super = Ratio<T::Super>;

    fn encapsulate(self) -> Self::Super {
        let (numerator, denominator) = self.into_raw();
        Ratio::new(numerator.encapsulate(), denominator.encapsulate())
    }
}

impl<T: Encapsulate + Clone + Integer> EncapsulatingAdd for Ratio<T> where T::Super: Clone + Integer {}
impl<T: Encapsulate + Clone + Integer> EncapsulatingMul for Ratio<T> where T::Super: Clone + Integer {}
impl<T: Encapsulate + Clone + Integer> EncapsulatingSub for Ratio<T> where T::Super: Clone + Integer {}

impl<T: Encapsulate + Clone + Integer + Neg<Output = T>> EncapsulatingNeg for Ratio<T> where
    T::Super: Clone + Integer + Neg<Output = T::Super>
{
}
