use core::ops::{Add, Div, Mul, Sub};

#[derive(core::marker::ConstParamTy, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NonNaNf32 {
    pub __private: u32,
}

impl NonNaNf32 {
    pub const fn inner(self) -> f32 {
        f32::from_bits(self.__private)
    }
}

impl NonNaNf32 {
    /// # Safety
    /// value cannot be NaN
    pub const unsafe fn new_unchecked(value: f32) -> Self {
        debug_assert!(!(value.is_nan()));
        Self {
            __private: value.to_bits(),
        }
    }

    pub const fn new(value: f32) -> Option<Self> {
        match value.is_nan() {
            true => None,
            false => Some(
                // SAFETY: we just checked the precondition
                unsafe { Self::new_unchecked(value) },
            ),
        }
    }
}

const impl Add for NonNaNf32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.inner() + rhs.inner()).expect("Addition failed, it evaluated to NaN")
    }
}

const impl Sub for NonNaNf32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.inner() - rhs.inner()).expect("Subtraction failed, it evaluated to NaN")
    }
}

const impl Mul for NonNaNf32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.inner() * rhs.inner()).expect("Multiplication failed, it evaluated to NaN")
    }
}

const impl Div for NonNaNf32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.inner() / rhs.inner()).expect("Division failed, it evaluated to NaN")
    }
}

impl<const LOWER: NonNaNf32, const UPPER: NonNaNf32> Rf32<LOWER, UPPER> {
    #[expect(unused)]
    type const ADD<const N: NonNaNf32, const M: NonNaNf32>: NonNaNf32 = const { N + M };
    #[expect(unused)]
    type const SUB<const N: NonNaNf32, const M: NonNaNf32>: NonNaNf32 = const { N - M };
    #[expect(unused)]
    type const MUL<const N: NonNaNf32, const M: NonNaNf32>: NonNaNf32 = const { N * M };
    #[expect(unused)]
    type const DIV<const N: NonNaNf32, const M: NonNaNf32>: NonNaNf32 = const { N / M };

    /// # Safety
    /// LOWER.inner() <= value.inner() && value.inner() <= UPPER.inner() must hold
    pub const unsafe fn new_unchecked(value: NonNaNf32) -> Self {
        debug_assert!(LOWER.inner() <= value.inner() && value.inner() <= UPPER.inner());
        Self(value)
    }

    pub const fn new(value: NonNaNf32) -> Option<Self> {
        match LOWER.inner() <= value.inner() && value.inner() <= UPPER.inner() {
            true => Some(
                // SAFETY: we just checked the precondition
                unsafe { Self::new_unchecked(value) },
            ),
            false => None,
        }
    }

    pub const fn inner(self) -> NonNaNf32 {
        self.0
    }
}

const impl<const A: NonNaNf32, const B: NonNaNf32, const X: NonNaNf32, const Y: NonNaNf32>
    Add<Rf32<X, Y>> for Rf32<A, B>
{
    type Output = Rf32<{ Self::ADD::<A, X> }, { Self::ADD::<B, Y> }>;

    fn add(self, rhs: Rf32<X, Y>) -> Self::Output {
        unsafe { Self::Output::new_unchecked(self.inner() + rhs.inner()) }
    }
}

pub struct Rf32<const LOWER: NonNaNf32, const UPPER: NonNaNf32>(NonNaNf32);

#[macro_export]
macro_rules! f32 {
    ($value:literal) => {
        const { $crate::float::NonNaNf32::new($value).unwrap() }
    };
}

#[macro_export]
macro_rules! Rf32 {
    ($lower:literal, $upper:literal) => {
        Rf32<
        { const { $crate::float::NonNaNf32::new($lower).unwrap() } },
        { const { $crate::float::NonNaNf32::new($upper).unwrap() } },
    >
    };
}
