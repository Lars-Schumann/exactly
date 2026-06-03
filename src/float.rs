crate::macros::float::define_non_nan_float!(
    [base_type: u16, float_type: f16, wrapper_name: NonNaNf16],
    [base_type: u32, float_type: f32, wrapper_name: NonNaNf32],
    [base_type: u64, float_type: f64, wrapper_name: NonNaNf64],
    [base_type: u128, float_type: f128, wrapper_name: NonNaNf128],
);

crate::macros::float::define_helper_macros!(
    [float_type_macro_name: MakeF16, range_name: Rf16, range_type_macro_name: MakeRangeF16, non_nan_float_name: NonNaNf16],
    [float_type_macro_name: MakeF32, range_name: Rf32, range_type_macro_name: MakeRangeF32, non_nan_float_name: NonNaNf32],
    [float_type_macro_name: MakeF64, range_name: Rf64, range_type_macro_name: MakeRangeF64, non_nan_float_name: NonNaNf64],
    [float_type_macro_name: MakeF128, range_name: Rf128, range_type_macro_name: MakeRangeF128, non_nan_float_name: NonNaNf128],
);

crate::macros::int::impl_int_common!([inner_type: crate::float::NonNaNf32, range_type_name: Rf32],);
