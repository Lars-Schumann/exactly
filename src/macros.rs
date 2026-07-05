macro_rules! if_signed {
    (u8,    {$($tt:tt)+}) => { /* nothing */};
    (u16,   {$($tt:tt)+}) => { /* nothing */};
    (u32,   {$($tt:tt)+}) => { /* nothing */};
    (u64,   {$($tt:tt)+}) => { /* nothing */};
    (u128,  {$($tt:tt)+}) => { /* nothing */};
    (usize, {$($tt:tt)+}) => { /* nothing */};
    (i8,    {$($tt:tt)+}) => {$($tt)+};
    (i16,   {$($tt:tt)+}) => {$($tt)+};
    (i32,   {$($tt:tt)+}) => {$($tt)+};
    (i64,   {$($tt:tt)+}) => {$($tt)+};
    (i128,  {$($tt:tt)+}) => {$($tt)+};
    (isize, {$($tt:tt)+}) => {$($tt)+};
}
pub(crate) use if_signed;

macro_rules! if_unsigned {
    (u8,    {$($tt:tt)+}) => {$($tt)+};
    (u16,   {$($tt:tt)+}) => {$($tt)+};
    (u32,   {$($tt:tt)+}) => {$($tt)+};
    (u64,   {$($tt:tt)+}) => {$($tt)+};
    (u128,  {$($tt:tt)+}) => {$($tt)+};
    (usize, {$($tt:tt)+}) => {$($tt)+};
    (i8,    {$($tt:tt)+}) => { /* nothing */};
    (i16,   {$($tt:tt)+}) => { /* nothing */};
    (i32,   {$($tt:tt)+}) => { /* nothing */};
    (i64,   {$($tt:tt)+}) => { /* nothing */};
    (i128,  {$($tt:tt)+}) => { /* nothing */};
    (isize, {$($tt:tt)+}) => { /* nothing */};
}
pub(crate) use if_unsigned;

macro_rules! impl_simple_unary_ops {
    ($([inner_t: $inner_t:ident, trait_fn_name: $trait_fn_name:ident, op_trait: $(::$op_trait:ident)+, op: $op:tt]),+ $(,)?) => {$(

        #[expect(non_snake_case)]
        mod ${concat(ඞඞ__,$inner_t,_,$trait_fn_name)} {
            use crate::base::Set;

            const CODOMAIN<const SET: &'static[$inner_t]>: &[$inner_t] = const {
                &core::array::from_fn::<$inner_t, { crate::base::LENGTH::<$inner_t, SET> }, _>(
                    const |i| {
                        let a: $inner_t = SET[i];
                        $op a
                    }
                )
            };

            const impl<const SET: &'static [$inner_t]> $(::$op_trait)+ for Set<$inner_t, SET> {
                type Output = Set<$inner_t, { CODOMAIN::<{ SET }> }>;

                fn $trait_fn_name(self) -> Self::Output {
                    let self_inner: $inner_t = self.inner();
                    let res_inner: $inner_t = $op self_inner;
                    unsafe { Set::new_unchecked(res_inner) }
                }
            }
        }
    )+}
}
pub(crate) use impl_simple_unary_ops;

macro_rules! impl_simple_binary_ops {
    ($([inner_t: $inner_t:ident, trait_fn_name: $trait_fn_name:ident, op_trait: $(::$op_trait:ident)+, op: $op:tt]),+ $(,)?) => {$(

        #[expect(non_snake_case)]
        mod ${concat(ඞඞ__,$inner_t,_,$trait_fn_name)} {
            use crate::base::Set;

            const CODOMAIN<const A: &'static[$inner_t], const B: &'static[$inner_t]>: &[$inner_t] = const {
                &core::array::from_fn::<$inner_t, { crate::base::CARTESIAN_LENGTH::<$inner_t, $inner_t, A, B> }, _>(
                    const |i| {
                        let b_len: usize = B.len();
                        let a_index: usize = i.strict_div(b_len);
                        let b_index: usize = i.strict_rem(b_len);
                        let a: $inner_t = A[a_index];
                        let b: $inner_t = B[b_index];
                        a $op b
                    }
                )
            };

            const impl<const A_SET: &'static [$inner_t], const B_SET: &'static [$inner_t]> $(::$op_trait)+<Set<$inner_t,B_SET> > for Set<$inner_t, A_SET> {
                type Output = Set<$inner_t, { CODOMAIN::<{ A_SET }, { B_SET }> }>;

                fn $trait_fn_name(self, rhs: Set<$inner_t, B_SET>) -> Self::Output {
                    let self_inner: $inner_t = self.inner();
                    let rhs_inner: $inner_t = rhs.inner();
                    let res_inner: $inner_t = self_inner $op rhs_inner;
                    unsafe { Set::new_unchecked(res_inner) }
                }
            }
        }
    )+}
}
pub(crate) use impl_simple_binary_ops;

macro_rules! impl_unary_fns {
    ($([inner_t: $inner_t:ident, signature: (self) -> $codomain_t:ident, fn_name: $fn_name:ident, fn_path: $fn_path:path]),+ $(,)?) => {$(

        #[expect(non_snake_case)]
        mod ${concat(ඞඞ__,$inner_t,_,$fn_name)} {
            use crate::base::Set;

            const CODOMAIN<const SET: &'static[$inner_t]>: &[$codomain_t] = const {
                &core::array::from_fn::<$codomain_t, { crate::base::LENGTH::<$inner_t, SET> }, _>(
                    const |i| {
                        let a: $inner_t = SET[i];
                        $fn_path(a)
                    }
                )
            };

            const impl<const SET: &'static [$inner_t]> Set<$inner_t, SET> {
                pub fn $fn_name(self) -> Set<$codomain_t, { CODOMAIN::<{ SET }> }> {
                    let self_inner: $inner_t = self.inner();
                    let res_inner: $codomain_t = $fn_path(self_inner);
                    unsafe { Set::new_unchecked(res_inner) }
                }
            }
        }

    )+}
}
pub(crate) use impl_unary_fns;

macro_rules! impl_binary_fns {
    ($([inner_t: $inner_t:ident, signature: (self, $rhs_t:ident) -> $codomain_t:ident, fn_name: $fn_name:ident, fn_path: $fn_path:path]),+ $(,)?) => {$(

        #[expect(non_snake_case)]
        mod ${concat(ඞඞ__,$inner_t,_,$fn_name)} {
            use crate::base::Set;

            const CODOMAIN<const A: &'static[$inner_t], const B: &'static[$rhs_t]>: &[$codomain_t] = const {
                &core::array::from_fn::<$codomain_t, { crate::base::CARTESIAN_LENGTH::<$inner_t, $rhs_t, A, B> }, _>(
                    const |i| {
                        let b_len: usize = B.len();
                        let a_index: usize = i.strict_div(b_len);
                        let b_index: usize = i.strict_rem(b_len);
                        let a: $inner_t = A[a_index];
                        let b: $rhs_t = B[b_index];
                        $fn_path(a, b)
                    }
                )
            };

            const impl<const SET: &'static [$inner_t]> Set<$inner_t, SET> {
                pub fn $fn_name<const RHS_SET: &'static [$inner_t]>(self, rhs: Set<$inner_t, RHS_SET>) -> Set<$codomain_t, { CODOMAIN::<{ SET }, { RHS_SET }> }> {
                    let self_inner: $inner_t = self.inner();
                    let rhs_inner: $rhs_t = rhs.inner();
                    let res_inner: $codomain_t = $fn_path(self_inner, rhs_inner);
                    unsafe { Set::new_unchecked(res_inner) }
                }
            }
        }

    )+}
}
pub(crate) use impl_binary_fns;

macro_rules! impl_ints {
    (the_dolla: $d:tt, $([num_t: $num_t:ident, uns_num_t: $uns_num_t:ident, t_alias: $t_alias:ident, wide_num_t: $wide_num_t:ident, private_macro_prefix: $private_macro_prefix:ident, extra_mod: $extra_mod:ident],)*) => {$(

        pub type $t_alias<const SET: &'static [$num_t]> = base::Set<$num_t, SET>;

        pub mod $extra_mod {

            const RANGE_LENGTH_HELPER<const MIN: $num_t, const MAX: $num_t, const IS_INCLUSIVE: bool>: usize = const {
                let wide_min = $wide_num_t::from(MIN);
                let wide_max = $wide_num_t::from(MAX);
                let exclusive_length = wide_max.strict_sub(wide_min);
                let exclusive_length: usize = usize::try_from(exclusive_length).ok().expect("Range length could not be converted into a usize.");
                let inclusive_addition: usize = usize::from(IS_INCLUSIVE);
                exclusive_length.strict_add(inclusive_addition)
            };

            pub const RANGE_HELPER<const MIN: $num_t, const MAX: $num_t, const IS_INCLUSIVE: bool>: &[$num_t] = const {
                let wide_min = $wide_num_t::from(MIN);

                &core::array::from_fn::<$num_t, { RANGE_LENGTH_HELPER::<MIN, MAX, IS_INCLUSIVE> }, _>(
                    const |i| {
                        let wide_index = $wide_num_t::try_from(i).ok().expect("");
                        $num_t::try_from(wide_min + wide_index).ok().unwrap()
                    }
                )
            };

            pub const RANGE             <const MIN: $num_t, const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<                MIN ,                 MAX   , false >;
            pub const RANGE_FROM        <const MIN: $num_t                   >: &[$num_t] = RANGE_HELPER::<                MIN ,{const { $num_t::MAX }}, true  >;
            // RANGE_FULL omitted
            pub const RANGE_INCLUSIVE   <const MIN: $num_t, const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<                MIN ,                 MAX   , true  >;
            pub const RANGE_TO          <                   const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<{const{ $num_t::MIN }},               MAX   , false >;
            pub const RANGE_TO_INCLUSIVE<                   const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<{const{ $num_t::MIN }},               MAX   , true  >;

            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, range) } {
                ( $min:literal ..  $max:literal ) => { $d crate::$extra_mod::RANGE::             <$min, $max> };
                ( $min:literal ..               ) => { $d crate::$extra_mod::RANGE_FROM::        <$min      > };
                // RANGE_FULL omitted
                ( $min:literal ..= $max:literal ) => { $d crate::$extra_mod::RANGE_INCLUSIVE::   <$min, $max> };
                (              ..  $max:literal ) => { $d crate::$extra_mod::RANGE_TO::          <      $max> };
                (              ..= $max:literal ) => { $d crate::$extra_mod::RANGE_TO_INCLUSIVE::<      $max> };
            }
            pub use ${ concat($private_macro_prefix, range) } as Range;

            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, union) } {
                ($d($set:expr),+ $d(,)?) => {
                    $d crate::base::UNION::<$num_t, { &[$d($set, )+] }>
                };
            }
            pub use ${ concat($private_macro_prefix, union) } as Union;

            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, intersection) } {
                ($d($set:expr),+ $d(,)?) => {
                    $d crate::base::INTERSECTION::<$num_t, { &[$d($set, )+] }>
                };
            }
            pub use ${ concat($private_macro_prefix, intersection) } as Intersection;

        }

        #[macro_export]
        macro_rules! $t_alias {
            ($elem:literal) => {
                $d crate::base::Set::<$num_t, { &[$elem] }>
            };
            ($set:expr) => {
                $d crate::base::Set::<$num_t, { $set }>
            };
            ($d($elem:expr),+ $d(,)?) => {
                $d crate::base::Set::<$num_t, { &[$d($elem, )+] }>
            };
        }

        macros::impl_simple_unary_ops! {
            [inner_t: $num_t, trait_fn_name: not    , op_trait: ::core::ops::Not     , op: !  ],
        }

        macros::if_signed!{ $num_t, { macros::impl_simple_unary_ops! {
            [inner_t: $num_t, trait_fn_name: neg    , op_trait: ::core::ops::Neg     , op: -  ],
        }}}

        macros::impl_simple_binary_ops! {
            [inner_t: $num_t, trait_fn_name: add    , op_trait: ::core::ops::Add     , op: +  ],
            [inner_t: $num_t, trait_fn_name: sub    , op_trait: ::core::ops::Sub     , op: -  ],
            [inner_t: $num_t, trait_fn_name: mul    , op_trait: ::core::ops::Mul     , op: *  ],
            [inner_t: $num_t, trait_fn_name: div    , op_trait: ::core::ops::Div     , op: /  ],

            [inner_t: $num_t, trait_fn_name: rem    , op_trait: ::core::ops::Rem     , op: %  ],

            [inner_t: $num_t, trait_fn_name: bitand , op_trait: ::core::ops::BitAnd  , op: &  ],
            [inner_t: $num_t, trait_fn_name: bitor  , op_trait: ::core::ops::BitOr   , op: |  ],
            [inner_t: $num_t, trait_fn_name: bitxor , op_trait: ::core::ops::BitXor  , op: ^  ],

            [inner_t: $num_t, trait_fn_name: shl    , op_trait: ::core::ops::Shl     , op: << ],
            [inner_t: $num_t, trait_fn_name: shr    , op_trait: ::core::ops::Shr     , op: >> ],
        }

        macros::impl_unary_fns! {
            [inner_t: $num_t, signature: (self) -> $num_t       , fn_name: reverse_bits , fn_path: ::core::primitive::$num_t::reverse_bits  ],
        }

        macros::if_signed!{ $num_t, { macros::impl_unary_fns! {
            [inner_t: $num_t, signature: (self)         -> $num_t       , fn_name: abs          , fn_path: ::core::primitive::$num_t::abs           ],
            [inner_t: $num_t, signature: (self)         -> $num_t       , fn_name: strict_abs   , fn_path: ::core::primitive::$num_t::strict_abs    ],
            [inner_t: $num_t, signature: (self)         -> $uns_num_t   , fn_name: unsigned_abs , fn_path: ::core::primitive::$num_t::unsigned_abs  ],
        }}}

        macros::impl_binary_fns! {
            [inner_t: $num_t, signature: (self, $num_t) -> $uns_num_t   , fn_name: abs_diff     , fn_path: ::core::primitive::$num_t::abs_diff      ],
            [inner_t: $num_t, signature: (self, $num_t) -> u32          , fn_name: ilog         , fn_path: ::core::primitive::$num_t::ilog          ],

            [inner_t: $num_t, signature: (self, $num_t) -> $num_t       , fn_name: strict_add   , fn_path: ::core::primitive::$num_t::strict_add    ],
            [inner_t: $num_t, signature: (self, $num_t) -> $num_t       , fn_name: strict_sub   , fn_path: ::core::primitive::$num_t::strict_sub    ],
            [inner_t: $num_t, signature: (self, $num_t) -> $num_t       , fn_name: strict_mul   , fn_path: ::core::primitive::$num_t::strict_mul    ],
            [inner_t: $num_t, signature: (self, $num_t) -> $num_t       , fn_name: strict_div   , fn_path: ::core::primitive::$num_t::strict_div    ],

            [inner_t: $num_t, signature: (self, $num_t) -> $num_t       , fn_name: wrapping_add , fn_path: ::core::primitive::$num_t::wrapping_add  ],
            [inner_t: $num_t, signature: (self, $num_t) -> $num_t       , fn_name: wrapping_sub , fn_path: ::core::primitive::$num_t::wrapping_sub  ],
            [inner_t: $num_t, signature: (self, $num_t) -> $num_t       , fn_name: wrapping_mul , fn_path: ::core::primitive::$num_t::wrapping_mul  ],
            [inner_t: $num_t, signature: (self, $num_t) -> $num_t       , fn_name: wrapping_div , fn_path: ::core::primitive::$num_t::wrapping_div  ],

        }

    )*}
}
pub(crate) use impl_ints;
