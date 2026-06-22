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

mod macros;

macros::impl_ints! {[inner_type: u8, wrap_t_name: Wu8, extra_tcm: extra_tcm_u8, sort_fn_name: into_sorted_u8_array],}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yee() {
        let foo: Wu8<{ const { &[1, 2, 3] } }> = Wu8(2);
        let bar: Wu8<{ const { &[10, 20] } }> = Wu8(10);

        let baz: Wu8<{ const { &[11, 21, 12, 22, 13, 23] } }> = foo.add(bar);

        let qux: Wu8<{ const { &[11, 12, 13, 21, 22, 23] } }> = baz.sort();
    }

    #[test]
    fn tee() {
        let foo: Wu8<{ const { &[1, 1, 1] } }> = Wu8(2);
        let bar: Wu8<{ const { &[10, 20] } }> = Wu8(10);
        let baz: Wu8<{ const { &[11, 21, 11, 21, 11, 21] } }> = foo.add(bar);

        let _qux: Wu8<{ const { &[11, 11, 11, 21, 21, 21] } }> = baz.sort();
        let _qox: Wu8<{ const { &[11, 21] } }> = baz.normalize();
    }
}
