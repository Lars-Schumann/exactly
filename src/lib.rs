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
    const_cmp
)]
#![allow(incomplete_features)]

use core::ops::{Add, Div, Mul, Sub};

macro_rules! pattern_type_at_home {
    ($($ty:ident,)*) => {$(
        #[derive(Debug, Copy, Clone)]
        #[repr(transparent)]
        pub struct ${concat(R,$ty)}<const LOWER: $ty, const UPPER: $ty>($ty);

        impl<const LOWER: $ty, const UPPER: $ty> ${concat(R,$ty)}<LOWER, UPPER> {
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
                if !(LOWER <= value && value <= UPPER) {
                    None
                } else {
                    Some(Self(value))
                }
            }

            /// # Safety
            /// LOWER <= value <= UPPER must hold.
            pub const unsafe fn new_unchecked(value: $ty) -> Self {
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

            #[expect(unused)]
            type const MIN_OF_CROSS_MULS<const A: $ty, const B: $ty, const X: $ty, const Y: $ty>: $ty = const { (A * X).min(A * Y).min(B * X).min(B * Y) };
            #[expect(unused)]
            type const MAX_OF_CROSS_MULS<const A: $ty, const B: $ty, const X: $ty, const Y: $ty>: $ty = const { (A * X).max(A * Y).max(B * X).max(B * Y) };

            #[expect(unused)]
            type const MIN_OF_CROSS_DIVS<const A: $ty, const B: $ty, const X: $ty, const Y: $ty>: $ty = const { (A / X).min(A / Y).min(B / X).min(B / Y) };
            #[expect(unused)]
            type const MAX_OF_CROSS_DIVS<const A: $ty, const B: $ty, const X: $ty, const Y: $ty>: $ty = const { (A / X).max(A / Y).max(B / X).max(B / Y) };

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

        impl<const A: $ty, const B: $ty, const X: $ty, const Y: $ty> const Mul<${concat(R,$ty)}<{ X }, { Y }>> for ${concat(R,$ty)}<{ A }, { B }>{
            type Output = ${concat(R,$ty)}<{ Self::MIN_OF_CROSS_MULS::<A, B, X, Y> }, { Self::MAX_OF_CROSS_MULS::<A, B, X, Y> }>;

            fn mul(self, rhs: ${concat(R,$ty)}<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() * rhs.inner()) }
            }
        }

        impl<const A: $ty, const B: $ty, const X: $ty, const Y: $ty> const Div<${concat(R,$ty)}<{ X }, { Y }>> for ${concat(R,$ty)}<{ A }, { B }>{
            type Output = ${concat(R,$ty)}<{ Self::MIN_OF_CROSS_DIVS::<A, B, X, Y> }, { Self::MAX_OF_CROSS_DIVS::<A, B, X, Y> }>;

            fn div(self, rhs: ${concat(R,$ty)}<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() / rhs.inner()) }
            }
        }
    )*}
}

pattern_type_at_home!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128,);

#[macro_export]
macro_rules! to_pattern_type {
    (let $name:ident: $ty:ident is $lower:literal..=$upper:literal = $value:expr) => {
        {
            let __assert_type: $crate::${concat(R,$ty)}::<$lower, $upper> = $value;
        }
        let $name: pattern_type!(::core::primitive::$ty is $lower..=$upper) = unsafe { ::core::mem::transmute($value) };
    };
}
