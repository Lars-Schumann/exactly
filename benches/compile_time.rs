#![allow(long_running_const_eval)]

#[cfg(feature = "bench_compile_time")]
fn main() {
    {
        use sure::SureU8;
        use sure::sure_u8::Range;

        let a: SureU8![Range![0..=127]] = SureU8::new(1).unwrap();
        let b: SureU8![Range![0..=127]] = SureU8::new(1).unwrap();
        let c: SureU8![Range![0..=254]] = (a + b).normalize();
        assert_eq!(c.inner(), 2);

        let x: SureU8![Range![0..=255]] = SureU8::new(1).unwrap();
        let y: SureU8![Range![1..=255]] = SureU8::new(1).unwrap();
        let z: SureU8![Range![0..=255]] = (x / y).normalize();
        assert_eq!(z.inner(), 1);
    }

    {
        use sure::SureI32;
        use sure::sure_i32::Range;

        let a: SureI32![Range![0..=50]] = SureI32::new(1).unwrap();
        let b: SureI32![Range![0..=50]] = SureI32::new(1).unwrap();
        let c: SureI32![Range![0..=100]] = (a + b).normalize();
        assert_eq!(c.inner(), 2);

        let x: SureI32![Range![0..=50]] = SureI32::new(1).unwrap();
        let y: SureI32![Range![0..=50]] = SureI32::new(1).unwrap();
        let z = (x * y).normalize();
        assert_eq!(z.inner(), 1);
    }

    println!("bench done!");
}

#[cfg(not(feature = "bench_compile_time"))]
fn main() {
    println!("bench skipped!")
}

// 29.5s
// 25.0s
// 26.7s
// 40.2s
