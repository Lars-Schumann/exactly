use core::marker::ConstParamTy_;
use core::marker::Destruct;
use core::marker::Freeze;

use alloc::vec::Vec;

pub(crate) mod base_macros;

const LEN<T: ConstParamTy_ + 'static, const SET: &'static[T]>: usize = const { SET.len()};
pub(crate) const CARTESIAN_LENGTH<T: ConstParamTy_ + 'static, const A: &'static[T], const B: &'static[T]>: usize = const { A.len() * B.len() };

pub const SORT<T: Copy + const Ord + Freeze + ConstParamTy_ + 'static, const SET: &'static[T]>: &[T]= const {
    let arr: [T; LEN::<T, SET>] = match SET.try_into() {
        Ok(arr) => arr,
        Err(_) => unreachable!()
    };
    &crate::const_helpers::sort(arr)
};

pub const NORMALIZE<T: Copy + const Ord + ConstParamTy_ + Freeze + const Destruct + 'static, const SET: &'static[T]>: &[T] = const { 'out: {
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

pub const UNION<T: Freeze  + 'static + ConstParamTy_  + Copy, const SETS: &'static [&'static [T]]>: &[T] = const {
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

pub const INTERSECTION<T: Freeze  + 'static + ConstParamTy_  + Copy  + const PartialEq  + const Destruct, const SETS: &'static [&'static [T]]>: &[T] = const { 'out: {
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
}};

pub(crate) const EMPTY<T: 'static>: &[T] = &[];

pub(crate) const SLICEINATOR<T: 'static + ConstParamTy_  + Freeze, const NUM: T>: &[T] = const {
        &[NUM]
    };

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Set<T: ConstParamTy_ + 'static, const SET: &'static [T]>(T);

impl<T> Set<T, { EMPTY::<T> }>
where
    T: Copy + const Ord + Freeze + ConstParamTy_ + const Destruct + 'static,
{
    pub const NEW<const NUM: T>: Set<T, { SLICEINATOR::<T,NUM> }> = const {
        const { Set::new(NUM).expect("This should be infallible, please file a bug report.") }
    };
}

impl<T, const SET: &'static [T]> Set<T, SET>
where
    T: Copy + const Ord + Freeze + ConstParamTy_ + const Destruct + 'static,
{
    pub const SET: &'static [T] = SET;

    pub const fn set(self) -> &'static [T] {
        SET
    }

    pub const fn new(value: T) -> Option<Self> {
        match Self::contains(&value) {
            true => Some(unsafe { Self::new_unchecked(value) }),
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
        unsafe { self.cast_unchecked() }
    }

    pub const fn cast<const NEW_SET: &'static [T]>(self) -> Option<Set<T, NEW_SET>> {
        match Set::<T, NEW_SET>::contains(&self.inner()) {
            false => None,
            true => Some(unsafe { self.cast_unchecked() }),
        }
    }

    pub const unsafe fn cast_unchecked<const NEW_SET: &'static [T]>(self) -> Set<T, NEW_SET> {
        unsafe { Set::new_unchecked(self.inner()) }
    }
}
