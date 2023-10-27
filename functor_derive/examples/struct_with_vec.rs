use functor_derive::Functor;
use functor_derive_lib::Functor;

fn main() {
    let x = VecStruct{
        field: 0usize,
        list: vec![0usize, 1, 2],
    };

    let y = x.fmap(|x| x as u64);
    dbg!(y);
}

#[derive(Debug, Functor)]
struct VecStruct<A> {
    field: A,
    list: Vec<A>,
}

