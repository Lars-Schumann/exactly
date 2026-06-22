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

            pub(super) type const UNIQUE_COUNT<const SLICE: &'static[$num_t]>: usize = const {
                let sorted = SORT::<SLICE>;
                match sorted {
                    [] => 0,
                    [first, tail @ ..] => {
                        let mut last_seen = *first;
                        let mut unique_count: usize = 1;
                        let mut i: usize = 0;

                        while i < tail.len(){
                            let current = tail[i];
                            match current == last_seen {
                                true => {},
                                false => {
                                    unique_count += 1;
                                    last_seen = current;
                                }
                            }
                            i += 1;
                        }
                        unique_count
                    }
                }
            };

            pub(super) type const NORMALIZE<const SLICE: &'static[$num_t]>: &[$num_t] = const {
                use core::mem::MaybeUninit;
                let sorted = SORT::<SLICE>;
                match sorted {
                    [] => &[],
                    [first, tail @ ..] => {

                        let mut mu: [MaybeUninit<$num_t>; UNIQUE_COUNT::<SLICE>] = [MaybeUninit::uninit(); UNIQUE_COUNT::<SLICE>];

                        mu[0] = MaybeUninit::new(*first);

                        let mut last_seen = *first;
                        let mut unique_count: usize = 1;
                        let mut i: usize = 0;

                        while i < tail.len(){
                            let current = tail[i];
                            match current == last_seen {
                                true => {},
                                false => {
                                    unique_count += 1;
                                    mu[unique_count - 1] = MaybeUninit::new(current);
                                    last_seen = current;
                                }
                            }
                            i += 1;
                        }
                        &unsafe { MaybeUninit::array_assume_init(mu) }
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
