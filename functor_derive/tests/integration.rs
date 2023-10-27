use functor_derive::Functor;
use functor_derive_lib::Functor;
use std::any::{Any, TypeId};
use std::collections::VecDeque;

#[test]
fn struct_simple() {
    #[derive(Debug, Functor)]
    struct StructSimple<A> {
        field_1: A,
        field_2: u32,
    }

    let x = StructSimple::<usize> {
        field_1: 42,
        field_2: 13,
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructSimple<u64>>()
    );
}

#[test]
fn struct_option() {
    #[derive(Debug, Functor)]
    struct StructOption<A> {
        field_1: Option<A>,
        field_2: Option<A>,
        field_3: u32,
    }

    let x = StructOption::<usize> {
        field_1: Some(42),
        field_2: None,
        field_3: 13,
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructOption<u64>>()
    );
}

#[test]
fn struct_vec() {
    #[derive(Debug, Functor)]
    struct StructVec<A> {
        field_1: A,
        field_2: Vec<A>,
    }

    let x = StructVec::<usize> {
        field_1: 42,
        field_2: vec![13, 14, 15],
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructVec<u64>>()
    );
}

#[test]
fn struct_vecdeque() {
    #[derive(Debug, Functor)]
    struct StructVecDeque<A> {
        field_1: A,
        field_2: VecDeque<A>,
    }

    let x = StructVecDeque::<usize> {
        field_1: 42,
        field_2: VecDeque::from([13, 14, 15]),
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructVecDeque<u64>>()
    );
}

#[test]
fn struct_tuple() {
    #[derive(Debug, Functor)]
    struct StructTuple<A> {
        field_1: (A, u8, A),
        field_2: u32,
    }

    let x = StructTuple::<usize> {
        field_1: (3, 5, 8),
        field_2: 13,
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructTuple<u64>>()
    );
}

// #[test]
// fn struct_hashmap(){
//     #[derive(Debug, Functor)]
//     struct StructHashMap<A> {
//         field_1: A,
//         field_2: HashMap<A, u8>,
//     }
//
//     let x = StructHashMap::<usize> {
//         field_1: 42,
//         field_2: HashMap::from([(13, 255)]),
//     };
//
//     assert_eq!(x.fmap(&mut |x| x as u64).type_id(), TypeId::of::<StructHashMap<u64>>());
// }
