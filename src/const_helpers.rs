use ::alloc::vec::Vec;
use ::core::marker::Destruct;

const fn not(value: bool) -> bool {
    match value {
        true => false,
        false => true,
    }
}

pub(crate) const fn ext_slice_contains<T>(slice: &[T], elem: &T) -> bool
where
    T: [const] PartialEq + [const] Destruct,
{
    let mut i: usize = 0;
    while i < slice.len() {
        if slice[i] == *elem {
            return true;
        }
        i += 1;
    }
    false
}

pub(crate) const fn ext_slice_is_subset<T>(sub: &[T], sup: &[T]) -> bool
where
    T: [const] PartialEq + [const] Destruct,
{
    let mut i: usize = 0;

    while i < sub.len() {
        if not(ext_slice_contains(sup, &sub[i])) {
            return false;
        }
        i += 1;
    }
    true
}

const fn ext_vec_swap_remove<T>(_self: &mut Vec<T>, index: usize) -> T {
    const fn assert_failed(_index: usize, _len: usize) -> ! {
        panic!("swap_remove index should be < len but isn't");
    }

    let len = _self.len();
    if index >= len {
        assert_failed(index, len);
    }
    unsafe {
        // We replace self[index] with the last element. Note that if the
        // bounds check above succeeds there must be a last element (which
        // can be self[index] itself).
        let value = core::ptr::read(_self.as_ptr().add(index));
        let base_ptr = _self.as_mut_ptr();
        core::ptr::copy(base_ptr.add(len - 1), base_ptr.add(index), 1);
        _self.set_len(len - 1);
        value
    }
}

pub(crate) const fn ext_vec_reduce_to_intersection_with<T>(
    running_intersection: &mut Vec<T>,
    set: &[T],
) where
    T: [const] PartialEq + [const] Destruct,
{
    let mut i: usize = 0;

    'outer: while i < running_intersection.len() {
        let mut j: usize = 0;
        while j < set.len() {
            if running_intersection[i] == set[j] {
                i += 1;
                continue 'outer;
            }
            j += 1;
        }
        ext_vec_swap_remove(running_intersection, i);
    }
}

pub(crate) const fn sort<T: [const] Ord, const N: usize>(mut a: [T; N]) -> [T; N] {
    let mut start = N / 2;
    while start > 0 {
        start -= 1;

        let mut root = start;
        loop {
            let left = 2 * root + 1;
            if left >= N {
                break;
            }

            let mut child = left;
            let right = left + 1;
            if right < N && a[child] < a[right] {
                child = right;
            }

            if a[root] < a[child] {
                a.swap(root, child);
                root = child;
            } else {
                break;
            }
        }
    }

    let mut end = N;
    while end > 1 {
        end -= 1;
        a.swap(0, end);

        let mut root = 0;
        loop {
            let left = 2 * root + 1;
            if left >= end {
                break;
            }

            let mut child = left;
            let right = left + 1;
            if right < end && a[child] < a[right] {
                child = right;
            }

            if a[root] < a[child] {
                a.swap(root, child);
                root = child;
            } else {
                break;
            }
        }
    }

    a
}
