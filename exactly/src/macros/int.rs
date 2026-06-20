macro_rules! impl_int_common {
    ($([inner_type: $num_t:ident, range_type_name: $range_t_name:ident],)*) => {$(
        #[derive(Debug, Copy, Clone)]
        #[repr(transparent)]
        pub struct $range_t_name<const LOWER: $num_t, const UPPER: $num_t>($num_t);

        impl<const LOWER: $num_t, const UPPER: $num_t> $range_t_name<LOWER, UPPER> {

            const __ASSERT_RANGE_NON_EMPTY: () = const { assert!(LOWER <= UPPER) };

            pub type const LOWER: $num_t = LOWER;
            pub type const UPPER: $num_t = UPPER;

            pub const fn lower(&self) -> $num_t {
                LOWER
            }

            pub const fn upper(&self) -> $num_t {
                UPPER
            }

            pub const fn inner(&self) -> $num_t {
                self.0
            }

            pub const fn includes(value: $num_t) -> bool {
                LOWER <= value && value <= UPPER
            }

            pub const fn new(value: $num_t) -> Option<Self> {
                match Self::includes(value) {
                    true => Some(
                        // SAFETY: we just checked the precondition
                        unsafe { Self::new_unchecked(value) }
                    ),
                    false => None,
                }
            }

            /// # Safety
            /// `Self::includes(value)`, which checks `LOWER <= value <= UPPER`, must hold.
            pub const unsafe fn new_unchecked(value: $num_t) -> Self {
                #[expect(path_statements)]
                Self::__ASSERT_RANGE_NON_EMPTY;
                debug_assert!(Self::includes(value));
                Self(value)
            }

            pub const fn widen<const NEW_LOWER: $num_t, const NEW_UPPER: $num_t>(self) -> $range_t_name<{ NEW_LOWER }, { NEW_UPPER }> {
                const { assert!(NEW_LOWER <= LOWER && UPPER <= NEW_UPPER) };
                // SAFETY: we just asserted the precondition
                unsafe { $range_t_name::<{ NEW_LOWER }, { NEW_UPPER }>::new_unchecked(self.inner()) }
            }
        }

        impl<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t> const ::core::ops::Add<$range_t_name<{ X }, { Y }>> for $range_t_name<{ A }, { B }>{
            type Output = $range_t_name<{ ::tcm::$num_t::ADD::<A, X> }, { ::tcm::$num_t::ADD::<B, Y> }>;

            fn add(self, rhs: $range_t_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() + rhs.inner()) }
            }
        }

        impl<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t> const ::core::ops::Sub<$range_t_name<{ X }, { Y }>> for $range_t_name<{ A }, { B }>{
            type Output = $range_t_name<{ ::tcm::$num_t::SUB::<A, Y> }, { ::tcm::$num_t::SUB::<B, X> }>;

            fn sub(self, rhs: $range_t_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() - rhs.inner()) }
            }
        }
)*}
}
pub(crate) use impl_int_common;

macro_rules! impl_int_unsigned {
    ($([inner_type: $num_t:ident, range_type_name: $range_t_name:ident],)*) => {$(

        impl<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t> const ::core::ops::Mul<$range_t_name<{ X }, { Y }>> for $range_t_name<{ A }, { B }>{
            type Output = $range_t_name<{ ::tcm::$num_t::MUL::<A, X> }, { ::tcm::$num_t::MUL::<B, Y> }>;

            fn mul(self, rhs: $range_t_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() * rhs.inner()) }
            }
        }

        impl<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t> const ::core::ops::Div<$range_t_name<{ X }, { Y }>> for $range_t_name<{ A }, { B }>{
            type Output = $range_t_name<{ ::tcm::$num_t::DIV::<A, Y> }, { ::tcm::$num_t::DIV::<B, X> }>;

            fn div(self, rhs: $range_t_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() / rhs.inner()) }
            }
        }
    )*}
}
pub(crate) use impl_int_unsigned;

macro_rules! impl_int_signed {
    ($([inner_type: $num_t:ident, range_type_name: $range_t_name:ident],)*) => {$(
        impl<const LOWER: $num_t, const UPPER: $num_t> $range_t_name<LOWER, UPPER> {

            #[expect(unused)]
            type const RANGE_MIN_MUL_RES<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t>: $num_t = const { (A * X).min(A * Y).min(B * X).min(B * Y) };
            #[expect(unused)]
            type const RANGE_MAX_MUL_RES<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t>: $num_t = const { (A * X).max(A * Y).max(B * X).max(B * Y) };

            #[expect(unused)]
            type const RANGE_MIN_DIV_RES<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t>: $num_t = const {
                if (X <= 0 && 0 <= Y) {
                    panic!("potential division by 0")
                }
                (A / X).min(A / Y).min(B / X).min(B / Y)
            };

            #[expect(unused)]
            type const RANGE_MAX_DIV_RES<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t>: $num_t = const {
                if (X <= 0 && 0 <= Y) {
                    panic!("potential division by 0")
                }
                (A / X).max(A / Y).max(B / X).max(B / Y)
            };
        }

        impl<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t> const ::core::ops::Mul<$range_t_name<{ X }, { Y }>> for $range_t_name<{ A }, { B }>{
            type Output = $range_t_name<{ Self::RANGE_MIN_MUL_RES::<A, B, X, Y> }, { Self::RANGE_MAX_MUL_RES::<A, B, X, Y> }>;

            fn mul(self, rhs: $range_t_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() * rhs.inner()) }
            }
        }

        impl<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t> const ::core::ops::Div<$range_t_name<{ X }, { Y }>> for $range_t_name<{ A }, { B }>{
            type Output = $range_t_name<{ Self::RANGE_MIN_DIV_RES::<A, B, X, Y> }, { Self::RANGE_MAX_DIV_RES::<A, B, X, Y> }>;

            fn div(self, rhs: $range_t_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() / rhs.inner()) }
            }
        }

    )*}
}
pub(crate) use impl_int_signed;
