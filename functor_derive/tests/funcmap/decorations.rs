use crate::funcmap::{T1, T2};
use functor_derive::Functor;
use std::marker::PhantomData;

#[test]
fn attributes_on_type_params_are_supported() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<#[cfg(test)] S, #[cfg(test)] T>(S, T);

    let src = Test(T1, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, T1));
}

#[test]
fn attributes_on_lifetime_params_are_supported() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<#[cfg(test)] 'a, T>(T, PhantomData<&'a ()>);

    let src = Test(T1, PhantomData);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, PhantomData));
}

#[test]
fn attributes_on_const_params_are_supported() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T, #[cfg(test)] const N: usize>(T);

    let src = Test::<_, 42>(T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2));
}

#[test]
fn defaults_on_type_params_are_supported() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<S = T1, T = T1>(S, T);

    let src = Test(T1, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, T1));
}

#[test]
fn defaults_on_const_params_are_supported() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T, const N: usize = 0>(T);

    let src = Test::<_>(T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2));
}

#[test]
fn impl_is_restricted_to_trait_bounds_on_generics_of_original_type() {
    trait TestTrait {}

    impl TestTrait for T1 {}
    impl TestTrait for T2 {}

    // derived impl is supposed to have the corresponding trait bounds
    #[derive(Functor, Debug, PartialEq)]
    struct Test<S: TestTrait, T: TestTrait>(S, T);

    let src = Test(T1, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, T1));
}

#[test]
fn impl_is_restricted_to_self_dependent_trait_bounds_on_generics_of_original_type() {
    trait TestTrait<T> {
        type Assoc;
    }

    impl<S> TestTrait<S> for T1 {
        type Assoc = S;
    }

    impl<S> TestTrait<S> for T2 {
        type Assoc = S;
    }

    // derived impl is supposed to have the corresponding trait bounds
    #[derive(Functor, Debug, PartialEq)]
    struct Test<S: TestTrait<S, Assoc = S>, T: TestTrait<T, Assoc = T>>(S, T);

    let src = Test(T1, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, T1));
}

#[test]
fn impl_is_restricted_to_cross_dependent_trait_bounds_on_generics_of_original_type() {
    trait TestTrait<T> {
        type Assoc;
    }

    impl<S> TestTrait<S> for T1 {
        type Assoc = S;
    }

    impl<S> TestTrait<S> for T2 {
        type Assoc = S;
    }

    // derived impl is supposed to have the corresponding trait bounds
    #[derive(Functor, Debug, PartialEq)]
    struct Test<S: TestTrait<T, Assoc = T>, T: TestTrait<S, Assoc = S>>(S, T);

    let src = Test(T1, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, T1));
}

#[test]
fn impl_is_restricted_to_maybe_sized_bound_on_unmapped_generic_of_original_type() {
    trait TestTrait<T: ?Sized> {}

    type Unsized = [()];

    impl TestTrait<T1> for Unsized {}
    impl TestTrait<T2> for Unsized {}

    // derived impl for mapping over S is supposed to have no `S: ?Sized` (more
    // precisely, no `A: ?Sized` and `B: ?Sized` where `A` and `B` are the
    // generic source and target types) but `T: ?Sized` (even though the bound
    // for T depends on S)
    #[derive(Functor, Debug, PartialEq)]
    struct Test<S: ?Sized, T: ?Sized + TestTrait<S>>(PhantomData<S>, PhantomData<T>);

    let src = Test::<T1, Unsized>(PhantomData, PhantomData);
    let dst = src.fmap(|_: T1| T2);

    assert_eq!(dst, Test(PhantomData, PhantomData));
}

#[test]
fn impl_is_restricted_to_lifetime_bounds_on_generics_of_original_type() {
    // derived impl is supposed to have the corresponding lifetime bounds
    #[derive(Functor, Debug, PartialEq)]
    struct Test<'a, T: 'a>(T, PhantomData<&'a ()>);

    let src = Test(T1, PhantomData);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, PhantomData));
}

#[test]
fn impl_is_restricted_to_trait_bounds_in_where_clause_on_original_type() {
    trait TestTrait {}

    impl TestTrait for T1 {}
    impl TestTrait for T2 {}

    // derived impl is supposed to have the corresponding where predicates
    #[derive(Functor, Debug, PartialEq)]
    struct Test<S, T>(S, T)
        where
            S: TestTrait,
            T: TestTrait;

    let src = Test(T1, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, T1));
}

#[test]
fn impl_is_restricted_to_self_dependent_trait_bounds_in_where_clause_on_original_type() {
    trait TestTrait<T> {
        type Assoc;
    }

    impl<S> TestTrait<S> for T1 {
        type Assoc = S;
    }

    impl<S> TestTrait<S> for T2 {
        type Assoc = S;
    }

    // derived impl is supposed to have the corresponding where predicates
    #[derive(Functor, Debug, PartialEq)]
    struct Test<S, T>(S, T)
        where
            S: TestTrait<S, Assoc = S>,
            T: TestTrait<T, Assoc = T>;

    let src = Test(T1, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, T1));
}

#[test]
fn impl_is_restricted_to_cross_dependent_trait_bounds_in_where_clause_on_original_type() {
    trait TestTrait<T> {
        type Assoc;
    }

    impl<S> TestTrait<S> for T1 {
        type Assoc = S;
    }

    impl<S> TestTrait<S> for T2 {
        type Assoc = S;
    }

    // derived impl is supposed to have the corresponding where predicates
    #[derive(Functor, Debug, PartialEq)]
    struct Test<S, T>(S, T)
        where
            S: TestTrait<T, Assoc = T>,
            T: TestTrait<S, Assoc = S>;

    let src = Test(T1, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, T1));
}

// todo
// #[test]
// fn impl_is_restricted_to_arbitrary_trait_bounds_in_where_clause_on_original_type() {
//     trait TestTrait<T> {
//         type Assoc;
//     }
//
//     impl<S> TestTrait<S> for T1 {
//         type Assoc = S;
//     }
//
//     impl<S> TestTrait<S> for T2 {
//         type Assoc = S;
//     }
//
//     // derived impl is supposed to have the corresponding where predicates
//     #[derive(Functor, Debug, PartialEq)]
//     struct Test<S, T>(S, T)
//         where
//             S: TestTrait<T>,
//             <S as TestTrait<T>>::Assoc: TestTrait<S>,
//             <<S as TestTrait<T>>::Assoc as TestTrait<S>>::Assoc: TestTrait<T>;
//
//     let src = Test(T1, T1);
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(T2, T1));
// }

#[test]
fn impl_is_restricted_to_trait_bounds_with_bound_lifetimes_in_where_clause_on_original_type() {
    trait TestTrait<'a> {}

    impl<'a> TestTrait<'a> for T1 {}
    impl<'a> TestTrait<'a> for T2 {}

    // derived impl is supposed to have the corresponding where predicates
    #[derive(Functor, Debug, PartialEq)]
    struct Test<S, T>(S, T)
        where
                for<'a> T: TestTrait<'a>;

    let src = Test(T1, T1);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, T1));
}

// todo
#[test]
fn impl_is_restricted_to_maybe_sized_bound_on_unmapped_generic_in_where_clause_of_original_type() {
    trait TestTrait<T: ?Sized> {}

    type Unsized = [()];

    impl TestTrait<T1> for Unsized {}
    impl TestTrait<T2> for Unsized {}

    // derived impl for mapping over S is supposed to have no `S: ?Sized` (more
    // precisely, no `A: ?Sized` and `B: ?Sized` where `A` and `B` are the
    // generic source and target types) but `T: ?Sized` (even though the bound
    // for T depends on S)
    #[derive(Debug, PartialEq)]
    struct Test<S, T>(PhantomData<S>, PhantomData<T>)
        where
            S: ?Sized,
            T: ?Sized + TestTrait<S>;

    let src = Test::<T1, Unsized>(PhantomData, PhantomData);
    let dst = src.fmap(|_: T1| T2);

    assert_eq!(dst, Test(PhantomData, PhantomData));
}

#[test]
fn impl_is_restricted_to_lifetime_bounds_in_where_clause_of_original_type() {
    // derived impl is supposed to have the corresponding where predicates
    #[derive(Functor, Debug, PartialEq)]
    struct Test<'a, T>(T, PhantomData<&'a ()>)
        where
            T: 'a;

    let src = Test(T1, PhantomData);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, PhantomData));
}

// Test is too different from how we construct Functors.
// #[test]
// fn impl_is_restricted_to_allow_mapping_of_inner_type() {
//     #[derive(Debug, PartialEq)]
//     struct Inner<T>(T);
//
//     // impl manually only for Inner<T1>, not generic Inner<T>
//     impl Functor<T1, T2> for Inner<T1> {
//         type Output = Inner<T2>;
//
//         fn fmap<F>(self, _: F) -> Self::Output
//             where
//                 F: FnMut(T1) -> T2,
//         {
//             Inner(T2)
//         }
//     }
//
//     // derived impl is supposed to have
//     // `Inner<A>: Functor<A, B, Output = Inner<B>>`
//     #[derive(Functor, Debug, PartialEq)]
//     struct Test<T>(Inner<T>);
//
//     let src = Test(Inner(T1));
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(Inner(T2)));
// }

// todo
// #[test]
// fn impl_is_restricted_to_sized_bound_on_unmapped_inner_type() {
//     trait TestTrait {
//         type Assoc: ?Sized;
//     }
//
//     type Unsized = [()];
//
//     impl TestTrait for T1 {
//         type Assoc = ();
//     }
//
//     impl TestTrait for T2 {
//         type Assoc = Unsized;
//     }
//
//     // derived impl for mapping over S is supposed to have `T::Assoc: Sized`, so
//     // `Test<S, T1>` implements `Functor` but `Test<S, T2>` doesn't
//     #[derive(Functor, Debug, PartialEq)]
//     #[functor(S)]
//     struct Test<S, T>(S, T::Assoc)
//         where
//             T: TestTrait;
//
//     let src: Test<T1, T1> = Test(T1, ());
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(T2, ()));
// }

#[test]
fn const_args_with_braces_are_supported() {
    #[derive(Debug, PartialEq)]
    struct Inner<const N: usize>;

    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(T, Inner<{ 41 + 1 }>);

    let src = Test(T1, Inner);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(T2, Inner));
}
