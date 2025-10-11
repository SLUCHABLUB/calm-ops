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

pub trait CheckedCast<Target>: TryInto<Target> {
    fn checked_cast(self) -> Option<Target>;
}

impl<Source, Target> CheckedCast<Target> for Source
where
    Source: TryInto<Target>,
{
    fn checked_cast(self) -> Option<Target> {
        self.try_into().ok()
    }
}

define!(SaturatingAdd, saturating_add);
define!(SaturatingMul, saturating_mul);
define!(SaturatingSub, saturating_sub);

pub trait SaturatingNeg: CheckedNeg {
    fn saturating_neg(self) -> Self;
}

// This implementation only works for 2's complement based .
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

pub trait SaturatingCast<Target> {
    fn saturating_cast(self) -> Target;
}

macro_rules! implement_saturating_cast {
    ($source:ty) => {
        implement_saturating_cast!($source as u8);
        implement_saturating_cast!($source as u16);
        implement_saturating_cast!($source as u32);
        implement_saturating_cast!($source as u64);
        implement_saturating_cast!($source as u128);
        implement_saturating_cast!($source as usize);

        implement_saturating_cast!($source as i8);
        implement_saturating_cast!($source as i16);
        implement_saturating_cast!($source as i32);
        implement_saturating_cast!($source as i64);
        implement_saturating_cast!($source as i128);
        implement_saturating_cast!($source as isize);
    };
    ($source:ty as $target:ty) => {
        impl SaturatingCast<$target> for $source {
            fn saturating_cast(self) -> $target {
                #![allow(unused_comparisons)]

                if <$source>::MIN == 0 {
                    //  Converting `MIN` will never fail.
                    return self.checked_cast().unwrap_or(<$target>::MAX);
                }

                let saturation_point = if self < 0 {
                    <$target>::MIN
                } else {
                    <$target>::MAX
                };

                self.checked_cast().unwrap_or(saturation_point)
            }
        }
    };
}

implement_saturating_cast!(u8);
implement_saturating_cast!(u16);
implement_saturating_cast!(u32);
implement_saturating_cast!(u64);
implement_saturating_cast!(u128);
implement_saturating_cast!(usize);

implement_saturating_cast!(i8);
implement_saturating_cast!(i16);
implement_saturating_cast!(i32);
implement_saturating_cast!(i64);
implement_saturating_cast!(i128);
implement_saturating_cast!(isize);

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
