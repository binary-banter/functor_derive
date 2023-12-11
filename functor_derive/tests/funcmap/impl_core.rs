use functor_derive::Functor;
use core::marker::PhantomData;
use std::cell::{Cell, RefCell, UnsafeCell};
use std::ops::ControlFlow;
use crate::funcmap::{T1, T2};

#[test]
fn field_of_array_type_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>([T; 2]);

    let src = Test([T1, T1]);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test([T2, T2]));
}

// todo: not implemented
// #[test]
// fn field_of_bound_type_is_mapped() {
//     #[derive(Functor, Debug, PartialEq)]
//     struct Test<T>(Bound<T>);
//
//     let src = Test(Bound::Included(T1));
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(Bound::Included(T2)));
// }

#[test]
fn field_of_cell_type_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T: Copy>(Cell<T>);

    let src = Test(Cell::new(T1));
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(Cell::new(T2)));
}

#[test]
fn field_of_control_flow_type_is_mapped_over_break() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(ControlFlow<T, ()>);

    let src = Test(ControlFlow::Break(T1));
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(ControlFlow::Break(T2)));
}

#[test]
fn field_of_control_flow_type_is_mapped_over_continue() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(ControlFlow<(), T>);

    let src = Test(ControlFlow::Continue(T1));
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(ControlFlow::Continue(T2)));
}

#[test]
fn field_of_option_type_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(Option<T>);

    let src = Test(Some(T1));
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(Some(T2)));
}

// todo: not implemented
// #[test]
// fn field_of_option_into_iter_type_is_mapped() {
//     #[derive(Functor, Debug)]
//     struct Test<T>(option::IntoIter<T>);
//
//     let src = Test(Some(T1).into_iter());
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst.0.collect::<Vec<_>>(), vec![T2]);
// }

#[test]
fn field_of_phantom_data_type_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(PhantomData<T>);

    let src = Test(PhantomData::<T1>);
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(PhantomData::<T2>));
}

// todo: not implemented
// #[test]
// fn field_of_poll_type_is_mapped() {
//     #[derive(Functor, Debug, PartialEq)]
//     struct Test<T>(Poll<T>);
//
//     let src = Test(Poll::Ready(T1));
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(Poll::Ready(T2)));
// }

// todo: not implemented
// #[test]
// fn field_of_range_type_is_mapped() {
//     #[derive(Functor, Debug, PartialEq)]
//     struct Test<T>(Range<T>);
//
//     let src = Test(T1..T1);
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(T2..T2));
// }

// todo: not implemented
// #[test]
// fn field_of_range_from_type_is_mapped() {
//     #[derive(Functor, Debug, PartialEq)]
//     struct Test<T>(RangeFrom<T>);
//
//     let src = Test(T1..);
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(T2..));
// }

// todo: not implemented
// #[test]
// fn field_of_range_inclusive_type_is_mapped() {
//     #[derive(Functor, Debug, PartialEq)]
//     struct Test<T>(RangeInclusive<T>);
//
//     let src = Test(T1..=T1);
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(T2..=T2));
// }

// todo: not implemented
// #[test]
// fn field_of_range_to_type_is_mapped() {
//     #[derive(Functor, Debug, PartialEq)]
//     struct Test<T>(RangeTo<T>);
//
//     let src = Test(..T1);
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(..T2));
// }

// todo: not implemented
// #[test]
// fn field_of_range_to_inclusive_type_is_mapped() {
//     #[derive(Functor, Debug, PartialEq)]
//     struct Test<T>(RangeToInclusive<T>);
//
//     let src = Test(..=T1);
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst, Test(..=T2));
// }

#[test]
fn field_of_ref_cell_type_is_mapped() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(RefCell<T>);

    let src = Test(RefCell::new(T1));
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(RefCell::new(T2)));
}

#[test]
fn field_of_result_type_is_mapped_over_value() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(Result<T, ()>);

    let src = Test(Ok(T1));
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(Ok(T2)));
}

#[test]
fn field_of_result_type_is_mapped_over_error() {
    #[derive(Functor, Debug, PartialEq)]
    struct Test<T>(Result<(), T>);

    let src = Test(Err(T1));
    let dst = src.fmap(|_| T2);

    assert_eq!(dst, Test(Err(T2)));
}

// todo: not implemented
// #[test]
// fn field_of_result_into_iter_type_is_mapped_over_value() {
//     #[derive(Functor, Debug)]
//     struct Test<T>(result::IntoIter<T>);
//
//     let src = Test(Result::<_, ()>::Ok(T1).into_iter());
//     let dst = src.fmap(|_| T2);
//
//     assert_eq!(dst.0.collect::<Vec<_>>(), vec![T2]);
// }

#[test]
fn field_of_unsafe_cell_type_is_mapped() {
    #[derive(Functor, Debug)]
    struct Test<T>(UnsafeCell<T>);

    let src = Test(UnsafeCell::new(T1));
    let dst = src.fmap(|_| T2);

    assert_eq!(dst.0.into_inner(), T2);
}
