/// # Safety
///
/// TODO
pub const unsafe trait SureEq: core::marker::ConstParamTy_ + const Eq {}

// SAFETY: we trust std
unsafe impl SureEq for () {}
// SAFETY: we trust std
unsafe impl SureEq for bool {}
// SAFETY: we trust std
unsafe impl SureEq for char {}
// SAFETY: we trust std
unsafe impl SureEq for i8 {}
// SAFETY: we trust std
unsafe impl SureEq for i16 {}
// SAFETY: we trust std
unsafe impl SureEq for i32 {}
// SAFETY: we trust std
unsafe impl SureEq for i64 {}
// SAFETY: we trust std
unsafe impl SureEq for i128 {}
// SAFETY: we trust std
unsafe impl SureEq for isize {}
// SAFETY: we trust std
unsafe impl SureEq for str {}
// SAFETY: we trust std
unsafe impl SureEq for u8 {}
// SAFETY: we trust std
unsafe impl SureEq for u16 {}
// SAFETY: we trust std
unsafe impl SureEq for u32 {}
// SAFETY: we trust std
unsafe impl SureEq for u64 {}
// SAFETY: we trust std
unsafe impl SureEq for u128 {}
// SAFETY: we trust std
unsafe impl SureEq for usize {}

// SAFETY: we trust std
unsafe impl<T: SureEq> SureEq for &T {}
// SAFETY: we trust std
unsafe impl<T: SureEq> SureEq for [T] {}
// SAFETY: we trust std
unsafe impl<T: SureEq, const N: usize> SureEq for [T; N] {}

macro_rules! unsafe_impl_sure_eq_for_tuples {
    ($(($($T:ident),+)),+,) => {
        // SAFETY: we trust std
        $(unsafe impl<$($T: SureEq),+> SureEq for ($($T),+,) {})+
    };
}

// implementation for Tuples up to length 12, because that is as far as std goes for ConstParamTy_
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
