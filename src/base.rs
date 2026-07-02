use core::marker::ConstParamTy_ as CPT;
use core::marker::Destruct;
use core::marker::Freeze;

use alloc::vec::Vec;

const LEN<T: CPT + 'static, const SET: &'static[T]>: usize = const { SET.len()};

pub const SORT<T: Copy + const Ord + Freeze + CPT + 'static, const SET: &'static[T]>: &[T]= const {
    let arr: [T; LEN::<T, SET>] = match SET.try_into() {
        Ok(arr) => arr,
        Err(_) => unreachable!()
    };
    &crate::const_helpers::sort(arr)
};

#[expect(unused)]
pub(super) const NORMALIZE<T: Copy + const Ord + CPT + Freeze + const Destruct + 'static, const SET: &'static[T]>: &[T] = const { 'out: {
        let set_sorted = SORT::<T, SET>;
        let mut normalized: Vec<T> = Vec::new();

        let [first, ..] = set_sorted else {
            break 'out &[]
        };

        normalized.push(*first);

        let mut i: usize = 1;

        while i < set_sorted.len() {
            let (previous, current) = (set_sorted[i - 1], set_sorted[i]);
            if previous != current {
                normalized.push(current)
            }
            i += 1;
        }
        normalized.const_make_global()
    }};

#[expect(unused)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Set<T: CPT + 'static, const SET: &'static [T]>(T);

impl<T, const SET: &'static [T]> Set<T, SET>
where
    T: Copy + const Ord + Freeze + CPT + const Destruct + 'static,
{
    pub const SET: &'static [T] = SET;

    pub const fn set(self) -> &'static [T] {
        SET
    }

    pub const fn new(value: T) -> Option<Self> {
        match Self::contains(&value) {
            true => Some(
                // SAFETY: we just checked precondition #1
                unsafe { Self::new_unchecked(value) },
            ),
            false => None,
        }
    }

    pub const unsafe fn new_unchecked(value: T) -> Self {
        debug_assert!(
            Self::contains(&value),
            concat!(
                "Tried to create a ",
                stringify!($wrap_t_name),
                " with a value thats not contained in its SET, this is UB."
            )
        );
        Self(value)
    }

    pub const fn contains(value: &T) -> bool {
        crate::const_helpers::ext_slice_contains(SET, value)
    }

    pub const fn inner(self) -> T {
        self.0
    }

    pub const fn sort(self) -> Set<T, { SORT::<T, SET> }> {
        unsafe { self.cast_unchecked() }
    }

    pub const fn normalize(self) -> Set<T, { NORMALIZE::<T, SET> }> {
        unsafe { self.cast_unchecked() }
    }

    pub const fn widen<const SUPER_SET: &'static [T]>(self) -> Set<T, SUPER_SET> {
        const {
            assert!(
                crate::const_helpers::ext_slice_is_subset(SET, SUPER_SET),
                concat!(
                    "Tried to widen a ",
                    stringify!($wrap_t_name),
                    ", which failed because the target's SET isn't a superset of the original."
                )
            );
        }
        // SAFETY: since self.inner() is a member of SET and we just asserted that
        // SET is a subset of SUPER_SET, self.inner() must also be a member of
        // SUPER_SET (precondition #2)
        unsafe { self.cast_unchecked() }
    }

    pub const fn cast<const NEW_SET: &'static [T]>(self) -> Option<Set<T, NEW_SET>> {
        match Set::<T, NEW_SET>::contains(&self.inner()) {
            false => None,
            true => Some(
                // SAFETY: we just checked precondition #1
                unsafe { self.cast_unchecked() },
            ),
        }
    }

    pub const unsafe fn cast_unchecked<const NEW_SET: &'static [T]>(self) -> Set<T, NEW_SET> {
        // SAFETY: the preconditions of this function guarantee that the preconditions
        // of new_unchecked will hold.
        unsafe { Set::new_unchecked(self.inner()) }
    }
}
