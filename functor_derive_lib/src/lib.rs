use std::collections::VecDeque;

pub trait Functor<A> {
    type Target<T>;

    fn fmap<B>(self, f: &mut impl FnMut(A) -> B) -> Self::Target<B>;
}

impl<A> Functor<A> for Option<A> {
    type Target<T> = Option<T>;

    fn fmap<B>(self, f: &mut impl FnMut(A) -> B) -> Self::Target<B> {
        self.map(f)
    }
}

impl<A> Functor<A> for Vec<A> {
    type Target<T> = Vec<T>;

    fn fmap<B>(self, f: &mut impl FnMut(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Functor<A> for VecDeque<A> {
    type Target<T> = VecDeque<T>;

    fn fmap<B>(self, f: &mut impl FnMut(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }
}

// impl<A, C> Functor<A> for HashMap<A, C> {
//     type Target<T> = HashMap<T, C>;
//
//     fn fmap<B>(self, f: &mut impl FnMut(A) -> B) -> Self::Target<B> {
//         self.into_iter().map(|(k,v)| (f(k),v)).collect()
//     }
// }
