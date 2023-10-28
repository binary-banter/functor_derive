#![allow(unused_parens)]
#![allow(dead_code)]

use functor_derive::Functor;
use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::marker::PhantomData;

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
fn struct_option() {
    #[derive(Functor)]
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
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructOption<u64>>()
    );
}

#[test]
fn struct_vec() {
    #[derive(Functor)]
    struct StructVec<A> {
        field_1: A,
        field_2: Vec<A>,
    }

    let x = StructVec::<usize> {
        field_1: 42,
        field_2: vec![13, 14, 15],
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructVec<u64>>()
    );
}

#[test]
fn struct_vecdeque() {
    #[derive(Functor)]
    struct StructVecDeque<A> {
        field_1: A,
        field_2: VecDeque<A>,
    }

    let x = StructVecDeque::<usize> {
        field_1: 42,
        field_2: VecDeque::from([13, 14, 15]),
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructVecDeque<u64>>()
    );
}

#[test]
fn struct_tuple_1() {
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
fn struct_tuple_2() {
    #[derive(Functor)]
    struct StructTuple<A> {
        field_1: (Vec<A>, u8, A),
        field_2: u32,
    }

    let x = StructTuple::<usize> {
        field_1: (vec![3], 5, 8),
        field_2: 13,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructTuple<u64>>()
    );
}

#[test]
fn struct_phantomdata() {
    #[derive(Functor)]
    struct StructPhantomData<A> {
        field_1: PhantomData<A>,
        field_2: u32,
    }

    let x = StructPhantomData::<usize> {
        field_1: PhantomData::default(),
        field_2: 13,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructPhantomData<u64>>()
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
fn struct_array_1() {
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
fn struct_array_2() {
    #[derive(Functor)]
    struct StructArray<A> {
        field_1: [(Vec<A>, usize); 3],
        field_2: u8,
    }

    let x = StructArray::<usize> {
        field_1: [(vec![1], 1), (vec![2], 2), (vec![3], 3)],
        field_2: 42,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructArray<u64>>()
    );
}

#[test]
fn struct_paren_1() {
    #[derive(Functor)]
    struct StructArray<A> {
        field_1: (A),
        field_2: u8,
    }

    let x = StructArray::<usize> {
        field_1: 1,
        field_2: 42,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructArray<u64>>()
    );
}

#[test]
fn struct_paren_2() {
    #[derive(Functor)]
    struct StructArray<A> {
        field_1: Vec<(A)>,
        field_2: u8,
    }

    let x = StructArray::<usize> {
        field_1: vec![1],
        field_2: 42,
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructArray<u64>>()
    );
}

#[test]
fn enum_simple_tuple() {
    #[derive(Functor)]
    enum EnumTuple<A> {
        Var1(A),
        Var2(bool),
        Var3,
    }

    let x = EnumTuple::<usize>::Var1(18);

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<EnumTuple<u64>>()
    );
}

#[test]
fn enum_simple_struct() {
    #[derive(Functor)]
    enum EnumStruct<A> {
        Var1 { x: A },
        Var2 { y: bool },
        Var3,
    }

    let x = EnumStruct::<usize>::Var1 { x: 18 };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<EnumStruct<u64>>()
    );
}

#[test]
fn enum_simple_mixed() {
    #[derive(Functor)]
    enum EnumMixed<A> {
        Var1 { x: A },
        Var2(bool),
        Var3,
    }

    let x = EnumMixed::<usize>::Var1 { x: 18 };

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
fn struct_simple_trait() {
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
fn struct_indirect_generic() {
    #[derive(Functor)]
    struct StructSimple<A> {
        field_1: Vec<Vec<A>>,
    }

    let x = StructSimple::<usize> {
        field_1: vec![vec![18]],
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructSimple<u64>>()
    );
}

#[test]
fn struct_super_indirect_generic() {
    #[derive(Functor)]
    struct StructSimple<A> {
        field_1: Vec<Vec<(usize, Vec<A>, Vec<Vec<A>>, usize)>>,
    }

    let x = StructSimple::<usize> {
        field_1: vec![vec![(18, vec![18], vec![vec![42]], 9)]],
    };

    assert_eq!(
        x.fmap(|x| x as u64).type_id(),
        TypeId::of::<StructSimple<u64>>()
    );
}

// #[test]
// fn struct_simple_explicit() {
//     #[derive(Functor)]
//     #[functor(B)]
//     struct StructSimple<A, B> {
//         field_1: A,
//         field_2: B,
//     }
//
//     let x = StructSimple::<usize, usize> {
//         field_1: 42,
//         field_2: 13,
//     };
//
//     assert_eq!(
//         x.fmap(|x| x as u64).type_id(),
//         TypeId::of::<StructSimple<usize, u64>>()
//     );
// }

// #[test]
// fn hashmap_key() {
//     #[derive(Functor)]
//     struct StructMap<A: Hash + Eq> {
//         #[functor_map(fmap_key)]
//         field_1: HashMap<A, usize>
//     }
//
//     // impl<A: Hash + Eq> ::functor_derive::Functor<A> for StructMap<A> {
//     //     type Target<__T> = StructMap<__T>;
//     //     fn fmap<__B>(self, __f: impl Fn(A) -> __B) -> Self::Target<__B> { Self::Target { field_1: self.field_1.fmap(&__f) } }
//     // }
//
//     // impl<A: Hash + Eq> StructMap<A> {
//     //     fn fmap<__B: Hash + Eq>(self, __f: impl Fn(A) -> __B) -> StructMap<__B> { StructMap { field_1: self.field_1.fmap_key(&__f) } }
//     // }
//     //
//     // let x = StructMap::<usize> {
//     //     field_1: HashMap::from([(1, 3), (2, 4)])
//     // };
//
//     assert_eq!(
//         x.fmap(|x| x as u64).type_id(),
//         TypeId::of::<StructMap<u64>>()
//     );
// }
