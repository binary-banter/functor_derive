#[test]
fn crate_path_uses_leading_colon_by_default() {
    use ::functor_derive::Functor;

    #[derive(Functor)]
    struct Test<T>(T);

    // would be referred to if derive macro produced `funcmap::..` instead of `::funcmap::..`
    mod functor_derive {}
}

// Our macro invocation does not support renaming the crate path.
// #[test]
// fn crate_path_can_be_configured() {
//     use fake_funcmap::FuncMap;
//
//     #[derive(FuncMap)]
//     #[funcmap(crate = "fake_funcmap")]
//     struct Test<T>(T);
//
//     // would be conflicting if `Test<T1>` implemented `funcmap::FuncMap<T1, T2>`
//     impl AssertNotOriginalFuncMap for Test<T1> {}
//
//     fake_funcmap::assert::<Test<T1>, T1, T2, fake_funcmap::TypeParam<0>>();
// }
//
// mod fake_funcmap {
//     pub use funcmap::*;
//
//     pub struct TypeParam<const N: usize>;
//
//     pub trait FuncMap<A, B, P = TypeParam<0>> {
//         type Output;
//
//         fn func_map<F>(self, _: F) -> Self::Output
//             where
//                 F: FnMut(A) -> B;
//     }
//
//     pub fn assert<T, A, B, P>()
//         where
//             T: FuncMap<A, B, P>,
//     {
//     }
// }
//
// trait AssertNotOriginalFuncMap {}
//
// impl<T> AssertNotOriginalFuncMap for T where T: funcmap::FuncMap<T1, T2> {}
