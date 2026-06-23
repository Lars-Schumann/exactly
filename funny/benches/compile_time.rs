#![allow(long_running_const_eval)]

#[cfg(feature = "bench_compile_time")]
fn main() {
    use funny::SetI32;
    use funny::extra_tcm_i32;

    let r1: SetI32<{ extra_tcm_i32::RANGE::<0, 115> }> = SetI32::new(1).unwrap();
    let r2: SetI32<{ extra_tcm_i32::RANGE::<1, 115> }> = SetI32::new(10).unwrap();

    let _q /* :SetI32<{ extra_tcm_i32::RANGE::<0, 115> }>*/ = (r1 / r2).normalize();
}
