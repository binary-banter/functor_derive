# Functor Derive
This crate can generate a Functor for generic structs and enums.

A `Functor` represents a type that can apply a function to its inner values
and produce new values of the same structure. This allows you to transform
the contents of a container (or any type) without changing the shape of the container.

```rust
use functor_derive::Functor;

#[derive(Functor)]
struct MyType<T> {
    value: T,
    list: Vec<T>,
    unaffected: bool,
}

fn main() {
    let original = MyType { value: 42, list: vec![1,3], unaffected: false };
    let transformed = original.fmap(|x| (x, x * 2));
        
    assert_eq!(transformed.value, (42, 84));
    assert_eq!(transformed.list, vec![(1,2), (3, 6)]);
}
```

## How to Contribute
This crate works for almost any struct or enum definition. It can handle arrays, tuples, and nested types perfectly.

- [x] Structs
    - [x] Named
    - [x] Unnamed
    - [ ] Unit - Rust does not allow generic Unit Structs.
- [x] Enums
    - [x] Named variants
    - [x] Unnamed variants
    - [x] Unit variants
- [x] Arrays
- [x] Tuples
- [x] `std::collections`: Vec, VecDeque, LinkedList, HashMap*, BTreeMap*, Result, Option, PhantomData

*The values are mapped.

If you find a case where the derive macro fails, feel free to open an issue [here](https://github.com/binary-banter/functor_derive/issues)

