#![allow(long_running_const_eval)]

#[cfg(feature = "bench_compile_time")]
fn main() {
    {
        use funny::SetU8;
        use funny::extra_tcm_u8;

        let a: SetU8<{ extra_tcm_u8::RANGE::<0, 127> }> = SetU8::new(1).unwrap();
        let b: SetU8<{ extra_tcm_u8::RANGE::<0, 127> }> = SetU8::new(1).unwrap();
        let c: SetU8<{ extra_tcm_u8::RANGE::<0, 254> }> = (a + b).normalize();
        assert_eq!(c.inner(), 2);

        let x: SetU8<{ extra_tcm_u8::RANGE::<0, 255> }> = SetU8::new(1).unwrap();
        let y: SetU8<{ extra_tcm_u8::RANGE::<1, 255> }> = SetU8::new(1).unwrap();
        let z: SetU8<{ extra_tcm_u8::RANGE::<0, 255> }> = (x / y).normalize();
        assert_eq!(z.inner(), 1);
    }

    {
        use funny::SetI32;
        use funny::extra_tcm_i32;

        let a: SetI32<{ extra_tcm_i32::RANGE::<0, 50> }> = SetI32::new(1).unwrap();
        let b: SetI32<{ extra_tcm_i32::RANGE::<0, 50> }> = SetI32::new(1).unwrap();
        let c: SetI32<{ extra_tcm_i32::RANGE::<0, 100> }> = (a + b).normalize();
        assert_eq!(c.inner(), 2);

        let x: SetI32<{ extra_tcm_i32::RANGE::<0, 50> }> = SetI32::new(1).unwrap();
        let y: SetI32<{ extra_tcm_i32::RANGE::<0, 50> }> = SetI32::new(1).unwrap();
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
