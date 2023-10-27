pub trait Functor<A> {
    type Target<T>;

    fn fmap<B>(self, f: impl FnMut(A) -> B) -> Self::Target<B>;
}

impl<A> Functor<A> for Vec<A> {
    type Target<T> = Vec<T>;

    fn fmap<B>(self, f: impl FnMut(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }
}
