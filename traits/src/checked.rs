use crate::macros::{
    define_shift_trait, define_trait, for_float, for_primitive, for_signed,
    for_signed_and_unsigned, for_unsigned, implement_division_trait, implement_shift_trait,
    implement_trait,
};
use std::num::NonZero;

define_trait!(CheckedAdd, checked_add, Option<Self::Output>);
define_trait!(CheckedDiv, checked_div, Option<Self::Output>);
define_trait!(CheckedMul, checked_mul, Option<Self::Output>);
define_trait!(CheckedRem, checked_rem, Option<Self::Output>);
define_trait!(CheckedSub, checked_sub, Option<Self::Output>);

define_shift_trait!(CheckedShl, checked_shl, Option<Self::Output>);
define_shift_trait!(CheckedShr, checked_shr, Option<Self::Output>);

pub trait CheckedCast<Output> {
    fn checked_cast(self) -> Option<Output>;
}

pub trait CheckedNeg: Sized {
    type Output;

    #[must_use]
    fn checked_neg(self) -> Option<Self::Output>;
}

// TODO: Check if it is faster to just do the round-trip comparison for checked cast.
//       I.e. `((self as Output) as T == self).then_some(self as Output)`

for_signed_and_unsigned! {
    type T;

    implement_trait!(CheckedAdd, checked_add, T, T, Option<T>);
    implement_trait!(CheckedDiv, checked_div, T, T, Option<T>);
    implement_trait!(CheckedMul, checked_mul, T, T, Option<T>);
    implement_trait!(CheckedRem, checked_rem, T, T, Option<T>);
    implement_trait!(CheckedSub, checked_sub, T, T, Option<T>);

    implement_division_trait!(CheckedDiv, checked_div, T, T, Option<T>);
    implement_division_trait!(CheckedRem, checked_rem, T, T, Option<T>);

    implement_shift_trait!(CheckedShl, checked_shl, T, T, Option<T>);
    implement_shift_trait!(CheckedShr, checked_shr, T, T, Option<T>);

    impl CheckedNeg for T {
        type Output = T;

        fn checked_neg(self) -> Option::<Self::Output> {
            self.checked_neg()
        }
    }

    for_signed_and_unsigned! {
        type Output;

        impl CheckedCast<Output> for T {
            fn checked_cast(self) -> Option<Output> {
                #![allow(clippy::unnecessary_fallible_conversions)]

                Output::try_from(self).ok()
            }
        }
    }

    for_float! {
        type Output;

        impl CheckedCast<Output> for T {
            fn checked_cast(self) -> Option<Output> {
                #![allow(clippy::cast_precision_loss, clippy::cast_lossless)]

                if T::BITS <= Output::MANTISSA_DIGITS {
                    return Some(self as Output)
                }

                let max = (1 as T) << Output::MANTISSA_DIGITS;
                let min = if T::MIN == 0 { 0 } else { max.strict_neg() };

                (min <= self && self <= max).then_some(self as Output)
            }
        }
    }
}

for_float! {
    type T;

    for_primitive! {
        type Output;

        impl CheckedCast<Output> for T {
            fn checked_cast(self) -> Option<Output> {
                #![allow(
                    clippy::cast_lossless,
                    clippy::cast_possible_truncation,
                    clippy::cast_precision_loss,
                    clippy::cast_sign_loss,
                    clippy::float_cmp
                )]

                let cast = self as Output;
                let round_trip = cast as T;

                (round_trip == self).then_some(cast)
            }
        }
    }

}
