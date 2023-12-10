use std::any::{Any, TypeId};
use functor_derive::Functor;

fn test_try(v: usize) -> Result<u64, ()> {
    Ok(v as u64)
}

#[test]
fn try_struct_simple() {
    #[derive(Functor)]
    struct StructSimple<A> {
        field_1: A,
        field_2: u32,
    }

    let x = StructSimple::<usize> {
        field_1: 42,
        field_2: 13,
    };

    assert_eq!(
        x.try_fmap(test_try).type_id(),
        TypeId::of::<Result<StructSimple<u64>, ()>>()
    );
}

#[test]
fn try_recursive() {
    #[derive(Functor)]
    struct StructSimple<A> {
        field_1: Option<Box<StructSimple<A>>>,
        field_2: A,
    }

    let x = StructSimple::<usize> {
        field_1: Some(Box::new(StructSimple {
            field_1: None,
            field_2: 15,
        })),
        field_2: 13,
    };

    assert_eq!(
        x.try_fmap(test_try).type_id(),
        TypeId::of::<Result<StructSimple<u64>, ()>>()
    );
}
