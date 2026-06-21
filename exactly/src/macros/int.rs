macro_rules! impl_int_common {
    ($([inner_type: $num_t:ident, range_t_name: $range_t_name:ident, exact_fn_name: $exact_fn_name:ident],)*) => {$(
        #[derive(Debug, Copy, Clone)]
        #[repr(transparent)]
        pub struct $range_t_name<const LOWER: $num_t, const UPPER: $num_t>($num_t);

        impl<const A: $num_t, const B: $num_t> $range_t_name<A, B> {

            pub type const LOWER: $num_t = A;
            pub type const UPPER: $num_t = B;

            const __ASSERT_RANGE_NON_EMPTY: () = const { assert!(Self::LOWER <= Self::UPPER) };

            pub const fn lower(&self) -> $num_t {
                Self::LOWER
            }

            pub const fn upper(&self) -> $num_t {
                Self::UPPER
            }

            pub const fn inner(&self) -> $num_t {
                self.0
            }

            pub const fn includes(value: $num_t) -> bool {
                Self::LOWER <= value && value <= Self::UPPER
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
                const { assert!(NEW_LOWER <= Self::LOWER && Self::UPPER <= NEW_UPPER) };
                // SAFETY: we just asserted the precondition
                unsafe { $range_t_name::<{ NEW_LOWER }, { NEW_UPPER }>::new_unchecked(self.inner()) }
            }


            pub const fn saturating_add<const X: $num_t, const Y: $num_t>(self, other: $range_t_name<{ X }, { Y }>) -> $range_t_name<{ ::tcm::$num_t::SATURATING_ADD::<A, X> }, { ::tcm::$num_t::SATURATING_ADD::<B, Y> }> {
                unsafe { $range_t_name::new_unchecked(self.inner().saturating_add(other.inner())) }
            }

            pub const fn saturating_sub<const X: $num_t, const Y: $num_t>(self, other: $range_t_name<{ X }, { Y }>) -> $range_t_name<{ ::tcm::$num_t::SATURATING_SUB::<A, Y> }, { ::tcm::$num_t::SATURATING_SUB::<B, X> }> {
                unsafe { $range_t_name::new_unchecked(self.inner().saturating_sub(other.inner())) }
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

        pub const fn $exact_fn_name<const NUM: $num_t>() -> $range_t_name::<NUM, NUM> {
            const { $range_t_name::<NUM, NUM>::new(NUM).expect("This should be infallible, please file a bug report.") }
        }
)*}
}
pub(crate) use impl_int_common;

macro_rules! impl_int_unsigned {
    ($([inner_type: $num_t:ident, range_t_name: $range_t_name:ident],)*) => {$(

        impl<const A: $num_t, const B: $num_t> $range_t_name<A, B> {

            pub const fn saturating_mul<const X: $num_t, const Y: $num_t>(self, other: $range_t_name<{ X }, { Y }>) -> $range_t_name<{ ::tcm::$num_t::SATURATING_MUL::<A, X> }, { ::tcm::$num_t::SATURATING_MUL::<B, Y> }> {
                unsafe { $range_t_name::new_unchecked(self.inner().saturating_mul(other.inner())) }
            }

            pub const fn saturating_div<const X: $num_t, const Y: $num_t>(self, other: $range_t_name<{ X }, { Y }>) -> $range_t_name<{ ::tcm::$num_t::SATURATING_DIV::<A, Y> }, { ::tcm::$num_t::SATURATING_DIV::<B, X> }> {
                unsafe { $range_t_name::new_unchecked(self.inner().saturating_div(other.inner())) }
            }

        }

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
    ($([inner_type: $num_t:ident, range_t_name: $range_t_name:ident],)*) => {$(
        impl<const A: $num_t, const B: $num_t> $range_t_name<A, B> {

            #[expect(unused)]
            type const MIN_SATURATING_MUL_RES<const _A: $num_t, const _B: $num_t, const _X: $num_t, const _Y: $num_t>: $num_t = const {
                    (_A.saturating_mul(_X))
                .min(_A.saturating_mul(_Y))
                .min(_B.saturating_mul(_X))
                .min(_B.saturating_mul(_Y))
            };

            #[expect(unused)]
            type const MAX_SATURATING_MUL_RES<const _A: $num_t, const _B: $num_t, const _X: $num_t, const _Y: $num_t>: $num_t = const {
                    (_A.saturating_mul(_X))
                .max(_A.saturating_mul(_Y))
                .max(_B.saturating_mul(_X))
                .max(_B.saturating_mul(_Y))
            };

            pub const fn saturating_mul<const X: $num_t, const Y: $num_t>(self, other: $range_t_name<{ X }, { Y }>) -> $range_t_name<{ Self::MIN_SATURATING_MUL_RES::<A, B, X, Y> }, { Self::MAX_SATURATING_MUL_RES::<A, B, X, Y> }> {
                unsafe { $range_t_name::new_unchecked(self.inner().saturating_mul(other.inner())) }
            }

            pub const fn saturating_div<const X: $num_t, const Y: $num_t>(self, other: $range_t_name<{ X }, { Y }>) -> $range_t_name<{ ::tcm::$num_t::SATURATING_DIV::<A, Y> }, { ::tcm::$num_t::SATURATING_DIV::<B, X> }> {
                unsafe { $range_t_name::new_unchecked(self.inner().saturating_div(other.inner())) }
            }

            #[expect(unused)]
            type const MIN_MUL_RES<const _A: $num_t, const _B: $num_t, const _X: $num_t, const _Y: $num_t>: $num_t = const {
                    (A * _X)
                .min(A * _Y)
                .min(B * _X)
                .min(B * _Y)
            };


            #[expect(unused)]
            type const MAX_MUL_RES<const _A: $num_t, const _B: $num_t, const _X: $num_t, const _Y: $num_t>: $num_t = const {
                    (_A * _X)
                .min(_A * _Y)
                .min(_B * _X)
                .min(_B * _Y)
            };

            #[expect(unused)]
            type const MIN_DIV_RES<const _A: $num_t, const _B: $num_t, const _X: $num_t, const _Y: $num_t>: $num_t = const {
                if (_X <= 0 && 0 <= _Y) {
                    panic!("potential division by 0")
                }
                    (_A / _X)
                .min(_A / _Y)
                .min(_B / _X)
                .min(_B / _Y)
            };

            #[expect(unused)]
            type const MAX_DIV_RES<const _A: $num_t, const _B: $num_t, const _X: $num_t, const _Y: $num_t>: $num_t = const {
                if (_X <= 0 && 0 <= _Y) {
                    panic!("potential division by 0")
                }
                    (_A / _X)
                .max(_A / _Y)
                .max(_B / _X)
                .max(_B / _Y)
            };
        }

        impl<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t> const ::core::ops::Mul<$range_t_name<{ X }, { Y }>> for $range_t_name<{ A }, { B }>{
            type Output = $range_t_name<{ Self::MIN_MUL_RES::<A, B, X, Y> }, { Self::MAX_MUL_RES::<A, B, X, Y> }>;

            fn mul(self, rhs: $range_t_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() * rhs.inner()) }
            }
        }

        impl<const A: $num_t, const B: $num_t, const X: $num_t, const Y: $num_t> const ::core::ops::Div<$range_t_name<{ X }, { Y }>> for $range_t_name<{ A }, { B }>{
            type Output = $range_t_name<{ Self::MIN_DIV_RES::<A, B, X, Y> }, { Self::MAX_DIV_RES::<A, B, X, Y> }>;

            fn div(self, rhs: $range_t_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() / rhs.inner()) }
            }
        }

    )*}
}
pub(crate) use impl_int_signed;
