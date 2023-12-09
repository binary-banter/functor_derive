use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;

pub use functor_derive_lib::*;

/// A `Functor` represents a type that can apply a function to its inner values
/// and produce new values of the same structure. This allows you to transform
/// the contents of a container (or any type) without changing the shape of the container.
///
/// # Examples
///
/// ```
/// use functor_derive::Functor;
///
/// // Define a custom type that implements the `Functor` trait.
/// struct MyType<T> {
///     value: T,
///     unaffected: bool,
/// }
///
/// impl<T> Functor<T> for MyType<T> {
///     type Target<U> = MyType<U>;
///
///     fn fmap_ref<B>(self, f: &impl Fn(T) -> B) -> Self::Target<B> {
///         MyType {
///             value: f(self.value),
///             unaffected: self.unaffected,
///         }
///     }
/// }
///
/// let original = MyType { value: 42, unaffected: false };
/// let transformed = original.fmap(|x| x * 2);
/// assert_eq!(transformed.value, 84);
/// ```
///
/// We can use the `functor_derive` crate to automate this for us.
/// ```ignore
/// use functor_derive::Functor;
///
/// #[derive(Functor)]
/// struct MyType<T> {
///     value: T,
/// }
///
/// let original = MyType { value: 42 };
/// let transformed = original.fmap(|x| x * 2);
/// assert_eq!(transformed.value, 84);
/// ```
pub trait Functor<A>: Sized {
    type Target<T>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_ref(&f)
    }

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;
}

impl<A> Functor<A> for Option<A> {
    type Target<T> = Option<T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }
}

impl<A, E> Functor<A> for Result<A, E> {
    type Target<T> = Result<T, E>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }
}

impl<A> Functor<A> for Vec<A> {
    type Target<T> = Vec<T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Functor<A> for Box<A> {
    type Target<T> = Box<T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        Box::new(f(*self))
    }
}

impl<A> Functor<A> for VecDeque<A> {
    type Target<T> = VecDeque<T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }
}

impl<K: Eq + Hash, A> Functor<A> for HashMap<K, A> {
    type Target<T> = HashMap<K, T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
}

impl<A> Functor<A> for PhantomData<A> {
    type Target<T> = PhantomData<T>;

    fn fmap_ref<B>(self, _f: &impl Fn(A) -> B) -> Self::Target<B> {
        PhantomData
    }
}

impl<const N: usize, A> Functor<A> for [A; N] {
    type Target<T> = [T; N];

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }
}

impl<K: Ord, A> Functor<A> for BTreeMap<K, A> {
    type Target<T> = BTreeMap<K, T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
}

impl<A> Functor<A> for LinkedList<A> {
    type Target<T> = LinkedList<T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }
}
