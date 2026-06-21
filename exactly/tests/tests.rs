#![feature(min_generic_const_args, generic_const_args, generic_const_items)]
#![allow(incomplete_features)]

use exactly::int::Ri32;
use exactly::int::Ru32;
use exactly::int::exact_u32;

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
    let a: Ri32<-2, 50> = Ri32::new(12).unwrap();
    let b: Ri32<1, 3> = Ri32::new(3).unwrap();

    let c: Ri32<-2, 50> = a / b;

    assert_eq!(c.inner(), 4);
}

#[test]
fn test_widen() {
    let a: Ri32<1, 3> = Ri32::new(1).unwrap();
    let b: Ri32<-1, 4> = a.widen();
    dbg!(b);
}

#[test]
fn range_exactly() {
    let a: Ru32<2, 4> = Ru32::new(3).unwrap();
    let b: Ru32<5, 5> = exact_u32::<5>();
    let c: Ru32<10, 20> = a * b;

    assert_eq!(c.inner(), 15);
}
