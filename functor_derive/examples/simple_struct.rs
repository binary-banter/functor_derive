use functor_derive::Functor;
use functor_derive_lib::Functor;

fn main() {
    let x = SimpleStruct {
        field_1: 0usize,
        field_2: 15u32,
    };

    let y = x.fmap(|x| x as u64);
    dbg!(y);
}

#[derive(Debug, Functor)]
struct SimpleStruct<A> {
    field_1: A,
    field_2: u32,
}
