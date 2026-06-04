macro_rules! impl_int_common {
    ($([inner_type: $inner_type:ty, range_type_name: $range_type_name:ident],)*) => {$(
        #[derive(Debug, Copy, Clone)]
        #[repr(transparent)]
        pub struct $range_type_name<const LOWER: $inner_type, const UPPER: $inner_type>($inner_type);

        impl<const LOWER: $inner_type, const UPPER: $inner_type> $range_type_name<LOWER, UPPER> {

            const __ASSERT_RANGE_NON_EMPTY: () = const { assert!(LOWER<=UPPER) };

            pub type const LOWER: $inner_type = LOWER;
            pub type const UPPER: $inner_type = UPPER;

            pub const fn lower(&self) -> $inner_type {
                LOWER
            }

            pub const fn upper(&self) -> $inner_type {
                UPPER
            }

            pub const fn inner(&self) -> $inner_type {
                self.0
            }

            pub const fn new(value: $inner_type) -> Option<Self> {
                match (LOWER <= value && value <= UPPER) {
                    true => Some(unsafe { Self::new_unchecked(value) }),
                    false => None,
                }
            }

            /// # Safety
            /// LOWER <= value <= UPPER must hold.
            pub const unsafe fn new_unchecked(value: $inner_type) -> Self {
                #[expect(path_statements)]
                Self::__ASSERT_RANGE_NON_EMPTY;
                debug_assert!(LOWER <= value && value <= UPPER);
                Self(value)
            }

            #[expect(unused)]
            type const ADD<const N: $inner_type, const M: $inner_type>: $inner_type = const { N + M };
            #[expect(unused)]
            type const SUB<const N: $inner_type, const M: $inner_type>: $inner_type = const { N - M };
            #[expect(unused)]
            type const MUL<const N: $inner_type, const M: $inner_type>: $inner_type = const { N * M };
            #[expect(unused)]
            type const DIV<const N: $inner_type, const M: $inner_type>: $inner_type = const { N / M };

            pub const fn add<const V: $inner_type>(&self) -> $range_type_name::<{Self::ADD::<LOWER, V>},{ Self::ADD::<UPPER, V>}> {
                unsafe { $range_type_name::<{Self::ADD::<LOWER, V>},{ Self::ADD::<UPPER, V>}>::new_unchecked(self.inner() + V) }
            }

            pub const fn sub<const V: $inner_type>(&self) -> $range_type_name::<{Self::SUB::<LOWER, V>},{ Self::SUB::<UPPER, V>}> {
                unsafe { $range_type_name::<{Self::SUB::<LOWER, V>},{ Self::SUB::<UPPER, V>}>::new_unchecked(self.inner() - V) }
            }

            pub const fn mul<const V: $inner_type>(&self) -> $range_type_name::<{Self::MUL::<LOWER, V>},{ Self::MUL::<UPPER, V>}> {
                unsafe { $range_type_name::<{Self::MUL::<LOWER, V>},{ Self::MUL::<UPPER, V>}>::new_unchecked(self.inner() * V) }
            }

            pub const fn div<const V: $inner_type>(&self) -> $range_type_name::<{Self::DIV::<LOWER, V>},{ Self::DIV::<UPPER, V>}> {
                unsafe { $range_type_name::<{Self::DIV::<LOWER, V>},{ Self::DIV::<UPPER, V>}>::new_unchecked(self.inner() / V) }
            }

        }


        impl<const A: $inner_type, const B: $inner_type, const X: $inner_type, const Y: $inner_type> const ::core::ops::Add<$range_type_name<{ X }, { Y }>> for $range_type_name<{ A }, { B }>{
            type Output = $range_type_name<{ Self::ADD::<A, X>}, { Self::ADD::<B, Y> }>;

            fn add(self, rhs: $range_type_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() + rhs.inner()) }
            }
        }

        impl<const A: $inner_type, const B: $inner_type, const X: $inner_type, const Y: $inner_type> const ::core::ops::Sub<$range_type_name<{ X }, { Y }>> for $range_type_name<{ A }, { B }>{
            type Output = $range_type_name<{ Self::SUB::<A, Y>}, { Self::SUB::<B, X> }>;

            fn sub(self, rhs: $range_type_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() - rhs.inner()) }
            }
        }
    )*}
}
pub(crate) use impl_int_common;

macro_rules! impl_int_unsigned {
    ($([inner_type: $inner_type:ty, range_type_name: $range_type_name:ident],)*) => {$(
        impl<const A: $inner_type, const B: $inner_type, const X: $inner_type, const Y: $inner_type> const ::core::ops::Mul<$range_type_name<{ X }, { Y }>> for $range_type_name<{ A }, { B }>{
            type Output = $range_type_name<{ Self::MUL::<A, X> }, { Self::MUL::<B, Y> }>;

            fn mul(self, rhs: $range_type_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() * rhs.inner()) }
            }
        }

        impl<const A: $inner_type, const B: $inner_type, const X: $inner_type, const Y: $inner_type> const ::core::ops::Div<$range_type_name<{ X }, { Y }>> for $range_type_name<{ A }, { B }>{
            type Output = $range_type_name<{ Self::DIV::<A, Y> }, { Self::DIV::<B, X> }>;

            fn div(self, rhs: $range_type_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() / rhs.inner()) }
            }
        }
    )*}
}
pub(crate) use impl_int_unsigned;

macro_rules! impl_int_signed {
    ($([inner_type: $inner_type:ty, range_type_name: $range_type_name:ident],)*) => {$(
        impl<const LOWER: $inner_type, const UPPER: $inner_type> $range_type_name<LOWER, UPPER> {
            #[expect(unused)]
            type const MIN_MUL_RES<const A: $inner_type, const B: $inner_type, const X: $inner_type, const Y: $inner_type>: $inner_type = const { (A * X).min(A * Y).min(B * X).min(B * Y) };
            #[expect(unused)]
            type const MAX_MUL_RES<const A: $inner_type, const B: $inner_type, const X: $inner_type, const Y: $inner_type>: $inner_type = const { (A * X).max(A * Y).max(B * X).max(B * Y) };

            #[expect(unused)]
            type const MIN_DIV_RES<const A: $inner_type, const B: $inner_type, const X: $inner_type, const Y: $inner_type>: $inner_type = const {
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

            };

            #[expect(unused)]
            type const MAX_DIV_RES<const A: $inner_type, const B: $inner_type, const X: $inner_type, const Y: $inner_type>: $inner_type = const {
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

        impl<const A: $inner_type, const B: $inner_type, const X: $inner_type, const Y: $inner_type> const ::core::ops::Mul<$range_type_name<{ X }, { Y }>> for $range_type_name<{ A }, { B }>{
            type Output = $range_type_name<{ Self::MIN_MUL_RES::<A, B, X, Y> }, { Self::MAX_MUL_RES::<A, B, X, Y> }>;

            fn mul(self, rhs: $range_type_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() * rhs.inner()) }
            }
        }

        impl<const A: $inner_type, const B: $inner_type, const X: $inner_type, const Y: $inner_type> const ::core::ops::Div<$range_type_name<{ X }, { Y }>> for $range_type_name<{ A }, { B }>{
            type Output = $range_type_name<{ Self::MIN_DIV_RES::<A, B, X, Y> }, { Self::MAX_DIV_RES::<A, B, X, Y> }>;

            fn div(self, rhs: $range_type_name<{ X }, { Y }>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.inner() / rhs.inner()) }
            }
        }

    )*}
}
pub(crate) use impl_int_signed;
