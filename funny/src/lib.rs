#![feature(
    const_trait_impl,
    generic_const_args,
    generic_const_items,
    min_generic_const_args,
    adt_const_params,
    unsized_const_params,
    const_closures,
    const_array,
    const_convert,
    maybe_uninit_array_assume_init
)]
#![allow(incomplete_features)]
#![allow(long_running_const_eval)]

mod macros;

macros::impl_ints! {
    [inner_type: u8, wrap_t_name: SetU8, range_fn_name: range_u8, type_macro_name: MSetU8, extra_tcm: extra_tcm_u8, sort_fn_name: into_sorted_u8_array],
    [inner_type: i8, wrap_t_name: SetI8, range_fn_name: range_i8, type_macro_name: MSetI8, extra_tcm: extra_tcm_i8, sort_fn_name: into_sorted_i8_array],
    [inner_type: u16, wrap_t_name: SetU16, range_fn_name: range_u16, type_macro_name: MSetU16, extra_tcm: extra_tcm_u16, sort_fn_name: into_sorted_u16_array],
}
