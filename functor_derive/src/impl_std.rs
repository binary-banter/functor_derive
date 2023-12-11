use crate::{Functor, Functor1, FunctorValues};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[doc(hidden)]
pub trait FunctorHashKeys<A: Hash + Eq>: Sized {
    type Target<B: Hash + Eq>;

    fn fmap_keys<B: Hash + Eq>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap_keys<B: Hash + Eq, E>(
        self,
        f: impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }

    fn __fmap_0_ref<B: Hash + Eq>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;

    fn __try_fmap_0_ref<B: Hash + Eq, E>(
        self,
        f: &impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E>;
}

#[doc(hidden)]
pub trait FunctorHashSet<A: Hash + Eq>: Sized {
    type Target<B: Hash + Eq>;

    fn fmap<B: Hash + Eq>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B: Hash + Eq, E>(
        self,
        f: impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }

    fn __fmap_0_ref<B: Hash + Eq>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;

    fn __try_fmap_0_ref<B: Hash + Eq, E>(
        self,
        f: &impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E>;
}

impl<A: Eq + Hash> FunctorHashSet<A> for HashSet<A> {
    type Target<B: Hash + Eq> = HashSet<B>;

    fn __fmap_0_ref<B: Hash + Eq>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn __try_fmap_0_ref<B: Hash + Eq, E>(
        self,
        f: &impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}

impl<A: Eq + Hash, V> FunctorHashKeys<A> for HashMap<A, V> {
    type Target<B: Hash + Eq> = HashMap<B, V>;

    fn __fmap_0_ref<B: Hash + Eq>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (f(k), v)).collect()
    }

    fn __try_fmap_0_ref<B: Hash + Eq, E>(
        self,
        f: &impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E> {
        self.into_iter()
            .map(|(k, v)| f(k).map(|k| (k, v)))
            .collect()
    }
}

impl<K: Eq + Hash, A> Functor1<A> for HashMap<K, A> {
    type Target<B> = HashMap<K, B>;

    fn __fmap_1_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }

    fn __try_fmap_1_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter()
            .map(|(k, v)| f(v).map(|v| (k, v)))
            .collect()
    }
}

impl<K: Eq + Hash, A> Functor<A> for HashMap<K, A> {
    type Target<B> = HashMap<K, B>;

    /// By default HashMaps map their Value generic.
    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_1_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_1_ref(&f)
    }
}

impl<K: Eq + Hash, A> FunctorValues<A> for HashMap<K, A> {
    type Target<B> = HashMap<K, B>;

    fn fmap_values<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_1_ref(&f)
    }

    fn try_fmap_values<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_1_ref(&f)
    }
}
