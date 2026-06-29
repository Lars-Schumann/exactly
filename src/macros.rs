macro_rules! impl_ints {
    (the_dolla: $d:tt, $([inner_type: $num_t:ident, largest_num_t_with_same_signedness: $largest_num_t_with_same_signedness:ident, wrap_t_name: $wrap_t_name:ident, range_fn_name: $range_fn_name:ident, private_macro_prefix: $private_macro_prefix:ident, extra_mod: $extra_mod:ident, sort_fn_name: $sort_fn_name:ident],)*) => {$(

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
            pub const SORT<const SET: &'static[$num_t]>: &[$num_t] = const {
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

            const RANGE_LENGTH_HELPER<const MIN: $num_t, const MAX: $num_t, const IS_INCLUSIVE: bool>: usize = const {
                match <$largest_num_t_with_same_signedness as ::core::convert::TryInto<usize>>::try_into($largest_num_t_with_same_signedness::from(MAX).strict_sub($largest_num_t_with_same_signedness::from(MIN))) {
                    Err(_) => panic!(),
                    Ok(len) => len.strict_add(usize::from(IS_INCLUSIVE)),
                }
            };

            pub const RANGE_HELPER<const MIN: $num_t, const MAX: $num_t, const IS_INCLUSIVE: bool>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { RANGE_LENGTH_HELPER::<MIN, MAX, IS_INCLUSIVE> }, _>(const |i| $num_t::try_from($largest_num_t_with_same_signedness::from(MIN) + <usize as TryInto<$largest_num_t_with_same_signedness>>::try_into(i).ok().unwrap()).ok().unwrap())
            };

            pub const RANGE             <const START: $num_t, const END : $num_t>: &[$num_t] = RANGE_HELPER::<                START ,                 END   , false >;
            pub const RANGE_FROM        <const START: $num_t                    >: &[$num_t] = RANGE_HELPER::<                START , const { $num_t::MAX } , true  >;
            // pub const RANGE_FULL     <                                       >: &[$num_t] = RANGE_HELPER::<const { $num_t::MIN } , const { $num_t::MAX } , true  >;
            pub const RANGE_INCLUSIVE   <const START: $num_t, const LAST: $num_t>: &[$num_t] = RANGE_HELPER::<                START ,                 LAST  , true  >;
            pub const RANGE_TO          <                     const END : $num_t>: &[$num_t] = RANGE_HELPER::<const { $num_t::MIN } ,                 END   , false >;
            pub const RANGE_TO_INCLUSIVE<                     const LAST: $num_t>: &[$num_t] = RANGE_HELPER::<const { $num_t::MIN } ,                 LAST  , true  >;

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

            pub const INTERSECTION<const SETS: &'static [&'static [$num_t]]>: &[$num_t] = const { 'out: {
                let [first_set, ..] = SETS else {
                    break 'out &[];
                };

                let mut smallest_set_index = 0;
                let mut smallest_set_length = first_set.len();

                let mut i: usize = 1;

                while i < SETS.len() {
                    let current_set_length = SETS[i].len();
                    if current_set_length < smallest_set_length {
                        smallest_set_length = current_set_length;
                        smallest_set_index = i;
                    }
                    i += 1;
                }

                let mut intersection: Vec<$num_t> = Vec::with_capacity(smallest_set_length);

                let mut k: usize = 0;

                while k < smallest_set_length {
                    intersection.push(SETS[smallest_set_index][k]);
                    k += 1;
                }

                let mut j: usize = 0;

                while j < SETS.len() {
                    if j == smallest_set_index {
                        j += 1;
                        continue;
                    }
                    intersection_of(&mut intersection, SETS[j]);
                    j += 1;
                }

                intersection.const_make_global()
            }};

            const fn intersection_of(running_intersection: &mut Vec<$num_t>, new_set: &[$num_t]) {
                let mut i: usize = 0;

                'outer: while i < running_intersection.len() {
                    let mut j: usize = 0;
                    while j < new_set.len() {
                        if running_intersection[i] == new_set[j] {
                            i += 1;
                            continue 'outer;
                        }
                        j += 1;
                    }
                    swap_remove(running_intersection, i);
                }
            }

            const fn swap_remove(_self: &mut Vec<$num_t>, index: usize) -> $num_t {

                const fn assert_failed(_index: usize, _len: usize) -> ! {
                    panic!("swap_remove index should be < len but isn't");
                }

                let len = _self.len();
                if index >= len {
                    assert_failed(index, len);
                }
                unsafe {
                    // We replace self[index] with the last element. Note that if the
                    // bounds check above succeeds there must be a last element (which
                    // can be self[index] itself).
                    let value = core::ptr::read(_self.as_ptr().add(index));
                    let base_ptr = _self.as_mut_ptr();
                    core::ptr::copy(base_ptr.add(len - 1), base_ptr.add(index), 1);
                    _self.set_len(len - 1);
                    value
                }
            }

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
            macro_rules! ${ concat($private_macro_prefix, intersection) } {
                ($d($set:expr),+ $d(,)?) => {
                    $d crate::$extra_mod::INTERSECTION::<{ &[$d($set, )+] }>
                };
            }
            pub use ${ concat($private_macro_prefix, intersection) } as Intersection;

            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, range) } {
                ( $start:literal ..  $end:literal  ) => { $d crate::$extra_mod::RANGE::             <$start, $end>  };
                ( $start:literal ..                ) => { $d crate::$extra_mod::RANGE_FROM::        <$start>        };
                // (                ..             ) => { $d crate::$extra_mod::RANGE_FULL                          };
                ( $start:literal ..= $last:literal ) => { $d crate::$extra_mod::RANGE_INCLUSIVE::   <$start, $last> };
                (                ..  $end:literal  ) => { $d crate::$extra_mod::RANGE_TO::          <$end>          };
                (                ..= $last:literal ) => { $d crate::$extra_mod::RANGE_TO_INCLUSIVE::<$last>         };
            }
            pub use ${ concat($private_macro_prefix, range) } as Range;
        }

        impl $wrap_t_name<{ const { &[] } }> {
            pub const NEW<const NUM: $num_t>: $wrap_t_name<{ $extra_mod::SLICEINATOR::<NUM> }> = const {
                const { $wrap_t_name::new(NUM).expect("This should be infallible, please file a bug report.") }
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
