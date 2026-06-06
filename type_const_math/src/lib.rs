#![no_std]
#![forbid(unsafe_code)]

#![feature( //
    generic_const_args, 
    min_generic_const_args,
    generic_const_items
)]
#![allow(incomplete_features)]

macro_rules! if_unsigned {
    (u8, $($tt:tt)+) => {$($tt)+};
    (u16, $($tt:tt)+) => {$($tt)+};
    (u32, $($tt:tt)+) => {$($tt)+};
    (u64, $($tt:tt)+) => {$($tt)+};
    (u128, $($tt:tt)+) => {$($tt)+};

    (i8, $($tt:tt)+) => { /* nothing, i8 is signed */ };
    (i16, $($tt:tt)+) => { /* nothing, i16 is signed*/ };
    (i32, $($tt:tt)+) => { /* nothing, i32 is signed*/ };
    (i64, $($tt:tt)+) => { /* nothing, i64 is signed*/ };
    (i128, $($tt:tt)+) => { /* nothing, i128 is signed*/ };
}

macro_rules! if_signed {
    (u8, $($tt:tt)+) => { /* nothing, u8 is unsigned */  };
    (u16, $($tt:tt)+) => { /* nothing, u16 is unsigned */ };
    (u32, $($tt:tt)+) => { /* nothing, u32 is unsigned */ };
    (u64, $($tt:tt)+) => { /* nothing, u64 is unsigned */ };
    (u128, $($tt:tt)+) => { /* nothing, u128 is unsigned */ };

    (i8, $($tt:tt)+) => { $($tt)+};
    (i16, $($tt:tt)+) => { $($tt)+};
    (i32, $($tt:tt)+) => { $($tt)+};
    (i64, $($tt:tt)+) => { $($tt)+};
    (i128, $($tt:tt)+) => { $($tt)+};
}

macro_rules! impl_math_common {
    ($([for_ty:[$ty:ty,$ty_ident:ident], ty_unsigned: $ty_unsigned:ty],)*) => {$(
        pub mod $ty_ident {

            pub type const ADD<const L: $ty, const R: $ty>: $ty = const { L + R };
            pub type const SUB<const L: $ty, const R: $ty>: $ty = const { L - R };
            pub type const MUL<const L: $ty, const R: $ty>: $ty = const { L * R };
            pub type const DIV<const L: $ty, const R: $ty>: $ty = const { L / R };

            pub type const ABS_DIFF<const L: $ty, const R: $ty>: $ty_unsigned = const { <$ty>::abs_diff(L, R) };

            // CHECKED_* omitted, since they all return Option<_>, which doesn't impl ConstParamTy

            pub type const COUNT_ONES<const N: $ty>: u32 = const { <$ty>::count_ones(N) };

            pub type const COUNT_ZEROS<const N: $ty>: u32 = const { <$ty>::count_zeros(N) };

            pub type const DIV_EUCLID<const L: $ty, const R: $ty>: $ty = const { <$ty>::div_euclid(L, R) };

            // DIV_EXACT omitted, returns Option<_>

            // EXTEND omitted, idk how to do this atm

            // FORMAT_INTO omitted, doesn't apply

        
        }
    )*};
}

impl_math_common!(
    [for_ty: [u8, u8], ty_unsigned: u8],
    [for_ty: [u16, u16], ty_unsigned: u16],
    [for_ty: [u32, u32], ty_unsigned: u32],
    [for_ty: [u64, u64], ty_unsigned: u64],
    [for_ty: [u128, u128], ty_unsigned: u128],
    [for_ty: [i8, i8], ty_unsigned: u8],
    [for_ty: [i16, i16], ty_unsigned: u16],
    [for_ty: [i32, i32], ty_unsigned: u32],
    [for_ty: [i64, i64], ty_unsigned: u64],
    [for_ty: [i128, i128], ty_unsigned: u128],
);

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    
    use std::println;

    #[test]
    fn it_works() {
        println!("gay")
    }
}