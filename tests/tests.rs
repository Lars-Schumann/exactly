#[cfg(not(feature = "bench_compile_time"))]
mod tests {

    use exactly::*;

    #[test]
    fn one() {
        let foo: SetU8![1, 2, 3] = SetU8::new(2).unwrap();
        let bar: SetU8![10, 20] = SetU8::new(10).unwrap();
        let baz: SetU8![11, 21, 12, 22, 13, 23] = foo + bar;

        let _qux: SetU8![11, 12, 13, 21, 22, 23] = baz.sort();
    }

    #[test]
    fn two() {
        let foo: SetU8![1, 1, 1] = SetU8::new(1).unwrap();
        let bar: SetU8![10, 20] = SetU8::new(10).unwrap();
        let baz: SetU8![11, 21, 11, 21, 11, 21] = foo + bar;

        let _qux: SetU8![11, 11, 11, 21, 21, 21] = baz.sort();
        let _qox: SetU8![11, 21] = baz.normalize();
    }

    #[test]
    fn three() {
        let foo: SetU8![1, 1, 1, 2, 2] = SetU8::new(2).unwrap();
        let bar: SetU8![1, 2, 3] = SetU8::new(2).unwrap();
        let baz: SetU8![2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5] = (foo + bar).sort();

        let _qox: SetU8![2, 3, 4, 5] = baz.normalize();
    }

    #[test]
    fn four() {
        let foo: SetU8![2, 4] = SetU8::new(2).unwrap();
        let bar: SetU8![1, 2, 3] = SetU8::new(3).unwrap();
        let baz: SetU8![2, 4, 4, 6, 8, 12] = (foo * bar).sort();

        let _qox: SetU8![2, 4, 6, 8, 12] = baz.normalize();
    }

    #[test]
    fn five() {
        let a: SetI8![2, 4] = SetI8::new(4).unwrap();
        let b: SetI8![1, 3] = SetI8::new(1).unwrap();

        let _c: SetI8![3, 5, 7] = (a + b).normalize();
    }

    #[test]
    fn huge() {
        use set_u16::*;
        let r1: SetU16![Range![0..=4]] = SetU16::new(1).unwrap();
        let r2: SetU16![Range![10..=12]] = SetU16::new(10).unwrap();

        let _q: SetU16![0, 10, 11, 12, 20, 22, 24, 30, 33, 36, 40, 44, 48] = (r1 * r2).normalize();
    }

    #[test]
    fn bleh() {
        let _r1: SetU8![4] = SetU8::NEW::<4>;
    }

    #[test]
    fn onion() {
        use set_u32::{Range, Union};

        let _r1: SetU32![Union![Range![0..=2], Range![4..=5]]] =
            <SetU32![0, 1, 2, 4, 5]>::new(2).unwrap();
    }

    #[test]
    fn onion2() {
        use set_u32::{Range, Union};

        let _r1: SetU32![Union![Range![0..=2], Range![4..=5]]] =
            <SetU32![0, 1, 2, 4, 5]>::new(2).unwrap();
    }

    #[test]
    fn ranges() {
        use set_i8::Range;

        assert_eq!(Range![-3..2], &[-3, -2, -1, 0, 1]);
        assert_eq!(Range![125..], &[125, 126, 127]);
        assert_eq!(Range![-3..=2], &[-3, -2, -1, 0, 1, 2]);
        assert_eq!(Range![..-125], &[-128, -127, -126]);
        assert_eq!(Range![..=-125], &[-128, -127, -126, -125]);
    }

    #[test]
    fn intersections() {
        use set_i8::Intersection;
        use set_i8::Range;
        use set_i8::SORT;

        // Intersection on one set should be the identity
        assert_eq!(Intersection![Range![-3..27]], Range![-3..27]);

        // Intersection of one set with itself should be the identity
        assert_eq!(
            Intersection![Range![-3..27], Range![-3..27],],
            Range![-3..27]
        );

        // Intersection with the "full" set should be the identity
        assert_eq!(
            Intersection![Range![-3..27], Range![-128..=127]],
            Range![-3..27]
        );

        assert_eq!(
            SORT::<{ Intersection![Range![1..=20], Range![10..=30]] }>,
            Range![10..=20]
        );

        assert_eq!(
            SORT::<{ Intersection![Range![10..=50], Range![20..=100], Range![30..=40]] }>,
            Range![30..=40]
        );
    }

    #[test]
    fn widen() {
        // {5} is a subset of 5
        let _: SetU32<{ &[5] }> = SetU32::NEW::<5>.widen();

        // {4} is a subset of {1, 2, 3}
        let _: SetU32![1, 2, 3] = SetU32::NEW::<3>.widen();

        // {6, 4} is a subset of {3, 4, 5, 6}
        let _: SetU32![3, 4, 5, 6] = <SetU32![6, 4]>::new(6).unwrap().widen();

        // {4, 4, 4} is a subset of {4}, for now? subject to change?
        let _: SetU32<{ &[4] }> = <SetU32![4, 4, 4]>::new(4).unwrap().widen();
    }

    #[test]
    fn cast() {
        let _: SetU32<{ &[5] }> = SetU32::NEW::<5>.cast().unwrap();

        let _: SetU32![1, 2, 3] = SetU32::NEW::<3>.cast().unwrap();

        let _: SetU32![1, 2] = <SetU32![1, 2, 3]>::new(2).unwrap().cast().unwrap();

        // let _: SetU32![1, 2] = <SetU32![3, 4]>::new(4).unwrap().cast().unwrap();
    }

    #[test]
    fn generic() {
        use exactly::base::Set;

        let a: Set<u8, { &[5, 4] }> = Set::new(5).unwrap();
        let b: Set<u8, { &[1, 2] }> = Set::new(2).unwrap();

        let _c: Set<u8, { &[5, 6, 7] }> = (b + a).normalize();

        let x: Set<isize, { &[5, 4] }> = Set::new(5).unwrap();
        let y: Set<isize, { &[1, 2] }> = Set::new(2).unwrap();

        let _z: Set<isize, { &[4, 5, 8, 10] }> = (x * y).normalize();
    }

    #[test]
    fn generic2() {
        use exactly::base::Set;
        use exactly::base::set_u16::Range;

        let a: Set<u16, { Range![1..=4] }> = Set::new(5).unwrap();
        let b: Set<u16, { Range![2..=6] }> = Set::new(2).unwrap();

        let _c: Set<u16, { Range![3..=10] }> = (a + b).normalize();
    }
}
