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
    const_heap,
    const_destruct
)]
#![allow(incomplete_features)]
#![allow(long_running_const_eval)]

mod macros;

macros::impl_ints! {
    the_dolla: $,
    [inner_type: u8,    wrap_t_name: SetU8,     range_fn_name: range_u8,    type_macro_name: MSetU8,    extra_mod: set_u8,    sort_fn_name: into_sorted_u8_array],
    [inner_type: u16,   wrap_t_name: SetU16,    range_fn_name: range_u16,   type_macro_name: MSetU16,   extra_mod: set_u16,   sort_fn_name: into_sorted_u16_array],
    [inner_type: u32,   wrap_t_name: SetU32,    range_fn_name: range_u32,   type_macro_name: MSetU32,   extra_mod: set_u32,   sort_fn_name: into_sorted_u32_array],
    [inner_type: u64,   wrap_t_name: SetU64,    range_fn_name: range_u64,   type_macro_name: MSetU64,   extra_mod: set_u64,   sort_fn_name: into_sorted_u64_array],
    [inner_type: u128,  wrap_t_name: SetU128,   range_fn_name: range_u128,  type_macro_name: MSetU128,  extra_mod: set_u128,  sort_fn_name: into_sorted_u128_array],
    [inner_type: usize, wrap_t_name: SetUsize,  range_fn_name: range_usize, type_macro_name: MSetUsize, extra_mod: set_usize, sort_fn_name: into_sorted_usize_array],
    [inner_type: i8,    wrap_t_name: SetI8,     range_fn_name: range_i8,    type_macro_name: MSetI8,    extra_mod: set_i8,    sort_fn_name: into_sorted_i8_array],
    [inner_type: i16,   wrap_t_name: SetI16,    range_fn_name: range_i16,   type_macro_name: MSetI16,   extra_mod: set_i16,   sort_fn_name: into_sorted_i16_array],
    [inner_type: i32,   wrap_t_name: SetI32,    range_fn_name: range_i32,   type_macro_name: MSetI32,   extra_mod: set_i32,   sort_fn_name: into_sorted_i32_array],
    [inner_type: i64,   wrap_t_name: SetI64,    range_fn_name: range_i64,   type_macro_name: MSetI64,   extra_mod: set_i64,   sort_fn_name: into_sorted_i64_array],
    [inner_type: i128,  wrap_t_name: SetI128,   range_fn_name: range_i128,  type_macro_name: MSetI128,  extra_mod: set_i128,  sort_fn_name: into_sorted_i128_array],
    [inner_type: isize, wrap_t_name: SetIsize,  range_fn_name: range_isize, type_macro_name: MSetIsize, extra_mod: set_isize, sort_fn_name: into_sorted_isize_array],
}
