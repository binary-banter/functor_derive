use crate::funcmap::{T1, T2};
use functor_derive_lib::Functor;

#[test]
fn generics_to_be_mapped_can_be_configured() {
    fn noop() {}

    #[derive(Functor, Debug, PartialEq)]
    #[functor(S as s, U as u)]
    struct Test<S, T, U> {
        value1: S,
        not_mappable: fn() -> T,
        value2: U,
    }

    let src = Test {
        value1: T1,
        not_mappable: noop,
        value2: T1,
    };
    let dst = src.fmap_s(|_| T2).fmap_u(|_| T2);

    assert_eq!(
        dst,
        Test {
            value1: T2,
            not_mappable: noop,
            value2: T2,
        }
    );
}

// Our macro invocation does *not* allow trailing commas.
// #[test]
// fn opts_accept_trailing_comma() {
//     #[derive(Functor)]
//     #[functor(S as s, T as t)]
//     struct Test<S, T>(S, T);
// }
//
// #[test]
// fn params_opt_accepts_trailing_comma() {
//     #[derive(Functor)]
//     #[functor(S as s, T as t,)]
//     struct Test<S, T>(S, T);
// }
