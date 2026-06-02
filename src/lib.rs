#![feature(
    macro_metavar_expr_concat,
    min_generic_const_args,
    generic_const_args,
    generic_const_items,
    inherent_associated_types,
    pattern_types,
    pattern_type_macro,
    const_ops,
    const_trait_impl,
    const_cmp,
    min_adt_const_params,
    structural_match
)]
#![allow(incomplete_features)]

pub mod float;

use core::ops::{Add, Div, Mul, Sub};

macro_rules! impl_int_common {
    ($($ty:ident,)*) => {$(
        #[derive(Debug, Copy, Clone)]
        #[repr(transparent)]
        pub struct ${concat(R,$ty)}<const LOWER: $ty, const UPPER: $ty>($ty);

        impl<const LOWER: $ty, const UPPER: $ty> ${concat(R,$ty)}<LOWER, UPPER> {

            const __ASSERT_RANGE_NON_EMPTY: () = const { assert!(LOWER<=UPPER) };

            pub type const LOWER: $ty = LOWER;
            pub type const UPPER: $ty = UPPER;

            pub const fn lower(&self) -> $ty {
                LOWER
            }

            pub const fn upper(&self) -> $ty {
                UPPER
            }

            pub const fn inner(&self) -> $ty {
                self.0
            }

            pub const fn new(value: $ty) -> Option<Self> {
                match (LOWER <= value && value <= UPPER) {
                    true => Some(unsafe { Self::new_unchecked(value) }),
                    false => None,
                }
            }

            /// # Safety
            /// LOWER <= value <= UPPER must hold.
            pub const unsafe fn new_unchecked(value: $ty) -> Self {
                #[expect(path_statements)]
                Self::__ASSERT_RANGE_NON_EMPTY;
                debug_assert!(LOWER <= value && value <= UPPER);
                Self(value)
            }

            #[expect(unused)]
            type const ADD<const N: $ty, const M: $ty>: $ty = const { N + M };
            #[expect(unused)]
            type const SUB<const N: $ty, const M: $ty>: $ty = const { N - M };
            #[expect(unused)]
            type const MUL<const N: $ty, const M: $ty>: $ty = const { N * M };
            #[expect(unused)]
            type const DIV<const N: $ty, const M: $ty>: $ty = const { N / M };

            pub const fn add<const V: $ty>(&self) -> ${concat(R,$ty)}::<{Self::ADD::<LOWER, V>},{ Self::ADD::<UPPER, V>}> {
                unsafe { ${concat(R,$ty)}::<{Self::ADD::<LOWER, V>},{ Self::ADD::<UPPER, V>}>::new_unchecked(self.inner() + V) }
            }

            pub const fn sub<const V: $ty>(&self) -> ${concat(R,$ty)}::<{Self::SUB::<LOWER, V>},{ Self::SUB::<UPPER, V>}> {
                unsafe { ${concat(R,$ty)}::<{Self::SUB::<LOWER, V>},{ Self::SUB::<UPPER, V>}>::new_unchecked(self.inner() - V) }
            }

            pub const fn mul<const V: $ty>(&self) -> ${concat(R,$ty)}::<{Self::MUL::<LOWER, V>},{ Self::MUL::<UPPER, V>}> {
                unsafe { ${concat(R,$ty)}::<{Self::MUL::<LOWER, V>},{ Self::MUL::<UPPER, V>}>::new_unchecked(self.inner() * V) }
            }

            pub const fn div<const V: $ty>(&self) -> ${concat(R,$ty)}::<{Self::DIV::<LOWER, V>},{ Self::DIV::<UPPER, V>}> {
                unsafe { ${concat(R,$ty)}::<{Self::DIV::<LOWER, V>},{ Self::DIV::<UPPER, V>}>::new_unchecked(self.inner() / V) }
            }
        }

        impl<const A: $ty, const B: $ty, const X: $ty, const Y: $ty> const Add<${concat(R,$ty)}<{ X }, { Y }>> for ${concat(R,$ty)}<{ A }, { B }>{
            type Output = ${concat(R,$ty)}<{ Self::ADD::<A, X>}, { Self::ADD::<B, Y> }>;

            fn add(self, rhs: ${concat(R,$ty)}<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() + rhs.inner()) }
            }
        }

        impl<const A: $ty, const B: $ty, const X: $ty, const Y: $ty> const Sub<${concat(R,$ty)}<{ X }, { Y }>> for ${concat(R,$ty)}<{ A }, { B }>{
            type Output = ${concat(R,$ty)}<{ Self::SUB::<A, Y>}, { Self::SUB::<B, X> }>;

            fn sub(self, rhs: ${concat(R,$ty)}<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() - rhs.inner()) }
            }
        }
    )*}
}

macro_rules! impl_int_unsigned {
    ($($ty:ident,)*) => {$(
        impl<const A: $ty, const B: $ty, const X: $ty, const Y: $ty> const Mul<${concat(R,$ty)}<{ X }, { Y }>> for ${concat(R,$ty)}<{ A }, { B }>{
            type Output = ${concat(R,$ty)}<{ Self::MUL::<A, X> }, { Self::MUL::<B, Y> }>;

            fn mul(self, rhs: ${concat(R,$ty)}<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() * rhs.inner()) }
            }
        }

        impl<const A: $ty, const B: $ty, const X: $ty, const Y: $ty> const Div<${concat(R,$ty)}<{ X }, { Y }>> for ${concat(R,$ty)}<{ A }, { B }>{
            type Output = ${concat(R,$ty)}<{ Self::DIV::<A, Y> }, { Self::DIV::<B, X> }>;

            fn div(self, rhs: ${concat(R,$ty)}<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() / rhs.inner()) }
            }
        }
    )*}
}

macro_rules! impl_int_signed {
    ($($ty:ident,)*) => {$(
        impl<const LOWER: $ty, const UPPER: $ty> ${concat(R,$ty)}<LOWER, UPPER> {
            #[expect(unused)]
            type const MIN_MUL_RES<const A: $ty, const B: $ty, const X: $ty, const Y: $ty>: $ty = const { (A * X).min(A * Y).min(B * X).min(B * Y) };
            #[expect(unused)]
            type const MAX_MUL_RES<const A: $ty, const B: $ty, const X: $ty, const Y: $ty>: $ty = const { (A * X).max(A * Y).max(B * X).max(B * Y) };

            #[expect(unused)]
            type const MIN_DIV_RES<const A: $ty, const B: $ty, const X: $ty, const Y: $ty>: $ty = const { 'out: {
                if (X == 0 && Y == 0) {
                    panic!("unconditional division by 0")
                }

                let furthest_hopefully_pos_divisor_from_0 = if Y == 0 {-1} else {Y};

                let closest_hopefully_neg_divisor_to_0 = match (X < 0, Y < 0){
                    (true, true) => Y,
                    (false, false) => X,
                    (true, false) => -1,
                    (false, true) => unreachable!() // Y can't be less than X
                };

                (A / furthest_hopefully_pos_divisor_from_0).min(B / closest_hopefully_neg_divisor_to_0)

            }};

            #[expect(unused)]
            type const MAX_DIV_RES<const A: $ty, const B: $ty, const X: $ty, const Y: $ty>: $ty = const {
                if (X == 0 && Y == 0) {
                    panic!("unconditional division by 0")
                }

                let closest_hopefully_pos_divisor_to_0 = match (0 < X , 0 < Y){
                    (true, true) => X,
                    (false, false) => Y,
                    (false, true) => 1,
                    (true, false) => unreachable!() // Y can't be less than X
                };

                let furthest_hopefully_neg_divisor_from_0 = if X == 0 {1} else {X};



                (B / closest_hopefully_pos_divisor_to_0).max(A / furthest_hopefully_neg_divisor_from_0)
            };
        }

        impl<const A: $ty, const B: $ty, const X: $ty, const Y: $ty> const Mul<${concat(R,$ty)}<{ X }, { Y }>> for ${concat(R,$ty)}<{ A }, { B }>{
            type Output = ${concat(R,$ty)}<{ Self::MIN_MUL_RES::<A, B, X, Y> }, { Self::MAX_MUL_RES::<A, B, X, Y> }>;

            fn mul(self, rhs: ${concat(R,$ty)}<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() * rhs.inner()) }
            }
        }

        impl<const A: $ty, const B: $ty, const X: $ty, const Y: $ty> const Div<${concat(R,$ty)}<{ X }, { Y }>> for ${concat(R,$ty)}<{ A }, { B }>{
            type Output = ${concat(R,$ty)}<{ Self::MIN_DIV_RES::<A, B, X, Y> }, { Self::MAX_DIV_RES::<A, B, X, Y> }>;

            fn div(self, rhs: ${concat(R,$ty)}<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() / rhs.inner()) }
            }
        }

    )*}
}

impl_int_common!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128,);

impl_int_unsigned!(u8, u16, u32, u64, u128,);

impl_int_signed!(i8, i16, i32, i64, i128,);

#[macro_export]
macro_rules! to_pattern_type {
    (let $name:ident: $ty:ident is $lower:literal..=$upper:literal = $value:expr) => {
        {
            let __assert_type: $crate::${concat(R,$ty)}::<$lower, $upper> = $value;
        }
        let $name: pattern_type!(::core::primitive::$ty is $lower..=$upper) = unsafe { ::core::mem::transmute($value) };
    };
}
