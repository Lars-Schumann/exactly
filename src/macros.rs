macro_rules! impl_ints {
(the_dolla: $d:tt, $([inner_type: $num_t:ident, largest_num_t_with_same_signedness: $largest_num_t_with_same_signedness:ident, wrap_t_name: $wrap_t_name:ident, range_fn_name: $range_fn_name:ident, private_macro_prefix: $private_macro_prefix:ident, extra_mod: $extra_mod:ident, sort_fn_name: $sort_fn_name:ident],)*) => {$(

#[derive(Debug, Copy, Clone,)]
#[repr(transparent)]
pub struct $wrap_t_name<const SET: &'static [$num_t]>($num_t);

pub mod $extra_mod {
    use ::alloc::vec::Vec;
    use crate::const_helpers::ext_vec_reduce_to_intersection_with;

    const LEN<const SET: &'static[$num_t]>: usize = const { SET.len()};
    const CARTESIAN_LENGTH<const A: &'static[$num_t], const B: &'static[$num_t]>: usize = const { A.len() * B.len() };

    macro_rules! define_cartesian_ops {
        ($d([const_name: $const_name:ident, op: $op:tt]),+ $d(,)?) => {$d(
            pub(super) const $const_name<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { CARTESIAN_LENGTH::<A, B> }, _>(
                    const |i| {
                        let a = A[i / B.len()];
                        let b = B[i % B.len()];
                        a $op b
                    }
                )
            };
        )+}
    }

    macro_rules! define_cartesian_fns {
        ($d([const_name: $const_name:ident, fn: $fn:path]),+ $d(,)?) => {$d(
            pub(super) const $const_name<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { CARTESIAN_LENGTH::<A, B> }, _>(
                    const |i| {
                        let a = A[i / B.len()];
                        let b = B[i % B.len()];
                        $fn(a, b)
                    }
                )
            };
        )+}
    }

    define_cartesian_ops! {
        [const_name: CARTESIAN_ADD      , op: + ],
        [const_name: CARTESIAN_SUB      , op: - ],
        [const_name: CARTESIAN_MUL      , op: * ],
        [const_name: CARTESIAN_DIV      , op: / ],

        [const_name: CARTESIAN_REM      , op: % ],

        [const_name: CARTESIAN_BIT_AND  , op: & ],
        [const_name: CARTESIAN_BIT_OR   , op: | ],
        [const_name: CARTESIAN_BIT_XOR  , op: ^ ],

        [const_name: CARTESIAN_SHL      , op: <<],
        [const_name: CARTESIAN_SHR      , op: >>],
    }

    define_cartesian_fns! {
        [const_name: CARTESIAN_STRICT_ADD   , fn: ::core::primitive::$num_t::strict_add],
        [const_name: CARTESIAN_STRICT_SUB   , fn: ::core::primitive::$num_t::strict_sub],
        [const_name: CARTESIAN_STRICT_MUL   , fn: ::core::primitive::$num_t::strict_mul],
        [const_name: CARTESIAN_STRICT_DIV   , fn: ::core::primitive::$num_t::strict_div],

        [const_name: CARTESIAN_WRAPPING_ADD , fn: ::core::primitive::$num_t::wrapping_add],
        [const_name: CARTESIAN_WRAPPING_SUB , fn: ::core::primitive::$num_t::wrapping_sub],
        [const_name: CARTESIAN_WRAPPING_MUL , fn: ::core::primitive::$num_t::wrapping_mul],
        [const_name: CARTESIAN_WRAPPING_DIV , fn: ::core::primitive::$num_t::wrapping_div],
    }


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
            true => Some(unsafe { Self::new_unchecked(value) }),
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
        debug_assert!(Self::contains(value));
        Self(value)
    }

    pub const fn contains(value: $num_t) -> bool {
        crate::const_helpers::ext_slice_contains(SET, &value)
    }

    pub const fn inner(self) -> $num_t {
        self.0
    }

    pub const fn sort(self) -> $wrap_t_name<{ $extra_mod::SORT::<SET> }> {
        unsafe { self.cast_unchecked() }
    }

    pub const fn normalize(self) -> $wrap_t_name<{ $extra_mod::NORMALIZE::<SET> }> {
        unsafe { self.cast_unchecked() }
    }

    pub const fn widen<const SUPER_SET: &'static [$num_t]>(self) -> $wrap_t_name<SUPER_SET> {
        const { assert!(crate::const_helpers::ext_slice_is_subset(SET, SUPER_SET)); }
        unsafe { self.cast_unchecked() }
    }

    pub const fn cast<const NEW_SET: &'static [$num_t]>(self) -> Option<$wrap_t_name<NEW_SET>> {
        match $wrap_t_name::<NEW_SET>::contains(self.inner()) {
            false => None,
            true => Some( unsafe { self.cast_unchecked() } )
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
        unsafe { $wrap_t_name::new_unchecked(self.inner()) }
    }
}

macro_rules! ${concat(impl_ops_, $num_t)} {
    ($d([op_trait: $d(::$op_trait:ident)+, op_fn_name: $op_fn_name:ident, output_const: $output_const:ident, op: $op:tt]),+ $d(,)?) => {$d(
        impl<const A_SET: &'static [$num_t], const B_SET: &'static [$num_t]> $d(::$op_trait)+<$wrap_t_name<B_SET> > for $wrap_t_name<A_SET> {
            type Output = $wrap_t_name<{ $extra_mod::$output_const::<{ A_SET }, { B_SET }> }>;

            fn $op_fn_name(self, rhs: $wrap_t_name<B_SET>) -> Self::Output {
                unsafe { $wrap_t_name::new_unchecked(self.inner() $op rhs.inner()) }
            }
        }
    )+}
}

${concat(impl_ops_, $num_t)}! {
    [op_trait: ::core::ops::Add     , op_fn_name: add   , output_const: CARTESIAN_ADD       , op: + ],
    [op_trait: ::core::ops::Sub     , op_fn_name: sub   , output_const: CARTESIAN_SUB       , op: - ],
    [op_trait: ::core::ops::Mul     , op_fn_name: mul   , output_const: CARTESIAN_MUL       , op: * ],
    [op_trait: ::core::ops::Div     , op_fn_name: div   , output_const: CARTESIAN_DIV       , op: / ],

    [op_trait: ::core::ops::Rem     , op_fn_name: rem   , output_const: CARTESIAN_REM       , op: % ],

    [op_trait: ::core::ops::BitAnd  , op_fn_name: bitand, output_const: CARTESIAN_BIT_AND   , op: & ],
    [op_trait: ::core::ops::BitOr   , op_fn_name: bitor , output_const: CARTESIAN_BIT_OR    , op: | ],
    [op_trait: ::core::ops::BitXor  , op_fn_name: bitxor, output_const: CARTESIAN_BIT_XOR   , op: ^ ],

    [op_trait: ::core::ops::Shl     , op_fn_name: shl   , output_const: CARTESIAN_SHL       , op: <<],
    [op_trait: ::core::ops::Shr     , op_fn_name: shr   , output_const: CARTESIAN_SHR       , op: >>],
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

)*}
}
pub(crate) use impl_ints;
