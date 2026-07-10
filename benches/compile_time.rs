#![allow(long_running_const_eval)]

#[cfg(feature = "bench_compile_time")]
fn main() {
    {
        use sure::SetU8;
        use sure::set_u8::Range;

        let a: SetU8![Range![0..=127]] = SetU8::new(1).unwrap();
        let b: SetU8![Range![0..=127]] = SetU8::new(1).unwrap();
        let c: SetU8![Range![0..=254]] = (a + b).normalize();
        assert_eq!(c.inner(), 2);

        let x: SetU8![Range![0..=255]] = SetU8::new(1).unwrap();
        let y: SetU8![Range![1..=255]] = SetU8::new(1).unwrap();
        let z: SetU8![Range![0..=255]] = (x / y).normalize();
        assert_eq!(z.inner(), 1);
    }

    {
        use sure::SetI32;
        use sure::set_i32::Range;

        let a: SetI32![Range![0..=50]] = SetI32::new(1).unwrap();
        let b: SetI32![Range![0..=50]] = SetI32::new(1).unwrap();
        let c: SetI32![Range![0..=100]] = (a + b).normalize();
        assert_eq!(c.inner(), 2);

        let x: SetI32![Range![0..=50]] = SetI32::new(1).unwrap();
        let y: SetI32![Range![0..=50]] = SetI32::new(1).unwrap();
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
