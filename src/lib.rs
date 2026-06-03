#![feature(
    macro_metavar_expr_concat,
    min_generic_const_args,
    generic_const_args,
    generic_const_items,
    inherent_associated_types,
    pattern_types,
    pattern_type_macro,
    const_ops,
    const_trait_impl,
    const_cmp,
    min_adt_const_params,
    derive_const,
    f16,
    f128
)]
#![allow(incomplete_features)]

pub mod float;
pub mod int;
pub(crate) mod macros;

#[macro_export]
macro_rules! to_pattern_type {
    (let $name:ident: $ty:ident is $lower:literal..=$upper:literal = $value:expr) => {
        {
            let __assert_type: $crate::int::${concat(R,$ty)}::<$lower, $upper> = $value;
        }
        let $name: pattern_type!(::core::primitive::$ty is $lower..=$upper) = unsafe { ::core::mem::transmute($value) };
    };
}
