/// # Safety
///
/// You may only implement this trait for a type if it meets all of the following conditions:
/// 1. This type must not be a union, pointer, or reference.
/// 2. Condition #1 must hold recursively for this types fields or variants (if it has any).
/// 3. This types [`PartialEq`] impl must be equivalent to bytewise comparing all non-padding bytes of the data.
///
/// If the type fails to meet any one of these, implementing this trait is considered to be UB.
pub unsafe trait SureEq: core::marker::ConstParamTy_ + const Eq {}

// SAFETY:
// For the primitives below, these are the reasons why each required condition is satisfied.
// #1 Because this is a primitive type that is not any of union, pointer, or reference.
// #2 Because this is a primitive type without any fields or variants.
// #3 Because we trust `std` to have a `PartialEq` implementation that is equivalent to this requirement.

// SAFETY: see above
unsafe impl SureEq for () {}
// SAFETY: see above
unsafe impl SureEq for bool {}
// SAFETY: see above
unsafe impl SureEq for char {}
// SAFETY: see above
unsafe impl SureEq for i8 {}
// SAFETY: see above
unsafe impl SureEq for i16 {}
// SAFETY: see above
unsafe impl SureEq for i32 {}
// SAFETY: see above
unsafe impl SureEq for i64 {}
// SAFETY: see above
unsafe impl SureEq for i128 {}
// SAFETY: see above
unsafe impl SureEq for isize {}
// SAFETY: see above
unsafe impl SureEq for u8 {}
// SAFETY: see above
unsafe impl SureEq for u16 {}
// SAFETY: see above
unsafe impl SureEq for u32 {}
// SAFETY: see above
unsafe impl SureEq for u64 {}
// SAFETY: see above
unsafe impl SureEq for u128 {}
// SAFETY: see above
unsafe impl SureEq for usize {}

// SAFETY: TODO
unsafe impl<T: SureEq, const N: usize> SureEq for [T; N] {}

macro_rules! unsafe_impl_sure_eq_for_tuples {
    ($(($($T:ident),+)),+,) => {
        // SAFETY: TODO
        $(unsafe impl<$($T: SureEq),+> SureEq for ($($T),+,) {})+
    };
}

// implementations for tuples up to length 12, because that is as far as std goes for ConstParamTy_
unsafe_impl_sure_eq_for_tuples! {
    (A),
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F),
    (A, B, C, D, E, F, G),
    (A, B, C, D, E, F, G, H),
    (A, B, C, D, E, F, G, H, I),
    (A, B, C, D, E, F, G, H, I, J),
    (A, B, C, D, E, F, G, H, I, J, K),
    (A, B, C, D, E, F, G, H, I, J, K, L),
}
