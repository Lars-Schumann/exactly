macro_rules! impl_ints {
    ($([inner_type: $num_t:ident, wrap_t_name: $wrap_t_name:ident, extra_tcm: $extra_tcm:ident, sort_fn_name: $sort_fn_name:ident],)*) => {$(

        #[derive(Debug, Copy, Clone,)]
        #[repr(transparent)]
        pub struct $wrap_t_name<const VALUES: &'static [$num_t]>(pub $num_t);

        mod $extra_tcm {
            pub(super) type const LEN<const SLICE: &'static[$num_t]>: usize = const { SLICE.len()};

            pub(super) type const ADD<const A: &'static[$num_t], const B: &'static[$num_t]>: &[u8] = const {
                &core::array::from_fn::<u8, { ::tcm::usize::MUL::<{ LEN::<A> }, { LEN::<B> }> }, _>(const |i| A[i / B.len()] + B[i % B.len()])
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
                            match previous == current {
                                true => {},
                                false => {
                                    unique_element_count += 1;
                                }
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
                            match previous == current {
                                true => {},
                                false => {
                                    unique_count += 1;
                                    normalized[unique_count - 1] = MU::new(current);
                                }
                            }
                            i += 1;
                        }
                        &unsafe { MU::array_assume_init(normalized) }
                    }
                }
            };
        }

        impl<const VALUES: &'static [$num_t]> $wrap_t_name<VALUES> {

            pub const fn add<const OTHER: &'static [$num_t]>(self, other: $wrap_t_name<OTHER>) -> $wrap_t_name<{ $extra_tcm::ADD::<VALUES, OTHER> }> {
                $wrap_t_name::<{ $extra_tcm::ADD::<VALUES, OTHER> }>(self.0 + other.0)
            }

            pub const fn sort(self) -> $wrap_t_name<{ $extra_tcm::SORT::<VALUES> }> {
                $wrap_t_name::<{ $extra_tcm::SORT::<VALUES> }>(self.0)
            }

            pub const fn normalize(self) -> $wrap_t_name<{ $extra_tcm::NORMALIZE::<VALUES> }> {
                $wrap_t_name::<{ $extra_tcm::NORMALIZE::<VALUES> }>(self.0)
            }
        }


    )*}
}
pub(crate) use impl_ints;
