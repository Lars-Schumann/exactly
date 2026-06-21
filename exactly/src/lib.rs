#![feature(
    const_cmp,
    const_ops,
    const_trait_impl,
    generic_const_args,
    generic_const_items,
    inherent_associated_types,
    min_adt_const_params,
    min_generic_const_args
)]
#![allow(incomplete_features)]

mod macros;

macros::impl_int_common!(
    [inner_type: u8,    range_t_name: RangedU8,     range_t_alias: ru8,     exact_fn_name: exact_u8],
    [inner_type: u16,   range_t_name: RangedU16,    range_t_alias: ru16,    exact_fn_name: exact_u16],
    [inner_type: u32,   range_t_name: RangedU32,    range_t_alias: ru32,    exact_fn_name: exact_u32],
    [inner_type: u64,   range_t_name: RangedU64,    range_t_alias: ru64,    exact_fn_name: exact_u64],
    [inner_type: u128,  range_t_name: RangedU128,   range_t_alias: ru128,   exact_fn_name: exact_u128],
    [inner_type: usize, range_t_name: RangedUsize,  range_t_alias: rusize,  exact_fn_name: exact_usize],
    [inner_type: i8,    range_t_name: RangedI8,     range_t_alias: ri8,     exact_fn_name: exact_i8],
    [inner_type: i16,   range_t_name: RangedI16,    range_t_alias: ri16,    exact_fn_name: exact_i16],
    [inner_type: i32,   range_t_name: RangedI32,    range_t_alias: ri32,    exact_fn_name: exact_i32],
    [inner_type: i64,   range_t_name: RangedI64,    range_t_alias: ri64,    exact_fn_name: exact_i64],
    [inner_type: i128,  range_t_name: RangedI128,   range_t_alias: ri128,   exact_fn_name: exact_i128],

    [inner_type: isize, range_t_name: RangedIsize,  range_t_alias: risize,  exact_fn_name: exact_isize],
);

macros::impl_int_unsigned!(
    [inner_type: u8,    range_t_name: RangedU8],
    [inner_type: u16,   range_t_name: RangedU16],
    [inner_type: u32,   range_t_name: RangedU32],
    [inner_type: u64,   range_t_name: RangedU64],
    [inner_type: u128,  range_t_name: RangedU128],
    [inner_type: usize, range_t_name: RangedUsize],
);

macros::impl_int_signed!(
    [inner_type: i8,    range_t_name: RangedI8,     extra_tcm: extra_tcm_i8],
    [inner_type: i16,   range_t_name: RangedI16,    extra_tcm: extra_tcm_i16],
    [inner_type: i32,   range_t_name: RangedI32,    extra_tcm: extra_tcm_i32],
    [inner_type: i64,   range_t_name: RangedI64,    extra_tcm: extra_tcm_i64],
    [inner_type: i128,  range_t_name: RangedI128,   extra_tcm: extra_tcm_i128],
    [inner_type: isize, range_t_name: RangedIsize,  extra_tcm: extra_tcm_isize],
);
