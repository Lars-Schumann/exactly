#![feature(
    macro_metavar_expr_concat,
    min_generic_const_args,
    generic_const_args,
    generic_const_items,
    inherent_associated_types,
    pattern_types,
    pattern_type_macro
)]
#![allow(incomplete_features)]

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

            pub const fn new(value: $ty) -> Self {
                assert!(LOWER <= value && value <= UPPER);
                Self(value)
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

        impl<const A_LOWER: $ty, const A_UPPER: $ty, const B_LOWER: $ty, const B_UPPER: $ty> ::core::ops::Add<${concat(R,$ty)}<{ B_LOWER }, { B_UPPER }>> for ${concat(R,$ty)}<{ A_LOWER }, { A_UPPER }>{
            type Output = ${concat(R,$ty)}<{ Self::ADD::<A_LOWER, B_LOWER>}, { Self::ADD::<A_UPPER, B_UPPER> }>;

            fn add(self, rhs: ${concat(R,$ty)}<{ B_LOWER }, { B_UPPER }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() + rhs.inner()) }
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
