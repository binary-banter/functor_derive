use functor_derive::Functor;

#[derive(Functor)]
struct MyType<T> {
    value: Appeltaart<T, T>,
    list: Vec<T>,
    unaffected: bool,
}

#[derive(Functor)]
struct Appeltaart<S, T> {
    deeg: S,
    appel: T,
}

fn main() {
    let original = MyType {
        value: Appeltaart{ deeg: 15, appel: 13 },
        list: vec![1, 3],
        unaffected: false,
    };
    let transformed = original.fmap(|x| (x, x * 2));
    
    // assert_eq!(transformed.value, (42, 84));
    // assert_eq!(transformed.list, vec![(1, 2), (3, 6)]);
}
