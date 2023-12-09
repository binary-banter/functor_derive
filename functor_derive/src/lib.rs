use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

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
///     fn fmap_ref<B>(self, f: &impl Fn(T) -> B) -> Self::Target<B> {
///         MyType {
///             value: f(self.value),
///             unaffected: self.unaffected,
///         }
///     }
///     fn try_fmap_ref<B, E>(self, f: &impl Fn(T) -> Result<B, E>) -> Result<Self::Target<B>, E> {
///         Ok(MyType {
///             value: f(self.value)?,
///             unaffected: self.unaffected,
///         })
///     }
/// }
/// let original = MyType {
///     value: 42,
///     unaffected: false,
/// };
///
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

    /// Should not be used directly. Use [`fmap`](fmap) instead.
    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_ref(&f)
    }

    /// Should not be used directly. Use [`try_fmap`](try_fmap) instead.
    fn try_fmap_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E>;
}

impl<A> Functor<A> for Option<A> {
    type Target<T> = Option<T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }

    fn try_fmap_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.map(f).transpose()
    }
}

impl<A, E> Functor<A> for Result<A, E> {
    // Results are always mapped over their Ok generic.
    type Target<T> = Result<T, E>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }

    fn try_fmap_ref<B, E2>(self, f: &impl Fn(A) -> Result<B, E2>) -> Result<Self::Target<B>, E2> {
        match self.map(f) {
            Ok(Ok(v)) => Ok(Ok(v)),
            Ok(Err(e)) => Err(e),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl<A> Functor<A> for Vec<A> {
    type Target<T> = Vec<T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn try_fmap_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Functor<A> for Box<A> {
    type Target<T> = Box<T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        Box::new(f(*self))
    }

    fn try_fmap_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        f(*self).map(Box::new)
    }
}

impl<A> Functor<A> for VecDeque<A> {
    type Target<T> = VecDeque<T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn try_fmap_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}

impl<K: Eq + Hash, A> Functor<A> for HashMap<K, A> {
    type Target<T> = HashMap<K, T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }

    fn try_fmap_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter()
            .map(|(k, v)| f(v).map(|v| (k, v)))
            .collect()
    }
}

impl<A> Functor<A> for PhantomData<A> {
    type Target<T> = PhantomData<T>;

    fn fmap_ref<B>(self, _f: &impl Fn(A) -> B) -> Self::Target<B> {
        PhantomData
    }

    fn try_fmap_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        Ok(PhantomData)
    }
}

impl<const N: usize, A> Functor<A> for [A; N] {
    type Target<T> = [T; N];

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }

    fn try_fmap_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        // Safety: creates an array of uninits from an uninit array, which is sound since the memory layout is unchanged.
        let mut data: [MaybeUninit<B>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for (i, v) in self.into_iter().enumerate() {
            data[i] = MaybeUninit::new(f(v)?);
        }

        // Safety: We just initialized all elements of the array. We made `data` the same size as `self` so this guaranteed.
        Ok(data.map(|v| unsafe { v.assume_init() }))
    }
}

impl<K: Ord, A> Functor<A> for BTreeMap<K, A> {
    type Target<T> = BTreeMap<K, T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }

    fn try_fmap_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter()
            .map(|(k, v)| f(v).map(|v| (k, v)))
            .collect()
    }
}

impl<A> Functor<A> for LinkedList<A> {
    type Target<T> = LinkedList<T>;

    fn fmap_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn try_fmap_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}
