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

macro_rules! impl_math {
    ($([for_ty:[$ty:ty,$ty_ident:ident], ty_unsigned: $ty_unsigned:ty, ty_signed: $ty_signed:ty],)*) => {$(
        pub mod $ty_ident {

            pub type const ADD<const L: $ty, const R: $ty>: $ty = const { L + R };
            pub type const SUB<const L: $ty, const R: $ty>: $ty = const { L - R };
            pub type const MUL<const L: $ty, const R: $ty>: $ty = const { L * R };
            pub type const DIV<const L: $ty, const R: $ty>: $ty = const { L / R };

            pub type const ABS_DIFF<const L: $ty, const R: $ty>: $ty_unsigned = const { <$ty>::abs_diff(L, R) };

            if_unsigned!{ $ty_ident,
            pub type const CAST_SIGNED<const N: $ty>: $ty_signed = const { <$ty>::cast_signed(N) };
            }

            // CHECKED_* omitted, since they all return Option<_>, which doesn't impl ConstParamTy
            
            pub type const COUNT_ONES<const N: $ty>: u32 = const { <$ty>::count_ones(N) };
            pub type const COUNT_ZEROS<const N: $ty>: u32 = const { <$ty>::count_zeros(N) };

            pub type const DIV_EUCLID<const L: $ty, const R: $ty>: $ty = const { <$ty>::div_euclid(L, R) };

            pub type const ILOG<const N: $ty, const BASE: $ty>: u32 = const { <$ty>::ilog(N, BASE) };
            pub type const ILOG2<const N: $ty>: u32 = const { <$ty>::ilog2(N) };
            pub type const ILOG10<const N: $ty>: u32 = const { <$ty>::ilog10(N) };

            if_unsigned!{ $ty_ident,
            pub type const IS_MULTIPLE_OF<const L: $ty, const R: $ty>: bool = const { <$ty>::is_multiple_of(L, R) };
            pub type const IS_POWER_OF_TWO<const N: $ty>: bool = const { <$ty>::is_power_of_two(N) };
            }

            pub type const ISQRT<const N: $ty>: $ty = const { <$ty>::isqrt(N) };

            pub type const LEADING_ONES<const N: $ty>: u32 = const { <$ty>::leading_ones(N) };
            pub type const LEADING_ZEROS<const N: $ty>: u32 = const { <$ty>::leading_zeros(N) };

            pub type const MIDPOINT<const L: $ty, const R: $ty>: $ty = const { <$ty>::midpoint(L, R) };

            if_unsigned!{ $ty_ident,
            pub type const NEXT_MULTIPLE_OF<const L: $ty, const R: $ty>: $ty = const { <$ty>::next_multiple_of(L, R) };

            pub type const NEXT_POWER_OF_TWO<const N: $ty>: $ty = const { <$ty>::next_power_of_two(N) };
            }

            pub type const OVERFLOWING_ADD<const L: $ty, const R: $ty>: ($ty, bool) = const { <$ty>::overflowing_add(L, R) };
            if_unsigned!{ $ty_ident,
            pub type const OVERFLOWING_ADD_SIGNED<const L: $ty, const R: $ty_signed>: ($ty, bool) = const { <$ty>::overflowing_add_signed(L, R) };
            }
            pub type const OVERFLOWING_DIV<const L: $ty, const R: $ty>: ($ty, bool) = const { <$ty>::overflowing_div(L, R) };
            pub type const OVERFLOWING_DIV_EUCLID<const L: $ty, const R: $ty>: ($ty, bool) = const { <$ty>::overflowing_div_euclid(L, R) };
            pub type const OVERFLOWING_MUL<const L: $ty, const R: $ty>: ($ty, bool) = const { <$ty>::overflowing_mul(L, R) };
            pub type const OVERFLOWING_NEG<const N: $ty>: ($ty, bool) = const { <$ty>::overflowing_neg(N) };
            pub type const OVERFLOWING_POW<const N: $ty, const POW: u32>: ($ty, bool) = const { <$ty>::overflowing_pow(N, POW) };
            pub type const OVERFLOWING_REM<const L: $ty, const R: $ty>: ($ty, bool) = const { <$ty>::overflowing_rem(L, R) };
            pub type const OVERFLOWING_REM_EUCLID<const L: $ty, const R: $ty>: ($ty, bool) = const { <$ty>::overflowing_rem_euclid(L, R) };
            pub type const OVERFLOWING_SHL<const L: $ty, const R: u32>: ($ty, bool) = const { <$ty>::overflowing_shl(L, R) };
            pub type const OVERFLOWING_SHR<const L: $ty, const R: u32>: ($ty, bool) = const { <$ty>::overflowing_shr(L, R) };
            pub type const OVERFLOWING_SUB<const L: $ty, const R: $ty>: ($ty, bool) = const { <$ty>::overflowing_sub(L, R) };
            if_unsigned!{ $ty_ident,
            pub type const OVERFLOWING_SUB_SIGNED<const L: $ty, const R: $ty_signed>: ($ty, bool) = const { <$ty>::overflowing_sub_signed(L, R) };
            }

            pub type const POW<const L: $ty, const R: u32>: $ty = const { <$ty>::pow(L, R) };

            pub type const REM_EUCLID<const L: $ty, const R: $ty>: $ty = const { <$ty>::rem_euclid(L, R) };

            pub type const REVERSE_BITS<const N: $ty>: $ty = const { <$ty>::reverse_bits(N) };

            pub type const ROTATE_LEFT<const L: $ty, const R: u32>: $ty = const { <$ty>::rotate_left(L, R) };
            pub type const ROTATE_RIGHT<const L: $ty, const R: u32>: $ty = const { <$ty>::rotate_right(L, R) };
            
            pub type const SATURATING_ADD<const L: $ty, const R: $ty>: $ty = const { <$ty>::saturating_add(L, R) };
            if_unsigned!{ $ty_ident,
            pub type const SATURATING_ADD_SIGNED<const L: $ty, const R: $ty_signed>: $ty = const { <$ty>::saturating_add_signed(L, R) };
            }
            pub type const SATURATING_DIV<const L: $ty, const R: $ty>: $ty = const { <$ty>::saturating_div(L, R) };
            pub type const SATURATING_MUL<const L: $ty, const R: $ty>: $ty = const { <$ty>::saturating_mul(L, R) };
            pub type const SATURATING_POW<const L: $ty, const R: u32>: $ty = const { <$ty>::saturating_pow(L, R) };
            pub type const SATURATING_SUB<const L: $ty, const R: $ty>: $ty = const { <$ty>::saturating_mul(L, R) };
            if_unsigned!{ $ty_ident,
            pub type const SATURATING_SUB_SIGNED<const L: $ty, const R: $ty_signed>: $ty = const { <$ty>::saturating_sub_signed(L, R) };
            }
        }
    )*};
}

impl_math!(
    [for_ty: [u8, u8],      ty_unsigned: u8,    ty_signed: i8],
    [for_ty: [u16, u16],    ty_unsigned: u16,   ty_signed: i16],
    [for_ty: [u32, u32],    ty_unsigned: u32,   ty_signed: i32],
    [for_ty: [u64, u64],    ty_unsigned: u64,   ty_signed: i64],
    [for_ty: [u128, u128],  ty_unsigned: u128,  ty_signed: i128],
    [for_ty: [i8, i8],      ty_unsigned: u8,    ty_signed: i8],
    [for_ty: [i16, i16],    ty_unsigned: u16,   ty_signed: i16],
    [for_ty: [i32, i32],    ty_unsigned: u32,   ty_signed: i32],
    [for_ty: [i64, i64],    ty_unsigned: u64,   ty_signed: i64],
    [for_ty: [i128, i128],  ty_unsigned: u128,  ty_signed: i128],
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