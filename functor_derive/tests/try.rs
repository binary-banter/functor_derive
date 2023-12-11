use functor_derive::Functor;
use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicBool, Ordering};

fn test_try(v: usize) -> Result<u64, ()> {
    Ok(v as u64)
}

#[test]
fn try_struct_simple() {
    #[derive(Functor)]
    struct StructSimple<A> {
        field_1: A,
        field_2: u32,
    }

    let x = StructSimple::<usize> {
        field_1: 42,
        field_2: 13,
    };

    assert_eq!(
        x.try_fmap(test_try).type_id(),
        TypeId::of::<Result<StructSimple<u64>, ()>>()
    );
}

#[test]
fn try_recursive() {
    #[derive(Functor)]
    struct StructSimple<A> {
        field_1: Option<Box<StructSimple<A>>>,
        field_2: A,
    }

    let x = StructSimple::<usize> {
        field_1: Some(Box::new(StructSimple {
            field_1: None,
            field_2: 15,
        })),
        field_2: 13,
    };

    assert_eq!(
        x.try_fmap(test_try).type_id(),
        TypeId::of::<Result<StructSimple<u64>, ()>>()
    );
}

#[test]
fn try_array() {
    let x = ["1".to_string(), "2".to_string(), "3".to_string()];
    let v = x.try_fmap(|x| x.parse::<usize>());

    assert_eq!(v, Ok([1, 2, 3]));
}

#[test]
fn try_array_fail() {
    let x = ["1".to_string(), "two".to_string(), "3".to_string()];
    let v = x.try_fmap(|x| x.parse::<usize>());

    assert!(v.is_err());
}

#[test]
fn try_array_drops() {
    struct DropCheck<'a> {
        dropped: &'a AtomicBool,
        should_fail: bool,
    }

    impl Drop for DropCheck<'_> {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::Relaxed);
        }
    }

    let dropped_1 = AtomicBool::new(false);
    let dropped_2 = AtomicBool::new(false);

    let x = [
        DropCheck {
            dropped: &dropped_1,
            should_fail: false,
        },
        DropCheck {
            dropped: &dropped_2,
            should_fail: true,
        },
    ];

    let drop_check_array = x.try_fmap(|drop_check| {
        if drop_check.should_fail {
            Err(())
        } else {
            Ok(drop_check)
        }
    });

    assert!(drop_check_array.is_err());
    // Dropped because it's already initialized
    assert!(dropped_1.load(Ordering::Relaxed));
    // Dropped by the iterator
    assert!(dropped_2.load(Ordering::Relaxed));
}
