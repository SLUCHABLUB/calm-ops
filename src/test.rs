use crate::{checked, saturating, wrapping};
use paste::paste;
use proptest::property_test;

// TODO: remove
use crate as calm_ops;

macro_rules! test {
    // Negation.
    ($type:ident $prefix:ident neg) => {
        paste! {
            #[property_test]
            fn [<$prefix _neg_on_ $type>](operand: $type) {
                let using_macro = $prefix ! (-operand);
                let using_std = operand.[<$prefix _neg>]();

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
    // Shift operations.
    ($type:ident $prefix:ident $suffix:ident $operator:tt : shift) => {
        test!($type $prefix $suffix $operator u32);
    };

    // Heterogeneous binary operations.
    ($type:ident $prefix:ident $suffix:ident $operator:tt $operand:ident) => {
        paste! {
            #[property_test]
            fn [<$prefix _ $suffix _on_ $type>](left: $type, right: $operand) {
                let using_macro = $prefix ! (left $operator right);
                let using_std = left.[<$prefix _ $suffix>](right);

                assert_eq!(
                    using_macro,
                    using_std,
                );
            }
        }
    };

    // If no type is specified, test for each primitive integer.
    ($($token:tt)*) => {
        test!(i8    $($token)*);
        test!(i16   $($token)*);
        test!(i32   $($token)*);
        test!(i64   $($token)*);
        test!(i128  $($token)*);
        test!(isize $($token)*);

        test!(u8    $($token)*);
        test!(u16   $($token)*);
        test!(u32   $($token)*);
        test!(u64   $($token)*);
        test!(u128  $($token)*);
        test!(usize $($token)*);
    };
}

test!(checked add +);
test!(checked div /);
test!(checked mul *);
test!(checked rem %);
test!(checked sub -);
test!(checked neg);

test!(checked shl << : shift);
test!(checked shr >> : shift);

//TODO: test!(checked cast);

// TODO: Test encapsulating operations.

test!(saturating add +);
test!(saturating mul *);
test!(saturating sub -);
//TODO: test!(saturating neg);

//TODO: test!(saturating cast);

test!(wrapping add +);
test!(wrapping mul *);
test!(wrapping sub -);
test!(wrapping neg);

test!(wrapping shl << : shift);
test!(wrapping shr >> : shift);
