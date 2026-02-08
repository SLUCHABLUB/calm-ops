use crate::macros::{
    define_assign_trait, define_shift_trait, define_trait, for_float, for_primitive, for_signed,
    for_signed_and_unsigned, for_unsigned, implement_assign_trait, implement_division_trait,
    implement_trait,
};
use std::num::NonZero;

define_trait!(SaturatingAdd, saturating_add);
define_trait!(SaturatingDiv, saturating_div);
define_trait!(SaturatingMul, saturating_mul);
define_trait!(SaturatingRem, saturating_rem);
define_trait!(SaturatingSub, saturating_sub);

define_shift_trait!(SaturatingShl, saturating_shl);
define_shift_trait!(SaturatingShr, saturating_shr);

// TODO: Figure out how should we handle shift operators.

pub trait SaturatingCast<Output> {
    fn saturating_cast(self) -> Output;
}

pub trait SaturatingNeg: Sized {
    type Output;

    fn saturating_neg(self) -> Self::Output;
}

define_assign_trait!(SaturatingAddAssign, saturating_add_assign);
define_assign_trait!(SaturatingDivAssign, saturating_div_assign);
define_assign_trait!(SaturatingMulAssign, saturating_mul_assign);
define_assign_trait!(SaturatingRemAssign, saturating_rem_assign);
define_assign_trait!(SaturatingSubAssign, saturating_sub_assign);

// TODO: Add shift assign operators if we add shift operators.

for_signed_and_unsigned! {
    type T;

    implement_trait!(SaturatingAdd, saturating_add, T);
    implement_trait!(SaturatingMul, saturating_mul, T);
    implement_trait!(SaturatingSub, saturating_sub, T);

    implement_division_trait!(SaturatingDiv, saturating_div, T);

    impl SaturatingRem<NonZero<T>> for T {
        type Output = T;

        fn saturating_rem(self, rhs: NonZero<T>) -> T {
            #![allow(clippy::arithmetic_side_effects, reason = "false positive: rhs is non-zero")]
            // The only problem case for % is `rhs == 0` and `rhs == -1 && self == MIN`.
            // `NonZero` takes care of the former and `wrapping_rem` gives the correct answer for the latter.
            self.wrapping_rem(rhs.get())
        }
    }

    implement_assign_trait!(SaturatingAddAssign, saturating_add_assign, saturating_add, T);
    implement_assign_trait!(SaturatingMulAssign, saturating_mul_assign, saturating_mul, T);
    implement_assign_trait!(SaturatingSubAssign, saturating_sub_assign, saturating_sub, T);

    implement_assign_trait!(SaturatingDivAssign, saturating_div_assign, SaturatingDiv, saturating_div, T, NonZero<T>);
    implement_assign_trait!(SaturatingRemAssign, saturating_rem_assign, SaturatingRem, saturating_rem, T, NonZero<T>);

    for_signed_and_unsigned! {
        type Output;

        impl SaturatingCast<Output> for T {
            fn saturating_cast(self) -> Output {
                #![allow(
                    clippy::absurd_extreme_comparisons,
                    clippy::unnecessary_fallible_conversions,
                    unused_comparisons,
                )]
                let saturation_point = if self < 0 { Output::MIN } else { Output::MAX };

                Output::try_from(self).unwrap_or(saturation_point)
            }
        }
    }
}

for_signed! {
    type T;

    impl SaturatingNeg for T {
        type Output = T;

        fn saturating_neg(self) -> T {
            self.saturating_neg()
        }
    }
}

for_unsigned! {
    type T;

    impl SaturatingNeg for T {
        type Output = T;

        fn saturating_neg(self) -> T {
            0
        }
    }
}

for_float! {
    type T;

    for_signed_and_unsigned! {
        type Output;

        impl SaturatingCast<Output> for T {
            fn saturating_cast(self) -> Output {
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                )]
                // `as` cast from floats saturate.
                self as Output
            }
        }
    }
}

for_primitive! {
    type T;

    for_float! {
        type Output;

        impl SaturatingCast<Output> for T {
            fn saturating_cast(self) -> Output {
                #![allow(
                    clippy::cast_lossless,
                    clippy::cast_possible_truncation,
                    clippy::cast_precision_loss,
                )]
                // `as` cast to floats saturate.
                self as Output
            }
        }
    }
}
