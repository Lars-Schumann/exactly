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
    //import_trait_associated_functions,
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
    [num_t: u8,     unsigned_num_t: u8,      signed_num_t: i8,      t_alias: SetU8,     wide_num_t: u16,     private_macro_prefix: ඞඞ__private_macro_set_u8_,    extra_mod: set_u8    ],
    [num_t: u16,    unsigned_num_t: u16,     signed_num_t: i16,     t_alias: SetU16,    wide_num_t: u32,     private_macro_prefix: ඞඞ__private_macro_set_u16_,   extra_mod: set_u16   ],
    [num_t: u32,    unsigned_num_t: u32,     signed_num_t: i32,     t_alias: SetU32,    wide_num_t: u64,     private_macro_prefix: ඞඞ__private_macro_set_u32_,   extra_mod: set_u32   ],
    [num_t: u64,    unsigned_num_t: u64,     signed_num_t: i64,     t_alias: SetU64,    wide_num_t: u128,    private_macro_prefix: ඞඞ__private_macro_set_u64_,   extra_mod: set_u64   ],
    [num_t: u128,   unsigned_num_t: u128,    signed_num_t: i128,    t_alias: SetU128,   wide_num_t: u128,    private_macro_prefix: ඞඞ__private_macro_set_u128_,  extra_mod: set_u128  ],
    [num_t: usize,  unsigned_num_t: usize,   signed_num_t: isize,   t_alias: SetUsize,  wide_num_t: usize,   private_macro_prefix: ඞඞ__private_macro_set_usize_, extra_mod: set_usize ],
    [num_t: i8,     unsigned_num_t: u8,      signed_num_t: i8,      t_alias: SetI8,     wide_num_t: i16,     private_macro_prefix: ඞඞ__private_macro_set_i8_,    extra_mod: set_i8    ],
    [num_t: i16,    unsigned_num_t: u16,     signed_num_t: i16,     t_alias: SetI16,    wide_num_t: i32,     private_macro_prefix: ඞඞ__private_macro_set_i16_,   extra_mod: set_i16   ],
    [num_t: i32,    unsigned_num_t: u32,     signed_num_t: i32,     t_alias: SetI32,    wide_num_t: i64,     private_macro_prefix: ඞඞ__private_macro_set_i32_,   extra_mod: set_i32   ],
    [num_t: i64,    unsigned_num_t: u64,     signed_num_t: i64,     t_alias: SetI64,    wide_num_t: i128,    private_macro_prefix: ඞඞ__private_macro_set_i64_,   extra_mod: set_i64   ],
    [num_t: i128,   unsigned_num_t: u128,    signed_num_t: i128,    t_alias: SetI128,   wide_num_t: i128,    private_macro_prefix: ඞඞ__private_macro_set_i128_,  extra_mod: set_i128  ],
    [num_t: isize,  unsigned_num_t: usize,   signed_num_t: isize,   t_alias: SetIsize,  wide_num_t: isize,   private_macro_prefix: ඞඞ__private_macro_set_isize_, extra_mod: set_isize ],
}
