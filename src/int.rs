crate::macros::int::impl_int_common!(
    [inner_type: u8, range_type_name: Ru8],
    [inner_type: u16, range_type_name: Ru16 ],
    [inner_type: u32, range_type_name: Ru32 ],
    [inner_type: u64, range_type_name: Ru64 ],
    [inner_type: u128, range_type_name: Ru128 ],
    [inner_type: i8, range_type_name: Ri8 ],
    [inner_type: i16, range_type_name: Ri16 ],
    [inner_type: i32, range_type_name: Ri32 ],
    [inner_type: i64, range_type_name: Ri64 ],
    [inner_type: i128, range_type_name: Ri128 ],

    //[inner_type: crate::float::NonNaNf32, range_type_name: Rf32],
);

crate::macros::int::impl_int_unsigned!(u8, u16, u32, u64, u128,);

crate::macros::int::impl_int_signed!(i8, i16, i32, i64, i128,);
