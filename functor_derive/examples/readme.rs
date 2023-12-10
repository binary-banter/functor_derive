use functor_derive::Functor;

// #[derive(Functor)]
struct MyType<T, S> {
    value: T,
    list: Vec<S>,
    unaffected: bool,
}
impl<T, S> ::functor_derive::Functor<T> for MyType<T, S> {
    type Target<__B> = MyType<__B, S>;
    fn fmap<__B>(self, __f: impl Fn(T) -> __B) -> MyType<__B, S> {
        use ::functor_derive::*;
        self.fmap_0_ref(&__f)
    }
    fn try_fmap<__B, __E>(self, __f: impl Fn(T) -> Result<__B, __E>) -> Result<MyType<__B, S>, __E> {
        use ::functor_derive::*;
        self.try_fmap_0_ref(&__f)
    }
}
impl<T, S> ::functor_derive::Functor0<T> for MyType<T, S> {
    type Target<__B> = MyType<__B, S>;
    fn fmap_0_ref<__B>(self, __f: &impl Fn(T) -> __B) -> MyType<__B, S> {
        use ::functor_derive::*;
        MyType { value: __f(self.value), list: self.list, unaffected: self.unaffected }
    }
    fn try_fmap_0_ref<__B, __E>(self, __f: &impl Fn(T) -> Result<__B, __E>) -> Result<MyType<__B, S>, __E> {
        use ::functor_derive::*;
        Ok(MyType { value: __f(self.value)?, list: self.list, unaffected: self.unaffected })
    }
}
impl<T, S> ::functor_derive::Functor1<S> for MyType<T, S> {
    type Target<__B> = MyType<T, __B>;
    fn fmap_1_ref<__B>(self, __f: &impl Fn(S) -> __B) -> MyType<T, __B> {
        use ::functor_derive::*;
        MyType { value: self.value, list: self.list.fmap_ref(__f), unaffected: self.unaffected }
    }
    fn try_fmap_1_ref<__B, __E>(self, __f: &impl Fn(S) -> Result<__B, __E>) -> Result<MyType<T, __B>, __E> {
        use ::functor_derive::*;
        Ok(MyType { value: self.value, list: self.list.try_fmap_ref(__f)?, unaffected: self.unaffected })
    }
}


fn main() {
    let original = MyType {
        value: 42,
        list: vec![1, 3],
        unaffected: false,
    };
    let transformed = original.fmap(|x| (x, x * 2));

    assert_eq!(transformed.value, (42, 84));
    assert_eq!(transformed.list, vec![(1, 2), (3, 6)]);
}
