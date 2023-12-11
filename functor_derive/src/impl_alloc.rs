use crate::{Functor, Functor0, Functor1, FunctorOrdKeys, FunctorValues};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

impl<A> Functor<A> for Vec<A> {
    type Target<B> = Vec<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for Vec<A> {
    type Target<B> = Vec<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Functor<A> for Box<A> {
    type Target<B> = Box<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for Box<A> {
    type Target<B> = Box<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        Box::new(f(*self))
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        f(*self).map(Box::new)
    }
}

impl<A> Functor<A> for VecDeque<A> {
    type Target<B> = VecDeque<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for VecDeque<A> {
    type Target<B> = VecDeque<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Functor<A> for LinkedList<A> {
    type Target<B> = LinkedList<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for LinkedList<A> {
    type Target<B> = LinkedList<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}

impl<K: Ord, A> Functor<A> for BTreeMap<K, A> {
    type Target<B> = BTreeMap<K, B>;

    /// By default BTreeMaps map their Value generic.
    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_1_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_1_ref(&f)
    }
}

impl<A: Ord, V> FunctorOrdKeys<A> for BTreeMap<A, V> {
    type Target<B: Ord> = BTreeMap<B, V>;

    fn __fmap_0_ref<B: Ord>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (f(k), v)).collect()
    }

    fn __try_fmap_0_ref<B: Ord, E>(
        self,
        f: &impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E> {
        self.into_iter()
            .map(|(k, v)| f(k).map(|k| (k, v)))
            .collect()
    }
}

impl<K: Ord, A> FunctorValues<A> for BTreeMap<K, A> {
    type Target<B> = BTreeMap<K, B>;

    fn fmap_values<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_1_ref(&f)
    }

    fn try_fmap_values<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_1_ref(&f)
    }
}

impl<K: Ord, A> Functor1<A> for BTreeMap<K, A> {
    type Target<B> = BTreeMap<K, B>;

    fn __fmap_1_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }

    fn __try_fmap_1_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter()
            .map(|(k, v)| f(v).map(|v| (k, v)))
            .collect()
    }
}

impl<A: Ord> FunctorOrd<A> for BTreeSet<A> {
    type Target<B: Ord> = BTreeSet<B>;

    fn __fmap_0_ref<B: Ord>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn __try_fmap_0_ref<B: Ord, E>(
        self,
        f: &impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}

#[doc(hidden)]
pub trait FunctorOrd<A: Ord>: Sized {
    type Target<B: Ord>;

    fn fmap<B: Ord>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B: Ord, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }

    fn __fmap_0_ref<B: Ord>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;

    fn __try_fmap_0_ref<B: Ord, E>(
        self,
        f: &impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E>;
}

impl<A: Ord> FunctorOrd<A> for BinaryHeap<A> {
    type Target<B: Ord> = BinaryHeap<B>;

    fn __fmap_0_ref<B: Ord>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn __try_fmap_0_ref<B: Ord, E>(
        self,
        f: &impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}
