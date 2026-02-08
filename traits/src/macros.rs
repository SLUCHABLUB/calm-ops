macro_rules! define_trait {
    ($name:ident, $method:ident) => {
        define_trait!($name, $method, Self::Output);
    };
    ($name:ident, $method:ident, $return_type:path) => {
        pub trait $name<Rhs = Self>: Sized {
            type Output;

            #[must_use]
            fn $method(self, rhs: Rhs) -> $return_type;
        }
    };
}

macro_rules! define_shift_trait {
    ($name:ident, $method:ident) => {
        define_shift_trait!($name, $method, Self::Output);
    };
    ($name:ident, $method:ident, $return_type:path) => {
        pub trait $name: Sized {
            type Output;

            #[must_use]
            fn $method(self, rhs: u32) -> $return_type;
        }
    };
}

macro_rules! define_assign_trait {
    ($name:ident, $method:ident) => {
        pub trait $name<Rhs = Self> {
            fn $method(&mut self, rhs: Rhs);
        }
    };
}

macro_rules! define_shift_assign_trait {
    ($name:ident, $method:ident) => {
        pub trait $name: Sized {
            fn $method(&mut self, rhs: u32);
        }
    };
}

macro_rules! implement_trait {
    ($name:ident, $method:ident, $T:ident) => {
        implement_trait!($name, $method, $T, $T);
    };
    ($name:ident, $method:ident, $T:ident, $Output:path) => {
        implement_trait!($name, $method, $T, $Output, $Output);
    };
    ($name:ident, $method:ident, $T:ident, $Output:path, $return_type:path) => {
        impl $name for $T {
            type Output = $Output;

            #[inline]
            fn $method(self, rhs: $T) -> $return_type {
                self.$method(rhs)
            }
        }
    };
}

macro_rules! implement_shift_trait {
    ($name:ident, $method:ident, $T:ident) => {
        implement_shift_trait!($name, $method, $T, $T, $T);
    };
    ($name:ident, $method:ident, $T:ident, $Output:path, $return_type:path) => {
        impl $name for $T {
            type Output = $Output;

            #[inline]
            fn $method(self, rhs: u32) -> $return_type {
                self.$method(rhs)
            }
        }
    };
}

macro_rules! implement_division_trait {
    ($name:ident, $method:ident, $T:ident) => {
        implement_division_trait!($name, $method, $T, $T, $T);
    };
    ($name:ident, $method:ident, $T:ident, $Output:path, $return_type:path) => {
        impl $name<NonZero<$T>> for $T {
            type Output = $Output;

            #[inline]
            fn $method(self, rhs: NonZero<$T>) -> $return_type {
                #![allow(
                    clippy::arithmetic_side_effects,
                    reason = "false positive: rhs is non-zero"
                )]
                self.$method(rhs.get())
            }
        }
    };
}

macro_rules! implement_assign_trait {
    ($name:ident, $method:ident, $non_asign_method:ident, $T:ident) => {
        implement_assign_trait!($name, $method, $non_asign_method, $T, $T);
    };
    ($name:ident, $method:ident, $non_asign_method:ident, $T:ident, $Rhs:path) => {
        impl $name<$Rhs> for $T {
            #[inline]
            fn $method(&mut self, rhs: $Rhs) {
                *self = self.$non_asign_method(rhs);
            }
        }
    };
    ($name:ident, $method:ident, $non_asign_trait:ident, $non_asign_method:ident, $T:ident, $Rhs:path) => {
        impl $name<$Rhs> for $T {
            #[inline]
            fn $method(&mut self, rhs: $Rhs) {
                *self = $non_asign_trait::$non_asign_method(*self, rhs);
            }
        }
    };
}

macro_rules! implement_shift_assign_trait {
    ($name:ident, $method:ident, $non_asign_method:ident, $T:ident) => {
        impl $name for $T {
            #[inline]
            fn $method(&mut self, rhs: u32) {
                *self = self.$non_asign_method(rhs);
            }
        }
    };
}

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

macro_rules! for_float {
    (type $T:ident; $($tt:tt)+) => {
        mod __for_float_f32 { use super::*; type $T = f32; $($tt)+ }
        mod __for_float_f64 { use super::*; type $T = f64; $($tt)+ }
    };
}

macro_rules! for_primitive {
    (type $T:ident; $($tt:tt)+) => {
        mod __for_primitive {
            use super::*;

            for_signed_and_unsigned!(type $T; $($tt)+);
            for_float!(type $T; $($tt)+);
        }
    };
}

pub(super) use {
    define_assign_trait, define_shift_assign_trait, define_shift_trait, define_trait, for_float,
    for_primitive, for_signed, for_signed_and_unsigned, for_unsigned, implement_assign_trait,
    implement_division_trait, implement_shift_assign_trait, implement_shift_trait, implement_trait,
};
