// TODO: make num create a feature
// TODO: implement for floats

use paste::paste;

trait Primitive: Copy {}

impl Primitive for f32 {}
impl Primitive for f64 {}

impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for i128 {}
impl Primitive for isize {}

impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for u128 {}
impl Primitive for usize {}

macro_rules! helper {
    // --- TRAIT IMPLEMENTATION ---

    (impl $trait_name:ident for $type:ident fn $method:ident ($($rhs_type:ty)?) -> $return_type:ty = $function:expr) => {
        impl $trait_name for $type {
            fn $method(self, $(rhs: $rhs_type)?) -> $return_type {
                let function: fn(Self, $($rhs_type)?) -> $return_type = $function;

                function(self, $(rhs as $rhs_type)?)
            }
        }
    };

    // --- NULL IMPLEMENTATION ---

    (
        impl $trait_name:ident
            fn $method:ident ($($rhs_type:ty)?) -> $return_type:ty
    ) => {};

    // --- IMPLEMENTATIONS FOR MULTIPLE TYPES ---

    (
        impl $trait_name:ident for i*
            fn $method:ident ($($rhs_type:ty)?) -> $return_type:ty = $function:expr
    ) => {
        helper!(impl $trait_name for i8    fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for i16   fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for i32   fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for i64   fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for i128  fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for isize fn $method ($($rhs_type)?) -> $return_type = $function);
    };
    (
        impl $trait_name:ident for u*
            fn $method:ident ($($rhs_type:ty)?) -> $return_type:ty = $function:expr
    ) => {
        helper!(impl $trait_name for u8    fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for u16   fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for u32   fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for u64   fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for u128  fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for usize fn $method ($($rhs_type)?) -> $return_type = $function);
    };
    (
        impl $trait_name:ident
            fn $method:ident ($($rhs_type:ty)?) -> $return_type:ty = $function:expr
    ) => {
        helper!(impl $trait_name for i* fn $method ($($rhs_type)?) -> $return_type = $function);
        helper!(impl $trait_name for u* fn $method ($($rhs_type)?) -> $return_type = $function);
    };

    // --- TRAIT DEFINING RULES ---

    // Assignment trait
    (#[assign] trait $trait_name:ident fn $method:ident ($rhs_type:ty) -> Self $(= $function:expr)?) => {
        helper!(trait $trait_name fn $method ($rhs_type) -> Self $(= $function)?);

        paste! {
            pub trait [<$trait_name Assign>] {
                fn [<$method _assign>](&mut self, rhs: $rhs_type);
            }

            impl<T: $trait_name + Primitive> [<$trait_name Assign>] for T {
                fn [<$method _assign>](&mut self, rhs: $rhs_type) {
                    *self = self.$method(rhs);
                }
            }
        }
    };
    // "Normal" trait
    (trait $trait_name:ident fn $method:ident ($($rhs_type:ty)?) -> $return_type:ty $(= $function:expr)?) => {
        pub trait $trait_name: Sized {
            #[must_use]
            fn $method(self, $(rhs: $rhs_type)?) -> $return_type;
        }

        helper!(impl $trait_name fn $method ($($rhs_type)?) -> $return_type $(= $function)?);
    };

    // --- PARAMETER DEFAULTS ---

    // Default rhs's type to `Self`
    ($(#[$meta:ident])? trait $trait_name:ident fn $method:ident $(-> $return_type:ty)? $(= $function:expr)?) => {
        helper!($(#[$meta])? trait $trait_name fn $method (Self) $(-> $return_type)? $(= $function)?);
    };
    // Default return type to `Self`
    ($(#[$meta:ident])? trait $trait_name:ident fn $method:ident ($($rhs_type:ty)?) $(= $function:expr)?) => {
        helper!($(#[$meta])? trait $trait_name fn $method ($($rhs_type)?) -> Self $(= $function)?);
    };
}

helper!(trait CheckedAdd fn checked_add       -> Option<Self> = Self::checked_add);
helper!(trait CheckedDiv fn checked_div       -> Option<Self> = Self::checked_div);
helper!(trait CheckedMul fn checked_mul       -> Option<Self> = Self::checked_mul);
helper!(trait CheckedRem fn checked_rem       -> Option<Self> = Self::checked_rem);
helper!(trait CheckedShl fn checked_shl (u32) -> Option<Self> = Self::checked_shl);
helper!(trait CheckedShr fn checked_shr (u32) -> Option<Self> = Self::checked_shr);
helper!(trait CheckedSub fn checked_sub       -> Option<Self> = Self::checked_sub);

helper!(trait CheckedNeg fn checked_neg () -> Option<Self> = Self::checked_neg);

pub trait CheckedCast<Target> {
    fn checked_cast(self) -> Option<Target>;
}

impl<Source, Target> CheckedCast<Target> for Source
where
    Source: TryInto<Target> + Primitive,
{
    fn checked_cast(self) -> Option<Target> {
        self.try_into().ok()
    }
}

helper!(#[assign] trait SaturatingAdd fn saturating_add = Self::saturating_add);
helper!(#[assign] trait SaturatingMul fn saturating_mul = Self::saturating_mul);
helper!(#[assign] trait SaturatingSub fn saturating_sub = Self::saturating_sub);
// TODO: Div && Rem

helper!(trait SaturatingNeg fn saturating_neg ());

helper!(impl SaturatingNeg for i* fn saturating_neg () -> Self = |this| this.checked_neg().unwrap_or(Self::MAX));
helper!(impl SaturatingNeg for u* fn saturating_neg () -> Self = |_| 0);

pub trait SaturatingCast<Target> {
    fn saturating_cast(self) -> Target;
}

// TODO: add generics support to the macro
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

helper!(#[assign] trait WrappingAdd fn wrapping_add = Self::wrapping_add);
helper!(#[assign] trait WrappingMul fn wrapping_mul = Self::wrapping_mul);
helper!(#[assign] trait WrappingSub fn wrapping_sub = Self::wrapping_sub);
// TODO: Div && Rem

helper!(#[assign] trait WrappingShl fn wrapping_shl (u32) = Self::wrapping_shl);
helper!(#[assign] trait WrappingShr fn wrapping_shr (u32) = Self::wrapping_shr);

helper!(trait WrappingNeg fn wrapping_neg () = Self::wrapping_neg);
// TODO: Cast
