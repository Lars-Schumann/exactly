macro_rules! ops_implinator {
    ($([num_t: $num_t:ident, wrap_t_name: $wrap_t_name:ident, extra_mod: $extra_mod:ident, cartesian_const_name: $cartesian_const_name:ident, trait_fn_name: $trait_fn_name:ident, op_trait: $(::$op_trait:ident)+, op: $op:tt]),+ $(,)?) => {$(

        #[expect(nonstandard_style)]
        mod ${concat(__mod_,$wrap_t_name,_,$cartesian_const_name)} {

            pub(crate) const $cartesian_const_name<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { crate::$extra_mod::CARTESIAN_LENGTH::<A, B> }, _>(
                    const |i| {
                        let a = A[i / B.len()];
                        let b = B[i % B.len()];
                        a $op b
                    }
                )
            };

        }

        const impl<const A_SET: &'static [$num_t], const B_SET: &'static [$num_t]> $(::$op_trait)+<$wrap_t_name<B_SET> > for $wrap_t_name<A_SET> {

            type Output = $wrap_t_name<{ ${concat(__mod_,$wrap_t_name,_,$cartesian_const_name)}::$cartesian_const_name::<{ A_SET }, { B_SET }> }>;

            fn $trait_fn_name(self, rhs: $wrap_t_name<B_SET>) -> Self::Output {
                unsafe { $wrap_t_name::new_unchecked(self.inner() $op rhs.inner()) }
            }

        }
    )+}
}
pub(crate) use ops_implinator;

macro_rules! implinator {
    ($([num_t: $num_t:ident, wrap_t_name: $wrap_t_name:ident, extra_mod: $extra_mod: ident, cartesian_const_name: $cartesian_const_name:ident, fn_name: $fn_name:ident, fn_path: $fn_path:path]),+ $(,)?) => {$(

        #[expect(nonstandard_style)]
        mod ${concat(__mod_,$wrap_t_name,_,$cartesian_const_name)} {

            pub(crate) const $cartesian_const_name<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { crate::$extra_mod::CARTESIAN_LENGTH::<A, B> }, _>(
                    const |i| {
                        let a = A[i / B.len()];
                        let b = B[i % B.len()];
                        $fn_path(a, b)
                    }
                )
            };

        }

        const impl<const SET: &'static [$num_t]> $wrap_t_name<SET> {

            pub fn $fn_name<const RHS_SET: &'static [$num_t]>(self, rhs: $wrap_t_name<RHS_SET>) -> $wrap_t_name<{ ${concat(__mod_,$wrap_t_name,_,$cartesian_const_name)}::$cartesian_const_name::<{ SET }, { RHS_SET }> }> {
                unsafe { $wrap_t_name::new_unchecked($fn_path(self.inner(), rhs.inner())) }
            }

        }

    )+}
}
pub(crate) use implinator;

macro_rules! impl_ints {
(the_dolla: $d:tt, $([inner_type: $num_t:ident, largest_num_t_with_same_signedness: $largest_num_t_with_same_signedness:ident, wrap_t_name: $wrap_t_name:ident, range_fn_name: $range_fn_name:ident, private_macro_prefix: $private_macro_prefix:ident, extra_mod: $extra_mod:ident, sort_fn_name: $sort_fn_name:ident],)*) => {$(

#[derive(Debug, Copy, Clone,)]
#[repr(transparent)]
pub struct $wrap_t_name<const SET: &'static [$num_t]>($num_t);

pub mod $extra_mod {
    use ::alloc::vec::Vec;
    use crate::const_helpers::ext_vec_reduce_to_intersection_with;

    const LEN<const SET: &'static[$num_t]>: usize = const { SET.len()};
    pub(crate) const CARTESIAN_LENGTH<const A: &'static[$num_t], const B: &'static[$num_t]>: usize = const { A.len() * B.len() };

    pub const SORT<const SET: &'static[$num_t]>: &[$num_t] = const {
        let arr: [$num_t; LEN::<SET>] = match SET.try_into() {
            Ok(arr) => arr,
            Err(_) => unreachable!()
        };
        &::compile_time_sort::$sort_fn_name(arr)
    };

    pub(super) const NORMALIZE<const SET: &'static[$num_t]>: &[$num_t] = const { 'out: {
        let set_sorted = SORT::<SET>;
        let mut normalized: Vec<$num_t> = Vec::new();

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

    const RANGE_LENGTH_HELPER<const MIN: $num_t, const MAX: $num_t, const IS_INCLUSIVE: bool>: usize = const {
        match <$largest_num_t_with_same_signedness as ::core::convert::TryInto<usize>>::try_into($largest_num_t_with_same_signedness::from(MAX).strict_sub($largest_num_t_with_same_signedness::from(MIN))) {
            Err(_) => panic!(),
            Ok(len) => len.strict_add(usize::from(IS_INCLUSIVE)),
        }
    };

    pub const RANGE_HELPER<const MIN: $num_t, const MAX: $num_t, const IS_INCLUSIVE: bool>: &[$num_t] = const {
        &core::array::from_fn::<$num_t, { RANGE_LENGTH_HELPER::<MIN, MAX, IS_INCLUSIVE> }, _>(const |i| $num_t::try_from($largest_num_t_with_same_signedness::from(MIN) + <usize as TryInto<$largest_num_t_with_same_signedness>>::try_into(i).ok().unwrap()).ok().unwrap())
    };

    pub const RANGE             <const START: $num_t, const END : $num_t>: &[$num_t] = RANGE_HELPER::<                START ,                 END   , false >;
    pub const RANGE_FROM        <const START: $num_t                    >: &[$num_t] = RANGE_HELPER::<                START ,{const { $num_t::MAX }}, true  >;
    // pub const RANGE_FULL     <                                       >: &[$num_t] = RANGE_HELPER::<const { $num_t::MIN } ,{const { $num_t::MAX }}, true  >;
    pub const RANGE_INCLUSIVE   <const START: $num_t, const LAST: $num_t>: &[$num_t] = RANGE_HELPER::<                START ,                 LAST  , true  >;
    pub const RANGE_TO          <                     const END : $num_t>: &[$num_t] = RANGE_HELPER::<{const{ $num_t::MIN }},                 END   , false >;
    pub const RANGE_TO_INCLUSIVE<                     const LAST: $num_t>: &[$num_t] = RANGE_HELPER::<{const{ $num_t::MIN }},                 LAST  , true  >;

    pub(crate) const SLICEINATOR<const N: $num_t>: &[$num_t] = const {
        &[N]
    };

    pub const UNION<const SETS: &'static [&'static [$num_t]]>: &[$num_t] = const {
        let mut onion: Vec<$num_t> = Vec::new();
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

    pub const INTERSECTION<const SETS: &'static [&'static [$num_t]]>: &[$num_t] = const { 'out: {
        let [first_set, ..] = SETS else {
            break 'out &[];
        };

        let mut intersection: Vec<$num_t> = Vec::with_capacity(first_set.len());

        let mut i: usize = 0;
        while i < first_set.len() {
            intersection.push(first_set[i]);
            i += 1;
        }

        let mut j: usize = 1;

        while j < SETS.len() {
            ext_vec_reduce_to_intersection_with(&mut intersection, SETS[j]);
            j += 1;
        }

        intersection.const_make_global()
    }};

    #[cfg_attr(doc, doc(hidden))]
    #[macro_export]
    macro_rules! ${ concat($private_macro_prefix, union) } {
        ($d($set:expr),+ $d(,)?) => {
            $d crate::$extra_mod::UNION::<{ &[$d($set, )+] }>
        };
    }
    pub use ${ concat($private_macro_prefix, union) } as Union;

    #[cfg_attr(doc, doc(hidden))]
    #[macro_export]
    macro_rules! ${ concat($private_macro_prefix, intersection) } {
        ($d($set:expr),+ $d(,)?) => {
            $d crate::$extra_mod::INTERSECTION::<{ &[$d($set, )+] }>
        };
    }
    pub use ${ concat($private_macro_prefix, intersection) } as Intersection;

    #[cfg_attr(doc, doc(hidden))]
    #[macro_export]
    macro_rules! ${ concat($private_macro_prefix, range) } {
        ( $start:literal ..  $end:literal  ) => { $d crate::$extra_mod::RANGE::             <$start, $end>  };
        ( $start:literal ..                ) => { $d crate::$extra_mod::RANGE_FROM::        <$start>        };
        // (                ..             ) => { $d crate::$extra_mod::RANGE_FULL                          };
        ( $start:literal ..= $last:literal ) => { $d crate::$extra_mod::RANGE_INCLUSIVE::   <$start, $last> };
        (                ..  $end:literal  ) => { $d crate::$extra_mod::RANGE_TO::          <$end>          };
        (                ..= $last:literal ) => { $d crate::$extra_mod::RANGE_TO_INCLUSIVE::<$last>         };
    }
    pub use ${ concat($private_macro_prefix, range) } as Range;
}

impl $wrap_t_name<{ const { &[] } }> {
    pub const NEW<const NUM: $num_t>: $wrap_t_name<{ $extra_mod::SLICEINATOR::<NUM> }> = const {
        const { $wrap_t_name::new(NUM).expect("This should be infallible, please file a bug report.") }
    };
}

impl<const SET: &'static [$num_t]> $wrap_t_name<SET> {

    pub const SET: &'static [$num_t] = SET;

    pub const fn set(self) -> &'static [$num_t] {
        SET
    }

    pub const fn new(value: $num_t) -> Option<Self> {
        match Self::contains(value) {
            true => Some(
                // SAFETY: we just checked precondition #1
                unsafe { Self::new_unchecked(value) }
            ),
            false => None,
        }
    }

    /// # Safety
    ///
    /// One of the following conditions must hold, they are all logically equivalent:
    /// 1. `Self::contains(value)` must be `true`
    /// 2. `Self::SET` contains `value`
    /// 3. `Self::new(value)` returns `Some(_)`
    pub const unsafe fn new_unchecked(value: $num_t) -> Self {
        debug_assert!(Self::contains(value), concat!("Tried to create a ", stringify!($wrap_t_name)," with a value thats not contained in its SET, this is UB."));
        Self(value)
    }

    pub const fn contains(value: $num_t) -> bool {
        crate::const_helpers::ext_slice_contains(SET, &value)
    }

    pub const fn inner(self) -> $num_t {
        self.0
    }

    pub const fn sort(self) -> $wrap_t_name<{ $extra_mod::SORT::<SET> }> {
        // SAFETY: we trust that SORT does not eliminate any elements of SET
        // and since SET contains self.inner(), the sorted set will also contain
        // self.inner() (precondition #2)
        unsafe { self.cast_unchecked() }
    }

    pub const fn normalize(self) -> $wrap_t_name<{ $extra_mod::NORMALIZE::<SET> }> {
        // SAFETY: we trust that NORMALIZE does not eliminate elements of SET
        // and since SET contains self.inner(), the normalized set will also contain
        // self.inner() (precondition #2)
        unsafe { self.cast_unchecked() }
    }

    pub const fn widen<const SUPER_SET: &'static [$num_t]>(self) -> $wrap_t_name<SUPER_SET> {
        const {
            assert!(
                crate::const_helpers::ext_slice_is_subset(SET, SUPER_SET),
                concat!("Tried to widen a ", stringify!($wrap_t_name),", which failed because the target's SET isn't a superset of the original.")
            );
        }
        // SAFETY: since self.inner() is a member of SET and we just asserted that
        // SET is a subset of SUPER_SET, self.inner() must also be a member of
        // SUPER_SET (precondition #2)
        unsafe { self.cast_unchecked() }
    }

    pub const fn cast<const NEW_SET: &'static [$num_t]>(self) -> Option<$wrap_t_name<NEW_SET>> {
        match $wrap_t_name::<NEW_SET>::contains(self.inner()) {
            false => None,
            true => Some(
                // SAFETY: we just checked precondition #1
                unsafe { self.cast_unchecked() }
            )
        }
    }

    #[doc = concat!(
    " # Safety\n",
    "\n",
    " One of the following conditions must hold, they are all logically equivalent:\n",
    " 1. `", stringify!($wrap_t_name), "::<NEW_SET>::contains(self.inner())` must be `true`\n",
    " 2. `NEW_SET` contains `value`\n",
    " 3. `Self::cast::<NEW_SET>::(value)` returns `Some(_)`"
    )]
    pub const unsafe fn cast_unchecked<const NEW_SET: &'static [$num_t]>(self) -> $wrap_t_name<NEW_SET> {
        // SAFETY: the preconditions of this function guarantee that the preconditions
        // of new_unchecked will hold.
        unsafe { $wrap_t_name::new_unchecked(self.inner()) }
    }
}


#[macro_export]
macro_rules! $wrap_t_name {
    ($elem:literal) => {
        $d crate::$wrap_t_name::<{ &[$elem] }>
    };
    ($set:expr) => {
        $d crate::$wrap_t_name::<{ $set }>
    };
    ($d($elem:expr),+ $d(,)?) => {
        $d crate::$wrap_t_name::<{ &[$d($elem, )+] }>
    };
}

crate::macros::implinator! {
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_ADD    , fn_name: strict_add   , fn_path: ::core::primitive::$num_t::strict_add    ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_SUB    , fn_name: strict_sub   , fn_path: ::core::primitive::$num_t::strict_sub    ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_MUL    , fn_name: strict_mul   , fn_path: ::core::primitive::$num_t::strict_mul    ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_DIV    , fn_name: strict_div   , fn_path: ::core::primitive::$num_t::strict_div    ],

    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_ADD  , fn_name: wrapping_add , fn_path: ::core::primitive::$num_t::wrapping_add  ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_SUB  , fn_name: wrapping_sub , fn_path: ::core::primitive::$num_t::wrapping_sub  ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_MUL  , fn_name: wrapping_mul , fn_path: ::core::primitive::$num_t::wrapping_mul  ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_DIV  , fn_name: wrapping_div , fn_path: ::core::primitive::$num_t::wrapping_div  ],
}

crate::macros::ops_implinator! {
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_ADD           , trait_fn_name: add    , op_trait: ::core::ops::Add     , op: +  ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_SUB           , trait_fn_name: sub    , op_trait: ::core::ops::Sub     , op: -  ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_MUL           , trait_fn_name: mul    , op_trait: ::core::ops::Mul     , op: *  ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_DIV           , trait_fn_name: div    , op_trait: ::core::ops::Div     , op: /  ],

    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_REM           , trait_fn_name: rem    , op_trait: ::core::ops::Rem     , op: %  ],

    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_BIT_AND       , trait_fn_name: bitand , op_trait: ::core::ops::BitAnd  , op: &  ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_BIT_OR        , trait_fn_name: bitor  , op_trait: ::core::ops::BitOr   , op: |  ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_BIT_XOR       , trait_fn_name: bitxor , op_trait: ::core::ops::BitXor  , op: ^  ],

    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_SHL           , trait_fn_name: shl    , op_trait: ::core::ops::Shl     , op: << ],
    [num_t: $num_t, wrap_t_name: $wrap_t_name, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_SHR           , trait_fn_name: shr    , op_trait: ::core::ops::Shr     , op: >> ],
}

)*}
}
pub(crate) use impl_ints;
