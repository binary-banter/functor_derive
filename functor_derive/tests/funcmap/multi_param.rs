use std::marker::PhantomData;
use functor_derive::Functor;
use crate::funcmap::{T1, T2};

#[test]
fn struct_with_multiple_generics_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    #[functor(S as s, T as t)]
    struct Test<S, T>(S, i32, T);

    let src = Test(T1, 42, T1);
    let dst = src.fmap_s(|_| T2);
    assert_eq!(dst, Test(T2, 42, T1));

    let src = Test(T1, 42, T1);
    let dst = src.fmap_t(|_| T2);
    assert_eq!(dst, Test(T1, 42, T2));
}

#[test]
fn struct_with_non_type_generics_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<'a, T, const N: usize>(T, PhantomData<&'a ()>);

    let src = Test::<'_, _, 42>(T1, PhantomData);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test::<'_, _, 42>(T2, PhantomData));
}

#[test]
fn struct_with_const_generics_before_type_generics_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    #[functor(T)]
    struct Test<const N: usize, S, const M: usize, T>(S, T);

    let src = Test::<42, _, 42, _>(T1, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T1, T2));
}

#[test]
fn field_of_generic_type_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Inner<'a, S, T, const N: usize>(S, T, PhantomData<&'a ()>);

    #[derive(Functor, Debug, PartialEq)]
    #[functor(S as s, T as t)]
    struct Test<'a, S, T, const N: usize>(Inner<'a, S, T, N>);

    let src = Test::<'_, _, _, 42>(Inner(T1, T1, PhantomData));
    let dst = src.fmap_s(|_| T2);
    assert_eq!(dst, Test::<'_, _, _, 42>(Inner(T2, T1, PhantomData)));

    let src = Test::<'_, _, _, 42>(Inner(T1, T1, PhantomData));
    let dst = src.fmap_t(|_| T2);
    assert_eq!(dst, Test::<'_, _, _, 42>(Inner(T1, T2, PhantomData)));
}

#[test]
fn field_of_repeated_generic_type_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Inner<'a, S, T, const N: usize>(S, T, PhantomData<&'a ()>);

    #[derive(Functor, Debug, PartialEq)]
    struct Test<'a, T, const N: usize>(Inner<'a, T, T, N>);

    let src = Test::<'_, _, 42>(Inner(T1, T1, PhantomData));
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test::<'_, _, 42>(Inner(T2, T2, PhantomData)));
}

#[test]
fn field_of_generic_type_with_const_literal_before_generic_type_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Inner<const N: usize, S, const M: usize, T>(S, T);

    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(Inner<42, T, 42, T>);

    let src = Test(Inner(T1, T1));
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(Inner(T2, T2)));
}

// todo: not sure yet how to fix this
// #[test]
// fn field_of_generic_type_with_const_alias_before_generic_type_is_mapped() {
//     #[derive(Functor, Debug, PartialEq)]
//     struct Inner<const N: usize, S, const M: usize, T>(S, T);
//
//     const N: usize = 42;
//
//     // here the derive macro cannot know whether `N` is a type or a const
//     // #[derive(Functor, Debug, PartialEq)]
//     struct Test<T>(Inner<N, T, N, T>);
//
//     #[allow(clippy::all)]
//     impl<T> ::functor_derive::Functor<T> for Test<T> {
//         type Target<__B> = Test<__B>;
//         fn fmap<__B>(self, __f: impl Fn(T) -> __B) -> Test<__B> {
//             use ::functor_derive::*;
//             self.__fmap_0_ref(&__f)
//         }
//         fn try_fmap<__B, __E>(self, __f: impl Fn(T) -> Result<__B, __E>) -> Result<Test<__B>, __E> {
//             use ::functor_derive::*;
//             self.__try_fmap_0_ref(&__f)
//         }
//     }
//     #[allow(clippy::all)]
//     impl<T> ::functor_derive::Functor0<T> for Test<T> {
//         type Target<__B> = Test<__B>;
//         fn __fmap_0_ref<__B>(self, __f: &impl Fn(T) -> __B) -> Test<__B> {
//             use ::functor_derive::*;
//             Test(self.0.__fmap_1_ref(__f).__fmap_3_ref(__f))
//         }
//         fn __try_fmap_0_ref<__B, __E>(self, __f: &impl Fn(T) -> Result<__B, __E>) -> Result<Test<__B>, __E> {
//             use ::functor_derive::*;
//             Ok(Test(self.0.__try_fmap_1_ref(__f)?.__try_fmap_3_ref(__f)?))
//         }
//     }
//
//     let src = Test(Inner(T1, T1));
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(Inner(T2, T2)));
// }
