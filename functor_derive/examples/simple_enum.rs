fn main() {
    // let var_1  = SimpleEnum::Var1(0usize);
    // let var_2 = SimpleEnum::Var2::<usize>(15u32);
    // let var_3 = SimpleEnum::Var3 { field_1: 16usize, field_2: 18u32 };
    //
    // dbg!(var_1.fmap(|x| x as u64));
    // dbg!(var_2.fmap(|x| x as u64));
    // dbg!(var_3.fmap(|x| x as u64));
}
//
// #[derive(Debug)]
// enum SimpleEnum<A> {
//     Var1(A),
//     Var2(u32),
//     Var3{
//         field_1: A,
//         field_2: u32,
//     }
// }
//
//
//
// impl<A> Functor<A> for SimpleEnum<A> {
//     type Target<T> = SimpleEnum<T>;
//
//     fn fmap<B>(self, mut f: impl FnMut(A) -> B) -> Self::Target<B> {
//         match self {
//             SimpleEnum::Var1(v) => SimpleEnum::Var1(f(v)),
//             SimpleEnum::Var2(v) => SimpleEnum::Var2(v),
//             SimpleEnum::Var3 { field_1, field_2 } => SimpleEnum::Var3 {
//                 field_1: f(field_1),
//                 field_2
//             }
//         }
//     }
// }
