#![feature(
    adt_const_params,
    const_array,
    const_closures,
    const_cmp,
    const_convert,
    const_destruct,
    const_heap,
    const_index,
    const_ops,
    const_result_trait_fn,
    const_trait_impl,
    freeze,
    generic_const_args,
    generic_const_items,
    generic_const_parameter_types,
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

pub mod base;
mod const_helpers;

mod macros;

macros::impl_ints! {
    the_dolla: $,
    [inner_type: u8,    t_alias: SetU8,     largest_num_t_with_same_signedness: u128,   wrap_t_name: SetU8,      private_macro_prefix: ඞඞ__private_macro_set_u8_,    extra_mod: set_u8    ],
    [inner_type: u16,   t_alias: SetU16,    largest_num_t_with_same_signedness: u128,   wrap_t_name: SetU16,     private_macro_prefix: ඞඞ__private_macro_set_u16_,   extra_mod: set_u16   ],
    [inner_type: u32,   t_alias: SetU32,    largest_num_t_with_same_signedness: u128,   wrap_t_name: SetU32,     private_macro_prefix: ඞඞ__private_macro_set_u32_,   extra_mod: set_u32   ],
    [inner_type: u64,   t_alias: SetU64,    largest_num_t_with_same_signedness: u128,   wrap_t_name: SetU64,     private_macro_prefix: ඞඞ__private_macro_set_u64_,   extra_mod: set_u64   ],
    [inner_type: u128,  t_alias: SetU128,   largest_num_t_with_same_signedness: u128,   wrap_t_name: SetU128,    private_macro_prefix: ඞඞ__private_macro_set_u128_,  extra_mod: set_u128  ],
    [inner_type: usize, t_alias: SetUsize,  largest_num_t_with_same_signedness: usize,  wrap_t_name: SetUsize,   private_macro_prefix: ඞඞ__private_macro_set_usize_, extra_mod: set_usize ],
    [inner_type: i8,    t_alias: SetI8,     largest_num_t_with_same_signedness: i128,   wrap_t_name: SetI8,      private_macro_prefix: ඞඞ__private_macro_set_i8_,    extra_mod: set_i8    ],
    [inner_type: i16,   t_alias: SetI16,    largest_num_t_with_same_signedness: i128,   wrap_t_name: SetI16,     private_macro_prefix: ඞඞ__private_macro_set_i16_,   extra_mod: set_i16   ],
    [inner_type: i32,   t_alias: SetI32,    largest_num_t_with_same_signedness: i128,   wrap_t_name: SetI32,     private_macro_prefix: ඞඞ__private_macro_set_i32_,   extra_mod: set_i32   ],
    [inner_type: i64,   t_alias: SetI64,    largest_num_t_with_same_signedness: i128,   wrap_t_name: SetI64,     private_macro_prefix: ඞඞ__private_macro_set_i64_,   extra_mod: set_i64   ],
    [inner_type: i128,  t_alias: SetI128,   largest_num_t_with_same_signedness: i128,   wrap_t_name: SetI128,    private_macro_prefix: ඞඞ__private_macro_set_i128_,  extra_mod: set_i128  ],
    [inner_type: isize, t_alias: SetIsize,  largest_num_t_with_same_signedness: isize,  wrap_t_name: SetIsize,   private_macro_prefix: ඞඞ__private_macro_set_isize_, extra_mod: set_isize ],
}
