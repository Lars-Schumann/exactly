#![feature(min_generic_const_args, generic_const_args, generic_const_items)]
#![allow(incomplete_features)]

use exactly::exact_u32;
use exactly::ri32;
use exactly::ru32;

#[test]
fn range_add() {
    let a: ru32<1, 3> = ru32::new(2).unwrap();
    let b: ru32<4, 5> = ru32::new(5).unwrap();
    let c: ru32<5, 8> = a + b;

    assert_eq!(c.inner(), 7);
}

#[test]
fn range_sub() {
    let a: ri32<5, 6> = ri32::new(5).unwrap();
    let b: ri32<2, 8> = ri32::new(7).unwrap();

    let c: ri32<-3, 4> = a - b;

    assert_eq!(c.inner(), -2);
}

#[test]
fn range_mul() {
    let a: ri32<-2, 4> = ri32::new(3).unwrap();
    let b: ri32<-10, 3> = ri32::new(-9).unwrap();

    let c: ri32<-40, 20> = a * b;

    assert_eq!(c.inner(), -27);
}

#[test]
fn range_div() {
    let a: ri32<-2, 50> = ri32::new(12).unwrap();
    let b: ri32<1, 3> = ri32::new(3).unwrap();

    let c: ri32<-2, 50> = a / b;

    assert_eq!(c.inner(), 4);
}

#[test]
fn test_widen() {
    let a: ri32<1, 3> = ri32::new(1).unwrap();
    let b: ri32<-1, 4> = a.widen();
    dbg!(b);
}

#[test]
fn range_exactly() {
    let a: ru32<2, 4> = ru32::new(3).unwrap();
    let b: ru32<5, 5> = exact_u32::<5>();
    let c: ru32<10, 20> = a * b;

    assert_eq!(c.inner(), 15);
}
