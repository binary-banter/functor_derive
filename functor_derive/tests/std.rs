use functor_derive::impl_std::{FunctorHashKeys, FunctorHashSet};
use functor_derive::{Functor, FunctorValues};
use std::collections::{HashMap, HashSet};

fn map(value: usize) -> u64 {
    value as u64
}

#[test]
fn option() {
    let x = Some(42usize);
    let y = None::<usize>;

    assert_eq!(x.fmap(map), Some(42u64));
    assert_eq!(y.fmap(map), None::<u64>);
}

#[test]
fn result_ok() {
    let x = Result::<usize, usize>::Ok(42usize);
    let y = Result::<usize, usize>::Err(13usize);

    assert_eq!(x.fmap(map), Ok(42u64));
    assert_eq!(y.fmap(map), Err(13usize));
}

#[test]
fn vec() {
    let x = vec![42usize, 13usize];

    assert_eq!(x.fmap(map), vec![42u64, 13u64]);
}

#[test]
fn hashmap_keys() {
    let x = HashMap::from([(42usize, 13usize)]);

    assert_eq!(x.fmap_keys(map), HashMap::from([(42u64, 13usize)]));
}

#[test]
fn hashmap_values() {
    let x = HashMap::from([(42usize, 13usize)]);

    assert_eq!(x.fmap_values(map), HashMap::from([(42usize, 13u64)]));
}

#[test]
fn hashset() {
    let x = HashSet::from([42usize, 13usize]);

    assert_eq!(x.fmap(map), HashSet::from([42u64, 13u64]));
}
