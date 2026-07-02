macro_rules! implinator {
    ($([num_t: $num_t:ident, extra_mod: $extra_mod:ident, cartesian_const_name: $cartesian_const_name:ident, fn_name: $fn_name:ident, fn_path: $fn_path:path]),+ $(,)?) => {$(

        #[expect(nonstandard_style)]
        mod ${concat(__mod_,$num_t,_,$cartesian_const_name)} {

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

        const impl<const SET: &'static [$num_t]> Set<$num_t, SET> {

            pub fn $fn_name<const RHS_SET: &'static [$num_t]>(self, rhs: Set<$num_t, RHS_SET>) -> Set<$num_t, { ${concat(__mod_,$num_t,_,$cartesian_const_name)}::$cartesian_const_name::<{ SET }, { RHS_SET }> }> {
                unsafe { Set::new_unchecked($fn_path(self.inner(), rhs.inner())) }
            }

        }

    )+}
}
pub(crate) use implinator;

macro_rules! ops_implinator {
    ($([num_t: $num_t:ident, extra_mod: $extra_mod:ident, cartesian_const_name: $cartesian_const_name:ident, trait_fn_name: $trait_fn_name:ident, op_trait: $(::$op_trait:ident)+, op: $op:tt]),+ $(,)?) => {$(

        #[expect(nonstandard_style)]
        mod ${concat(__mod_,$num_t,_,$cartesian_const_name)} {

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

        const impl<const A_SET: &'static [$num_t], const B_SET: &'static [$num_t]> $(::$op_trait)+<Set<$num_t,B_SET> > for Set<$num_t, A_SET> {

            type Output = Set<$num_t, { ${concat(__mod_,$num_t,_,$cartesian_const_name)}::$cartesian_const_name::<{ A_SET }, { B_SET }> }>;

            fn $trait_fn_name(self, rhs: Set<$num_t, B_SET>) -> Self::Output {
                unsafe { Set::new_unchecked(self.inner() $op rhs.inner()) }
            }

        }
    )+}
}
pub(crate) use ops_implinator;

macro_rules! impl_ints {
    (the_dolla: $d:tt, $([inner_type: $num_t:ident, largest_num_t_with_same_signedness: $largest_num_t_with_same_signedness:ident, wrap_t_name: $wrap_t_name:ident, private_macro_prefix: $private_macro_prefix:ident, extra_mod: $extra_mod:ident],)*) => {$(

        base_macros::implinator! {
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_ADD    , fn_name: strict_add   , fn_path: ::core::primitive::$num_t::strict_add    ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_SUB    , fn_name: strict_sub   , fn_path: ::core::primitive::$num_t::strict_sub    ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_MUL    , fn_name: strict_mul   , fn_path: ::core::primitive::$num_t::strict_mul    ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_STRICT_DIV    , fn_name: strict_div   , fn_path: ::core::primitive::$num_t::strict_div    ],

            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_ADD  , fn_name: wrapping_add , fn_path: ::core::primitive::$num_t::wrapping_add  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_SUB  , fn_name: wrapping_sub , fn_path: ::core::primitive::$num_t::wrapping_sub  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_MUL  , fn_name: wrapping_mul , fn_path: ::core::primitive::$num_t::wrapping_mul  ],
            [num_t: $num_t, extra_mod: $extra_mod, cartesian_const_name: CARTESIAN_WRAPPING_DIV  , fn_name: wrapping_div , fn_path: ::core::primitive::$num_t::wrapping_div  ],
        }

        base_macros::ops_implinator! {
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
