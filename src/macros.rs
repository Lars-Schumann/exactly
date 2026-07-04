macro_rules! impl_methods {
    ($([num_t: $num_t:ident, extra_mod: $extra_mod:ident, cartesian_const_name: $cartesian_const_name:ident, fn_name: $fn_name:ident, fn_path: $fn_path:path]),+ $(,)?) => {$(

        mod ${concat(__,$num_t,_,$fn_name)} {
            use crate::base::Set;

            pub(crate) const $cartesian_const_name<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { crate::base::CARTESIAN_LENGTH::<$num_t, A, B> }, _>(
                    const |i| {
                        let b_len: usize = B.len();
                        let a_index: usize = i.strict_div(b_len);
                        let b_index: usize = i.strict_rem(b_len);
                        let a: $num_t = A[a_index];
                        let b: $num_t = B[b_index];
                        $fn_path(a, b)
                    }
                )
            };

            const impl<const SET: &'static [$num_t]> Set<$num_t, SET> {
                pub fn $fn_name<const RHS_SET: &'static [$num_t]>(self, rhs: Set<$num_t, RHS_SET>) -> Set<$num_t, { $cartesian_const_name::<{ SET }, { RHS_SET }> }> {
                    let self_inner: $num_t = self.inner();
                    let rhs_inner: $num_t = rhs.inner();
                    let res_inner: $num_t = $fn_path(self_inner, rhs_inner);
                    unsafe { Set::new_unchecked(res_inner) }
                }
            }

        }

    )+}
}
pub(crate) use impl_methods;

macro_rules! impl_ops {
    ($([num_t: $num_t:ident, extra_mod: $extra_mod:ident, cartesian_const_name: $cartesian_const_name:ident, trait_fn_name: $trait_fn_name:ident, op_trait: $(::$op_trait:ident)+, op: $op:tt]),+ $(,)?) => {$(

        mod ${concat(__,$num_t,_,$trait_fn_name)} {
            use crate::base::Set;

            pub(crate) const $cartesian_const_name<const A: &'static[$num_t], const B: &'static[$num_t]>: &[$num_t] = const {
                &core::array::from_fn::<$num_t, { crate::base::CARTESIAN_LENGTH::<$num_t, A, B> }, _>(
                    const |i| {
                        let b_len: usize = B.len();
                        let a_index: usize = i.strict_div(b_len);
                        let b_index: usize = i.strict_rem(b_len);
                        let a: $num_t = A[a_index];
                        let b: $num_t = B[b_index];
                        a $op b
                    }
                )
            };

            const impl<const A_SET: &'static [$num_t], const B_SET: &'static [$num_t]> $(::$op_trait)+<Set<$num_t,B_SET> > for Set<$num_t, A_SET> {
                type Output = Set<$num_t, { $cartesian_const_name::<{ A_SET }, { B_SET }> }>;

                fn $trait_fn_name(self, rhs: Set<$num_t, B_SET>) -> Self::Output {
                    let self_inner: $num_t = self.inner();
                    let rhs_inner: $num_t = rhs.inner();
                    let res_inner: $num_t = self_inner $op rhs_inner;
                    unsafe { Set::new_unchecked(res_inner) }
                }
            }

        }
    )+}
}
pub(crate) use impl_ops;

macro_rules! impl_ints {
    (the_dolla: $d:tt, $([num_t: $num_t:ident, t_alias: $t_alias:ident, wide_num_t: $wide_num_t:ident, private_macro_prefix: $private_macro_prefix:ident, extra_mod: $extra_mod:ident],)*) => {$(

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
                ( $start:literal ..  $end:literal  ) => { $d crate::$extra_mod::RANGE::             <$start, $end>  };
                ( $start:literal ..                ) => { $d crate::$extra_mod::RANGE_FROM::        <$start>        };
                // RANGE_FULL omitted                          };
                ( $start:literal ..= $last:literal ) => { $d crate::$extra_mod::RANGE_INCLUSIVE::   <$start, $last> };
                (                ..  $end:literal  ) => { $d crate::$extra_mod::RANGE_TO::          <$end>          };
                (                ..= $last:literal ) => { $d crate::$extra_mod::RANGE_TO_INCLUSIVE::<$last>         };
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

        macros::impl_methods! {
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_ADD    , fn_name: strict_add   , fn_path: ::core::primitive::$num_t::strict_add    ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_SUB    , fn_name: strict_sub   , fn_path: ::core::primitive::$num_t::strict_sub    ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_MUL    , fn_name: strict_mul   , fn_path: ::core::primitive::$num_t::strict_mul    ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_DIV    , fn_name: strict_div   , fn_path: ::core::primitive::$num_t::strict_div    ],

            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_ADD  , fn_name: wrapping_add , fn_path: ::core::primitive::$num_t::wrapping_add  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_SUB  , fn_name: wrapping_sub , fn_path: ::core::primitive::$num_t::wrapping_sub  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_MUL  , fn_name: wrapping_mul , fn_path: ::core::primitive::$num_t::wrapping_mul  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_DIV  , fn_name: wrapping_div , fn_path: ::core::primitive::$num_t::wrapping_div  ],
        }

        macros::impl_ops! {
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_ADD      , trait_fn_name: add    , op_trait: ::core::ops::Add     , op: +  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_SUB      , trait_fn_name: sub    , op_trait: ::core::ops::Sub     , op: -  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_MUL      , trait_fn_name: mul    , op_trait: ::core::ops::Mul     , op: *  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_DIV      , trait_fn_name: div    , op_trait: ::core::ops::Div     , op: /  ],

            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_REM      , trait_fn_name: rem    , op_trait: ::core::ops::Rem     , op: %  ],

            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_BIT_AND  , trait_fn_name: bitand , op_trait: ::core::ops::BitAnd  , op: &  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_BIT_OR   , trait_fn_name: bitor  , op_trait: ::core::ops::BitOr   , op: |  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_BIT_XOR  , trait_fn_name: bitxor , op_trait: ::core::ops::BitXor  , op: ^  ],

            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_SHL      , trait_fn_name: shl    , op_trait: ::core::ops::Shl     , op: << ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_SHR      , trait_fn_name: shr    , op_trait: ::core::ops::Shr     , op: >> ],
        }

    )*}
}
pub(crate) use impl_ints;
