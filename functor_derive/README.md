# Functor Derive

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/binary-banter/functor_derive)
&ensp;[![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/functor_derive)
&ensp;[![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/functor_derive/)

This crate can generate a functor for generic structs and enums.

A functor is a trait that contains an `fmap` function that maps a generic parameter.
This enables you to transform the contents of any type without altering its shape.

The following example demonstrates how to derive a functor, providing you with an `fmap` method.
For more intricate examples, refer to the tests directory in
the [project repository](https://github.com/binary-banter/functor_derive/tree/main/functor_derive/tests).

```rust
use functor_derive::Functor;

#[derive(Functor)]
struct MyType<T> {
    value: T,
    list: Vec<T>,
    unaffected: bool,
}

fn main() {
    let original = MyType { value: 42, list: vec![1, 3], unaffected: false };
    let transformed = original.fmap(|x| (x, x * 2));

    assert_eq!(transformed.value, (42, 84));
    assert_eq!(transformed.list, vec![(1, 2), (3, 6)]);
}
```

Additionally, a `try_fmap` function is generated. This can be useful for fallible transformations.

```rust
let original = MyType { value: "42", list: vec!["1", "3"], unaffected: false };
let transformed = original.try_fmap(|x| x.parse::<u64>())?;
```

## Attribute

You can invoke the derive macro in multiple ways. Omitting the attribute defaults to deriving the `Functor` trait for
the first generic type parameter, as illustrated in the first example above.

Alternatively, you can specify a default type to override the derive macro, which will prevent the derive macro choosing
the first
generic type parameter. This is done as follows:

```rust
#[derive(Functor)]
#[functor(T2)]
struct MyType<T1, T2> {
    field_1: T1,
    field_2: T2,
}
```

Sometimes, you might want to rename your `fmap` function using the as keyword. The following example generates the
method `fmap_keys`.

```rust
#[derive(Functor)]
#[functor(K as keys)]
struct MyType<K> {
    keys: Vec<K>
}
```

The above options can be combined to generate multiple implementations, by separating the options with commas.
The code below generates 3 methods: `fmap`, `fmap_keys` and `fmap_values`.

```rust
use std::collections::HashMap;
use std::hash::Hash;

#[functor(V, K as keys, V as values)]
struct MyHashMap<K: Hash + Eq, V> {
    v: HashMap<K, V>
}
```

## Supported features

This crate can handle the following perfectly:

- Structs - except for unit structs, which cannot be generic
- Enums
- Arrays
- Tuples
- `std::collections`: Vec, VecDeque, LinkedList, HashSet, HashMap, BTreeMap, Result, Option, PhantomData
- Nested types, like `Option<Box<T>>`
- (Mutually) recursive types
- Bounded parameters, like `T: Display`

If you find a case where the derive macro fails, feel free to open an
issue [here](https://github.com/binary-banter/functor_derive/issues)
