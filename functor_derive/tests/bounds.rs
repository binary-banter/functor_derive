#![allow(unused_parens)]

use functor_derive::Functor;
use std::any::{Any, TypeId};
use std::fmt::Display;

#[test]
fn trait_bound() {
    #[derive(Functor)]
    struct StructSimple<A: Display> {
        field_1: A,
    }

    let x = StructSimple::<usize> { field_1: 42 };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructSimple<u64>>()
    );
}

#[test]
fn trait_bound_named() {
    #[derive(Functor)]
    #[functor(A as apple)]
    struct StructSimple<A: Display> {
        field_1: A,
    }

    let x = StructSimple::<usize> { field_1: 42 };

    assert_eq!(
        x.fmap_apple(|x| x as u64).type_id(),
        TypeId::of::<StructSimple<u64>>()
    );
}

#[test]
fn trait_ignored() {
    #[derive(Functor)]
    #[functor(A as apple)]
    struct StructSimple<A, B: Display> {
        field_1: A,
        field_2: B,
    }

    let x = StructSimple::<usize, usize> {
        field_1: 42,
        field_2: 43,
    };

    assert_eq!(
        x.fmap_apple(|x| x as u64).type_id(),
        TypeId::of::<StructSimple<u64, usize>>()
    );
}
