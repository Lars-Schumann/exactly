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

use exactly::Pu32;
use exactly::to_pattern_type;

#[test]
fn it_works() {
    let x: Pu32<1, 2> = Pu32::new(1);
    let y: Pu32<12, 14> = x.add::<5>().mul::<2>();
    dbg!(std::any::type_name_of_val(&y));

    to_pattern_type!(let out_name: u32 is 12..=14 = y);

    dbg!(std::any::type_name_of_val(&out_name));
}
