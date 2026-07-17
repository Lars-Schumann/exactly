use alloc::vec;
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

pub const SORT<T: const Ord + ConstParamTy_ + Copy + const Destruct + Freeze + 'static, const SET: &'static [T]>:
    &[T] = const {
    #[expect(clippy::ok_expect)]
    let arr: [T; LENGTH::<T, SET>] = SET.try_into().ok().expect("this is infallible");
    &const_helpers::sort(arr)
};

pub const NORMALIZE<
    T: SureEq + const Ord + Copy + const Destruct + 'static,
    const SET: &'static [T],
>: &[T] = const {
    let set_sorted = SORT::<T, SET>;
    let normalized: Vec<T> = deduped(set_sorted); 

    normalized.const_make_global()
};

pub const UNION<T: ConstParamTy_ + Copy + Freeze + 'static , const SETS: &'static [&'static [T]]>:
    &[T] = const {
    let mut onion: Vec<T> = vec![];
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
    T: SureEq + Copy + const Destruct + 'static ,
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

const fn deduped<T: SureEq + Copy>(slice: &[T]) -> Vec<T> {
    let [first, ..] = slice else { return vec![] };

    // FIXME: make this `vec![first]` once possible in const
    let mut deduped: Vec<T> = vec![];
    deduped.push(*first);

    let mut i = 1; // starting at the 2nd element, since the first one is always unique
    while i < slice.len() {
        let (previous, current) = (slice[i - 1], slice[i]);
        if previous != current {
            deduped.push(current);
        }
        i += 1;
    }
    deduped
}
