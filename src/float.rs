use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Sub};

macro_rules! define_non_nan_float {
    ($([base_type: $base_type:ty, float_type: $float_type:ty, wrapper_name: $wrapper_name:ident],)*) => {$(
        #[derive(core::marker::ConstParamTy, Debug, Clone, Copy)]
        #[derive_const(PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $wrapper_name {
            pub __private: $base_type,
        }

        impl $wrapper_name {
            pub const fn inner(self) -> $float_type {
                <$float_type>::from_bits(self.__private)
            }
        }

        impl $wrapper_name {
            /// # Safety
            /// value cannot be NaN
            pub const unsafe fn new_unchecked(value: $float_type) -> Self {
                debug_assert!(!(value.is_nan()));
                Self {
                    __private: value.to_bits(),
                }
            }

            pub const fn new(value: $float_type) -> Option<Self> {
                match value.is_nan() {
                    true => None,
                    false => Some(
                        // SAFETY: we just checked the precondition
                        unsafe { Self::new_unchecked(value) },
                    ),
                }
            }
        }

        const impl PartialOrd for $wrapper_name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                let (lhs, rhs) = (self.inner(), other.inner());

                if lhs == -0.0 && rhs == 0.0 {
                    return Some(Ordering::Less);
                }

                if lhs == 0.0 && rhs == -0.0 {
                    return Some(Ordering::Greater);
                }

                lhs.partial_cmp(&rhs)
            }
        }

        const impl Add for $wrapper_name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.inner() + rhs.inner()).expect("Addition failed, it evaluated to NaN")
            }
        }

        const impl Sub for $wrapper_name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.inner() - rhs.inner())
                    .expect("Subtraction failed, it evaluated to NaN")
            }
        }

        const impl Mul for $wrapper_name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(self.inner() * rhs.inner())
                    .expect("Multiplication failed, it evaluated to NaN")
            }
        }

        const impl Div for $wrapper_name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self::new(self.inner() / rhs.inner()).expect("Division failed, it evaluated to NaN")
            }
        }
    )*};
}

macro_rules! define_helper_macros {
    ($([float_type_macro_name: $float_type_macro_name:ident, range_name: $range_name:ident, range_type_macro_name: $range_type_macro_name:ident, non_nan_float_name: $non_nan_float_name:ident],)*) => {$(
        #[macro_export]
        macro_rules! $float_type_macro_name {
            ($value:expr) => {
                const { $crate::float::$non_nan_float_name::new($value).unwrap() }
            };
        }

        #[macro_export]
        macro_rules! $range_type_macro_name {
                    ($lower:expr, $upper:expr) => {
                        $range_name<
                            { const { $crate::float::$non_nan_float_name::new($lower).unwrap() } },
                            { const { $crate::float::$non_nan_float_name::new($upper).unwrap() } },
                        >
                    };
                }
    )*};
}

define_non_nan_float!(
    [base_type: u16, float_type: f16, wrapper_name: NonNaNf16],
    [base_type: u32, float_type: f32, wrapper_name: NonNaNf32],
    [base_type: u64, float_type: f64, wrapper_name: NonNaNf64],
    [base_type: u128, float_type: f128, wrapper_name: NonNaNf128],
);

define_helper_macros!(
    [float_type_macro_name: MakeF16, range_name: Rf16, range_type_macro_name: MakeRangeF16, non_nan_float_name: NonNaNf16],
    [float_type_macro_name: MakeF32, range_name: Rf32, range_type_macro_name: MakeRangeF32, non_nan_float_name: NonNaNf32],
    [float_type_macro_name: MakeF64, range_name: Rf64, range_type_macro_name: MakeRangeF64, non_nan_float_name: NonNaNf64],
    [float_type_macro_name: MakeF128, range_name: Rf128, range_type_macro_name: MakeRangeF128, non_nan_float_name: NonNaNf128],
);

crate::macros::impl_int_common!([inner_type: crate::float::NonNaNf32, range_type_name: Rf32],);
