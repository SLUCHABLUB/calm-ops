use crate::macros::{
    define_assign_trait, define_shift_assign_trait, define_shift_trait, define_trait, for_signed,
    for_signed_and_unsigned, for_unsigned, implement_assign_trait, implement_division_trait,
    implement_shift_assign_trait, implement_shift_trait, implement_trait,
};
use std::num::NonZero;

define_trait!(WrappingAdd, wrapping_add);
define_trait!(WrappingDiv, wrapping_div);
define_trait!(WrappingMul, wrapping_mul);
define_trait!(WrappingRem, wrapping_rem);
define_trait!(WrappingSub, wrapping_sub);

define_shift_trait!(WrappingShl, wrapping_shl);
define_shift_trait!(WrappingShr, wrapping_shr);

pub trait WrappingNeg<Rhs = Self> {
    type Output;

    #[must_use]
    fn wrapping_neg(self) -> Self::Output;
}

pub trait WrappingCast<Output> {
    #[must_use]
    fn wrapping_cast(self) -> Output;
}

define_assign_trait!(WrappingAddAssign, wrapping_add_assign);
define_assign_trait!(WrappingDivAssign, wrapping_div_assign);
define_assign_trait!(WrappingMulAssign, wrapping_mul_assign);
define_assign_trait!(WrappingRemAssign, wrapping_rem_assign);
define_assign_trait!(WrappingSubAssign, wrapping_sub_assign);

define_shift_assign_trait!(WrappingShlAssign, wrapping_shl_assign);
define_shift_assign_trait!(WrappingShrAssign, wrapping_shr_assign);

for_signed_and_unsigned! {
    type T;

    implement_trait!(WrappingAdd, wrapping_add, T);
    implement_trait!(WrappingMul, wrapping_mul, T);
    implement_trait!(WrappingSub, wrapping_sub, T);

    implement_division_trait!(WrappingDiv, wrapping_div, T);
    implement_division_trait!(WrappingRem, wrapping_rem, T);

    implement_shift_trait!(WrappingShl, wrapping_shl, T);
    implement_shift_trait!(WrappingShr, wrapping_shr, T);

    impl WrappingNeg for T {
        type Output = T;

        fn wrapping_neg(self) -> Self::Output {
            self.wrapping_neg()
        }
    }

    implement_assign_trait!(WrappingAddAssign, wrapping_add_assign, wrapping_add, T);
    implement_assign_trait!(WrappingMulAssign, wrapping_mul_assign, wrapping_mul, T);
    implement_assign_trait!(WrappingSubAssign, wrapping_sub_assign, wrapping_sub, T);

    implement_assign_trait!(WrappingDivAssign, wrapping_div_assign, WrappingDiv, wrapping_div, T, NonZero<T>);
    implement_assign_trait!(WrappingRemAssign, wrapping_rem_assign, WrappingRem, wrapping_rem, T, NonZero<T>);

    implement_shift_assign_trait!(WrappingShlAssign, wrapping_shl_assign, wrapping_shl, T);
    implement_shift_assign_trait!(WrappingShrAssign, wrapping_shr_assign, wrapping_shr, T);

    for_signed_and_unsigned! {
        type Output;

        impl WrappingCast<Output> for T {
            fn wrapping_cast(self) -> Output {
                #![allow(
                    clippy::cast_lossless,
                    clippy::cast_possible_truncation,
                    clippy::cast_possible_wrap,
                    clippy::cast_sign_loss,
                )]
                self as Output
            }
        }
    }
}
