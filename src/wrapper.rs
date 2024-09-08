use std::ops::Neg;

macro_rules! define {
    (impl $trait_name:ident) => {
        impl<T: num::traits::$trait_name> $trait_name for T {}
    };
    ($trait_name:ident, $method:ident -> $return_type:ty) => {
        pub trait $trait_name: num::traits::$trait_name {
            fn $method(self, rhs: Self) -> $return_type {
                num::traits::$trait_name::$method(&self, &rhs)
            }
        }

        define!(impl $trait_name);
    };
    ($trait_name:ident, $method:ident -> $return_type:ty : shift) => {
        pub trait $trait_name: num::traits::$trait_name {
            fn $method(self, rhs: u32) -> $return_type {
                num::traits::$trait_name::$method(&self, rhs)
            }
        }

        define!(impl $trait_name);
    };
    ($trait_name:ident, $method:ident $(: $shift:tt)?) => {
        define!($trait_name, $method -> Self $(: $shift)?);
    }
}

define!(CheckedAdd, checked_add -> Option<Self>);
define!(CheckedDiv, checked_div -> Option<Self>);
define!(CheckedMul, checked_mul -> Option<Self>);
define!(CheckedRem, checked_rem -> Option<Self>);
define!(CheckedShl, checked_shl -> Option<Self> : shift);
define!(CheckedShr, checked_shr -> Option<Self> : shift);
define!(CheckedSub, checked_sub -> Option<Self>);

pub trait CheckedNeg: num::traits::CheckedNeg {
    fn checked_neg(self) -> Option<Self> {
        num::traits::CheckedNeg::checked_neg(&self)
    }
}

impl<T: num::traits::CheckedNeg> CheckedNeg for T {}

define!(SaturatingAdd, saturating_add);
define!(SaturatingMul, saturating_mul);
define!(SaturatingSub, saturating_sub);

pub trait SaturatingNeg: Neg<Output = Self> {
    fn saturating_neg(self) -> Self;
}

macro_rules! impl_saturating_neg {
    ($ty:ty) => {
        impl SaturatingNeg for $ty {
            fn saturating_neg(self) -> Self {
                // `checked_neg` only fails `self` is `Self::MIN`, in which case we saturate to `Self::MAX`
                self.checked_neg().unwrap_or(Self::MAX)
            }
        }
    };
}

impl_saturating_neg!(i8);
impl_saturating_neg!(i16);
impl_saturating_neg!(i32);
impl_saturating_neg!(i64);
impl_saturating_neg!(i128);
impl_saturating_neg!(isize);

define!(WrappingAdd, wrapping_add);
define!(WrappingMul, wrapping_mul);
define!(WrappingSub, wrapping_sub);

define!(WrappingShl, wrapping_shl : shift);
define!(WrappingShr, wrapping_shr : shift);

pub trait WrappingNeg: num::traits::WrappingNeg {
    fn wrapping_neg(self) -> Self {
        num::traits::WrappingNeg::wrapping_neg(&self)
    }
}

impl<T: num::traits::WrappingNeg> WrappingNeg for T {}