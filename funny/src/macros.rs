macro_rules! impl_ints {
    ($([inner_type: $num_t:ident, wrap_t_name: $wrap_t_name:ident, extra_tcm: $extra_tcm:ident, sort_fn_name: $sort_fn_name:ident],)*) => {$(

        pub struct $wrap_t_name<const VALUES: &'static [$num_t]>(pub $num_t);

        mod $extra_tcm {
            pub(super) type const LEN<const SLICE: &'static[$num_t]>: usize = const { SLICE.len()};

            pub(super) type const ADD<const A: &'static[$num_t], const B: &'static[$num_t]>: &[u8] = const {
                &core::array::from_fn::<u8, { ::tcm::usize::MUL::<{ LEN::<A> }, { LEN::<B> }> }, _>(const |i| A[i / B.len()] + B[i % B.len()])
            };

            pub(super) type const SORT<const SLICE: &'static[$num_t]>: &[$num_t] = const {
                let arr: [$num_t; LEN::<SLICE>] = match SLICE.try_into() {
                    Ok(arr) => arr,
                    Err(_) => panic!()
                };
                &::compile_time_sort::$sort_fn_name(arr)
            };
        }

        impl<const VALUES: &'static [$num_t]> $wrap_t_name<VALUES> {

            pub const fn add<const OTHER: &'static [$num_t]>(self, other: $wrap_t_name<OTHER>) -> $wrap_t_name<{ $extra_tcm::ADD::<VALUES, OTHER> }> {
                $wrap_t_name::<{ $extra_tcm::ADD::<VALUES, OTHER> }>(self.0 + other.0)
            }

            pub const fn sort(self) -> $wrap_t_name<{ $extra_tcm::SORT::<VALUES> }> {
                $wrap_t_name::<{ $extra_tcm::SORT::<VALUES> }>(self.0)
            }
        }


    )*}
}
pub(crate) use impl_ints;
