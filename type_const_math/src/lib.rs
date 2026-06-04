#![feature(
    //
    generic_const_args, 
    min_generic_const_args, 
    generic_const_items,

    signed_bigint_helpers,
    const_unsigned_bigint_helpers,
)]
#![allow(incomplete_features)]

macro_rules! only_if_ty_is_unsigned {
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
    (u8, is_unsigned => $tt1:item, is_signed => $tt2:item) => { $tt1  };
    (u16, is_unsigned => $tt1:item, is_signed => $tt2:item) => { $tt1 };
    (u32, is_unsigned => $tt1:item, is_signed => $tt2:item) => { $tt1 };
    (u64, is_unsigned => $tt1:item, is_signed => $tt2:item) => { $tt1 };
    (u128, is_unsigned => $tt1:item, is_signed => $tt2:item) => { $tt1 };

    (i8, is_unsigned => $tt1:item, is_signed => $tt2:item) => { $tt2  };
    (i16, is_unsigned => $tt1:item, is_signed => $tt2:item) => { $tt2 };
    (i32, is_unsigned => $tt1:item, is_signed => $tt2:item) => { $tt2 };
    (i64, is_unsigned => $tt1:item, is_signed => $tt2:item) => { $tt2 };
    (i128, is_unsigned => $tt1:item, is_signed => $tt2:item) => { $tt2 };
}

macro_rules! impl_math_common {
    ($([for_type:[$ty:ty,$ty_ident:ident], unsigned_equiv_type: $unsigned_equiv_type:ty, dummy_struct_name: $dummy_struct_name:ident],)*) => {$(
        pub struct $dummy_struct_name;

        impl $dummy_struct_name {
            pub type const ADD<const L: $ty, const R: $ty>: $ty = const { L + R };
            pub type const SUB<const L: $ty, const R: $ty>: $ty = const { L - R };
            pub type const MUL<const L: $ty, const R: $ty>: $ty = const { L * R };
            pub type const DIV<const L: $ty, const R: $ty>: $ty = const { L / R };

            match_ty_signdness!{ $ty_ident,
                is_unsigned =>  pub type const ABS_DIFF<const L: $ty, const R: $ty>: $ty = const { <$ty>::abs_diff(L, R) }; ,
                is_signed   =>  pub type const ABS_DIFF<const L: $ty, const R: $ty>: $unsigned_equiv_type = const { <$ty>::abs_diff(L, R) }; 
            }

            only_if_ty_is_unsigned!{ $ty_ident, 
                pub type const BIT_WIDTH<const N: $ty>: u32 = const { <$ty>::bit_width(N) };
            }

            pub type const BORROWING_SUB<const L: $ty, const R: $ty, const BORROW: bool>: ($ty, bool) = const { <$ty>::borrowing_sub(L, R, BORROW) }; 

            pub type const CARRYING_ADD<const L: $ty, const R: $ty, const CARRY: bool>: ($ty, bool) = const { <$ty>::carrying_add(L, R, CARRY) }; 
            
        }
    )*};
}



impl_math_common!(
    [for_type: [u8, u8], unsigned_equiv_type: u8, dummy_struct_name: MathU8],
    [for_type: [u16, u16], unsigned_equiv_type: u16, dummy_struct_name: MathU16],
    [for_type: [u32, u32], unsigned_equiv_type: u32, dummy_struct_name: MathU32],
    [for_type: [u64, u64], unsigned_equiv_type: u64, dummy_struct_name: MathU64],
    [for_type: [u128, u128], unsigned_equiv_type: u128, dummy_struct_name: MathU128],
    [for_type: [u128, u128], unsigned_equiv_type: u128, dummy_struct_name: MathUsize],
    [for_type: [i8, i8], unsigned_equiv_type: u8, dummy_struct_name: MathI8],
    [for_type: [i16, i16], unsigned_equiv_type: u16, dummy_struct_name: MathI16],
    [for_type: [i32, i32], unsigned_equiv_type: u32, dummy_struct_name: MathI32],
    [for_type: [i64, i64], unsigned_equiv_type: u64, dummy_struct_name: MathI64],
    [for_type: [i128, i128], unsigned_equiv_type: u128, dummy_struct_name: MathI128],
    [for_type: [i128, i128], unsigned_equiv_type: u128, dummy_struct_name: MathIsize],
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("gay")
    }
    
}
