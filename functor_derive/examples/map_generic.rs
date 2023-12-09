use functor_derive::Functor;

#[derive(Functor)]
#[functor(T)]
struct MyType<S, T> {
    v1: S,
    v2: T,
}

fn main() {}
