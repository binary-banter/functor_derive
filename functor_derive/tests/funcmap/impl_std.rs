use std::{collections::{HashMap, HashSet}, hash::Hash};
use functor_derive::*;
use crate::funcmap::{T1, T2};

#[test]
fn field_of_hash_map_type_is_mapped_over_key() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T: Eq + Hash>(HashMap<T, ()>);

    let src = Test([(T1, ())].into());
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test([(T2, ())].into()));
}

#[test]
fn field_of_hash_map_type_is_mapped_over_value() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(HashMap<(), T>);

    let src = Test([((), T1)].into());
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test([((), T2)].into()));
}

// We don't have iterators in our std implementations.
// #[test]
// fn field_of_hash_map_into_iter_type_is_mapped_over_key() {
//     // #[derive(Functor, Debug)]
//     struct Test<T>(hash_map::IntoIter<T, ()>);
//
//     let src = Test(HashMap::from([(T1, ())]).into_iter());
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst.0.collect::<Vec<_>>(), [(T2, ())]);
// }
//
// #[test]
// fn field_of_hash_map_into_iter_type_is_mapped_over_value() {
//     #[derive(Functor, Debug)]
//     struct Test<T>(hash_map::IntoIter<(), T>);
//
//     let src = Test(HashMap::from([((), T1)]).into_iter());
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst.0.collect::<Vec<_>>(), [((), T2)]);
// }

#[test]
fn field_of_hash_set_type_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T: Eq + Hash>(HashSet<T>);

    let src = Test([T1].into());
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test([T2].into()));
}

// We don't have iterators in our std implementations.
// #[test]
// fn field_of_hash_set_into_iter_type_is_mapped() {
//     #[derive(Functor, Debug)]
//     struct Test<T>(hash_set::IntoIter<T>);
//
//     let src = Test(HashSet::from([T1]).into_iter());
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst.0.collect::<Vec<_>>(), [T2]);
// }
