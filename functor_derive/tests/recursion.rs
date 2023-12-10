#![allow(unused_parens)]

use functor_derive::Functor;
use std::any::{Any, TypeId};

#[test]
fn mutual_recursion() {
    #[derive(Functor)]
    struct TypeA<T> {
        b: Option<Box<TypeB<T>>>,
        v: T,
    }

    #[derive(Functor)]
    struct TypeB<T> {
        a: Option<Box<TypeA<T>>>,
    }

    let x = TypeA {
        b: Some(Box::new(TypeB {
            a: Some(Box::new(TypeA {
                b: None,
                v: 42usize,
            })),
        })),
        v: 42usize,
    };

    assert_eq!(x.fmap(|x| x as u64).type_id(), TypeId::of::<TypeA<u64>>());
}

#[test]
fn single_recursion() {
    #[derive(Functor)]
    struct TypeA<T> {
        b: Option<Box<TypeA<T>>>,
        v: T,
    }

    let x = TypeA {
        b: Some(Box::new(TypeA {
            b: Some(Box::new(TypeA {
                b: None,
                v: 42usize,
            })),
            v: 42usize,
        })),
        v: 42usize,
    };

    assert_eq!(x.fmap(|x| x as u64).type_id(), TypeId::of::<TypeA<u64>>());
}

#[test]
fn recursion_swap() {
    #[derive(Functor)]
    struct TypeA<S, T> {
        b: Option<Box<TypeA<T, S>>>,
        v1: S,
        v2: T,
    }

    let x = TypeA {
        b: Some(Box::new(TypeA {
            b: None,
            v1: 40usize,
            v2: 41usize,
        })),
        v1: 42usize,
        v2: 43usize,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<TypeA<u64, usize>>()
    );
}

#[test]
fn recursion_swap_mutual() {
    #[derive(Functor)]
    struct TypeA<S, T> {
        b: Option<Box<TypeB<T, S>>>,
        v1: S,
        v2: T,
    }

    #[derive(Functor)]
    struct TypeB<S, T> {
        b: Option<Box<TypeA<T, S>>>,
        v1: S,
        v2: T,
    }

    let x = TypeA {
        b: Some(Box::new(TypeB {
            b: None,
            v1: 40usize,
            v2: 41usize,
        })),
        v1: 42usize,
        v2: 43usize,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<TypeA<u64, usize>>()
    );
}
