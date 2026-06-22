#![feature(
    const_trait_impl,
    generic_const_args,
    generic_const_items,
    min_generic_const_args,
    adt_const_params,
    unsized_const_params,
    const_closures,
    const_array,
    const_convert,
    maybe_uninit_array_assume_init
)]
#![allow(incomplete_features)]
#![allow(long_running_const_eval)]

mod macros;

macros::impl_ints! {[inner_type: u8, wrap_t_name: SetU8, range_fn_name: range_u8, extra_tcm: extra_tcm_u8, sort_fn_name: into_sorted_u8_array],}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let foo: SetU8<{ const { &[1, 2, 3] } }> = SetU8::new(2).unwrap();
        let bar: SetU8<{ const { &[10, 20] } }> = SetU8::new(10).unwrap();

        let baz: SetU8<{ const { &[11, 21, 12, 22, 13, 23] } }> = foo + bar;

        let _qux: SetU8<{ const { &[11, 12, 13, 21, 22, 23] } }> = baz.sort();
    }

    #[test]
    fn two() {
        let foo: SetU8<{ const { &[1, 1, 1] } }> = SetU8::new(1).unwrap();
        let bar: SetU8<{ const { &[10, 20] } }> = SetU8::new(10).unwrap();
        let baz: SetU8<{ const { &[11, 21, 11, 21, 11, 21] } }> = foo + bar;

        let _qux: SetU8<{ const { &[11, 11, 11, 21, 21, 21] } }> = baz.sort();
        let _qox: SetU8<{ const { &[11, 21] } }> = baz.normalize();
    }

    #[test]
    fn three() {
        let foo: SetU8<{ const { &[1, 1, 1, 2, 2] } }> = SetU8::new(2).unwrap();
        let bar: SetU8<{ const { &[1, 2, 3] } }> = SetU8::new(2).unwrap();
        let baz: SetU8<{ const { &[2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5] } }> =
            (foo + bar).sort();

        let _qox: SetU8<{ const { &[2, 3, 4, 5] } }> = baz.normalize();
    }

    #[test]
    fn four() {
        let foo: SetU8<{ const { &[2, 4] } }> = SetU8::new(2).unwrap();
        let bar: SetU8<{ const { &[1, 2, 3] } }> = SetU8::new(3).unwrap();
        let baz: SetU8<{ const { &[2, 4, 4, 6, 8, 12] } }> = (foo * bar).sort();

        let _qox: SetU8<{ const { &[2, 4, 6, 8, 12] } }> = baz.normalize();
    }

    #[test]
    fn five() {
        let four: SetU8<{ const { &[4] } }> = SetU8::new(4).unwrap();
        let bar: SetU8<{ const { &[1, 2, 3] } }> = SetU8::new(3).unwrap();

        let _qox: SetU8<{ const { &[1, 2, 3] } }> = bar * four / four + four - four;
    }

    #[test]
    fn six() {
        let r1: SetU8<{ extra_tcm_u8::RANGE::<0, 255> }> = SetU8::new(2).unwrap();
        let r2: SetU8<{ extra_tcm_u8::RANGE::<1, 255> }> = SetU8::new(4).unwrap();

        let p: SetU8<{ extra_tcm_u8::RANGE::<0, 255> }> = (r1 / r2).normalize();
    }
}
