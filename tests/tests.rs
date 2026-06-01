#![feature(
    macro_metavar_expr_concat,
    min_generic_const_args,
    generic_const_args,
    generic_const_items,
    inherent_associated_types,
    pattern_types,
    pattern_type_macro
)]
#![allow(incomplete_features)]

use exactly::Ru32;
use exactly::to_pattern_type;
use std::any::type_name_of_val;
use std::mem::transmute;

#[test]
fn demo() {
    let foo: Ru32<1, 3> = Ru32::new(2).unwrap();
    let bar: Ru32<12, 16> = foo.add::<5>().mul::<2>();

    dbg!(type_name_of_val(&bar), bar);

    to_pattern_type!(let baz: u32 is 12..=16 = bar);

    dbg!(type_name_of_val(&baz), unsafe {
        transmute::<pattern_type!(u32 is 12..=16), u32>(baz)
    });

    let qux: Ru32<13, 19> = foo + bar;
}
