macro_rules! impl_ints {
    (the_dolla: $d:tt, $([inner_type: $num_t:ident, wrap_t_name: $wrap_t_name:ident, range_fn_name: $range_fn_name:ident, private_macro_prefix: $private_macro_prefix:ident, extra_mod: $extra_mod:ident, sort_fn_name: $sort_fn_name:ident],)*) => {$(

        #[derive(Debug, Copy, Clone,)]
        #[repr(transparent)]
        pub struct $wrap_t_name<const SET: &'static [$num_t]>($num_t);

        pub mod $extra_mod {
            const LEN<const SET: &'static[$num_t]>: usize = const { SET.len()};
            const PRODUCT_OF_LENGTHS<const A: &'static[$num_t], const B: &'static[$num_t]>: usize = const { A.len() * B.len() };

            pub(super) const ADD<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { PRODUCT_OF_LENGTHS::<A, B> }, _>(const |i| A[i / B.len()] + B[i % B.len()])
            };
            pub(super) const SUB<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { PRODUCT_OF_LENGTHS::<A, B> }, _>(const |i| A[i / B.len()] - B[i % B.len()])
            };
            pub(super) const MUL<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { PRODUCT_OF_LENGTHS::<A, B> }, _>(const |i| A[i / B.len()] * B[i % B.len()])
            };
            pub(super) const DIV<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { PRODUCT_OF_LENGTHS::<A, B> }, _>(const |i| A[i / B.len()] / B[i % B.len()])
            };
            pub(super) const SORT<const SET: &'static[$num_t]>: &[$num_t] = const {
                let arr: [$num_t; LEN::<SET>] = match SET.try_into() {
                    Ok(arr) => arr,
                    Err(_) => unreachable!()
                };
                &::compile_time_sort::$sort_fn_name(arr)
            };
            pub(super) const NORMALIZE<const SET: &'static[$num_t]>: &[$num_t] = const { 'out: {
                let set_sorted = SORT::<SET>;
                let mut normalized: Vec<$num_t> = Vec::new();

                let [first, ..] = set_sorted else {
                    break 'out &[]
                };

                normalized.push(*first);

                let mut i: usize = 1;

                while i < set_sorted.len() {
                    let (previous, current) = (set_sorted[i - 1], set_sorted[i]);
                    if previous != current {
                        normalized.push(current)
                    }
                    i += 1;
                }
                normalized.const_make_global()
            }};

            const RANGE_LENGTH<const START: $num_t, const END: $num_t>: usize = const {
                match <$num_t as ::core::convert::TryInto<usize>>::try_into(END - START) {
                    Err(_) => panic!(),
                    Ok(len) => len,
                }
            };

            pub const RANGE<const START: $num_t, const END: $num_t>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { RANGE_LENGTH::<START, END> }, _>(const |i| START + i as $num_t)
            };

            const RANGE_INCLUSIVE_LENGTH<const START: $num_t, const LAST: $num_t>: usize = const {
                match <$num_t as ::core::convert::TryInto<usize>>::try_into(LAST - START) {
                    Err(_) => panic!(),
                    Ok(len) => len + 1_usize,
                }
            };

            pub const RANGE_INCLUSIVE<const START: $num_t, const LAST: $num_t>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { RANGE_INCLUSIVE_LENGTH::<START, LAST> }, _>(const |i| START + i as $num_t)
            };

            pub(crate) const SLICEINATOR<const N: $num_t>: &[$num_t] = const {
                &[N]
            };

            pub const UNION<const SETS: &'static [&'static [$num_t]]>: &[$num_t] = const {
                let mut onion: Vec<$num_t> = Vec::new();
                let mut i: usize = 0;

                while i < SETS.len() {
                    let mut j: usize = 0;
                    while j < SETS[i].len() {
                        onion.push(SETS[i][j]);
                        j += 1;
                    }
                    i += 1;
                }

                onion.const_make_global()
            };

            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, union) } {
                ($d($set:expr),+ $d(,)?) => {
                    $d crate::$extra_mod::UNION::<{ &[$d($set, )+] }>
                };
            }
            pub use ${ concat($private_macro_prefix, union) } as Union;

            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, range) } {
                ($start:literal..$end:literal) => {
                    $d crate::$extra_mod::RANGE::<$start, $end>
                };
                ($start:literal..=$end:literal) => {
                    $d crate::$extra_mod::RANGE_INCLUSIVE::<$start, $end>
                };
            }
            pub use ${ concat($private_macro_prefix, range) } as Range;
        }

        impl $wrap_t_name<{ const { &[] } }> {
            pub const NEW<const NUM: $num_t>: $wrap_t_name<{ $extra_mod::SLICEINATOR::<NUM> }> = const {
                // TODO this is fucked up
                unsafe { ::core::mem::transmute(NUM) }
            };
        }

        impl<const SET: &'static [$num_t]> $wrap_t_name<SET> {

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

                while i < SET.len() {
                    if SET[i] == value {
                        return true;
                    }
                    i += 1;
                }
                false
            }

            pub const fn inner(self) -> $num_t {
                self.0
            }

            pub const fn sort(self) -> $wrap_t_name<{ $extra_mod::SORT::<SET> }> {
                unsafe { $wrap_t_name::new_unchecked(self.inner()) }
            }

            pub const fn normalize(self) -> $wrap_t_name<{ $extra_mod::NORMALIZE::<SET> }> {
                unsafe { $wrap_t_name::new_unchecked(self.inner()) }
            }
        }

        impl<const A_SET: &'static [$num_t], const B_SET: &'static [$num_t]> ::core::ops::Add<$wrap_t_name<B_SET>> for $wrap_t_name<A_SET> {
            type Output = $wrap_t_name<{ $extra_mod::ADD::<{ A_SET }, { B_SET }> }>;

            fn add(self, rhs: $wrap_t_name<B_SET>) -> Self::Output {
                unsafe { $wrap_t_name::new_unchecked(self.inner() + rhs.inner()) }
            }
        }

        impl<const A_SET: &'static [$num_t], const B_SET: &'static [$num_t]> ::core::ops::Sub<$wrap_t_name<B_SET>> for $wrap_t_name<A_SET> {
            type Output = $wrap_t_name<{ $extra_mod::SUB::<{ A_SET }, { B_SET }> }>;

            fn sub(self, rhs: $wrap_t_name<B_SET>) -> Self::Output {
                unsafe { $wrap_t_name::new_unchecked(self.inner() - rhs.inner()) }
            }
        }

        impl<const A_SET: &'static [$num_t], const B_SET: &'static [$num_t]> ::core::ops::Mul<$wrap_t_name<B_SET>> for $wrap_t_name<A_SET> {
            type Output = $wrap_t_name<{ $extra_mod::MUL::<{ A_SET }, { B_SET }> }>;

            fn mul(self, rhs: $wrap_t_name<B_SET>) -> Self::Output {
                unsafe { $wrap_t_name::new_unchecked(self.inner() * rhs.inner()) }
            }
        }

        impl<const A_SET: &'static [$num_t], const B_SET: &'static [$num_t]> ::core::ops::Div<$wrap_t_name<B_SET>> for $wrap_t_name<A_SET> {
            type Output = $wrap_t_name<{ $extra_mod::DIV::<{ A_SET }, { B_SET }> }>;

            fn div(self, rhs: $wrap_t_name<B_SET>) -> Self::Output {
                unsafe { $wrap_t_name::new_unchecked(self.inner() / rhs.inner()) }
            }
        }

        #[macro_export]
        macro_rules! $wrap_t_name {
            ($set:expr) => {
                $d crate::$wrap_t_name::<{ $set }>
            };
            ($d($elem:expr),+ $d(,)?) => {
                $d crate::$wrap_t_name::<{ &[$d($elem, )+] }>
            };
        }

    )*}
}
pub(crate) use impl_ints;
