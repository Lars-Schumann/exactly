use core::marker::ConstParamTy_;
use core::marker::Destruct;
use core::marker::Freeze;

use alloc::vec::Vec;

pub(crate) const LENGTH<T: ConstParamTy_ + 'static, const SET: &'static [T]>: usize =
    const { SET.len() };
pub(crate) const CARTESIAN_LENGTH<
    T: ConstParamTy_ + 'static,
    U: ConstParamTy_ + 'static,
    const A: &'static [T],
    const B: &'static [U],
>: usize = const { A.len() * B.len() };

pub const SORT<T: Copy + const Ord + Freeze + ConstParamTy_ + 'static, const SET: &'static [T]>:
    &[T] = const {
    let arr: [T; LENGTH::<T, SET>] = match SET.try_into() {
        Ok(arr) => arr,
        Err(_) => unreachable!(),
    };
    &crate::const_helpers::sort(arr)
};

pub const NORMALIZE<
    T: Copy + const Ord + ConstParamTy_ + Freeze + const Destruct + 'static,
    const SET: &'static [T],
>: &[T] = const {
    'out: {
        let set_sorted = SORT::<T, SET>;
        let mut normalized: Vec<T> = Vec::new();

        let [first, ..] = set_sorted else {
            break 'out &[];
        };

        normalized.push(*first);

        let mut i: usize = 1;

        while i < set_sorted.len() {
            let (previous, current) = (set_sorted[i - 1], set_sorted[i]);
            if previous != current {
                normalized.push(current);
            }
            i += 1;
        }
        normalized.const_make_global()
    }
};

pub const UNION<T: Freeze + 'static + ConstParamTy_ + Copy, const SETS: &'static [&'static [T]]>:
    &[T] = const {
    let mut onion: Vec<T> = Vec::new();
    let mut i: usize = 0;

    while i < SETS.len() {
        let mut j: usize = 0;
        while j < SETS[i].len() {
            onion.push(SETS[i][j]);
            j += 1;
        }
        i += 1;
    }

    onion.const_make_global()
};

pub const INTERSECTION<
    T: Freeze + 'static + ConstParamTy_ + Copy + const PartialEq + const Destruct,
    const SETS: &'static [&'static [T]],
>: &[T] = const {
    'out: {
        let [first_set, ..] = SETS else {
            break 'out &[];
        };

        let mut intersection: Vec<T> = Vec::with_capacity(first_set.len());

        let mut i: usize = 0;
        while i < first_set.len() {
            intersection.push(first_set[i]);
            i += 1;
        }

        let mut j: usize = 1;

        while j < SETS.len() {
            crate::const_helpers::ext_vec_reduce_to_intersection_with(&mut intersection, SETS[j]);
            j += 1;
        }

        intersection.const_make_global()
    }
};

pub(crate) const EMPTY<T: 'static>: &[T] = &[];

pub(crate) const SLICEINATOR<T: 'static + ConstParamTy_ + Freeze, const NUM: T>: &[T] =
    const { &[NUM] };

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Sure<T: ConstParamTy_ + 'static, const SET: &'static [T]>(T);

impl<T> Sure<T, { EMPTY::<T> }>
where
    T: Copy + const Ord + Freeze + ConstParamTy_ + const Destruct + 'static,
{
    pub const NEW<const NUM: T>: Sure<T, { SLICEINATOR::<T, NUM> }> = const {
        const { Sure::new(NUM).expect("This should be infallible, please file a bug report.") }
    };
}

impl<T, const SET: &'static [T]> Sure<T, SET>
where
    T: Copy + const Ord + Freeze + ConstParamTy_ + const Destruct + 'static,
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
    pub const fn sort(self) -> Sure<T, { SORT::<T, SET> }> {
        // SAFETY: `SORT` only sorts the elements in `SET`, so it's output will have identical elements.
        unsafe { self.cast_unchecked() }
    }

    #[must_use]
    pub const fn normalize(self) -> Sure<T, { NORMALIZE::<T, SET> }> {
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
