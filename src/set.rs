use alloc::vec::Vec;
use core::marker::ConstParamTy_;
use core::marker::Destruct;
use core::marker::Freeze;

use crate::const_helpers;
use crate::sure_eq::SureEq;

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
    &const_helpers::sort(arr)
};

pub const NORMALIZE<
    T: Copy + const Ord + SureEq + Freeze + const Destruct + 'static,
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
        let mut intersection: Vec<T> = const_helpers::slice_to_vec(first_set);

        let mut i: usize = 1;
        while i < SETS.len() {
            const_helpers::vec_reduce_to_intersection_with(&mut intersection, SETS[i]);
            i += 1;
        }

        intersection.const_make_global()
    }
};

pub(crate) const EMPTY<T: 'static>: &[T] = &[];

pub(crate) const SLICEINATOR<T: 'static + ConstParamTy_ + Freeze, const NUM: T>: &[T] =
    const { &[NUM] };
