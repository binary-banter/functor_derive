use functor_derive_lib::Functor;

#[test]
fn non_camel_case_types_lint_is_allowed_on_derived_impl() {
    #![deny(non_camel_case_types)]

    #[allow(non_camel_case_types)]
    #[derive(Functor)]
    struct Test<t>(t);
}

#[test]
fn unused_qualifications_lint_is_allowed_on_derived_impl() {
    #![deny(unused_qualifications)]

    #[allow(unused_qualifications)]
    #[derive(Functor)]
    struct Test<T>(core::option::Option<T>);
}

#[test]
fn deprecated_lint_is_allowed_on_derived_impl() {
    #![deny(deprecated)]

    #[deprecated]
    #[derive(Functor)]
    struct Deprecated<T>(T);

    #[allow(deprecated)]
    #[derive(Functor)]
    struct Test<T>(Deprecated<T>);
}

#[test]
fn drop_bounds_lint_is_allowed_on_derived_impl() {
    #![deny(drop_bounds)]

    #[allow(unused)]
    #[allow(drop_bounds)]
    #[derive(Functor)]
    struct Test<T>(T)
    where
        T: Drop;
}

#[test]
fn dyn_drop_lint_is_allowed_on_derived_impl() {
    #![deny(dyn_drop)]

    #[allow(dyn_drop)]
    #[allow(trivial_bounds)]
    #[derive(Functor)]
    struct Test<T>(T)
    where
        for<'a> &'a dyn Drop: Copy;
}

// We are pretty confident these tests will succeed - it's just too much of a hassle to implement in our test setup.
// #[test]
// fn clippy_disallowed_method_lint_is_allowed_on_derived_impl() {
//     #![deny(clippy::disallowed_method)]
//
//     // methods `func_map` and `func_map_over` are disallowed via `clippy.toml`
//     #[allow(clippy::disallowed_method)]
//     #[derive(Functor)]
//     struct Test<T>(Option<T>);
// }
//
// #[test]
// fn clippy_disallowed_type_lint_is_allowed_on_derived_impl() {
//     #![deny(clippy::disallowed_type)]
//
//     // type `Option` is disallowed via `clippy.toml`
//     #[allow(clippy::disallowed_type)]
//     #[derive(Functor)]
//     struct Test<T>(Option<T>);
// }
