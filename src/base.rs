use core::marker::ConstParamTy_;
use core::marker::Destruct;
use core::marker::Freeze;

use alloc::vec::Vec;

pub(crate) mod base_macros;

const LEN<T: ConstParamTy_ + 'static, const SET: &'static[T]>: usize = const { SET.len()};

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

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Set<T: ConstParamTy_ + 'static, const SET: &'static [T]>(T);

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

base_macros::impl_ints! {
    the_dolla: $,
    [inner_type: u8,    largest_num_t_with_same_signedness: u128,   wrap_t_name: SetU8,      private_macro_prefix: ඞඞ__private_macro_set_u8_,    extra_mod: set_u8    ],
    [inner_type: u16,   largest_num_t_with_same_signedness: u128,   wrap_t_name: SetU16,     private_macro_prefix: ඞඞ__private_macro_set_u16_,   extra_mod: set_u16   ],
    [inner_type: u32,   largest_num_t_with_same_signedness: u128,   wrap_t_name: SetU32,     private_macro_prefix: ඞඞ__private_macro_set_u32_,   extra_mod: set_u32   ],
    [inner_type: u64,   largest_num_t_with_same_signedness: u128,   wrap_t_name: SetU64,     private_macro_prefix: ඞඞ__private_macro_set_u64_,   extra_mod: set_u64   ],
    [inner_type: u128,  largest_num_t_with_same_signedness: u128,   wrap_t_name: SetU128,    private_macro_prefix: ඞඞ__private_macro_set_u128_,  extra_mod: set_u128  ],
    [inner_type: usize, largest_num_t_with_same_signedness: usize,  wrap_t_name: SetUsize,   private_macro_prefix: ඞඞ__private_macro_set_usize_, extra_mod: set_usize ],
    [inner_type: i8,    largest_num_t_with_same_signedness: i128,   wrap_t_name: SetI8,      private_macro_prefix: ඞඞ__private_macro_set_i8_,    extra_mod: set_i8    ],
    [inner_type: i16,   largest_num_t_with_same_signedness: i128,   wrap_t_name: SetI16,     private_macro_prefix: ඞඞ__private_macro_set_i16_,   extra_mod: set_i16   ],
    [inner_type: i32,   largest_num_t_with_same_signedness: i128,   wrap_t_name: SetI32,     private_macro_prefix: ඞඞ__private_macro_set_i32_,   extra_mod: set_i32   ],
    [inner_type: i64,   largest_num_t_with_same_signedness: i128,   wrap_t_name: SetI64,     private_macro_prefix: ඞඞ__private_macro_set_i64_,   extra_mod: set_i64   ],
    [inner_type: i128,  largest_num_t_with_same_signedness: i128,   wrap_t_name: SetI128,    private_macro_prefix: ඞඞ__private_macro_set_i128_,  extra_mod: set_i128  ],
    [inner_type: isize, largest_num_t_with_same_signedness: isize,  wrap_t_name: SetIsize,   private_macro_prefix: ඞඞ__private_macro_set_isize_, extra_mod: set_isize ],
}
