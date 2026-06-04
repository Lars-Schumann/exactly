#![feature(min_generic_const_args, generic_const_args, generic_const_items)]
#![allow(incomplete_features)]

use exactly::MakeF32;
use exactly::MakeRangeF32;
use exactly::float::Rf32;
use exactly::int::Ri32;
use exactly::int::Ru32;
use std::any::type_name_of_val;

#[test]
fn demo() {
    let foo: Ru32<1, 3> = Ru32::new(2).unwrap();
    let bar: Ru32<12, 16> = foo.add::<5>().mul::<2>();

    dbg!(type_name_of_val(&bar), bar);

    let _qux: Ru32<13, 19> = foo + bar;
}

#[test]
fn range_add() {
    let a: Ru32<1, 3> = Ru32::new(2).unwrap();
    let b: Ru32<4, 5> = Ru32::new(5).unwrap();

    let c: Ru32<5, 8> = a + b;

    assert_eq!(c.inner(), 7);
}

#[test]
fn range_sub() {
    let a: Ri32<5, 6> = Ri32::new(5).unwrap();
    let b: Ri32<2, 8> = Ri32::new(7).unwrap();

    let c: Ri32<-3, 4> = a - b;

    assert_eq!(c.inner(), -2);
}

#[test]
fn range_mul() {
    let a: Ri32<-2, 4> = Ri32::new(3).unwrap();
    let b: Ri32<-10, 3> = Ri32::new(-9).unwrap();

    let c: Ri32<-40, 20> = a * b;

    assert_eq!(c.inner(), -27);
}

#[test]
fn range_div() {
    let a: Ri32<-2, 50> = Ri32::new(10).unwrap();
    let b: Ri32<-25, 3> = Ri32::new(-5).unwrap();

    let c: Ri32<-50, 50> = a / b;

    assert_eq!(c.inner(), -2);
}

#[test]
fn test_float() {
    let a: MakeRangeF32![0.0, 3.0] = Rf32::new(MakeF32!(2.5)).unwrap();
    let b: MakeRangeF32![0.0, 8.5] = Rf32::new(MakeF32!(5.22)).unwrap();

    let c: MakeRangeF32![0.0, 11.5] = a + b;

    assert_eq!(c.inner().inner(), 7.72);
}
