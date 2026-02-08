#![allow(
    clippy::arithmetic_side_effects,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use crate::{checked, saturating, wrapping};
use paste::paste;
use proptest::property_test;
use std::num::NonZero;

// TODO: remove
use crate as calm_ops;

// These are copied from the traits crate.
// TODO: Move these to a separate crate.

macro_rules! for_signed {
    (type $T:ident; $($tt:tt)+) => {
        mod __for_signed_i8    { use super::*; type $T = i8;    $($tt)+ }
        mod __for_signed_i16   { use super::*; type $T = i16;   $($tt)+ }
        mod __for_signed_i32   { use super::*; type $T = i32;   $($tt)+ }
        mod __for_signed_i64   { use super::*; type $T = i64;   $($tt)+ }
        mod __for_signed_i128  { use super::*; type $T = i128;  $($tt)+ }
        mod __for_signed_isize { use super::*; type $T = isize; $($tt)+ }
    };
}

macro_rules! for_unsigned {
    (type $T:ident; $($tt:tt)+) => {
        mod __for_unsigned_u8    { use super::*; type $T = u8;    $($tt)+ }
        mod __for_unsigned_u16   { use super::*; type $T = u16;   $($tt)+ }
        mod __for_unsigned_u32   { use super::*; type $T = u32;   $($tt)+ }
        mod __for_unsigned_u64   { use super::*; type $T = u64;   $($tt)+ }
        mod __for_unsigned_u128  { use super::*; type $T = u128;  $($tt)+ }
        mod __for_unsigned_usize { use super::*; type $T = usize; $($tt)+ }
    };
}

macro_rules! for_signed_and_unsigned {
    (type $T:ident; $($tt:tt)+) => {
        mod __for_signed_and_unsigned {
            use super::*;

            for_signed!(type $T; $($tt)+);
            for_unsigned!(type $T; $($tt)+);
        }
    };
}

macro_rules! test {
    // Negation.
    ($type:ident $prefix:ident neg) => {
        paste! {
            #[property_test]
            fn [<$prefix _neg>](operand: $type) {
                let using_macro = $prefix ! (-operand);
                let using_std = operand.[<$prefix _neg>]();

                assert_eq!(
                    using_macro,
                    using_std,
                );
            }
        }
    };

    // Division.
    ($type:ident $prefix:ident $suffix:ident $operator:tt NonZero) => {
        paste! {
            #[property_test]
            fn [<$prefix _ $suffix _non_zero>](left: $type, right: NonZero<$type>) {
                let using_macro = $prefix ! (left $operator right);
                let using_std = left.[<$prefix _ $suffix>](right.get());

                assert_eq!(
                    using_macro,
                    using_std,
                );
            }
        }
    };

    // Homogenous binary operations.
    ($type:ident $prefix:ident $suffix:ident $operator:tt) => {
        test!($type $prefix $suffix $operator $type);
    };

    // Heterogeneous binary operations.
    ($type:ident $prefix:ident $suffix:ident $operator:tt $operand:path) => {
        paste! {
            #[property_test]
            fn [<$prefix _ $suffix>](left: $type, right: $operand) {
                let using_macro = $prefix ! (left $operator right);
                let using_std = left.[<$prefix _ $suffix>](right);

                assert_eq!(
                    using_macro,
                    using_std,
                );
            }
        }
    };
}

use test;

for_signed_and_unsigned! {
    type T;

    test!(T checked add +);
    test!(T checked div /);
    test!(T checked mul *);
    test!(T checked rem %);
    test!(T checked sub -);
    test!(T checked neg);

    test!(T checked shl << u32);
    test!(T checked shr >> u32);

    // TODO: Test encapsulating operations.

    test!(T saturating add +);
    test!(T saturating mul *);
    test!(T saturating sub -);

    // TODO: Test shift operations when they get added.

    test!(T wrapping add +);
    test!(T wrapping mul *);
    test!(T wrapping sub -);
    test!(T wrapping neg);

    test!(T wrapping shl << u32);
    test!(T wrapping shr >> u32);

    for_signed_and_unsigned! {
        type U;

        #[property_test]
        fn checked_cast(value: T) {
            let primitive_output = value as U;
            let checked_output = checked!(value as U);

            if let Some(checked_output) = checked_output {
                // Cast should be exact.

                assert_eq!(primitive_output, checked_output);

                let checked_roundtrip = checked!(checked_output as T);

                assert_eq!(checked_roundtrip, Some(value));
            } else {
                // Cast should fail.

               assert_ne!(primitive_output.to_string(), value.to_string());
            }
        }

        // TODO: Test saturating cast.
        // TODO: Test wrapping cast.
    }

    #[property_test]
    fn neg_equals_zero_sub(value: T) {
        assert_eq!(checked!(-value), checked!(0 - value));
        assert_eq!(saturating!(-value), saturating!(0 - value));
        assert_eq!(wrapping!(-value), wrapping!(0 - value));
    }

    mod division {
        use super::*;

        test!(T checked div / NonZero);
        test!(T checked rem % NonZero);

        // TODO: test saturating div && rem.

        test!(T wrapping div / NonZero);
        test!(T wrapping rem % NonZero);
    }
}

// TODO: Test floats.
// TODO: Test encapsulating ops.
