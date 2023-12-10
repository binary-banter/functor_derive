#![allow(unused_parens)]

use functor_derive::Functor;
use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap};

// recursion,
// mutual recursion,


#[test]
fn struct_simple() {
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
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructSimple<u64>>()
    );
}

#[test]
fn struct_chained() {
    #[derive(Functor)]
    struct StructChained<A> {
        field_1: Option<A>,
        field_2: Option<A>,
        field_3: u32,
    }

    let x = StructChained::<usize> {
        field_1: Some(42),
        field_2: None,
        field_3: 13,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructChained<u64>>()
    );
}

#[test]
fn struct_tuple() {
    #[derive(Functor)]
    struct StructTuple<A> {
        field_1: (A, u8, A),
        field_2: u32,
    }

    let x = StructTuple::<usize> {
        field_1: (3, 5, 8),
        field_2: 13,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructTuple<u64>>()
    );
}

#[test]
fn struct_hashmap() {
    #[derive(Functor)]
    struct StructHashMap<A> {
        field_1: A,
        field_2: HashMap<u8, A>,
    }

    let x = StructHashMap::<usize> {
        field_1: 42,
        field_2: HashMap::from([(13, 255)]),
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructHashMap<u64>>()
    );
}

#[test]
fn struct_btreemap() {
    #[derive(Functor)]
    struct StructBTreeMap<A> {
        field_1: A,
        field_2: BTreeMap<u8, A>,
    }

    let x = StructBTreeMap::<usize> {
        field_1: 42,
        field_2: BTreeMap::from([(13, 255)]),
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructBTreeMap<u64>>()
    );
}

#[test]
fn struct_array() {
    #[derive(Functor)]
    struct StructArray<A> {
        field_1: [A; 3],
        field_2: u8,
    }

    let x = StructArray::<usize> {
        field_1: [1, 2, 3],
        field_2: 42,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructArray<u64>>()
    );
}

#[test]
fn struct_paren() {
    #[derive(Functor)]
    struct StructParen<A> {
        field_1: (A),
        field_2: u8,
    }

    let x = StructParen::<usize> {
        field_1: 1,
        field_2: 42,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructParen<u64>>()
    );
}

#[test]
fn enum_tuple() {
    #[derive(Functor)]
    enum EnumTuple<A> {
        Var1(A),
        Var2(bool),
        Var3,
    }

    let x = EnumTuple::Var1(18usize);

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<EnumTuple<u64>>()
    );
}

#[test]
fn enum_struct() {
    #[derive(Functor)]
    enum EnumStruct<A> {
        Var1 { x: A },
        Var2 { y: bool },
        Var3,
    }

    let x = EnumStruct::Var1 { x: 18usize };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<EnumStruct<u64>>()
    );
}

#[test]
fn enum_mixed() {
    #[derive(Functor)]
    enum EnumMixed<A> {
        Var1 { x: A },
        Var2(bool),
        Var3,
    }

    let x = EnumMixed::Var1 { x: 18usize };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<EnumMixed<u64>>()
    );
}

#[test]
fn generic_overload() {
    #[derive(Functor)]
    struct StructLifeTimes<'a, 'b, const N: usize, A, T> {
        field_1: A,
        field_2: &'a u32,
        field_3: &'b bool,
        field_4: T,
        field_5: [A; N],
    }

    let x = StructLifeTimes::<2, usize, bool> {
        field_1: 42,
        field_2: &13,
        field_3: &false,
        field_4: true,
        field_5: [1, 2],
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructLifeTimes<2, u64, bool>>()
    );
}

#[test]
fn indirect_generic() {
    #[derive(Functor)]
    struct IndirectGeneric<A> {
        field_1: Option<Option<A>>,
    }

    let x = IndirectGeneric {
        field_1: Some(Some(18usize)),
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<IndirectGeneric<u64>>()
    );
}

#[test]
fn indirect_tuple_generic() {
    #[derive(Functor)]
    struct IndirectTupleGeneric<A> {
        field_1: Vec<Vec<(usize, Vec<A>, Vec<Vec<A>>, usize)>>,
    }

    let x = IndirectTupleGeneric::<usize> {
        field_1: vec![vec![(18, vec![18], vec![vec![42]], 9)]],
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<IndirectTupleGeneric<u64>>()
    );
}

#[test]
fn chained_fmap() {
    #[derive(Functor)]
    struct StructSimple<A> {
        field_1: ApplePie<A, A>
    }

    // This incredible struct name was brought to you by Jonathan :)!
    #[derive(Functor)]
    struct ApplePie<A, B> {
        apple: A,
        pie: B,
    }

    let x = StructSimple::<usize> {
        field_1: ApplePie {
            apple: 15,
            pie: 17
        }
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructSimple<u64>>()
    );
}
