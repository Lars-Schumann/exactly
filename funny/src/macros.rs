macro_rules! impl_ints {
    ($([inner_type: $num_t:ident, wrap_t_name: $wrap_t_name:ident, range_fn_name: $range_fn_name:ident, extra_tcm: $extra_tcm:ident, sort_fn_name: $sort_fn_name:ident],)*) => {$(

        #[derive(Debug, Copy, Clone,)]
        #[repr(transparent)]
        pub struct $wrap_t_name<const VALUES: &'static [$num_t]>($num_t);

        mod $extra_tcm {
            pub(super) type const LEN<const SLICE: &'static[$num_t]>: usize = const { SLICE.len()};

            pub(super) type const ADD<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { ::tcm::usize::MUL::<{ LEN::<A> }, { LEN::<B> }> }, _>(const |i| A[i / B.len()] + B[i % B.len()])
            };

            pub(super) type const SUB<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { ::tcm::usize::MUL::<{ LEN::<A> }, { LEN::<B> }> }, _>(const |i| A[i / B.len()] - B[i % B.len()])
            };

            pub(super) type const MUL<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { ::tcm::usize::MUL::<{ LEN::<A> }, { LEN::<B> }> }, _>(const |i| A[i / B.len()] * B[i % B.len()])
            };

            pub(super) type const DIV<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { ::tcm::usize::MUL::<{ LEN::<A> }, { LEN::<B> }> }, _>(const |i| A[i / B.len()] / B[i % B.len()])
            };

            pub(super) type const SORT<const SLICE: &'static[$num_t]>: &[$num_t] = const {
                let arr: [$num_t; LEN::<SLICE>] = match SLICE.try_into() {
                    Ok(arr) => arr,
                    Err(_) => unreachable!()
                };
                &::compile_time_sort::$sort_fn_name(arr)
            };

            pub(super) type const UNIQUE_ELEMENT_COUNT<const SLICE: &'static[$num_t]>: usize = const {
                let slice_sorted = SORT::<SLICE>;
                match slice_sorted {
                    [] => 0,
                    [_, ..] => {
                        let mut unique_element_count: usize = 1;
                        let mut i: usize = 1;

                        while i < slice_sorted.len() {
                            let previous = slice_sorted[i - 1];
                            let current = slice_sorted[i];
                            if previous != current {
                                unique_element_count += 1;
                            }
                            i += 1;
                        }
                        unique_element_count
                    }
                }
            };

            pub(super) type const NORMALIZE<const SLICE: &'static[$num_t]>: &[$num_t] = const {
                use core::mem::MaybeUninit as MU;
                let slice_sorted = SORT::<SLICE>;
                match slice_sorted {
                    [] => &[],
                    [first, ..] => {

                        let mut normalized: [MU<$num_t>; UNIQUE_ELEMENT_COUNT::<SLICE>] = [MU::uninit(); UNIQUE_ELEMENT_COUNT::<SLICE>];
                        normalized[0] = MU::new(*first);

                        let mut unique_count: usize = 1;
                        let mut i: usize = 1;

                        while i < slice_sorted.len() {
                            let previous = slice_sorted[i - 1];
                            let current = slice_sorted[i];
                            if previous != current {
                                unique_count += 1;
                                normalized[unique_count - 1] = MU::new(current);
                            }
                            i += 1;
                        }
                        &unsafe { MU::array_assume_init(normalized) }
                    }
                }
            };

            type const RANGE_LENGTH<const MIN: $num_t, const MAX: $num_t>: usize = const {
                match <$num_t as ::core::convert::TryInto<usize>>::try_into(MAX - MIN) {
                    Err(_) => panic!(),
                    Ok(len) => len + 1_usize,
                }
            };

            pub(super) type const RANGE<const MIN: $num_t, const MAX: $num_t>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { RANGE_LENGTH::<MIN, MAX> }, _>(const |i| MIN + i as $num_t)
            };
        }

        impl<const VALUES: &'static [$num_t]> $wrap_t_name<VALUES> {

            pub const fn new(value: $num_t) -> Option<Self> {
                match Self::includes(value) {
                    true => Some(unsafe { Self::new_unchecked(value) }),
                    false => None,
                }
            }

            /// # Safety
            ///
            /// TODO
            pub const unsafe fn new_unchecked(value: $num_t) -> Self {
                debug_assert!(Self::includes(value));
                Self(value)
            }

            pub const fn includes(value: $num_t) -> bool {
                let mut i: usize = 0;

                while i < VALUES.len() {
                    if VALUES[i] == value {
                        return true;
                    }
                    i += 1;
                }
                false
            }

            pub const fn inner(self) -> $num_t {
                self.0
            }

            pub const fn sort(self) -> $wrap_t_name<{ $extra_tcm::SORT::<VALUES> }> {
                unsafe { $wrap_t_name::new_unchecked(self.inner()) }
            }

            pub const fn normalize(self) -> $wrap_t_name<{ $extra_tcm::NORMALIZE::<VALUES> }> {
                unsafe { $wrap_t_name::new_unchecked(self.inner()) }
            }
        }

        impl<const A_VALUES: &'static [$num_t], const B_VALUES: &'static [$num_t]> ::core::ops::Add<$wrap_t_name<B_VALUES>> for $wrap_t_name<A_VALUES> {
            type Output = $wrap_t_name<{ $extra_tcm::ADD::<{ A_VALUES }, { B_VALUES }> }>;

            fn add(self, rhs: $wrap_t_name<B_VALUES>) -> Self::Output {
                unsafe { $wrap_t_name::new_unchecked(self.inner() + rhs.inner()) }
            }
        }

        impl<const A_VALUES: &'static [$num_t], const B_VALUES: &'static [$num_t]> ::core::ops::Sub<$wrap_t_name<B_VALUES>> for $wrap_t_name<A_VALUES> {
            type Output = $wrap_t_name<{ $extra_tcm::SUB::<{ A_VALUES }, { B_VALUES }> }>;

            fn sub(self, rhs: $wrap_t_name<B_VALUES>) -> Self::Output {
                unsafe { $wrap_t_name::new_unchecked(self.inner() - rhs.inner()) }
            }
        }

        impl<const A_VALUES: &'static [$num_t], const B_VALUES: &'static [$num_t]> ::core::ops::Mul<$wrap_t_name<B_VALUES>> for $wrap_t_name<A_VALUES> {
            type Output = $wrap_t_name<{ $extra_tcm::MUL::<{ A_VALUES }, { B_VALUES }> }>;

            fn mul(self, rhs: $wrap_t_name<B_VALUES>) -> Self::Output {
                unsafe { $wrap_t_name::new_unchecked(self.inner() * rhs.inner()) }
            }
        }

        impl<const A_VALUES: &'static [$num_t], const B_VALUES: &'static [$num_t]> ::core::ops::Div<$wrap_t_name<B_VALUES>> for $wrap_t_name<A_VALUES> {
            type Output = $wrap_t_name<{ $extra_tcm::DIV::<{ A_VALUES }, { B_VALUES }> }>;

            fn div(self, rhs: $wrap_t_name<B_VALUES>) -> Self::Output {
                unsafe { $wrap_t_name::new_unchecked(self.inner() / rhs.inner()) }
            }
        }

    )*}
}
pub(crate) use impl_ints;
