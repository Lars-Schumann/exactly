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
        let _r1: SetU8<{ &[4] }> = SetU8::NEW::<4>;
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
}
