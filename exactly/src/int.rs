crate::macros::int::impl_int_common!(
    [inner_type: u8, range_t_name: Ru8, exact_fn_name: exact_u8],
    [inner_type: u16, range_t_name: Ru16, exact_fn_name: exact_u16],
    [inner_type: u32, range_t_name: Ru32, exact_fn_name: exact_u32 ],
    [inner_type: u64, range_t_name: Ru64, exact_fn_name: exact_u64],
    [inner_type: u128, range_t_name: Ru128, exact_fn_name: exact_u128 ],
    [inner_type: i8, range_t_name: Ri8, exact_fn_name: exact_i8],
    [inner_type: i16, range_t_name: Ri16, exact_fn_name: exact_i16 ],
    [inner_type: i32, range_t_name: Ri32, exact_fn_name: exact_i32 ],
    [inner_type: i64, range_t_name: Ri64, exact_fn_name: exact_i64 ],
    [inner_type: i128, range_t_name: Ri128, exact_fn_name: exact_i128 ],

    //[inner_type: crate::float::NonNaNf32, range_t_name: Rf32],
);

crate::macros::int::impl_int_unsigned!(
    [inner_type: u8, range_t_name: Ru8],
    [inner_type: u16, range_t_name: Ru16],
    [inner_type: u32, range_t_name: Ru32],
    [inner_type: u64, range_t_name: Ru64],
    [inner_type: u128, range_t_name: Ru128],
);

crate::macros::int::impl_int_signed!(
    [inner_type: i8, range_t_name: Ri8],
    [inner_type: i16, range_t_name: Ri16],
    [inner_type: i32, range_t_name: Ri32],
    [inner_type: i64, range_t_name: Ri64],
    [inner_type: i128, range_t_name: Ri128],
);
