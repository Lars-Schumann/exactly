#![feature(
    //
    generic_const_args, 
    min_generic_const_args, 
    generic_const_items,

    signed_bigint_helpers,
    const_unsigned_bigint_helpers,
    uint_carryless_mul,
)]
#![allow(incomplete_features)]

macro_rules! only_if_ty_unsigned {
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

macro_rules! only_if_ty_is_signed {
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

macro_rules! match_ty_signdness {
    (u8, unsigned => $tt1:item, signed => $tt2:item) => { $tt1  };
    (u16, unsigned => $tt1:item, signed => $tt2:item) => { $tt1 };
    (u32, unsigned => $tt1:item, signed => $tt2:item) => { $tt1 };
    (u64, unsigned => $tt1:item, signed => $tt2:item) => { $tt1 };
    (u128, unsigned => $tt1:item, signed => $tt2:item) => { $tt1 };

    (i8, unsigned => $tt1:item, signed => $tt2:item) => { $tt2  };
    (i16, unsigned => $tt1:item, signed => $tt2:item) => { $tt2 };
    (i32, unsigned => $tt1:item, signed => $tt2:item) => { $tt2 };
    (i64, unsigned => $tt1:item, signed => $tt2:item) => { $tt2 };
    (i128, unsigned => $tt1:item, signed => $tt2:item) => { $tt2 };
}

macro_rules! impl_math_common {
    ($([for_ty:[$ty:ty,$ty_ident:ident], ty_unsigned: $ty_unsigned:ty, dummy_struct_name: $dummy_struct_name:ident],)*) => {$(
        pub struct $dummy_struct_name;

        impl $dummy_struct_name {
            pub type const ADD<const L: $ty, const R: $ty>: $ty = const { L + R };
            pub type const SUB<const L: $ty, const R: $ty>: $ty = const { L - R };
            pub type const MUL<const L: $ty, const R: $ty>: $ty = const { L * R };
            pub type const DIV<const L: $ty, const R: $ty>: $ty = const { L / R };
            
            pub type const ABS_DIFF<const L: $ty, const R: $ty>: $ty_unsigned = const { <$ty>::abs_diff(L, R) }; 
            
            only_if_ty_unsigned!{ $ty_ident, 
                pub type const BIT_WIDTH<const N: $ty>: u32 = const { <$ty>::bit_width(N) };
            }

            pub type const BORROWING_SUB<const L: $ty, const R: $ty, const BORROW: bool>: ($ty, bool) = const { <$ty>::borrowing_sub(L, R, BORROW) }; 

            pub type const CARRYING_ADD<const L: $ty, const R: $ty, const CARRY: bool>: ($ty, bool) = const { <$ty>::carrying_add(L, R, CARRY) }; 

            only_if_ty_unsigned!{ $ty_ident,
                pub type const CARRYING_CARRYLESS_MUL<const L: $ty, const R: $ty, const CARRY: $ty>: ($ty, $ty) = const { <$ty>::carrying_carryless_mul(L, R, CARRY) }; 
            }
            
            pub type const CARRYING_MUL<const L: $ty, const R: $ty, const CARRY: $ty>: ($ty_unsigned, $ty) = const { <$ty>::carrying_mul(L, R, CARRY) }; 
            
            


            
        }
    )*};
}



impl_math_common!(
    [for_ty: [u8, u8], ty_unsigned: u8, dummy_struct_name: MathU8],
    [for_ty: [u16, u16], ty_unsigned: u16, dummy_struct_name: MathU16],
    [for_ty: [u32, u32], ty_unsigned: u32, dummy_struct_name: MathU32],
    [for_ty: [u64, u64], ty_unsigned: u64, dummy_struct_name: MathU64],
    [for_ty: [u128, u128], ty_unsigned: u128, dummy_struct_name: MathU128],
    [for_ty: [u128, u128], ty_unsigned: u128, dummy_struct_name: MathUsize],
    [for_ty: [i8, i8], ty_unsigned: u8, dummy_struct_name: MathI8],
    [for_ty: [i16, i16], ty_unsigned: u16, dummy_struct_name: MathI16],
    [for_ty: [i32, i32], ty_unsigned: u32, dummy_struct_name: MathI32],
    [for_ty: [i64, i64], ty_unsigned: u64, dummy_struct_name: MathI64],
    [for_ty: [i128, i128], ty_unsigned: u128, dummy_struct_name: MathI128],
    [for_ty: [i128, i128], ty_unsigned: u128, dummy_struct_name: MathIsize],
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("gay")
    }
    
}
