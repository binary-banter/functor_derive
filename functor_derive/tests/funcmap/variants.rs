use crate::funcmap::{T1, T2};
use functor_derive::Functor;

#[test]
fn tuple_struct_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(T, i32, T);

    let src = Test(T1, 42, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, 42, T2));
}

#[test]
fn struct_with_named_fields_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T> {
        value0: T,
        value1: i32,
        value2: T,
    }

    let src = Test {
        value0: T1,
        value1: 42,
        value2: T1,
    };
    let dst = src.fmap(|_| T2);

    assert_eq!(
        dst,
        Test {
            value0: T2,
            value1: 42,
            value2: T2
        }
    );
}

#[test]
fn enum_unit_variant_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    enum Test<T> {
        Unit,
        Tuple(T, i32, T),
        Named {
            value_0: T,
            value_1: i32,
            value_2: T,
        },
    }

    let src: Test<T1> = Test::Unit;
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test::Unit);
}

#[test]
fn enum_tuple_variant_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    enum Test<T> {
        Unit,
        Tuple(T, i32, T),
        Named {
            value_0: T,
            value_1: i32,
            value_2: T,
        },
    }

    let src = Test::Tuple(T1, 42, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test::Tuple(T2, 42, T2));
}

#[test]
fn enum_variant_with_named_fields_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    enum Test<T> {
        Unit,
        Tuple(T, i32, T),
        Named {
            value_0: T,
            value_1: i32,
            value_2: T,
        },
    }

    let src = Test::Named {
        value_0: T1,
        value_1: 42,
        value_2: T1,
    };
    let dst = src.fmap(|_| T2);

    assert_eq!(
        dst,
        Test::Named {
            value_0: T2,
            value_1: 42,
            value_2: T2
        }
    );
}
