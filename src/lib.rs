#![feature(
    adt_const_params,
    const_array,
    const_closures,
    const_cmp,
    const_convert,
    const_destruct,
    const_heap,
    const_index,
    const_result_trait_fn,
    const_trait_impl,
    generic_const_args,
    generic_const_items,
    import_trait_associated_functions,
    macro_metavar_expr_concat,
    min_generic_const_args,
    unsized_const_params
)]
#![allow(incomplete_features)]
#![allow(long_running_const_eval)]
//
#![no_std]
extern crate alloc;

mod const_helpers;
mod macros;

macros::impl_ints! {
    the_dolla: $,
    [inner_type: u8,    largest_num_t_with_same_signedness: u128, wrap_t_name: SetU8,     range_fn_name: range_u8,    private_macro_prefix: ඞ__private_macro_set_u8_,    extra_mod: set_u8,    sort_fn_name: into_sorted_u8_array],
    [inner_type: u16,   largest_num_t_with_same_signedness: u128, wrap_t_name: SetU16,    range_fn_name: range_u16,   private_macro_prefix: ඞ__private_macro_set_u16_,   extra_mod: set_u16,   sort_fn_name: into_sorted_u16_array],
    [inner_type: u32,   largest_num_t_with_same_signedness: u128, wrap_t_name: SetU32,    range_fn_name: range_u32,   private_macro_prefix: ඞ__private_macro_set_u32_,   extra_mod: set_u32,   sort_fn_name: into_sorted_u32_array],
    [inner_type: u64,   largest_num_t_with_same_signedness: u128, wrap_t_name: SetU64,    range_fn_name: range_u64,   private_macro_prefix: ඞ__private_macro_set_u64_,   extra_mod: set_u64,   sort_fn_name: into_sorted_u64_array],
    [inner_type: u128,  largest_num_t_with_same_signedness: u128, wrap_t_name: SetU128,   range_fn_name: range_u128,  private_macro_prefix: ඞ__private_macro_set_u128_,  extra_mod: set_u128,  sort_fn_name: into_sorted_u128_array],
    [inner_type: usize, largest_num_t_with_same_signedness: usize,wrap_t_name: SetUsize,  range_fn_name: range_usize, private_macro_prefix: ඞ__private_macro_set_usize_, extra_mod: set_usize, sort_fn_name: into_sorted_usize_array],
    [inner_type: i8,    largest_num_t_with_same_signedness: i128, wrap_t_name: SetI8,     range_fn_name: range_i8,    private_macro_prefix: ඞ__private_macro_set_i8_,    extra_mod: set_i8,    sort_fn_name: into_sorted_i8_array],
    [inner_type: i16,   largest_num_t_with_same_signedness: i128, wrap_t_name: SetI16,    range_fn_name: range_i16,   private_macro_prefix: ඞ__private_macro_set_i16_,   extra_mod: set_i16,   sort_fn_name: into_sorted_i16_array],
    [inner_type: i32,   largest_num_t_with_same_signedness: i128, wrap_t_name: SetI32,    range_fn_name: range_i32,   private_macro_prefix: ඞ__private_macro_set_i32_,   extra_mod: set_i32,   sort_fn_name: into_sorted_i32_array],
    [inner_type: i64,   largest_num_t_with_same_signedness: i128, wrap_t_name: SetI64,    range_fn_name: range_i64,   private_macro_prefix: ඞ__private_macro_set_i64_,   extra_mod: set_i64,   sort_fn_name: into_sorted_i64_array],
    [inner_type: i128,  largest_num_t_with_same_signedness: i128, wrap_t_name: SetI128,   range_fn_name: range_i128,  private_macro_prefix: ඞ__private_macro_set_i128_,  extra_mod: set_i128,  sort_fn_name: into_sorted_i128_array],
    [inner_type: isize, largest_num_t_with_same_signedness: isize, wrap_t_name: SetIsize,  range_fn_name: range_isize, private_macro_prefix: ඞ__private_macro_set_isize_, extra_mod: set_isize, sort_fn_name: into_sorted_isize_array],
}
