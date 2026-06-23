#![allow(long_running_const_eval)]

#[cfg(feature = "bench_compile_time")]
fn main() {
    use funny::SetU16;
    use funny::extra_tcm_u16;

    let r1: SetU16<{ extra_tcm_u16::RANGE::<0, 131> }> = SetU16::new(1).unwrap();
    let r2: SetU16<{ extra_tcm_u16::RANGE::<1, 131> }> = SetU16::new(10).unwrap();

    let _q /* :SetU16<{ extra_tcm_u16::RANGE::<0, 131> }>*/ = (r1 / r2).normalize();
}
