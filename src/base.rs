use core::marker::Destruct;
use core::marker::Freeze;

use crate::set;
use crate::sure_eq::SureEq;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Sure<T: SureEq + 'static, const SET: &'static [T]>(T);

impl<T> Sure<T, { set::EMPTY::<T> }>
where
    T: Copy + const Ord + Freeze + SureEq + const Destruct + 'static,
{
    pub const NEW<const NUM: T>: Sure<T, { set::SLICEINATOR::<T, NUM> }> = const {
        const { Sure::new(NUM).expect("This should be infallible, please file a bug report.") }
    };
}

impl<T, const SET: &'static [T]> Sure<T, SET>
where
    T: Copy + const Destruct + Freeze + SureEq + const Ord + 'static,
{
    pub const SET: &'static [T] = SET;

    #[must_use]
    pub const fn set(self) -> &'static [T] {
        SET
    }

    pub const fn new(value: T) -> Option<Self> {
        match Self::contains(&value) {
            true => Some(
                // SAFETY: we just checked precondition #1: `Self::contains(value)`
                unsafe { Self::new_unchecked(value) },
            ),
            false => None,
        }
    }

    /// # Safety
    ///
    /// One of the following conditions must hold, they are all logically equivalent:\
    /// 1. `Self::contains(value)`\
    /// 2. `Self::new(value).is_some()`\
    /// 3. `Self::SET` contains `value`
    #[must_use]
    pub const unsafe fn new_unchecked(value: T) -> Self {
        debug_assert!(
            Self::contains(&value),
            "Tried to create a Sure with a value thats not contained in its SET, this is UB."
        );
        Self(value)
    }

    pub const fn contains(value: &T) -> bool {
        crate::const_helpers::ext_slice_contains(SET, value)
    }

    #[must_use]
    pub const fn inner(self) -> T {
        self.0
    }

    #[must_use]
    pub const fn sort(self) -> Sure<T, { set::SORT::<T, SET> }> {
        // SAFETY: `SORT` only sorts the elements in `SET`, so it's output will have identical elements.
        unsafe { self.cast_unchecked() }
    }

    #[must_use]
    pub const fn normalize(self) -> Sure<T, { set::NORMALIZE::<T, SET> }> {
        // SAFETY: `NORMALIZE` only sorts and deduplicates the elements in `SET`, so it's output will have identical elements.
        // We rely on a sensible `Eq` impl for this, which currently isn't anywhere in any of the trait bounds.
        // So this whole thing is technically unsound, but I'll fix that later.
        unsafe { self.cast_unchecked() }
    }

    #[must_use]
    pub const fn widen<const SUPER_SET: &'static [T]>(self) -> Sure<T, SUPER_SET> {
        const {
            assert!(
                crate::const_helpers::ext_slice_is_subset(SET, SUPER_SET),
                "Tried to widen a Sure which failed because the target's SET isn't a superset of the original."
            );
        }
        // SAFETY: We just asserted that `SET` is a subset of `SUPER_SET`
        unsafe { self.cast_unchecked() }
    }

    #[must_use]
    pub const fn cast<const NEW_SET: &'static [T]>(self) -> Option<Sure<T, NEW_SET>> {
        match Sure::<T, NEW_SET>::contains(&self.inner()) {
            true => Some(
                // SAFETY: we just checked precondition #1: `Self::contains(value)`
                unsafe { self.cast_unchecked() },
            ),
            false => None,
        }
    }

    /// # SAFETY
    ///
    /// This inherits the preconditions from `Sure<T, NEW_SET>::new_unchecked(self.inner())`.\
    /// The most common way to argue this is by making sure that `SET` has identical elements to `NEW_SET`,\
    /// or that it's elements are a subset of `NEW_SET`.
    #[must_use]
    pub const unsafe fn cast_unchecked<const NEW_SET: &'static [T]>(self) -> Sure<T, NEW_SET> {
        // SAFETY: we pass the preconditions to the caller
        unsafe { Sure::new_unchecked(self.inner()) }
    }
}
