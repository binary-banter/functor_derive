use functor_derive::Functor;

#[derive(Functor)]
struct MyType<T> {
    value: MyBox<MyBox<T>>,
    unaffected: bool,
}

#[derive(Functor)]
struct MyBox<T> {
    inner: T,
}

fn main() {
    let original = MyType {
        value: MyBox {
            inner: MyBox {
                inner: String::from("42"),
            },
        },
        unaffected: false,
    };
    let transformed = original.try_fmap(|x| x.parse::<usize>());

    assert_eq!(transformed.unwrap().value.inner.inner, 42);
}

impl<T> MyType<T> {
    fn try_fmap<B, E>(self, __f: impl Fn(T) -> Result<B, E>) -> Result<MyType<B>, E> {
        self.try_fmap_ref(&__f)
    }

    fn try_fmap_ref<B, E>(self, __f: &impl Fn(T) -> Result<B, E>) -> Result<MyType<B>, E> {
        Ok(MyType {
            value: self.value.try_fmap_ref(&|v| v.try_fmap(__f))?,
            unaffected: self.unaffected,
        })
    }
}

impl<T> MyBox<T> {
    fn try_fmap<B, E>(self, __f: impl Fn(T) -> Result<B, E>) -> Result<MyBox<B>, E> {
        self.try_fmap_ref(&__f)
    }

    fn try_fmap_ref<B, E>(self, __f: &impl Fn(T) -> Result<B, E>) -> Result<MyBox<B>, E> {
        Ok(MyBox {
            inner: __f(self.inner)?,
        })
    }
}
