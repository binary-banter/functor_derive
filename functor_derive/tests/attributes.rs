use functor_derive::Functor;
use std::any::{Any, TypeId};

// attribute with default,
// attribute with name_map,
// attribute with default and name_map

#[test]
fn default_single() {
    #[derive(Functor)]
    #[functor(S)]
    struct MyType<S> {
        v1: S,
        v2: bool,
    }

    let x = MyType {
        v1: 42usize,
        v2: true,
    };

    assert_eq!(x.fmap(|x| x as u64).type_id(), TypeId::of::<MyType<u64>>());
}

#[test]
fn default_multiple_out_of_order() {
    #[derive(Functor)]
    #[functor(T)]
    struct MyType<S, T> {
        v1: S,
        v2: T,
    }

    let x = MyType {
        v1: true,
        v2: 18usize,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<MyType<bool, u64>>()
    );
}

#[test]
fn map_specified_and_name() {
    #[derive(Copy, Clone, Functor)]
    #[functor(S, S as stuff, S as gunk)]
    struct MyType<S> {
        v1: S,
        v2: bool,
    }

    let x = MyType {
        v1: 42usize,
        v2: true,
    };

    assert_eq!(x.fmap(|x| x as u64).type_id(), TypeId::of::<MyType<u64>>());

    assert_eq!(
        x.fmap_stuff(|x| x as u64).type_id(),
        TypeId::of::<MyType<u64>>()
    );

    assert_eq!(
        x.fmap_gunk(|x| x as u64).type_id(),
        TypeId::of::<MyType<u64>>()
    );
}

#[test]
fn map_multi() {
    #[derive(Copy, Clone, Functor)]
    #[functor(T, S as wonky)]
    struct MyType<S, T> {
        v1: S,
        v2: bool,
        v3: T,
    }

    let x = MyType {
        v1: 42usize,
        v2: true,
        v3: false,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<MyType<usize, u64>>()
    );

    assert_eq!(
        x.fmap_wonky(|x| x as u64).type_id(),
        TypeId::of::<MyType<u64, bool>>()
    );
}
