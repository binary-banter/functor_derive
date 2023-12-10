#![allow(unused_parens)]

use functor_derive::Functor;
use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;

//TODO
// #[test]
// fn struct_simple_trait() {
//     #[derive(Functor)]
//     struct StructSimple<A: Display> {
//         field_1: A,
//     }
//
//     let x = StructSimple::<usize> { field_1: 42 };
//
//     assert_eq!(
//         x.fmap(|x| x as u64).type_id(),
//         TypeId::of::<StructSimple<u64>>()
//     );
