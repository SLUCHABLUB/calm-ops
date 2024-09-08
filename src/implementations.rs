use crate::{Encapsulate, EncapsulatingAdd, EncapsulatingMul, EncapsulatingNeg, EncapsulatingSub};
use num::rational::Ratio;
use num::Integer;
use std::ops::Neg;

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
