use functor_derive_lib::Functor;

#[derive(Functor)]
#[functor(S as some, T as tome)]
struct MyThing<S, T> {
    a: S,
    b: T,
}

impl<S, T> MyThing<S, T> {
    pub fn fmap_some<__B:>(self, __f: impl Fn(S) -> __B) -> MyThing<__B, T> {
        self.fmap_ref(&__f)
    }
    pub fn fmap_ref_some<__B:>(self, __f: &impl Fn(S) -> __B) -> MyThing<__B, T> {
        MyThing { a: __f(self.a), b: self.b }
    }
    pub fn try_fmap_some<__B:, __E>(self, __f: impl Fn(S) -> Result<__B, __E>) -> Result<MyThing<__B, T>, __E> { self.try_fmap_ref(&__f) }
    pub fn try_fmap_ref_some<__B:, __E>(self, __f: &impl Fn(S) -> Result<__B, __E>) -> Result<MyThing<__B, T>, __E> { Ok(MyThing { a: __f(self.a)?, b: self.b }) }
}
impl<S, T> MyThing<S, T> {
    pub fn fmap_tome<__B:>(self, __f: impl Fn(T) -> __B) -> MyThing<S, __B> { self.fmap_ref(&__f) }
    pub fn fmap_ref_tome<__B:>(self, __f: &impl Fn(T) -> __B) -> MyThing<S, __B> { MyThing { a: self.a, b: __f(self.b) } }
    pub fn try_fmap_tome<__B:, __E>(self, __f: impl Fn(T) -> Result<__B, __E>) -> Result<MyThing<S, __B>, __E> { self.try_fmap_ref(&__f) }
    pub fn try_fmap_ref_tome<__B:, __E>(self, __f: &impl Fn(T) -> Result<__B, __E>) -> Result<MyThing<S, __B>, __E> { Ok(MyThing { a: self.a, b: __f(self.b)? }) }
}


fn main() {

}