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
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                let (lhs, rhs) = (self.inner(), other.inner());

                if lhs == -0.0 && rhs == 0.0 {
                    return Some(::core::cmp::Ordering::Less);
                }

                if lhs == 0.0 && rhs == -0.0 {
                    return Some(::core::cmp::Ordering::Greater);
                }

                lhs.partial_cmp(&rhs)
            }
        }

        const impl ::core::ops::Add for $wrapper_name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.inner() + rhs.inner()).expect("Addition failed, it evaluated to NaN")
            }
        }

        const impl ::core::ops::Sub for $wrapper_name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.inner() - rhs.inner())
                    .expect("Subtraction failed, it evaluated to NaN")
            }
        }

        const impl ::core::ops::Mul for $wrapper_name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(self.inner() * rhs.inner())
                    .expect("Multiplication failed, it evaluated to NaN")
            }
        }

        const impl ::core::ops::Div for $wrapper_name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self::new(self.inner() / rhs.inner()).expect("Division failed, it evaluated to NaN")
            }
        }
    )*};
}
pub(crate) use define_non_nan_float;

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
pub(crate) use define_helper_macros;
