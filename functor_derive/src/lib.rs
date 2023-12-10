use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

// Re-export derive macro.
pub use functor_derive_lib::*;

pub trait Functor<A>: Sized {
    type Target<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B>;

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E>;
}

pub trait Functor0<A>: Sized {
    type Target<B>;

    fn fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;

    fn try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E>;
}

pub trait Functor1<A>: Sized {
    type Target<B>;

    fn fmap_1_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;

    fn try_fmap_1_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E>;
}

pub trait FunctorValues<A>: Sized {
    type Target<B>;

    fn fmap_values<B>(self, f: impl Fn(A) -> B) -> Self::Target<B>;

    fn try_fmap_values<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E>;
}

pub trait FunctorHashKeys<A: Hash + Eq>: Sized {
    type Target<B: Hash + Eq>;

    fn fmap_keys<B: Hash + Eq>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_0_ref(&f)
    }

    fn try_fmap_keys<B: Hash + Eq, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_0_ref(&f)
    }

    fn fmap_0_ref<B: Hash + Eq>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;

    fn try_fmap_0_ref<B: Hash + Eq, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E>;
}

pub trait FunctorOrdKeys<A: Ord>: Sized {
    type Target<B: Ord>;

    fn fmap_keys<B: Ord>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_0_ref(&f)
    }

    fn try_fmap_keys<B: Ord, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_0_ref(&f)
    }

    fn fmap_0_ref<B: Ord>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;

    fn try_fmap_0_ref<B: Ord, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E>;
}

impl<A> Functor<A> for Option<A> {
    type Target<B> = Option<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for Option<A> {
    type Target<B> = Option<B>;

    fn fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }

    fn try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.map(f).transpose()
    }
}

impl<A, E> Functor<A> for Result<A, E> {
    type Target<B> = Result<B, E>;

    /// By default Results map their Ok generic.
    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_0_ref(&f)
    }

    fn try_fmap<B, E2>(self, f: impl Fn(A) -> Result<B, E2>) -> Result<Self::Target<B>, E2> {
        self.try_fmap_0_ref(&f)
    }
}

impl<A, E> Functor0<A> for Result<A, E> {
    type Target<B> = Result<B, E>;

    fn fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }

    fn try_fmap_0_ref<B, E2>(self, f: &impl Fn(A) -> Result<B, E2>) -> Result<Self::Target<B>, E2> {
        match self.map(f) {
            Ok(Ok(v)) => Ok(Ok(v)),
            Ok(Err(e)) => Err(e),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl<O, A> Functor1<A> for Result<O, A> {
    type Target<B> = Result<O, B>;

    fn fmap_1_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map_err(f)
    }

    fn try_fmap_1_ref<B, E2>(self, f: &impl Fn(A) -> Result<B, E2>) -> Result<Self::Target<B>, E2> {
        match self.map_err(f) {
            Ok(v) => Ok(Ok(v)),
            Err(Ok(e)) => Ok(Err(e)),
            Err(Err(e)) => Err(e),
        }
    }
}

impl<A> Functor<A> for Vec<A> {
    type Target<B> = Vec<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for Vec<A> {
    type Target<B> = Vec<B>;

    fn fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Functor<A> for Box<A> {
    type Target<B> = Box<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for Box<A> {
    type Target<B> = Box<B>;

    fn fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        Box::new(f(*self))
    }

    fn try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        f(*self).map(Box::new)
    }
}

impl<A> Functor<A> for VecDeque<A> {
    type Target<B> = VecDeque<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for VecDeque<A> {
    type Target<B> = VecDeque<B>;

    fn fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Functor<A> for PhantomData<A> {
    type Target<B> = PhantomData<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for PhantomData<A> {
    type Target<B> = PhantomData<B>;

    fn fmap_0_ref<B>(self, _f: &impl Fn(A) -> B) -> Self::Target<B> {
        PhantomData
    }

    fn try_fmap_0_ref<B, E>(self, _f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        Ok(PhantomData)
    }
}

impl<A> Functor<A> for LinkedList<A> {
    type Target<B> = LinkedList<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for LinkedList<A> {
    type Target<B> = LinkedList<B>;

    fn fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }

    fn try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter().map(f).collect()
    }
}

impl<K: Eq + Hash, A> Functor<A> for HashMap<K, A> {
    type Target<B> = HashMap<K, B>;

    /// By default HashMaps map their Value generic.
    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_1_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_1_ref(&f)
    }
}

impl<A: Eq + Hash, V> FunctorHashKeys<A> for HashMap<A, V> {
    type Target<B: Hash + Eq> = HashMap<B, V>;

    fn fmap_0_ref<B: Hash + Eq>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (f(k), v)).collect()
    }

    fn try_fmap_0_ref<B: Hash + Eq, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter()
            .map(|(k, v)| f(k).map(|k| (k, v)))
            .collect()
    }
}

impl<K: Eq + Hash, A> FunctorValues<A> for HashMap<K, A> {
    type Target<B> = HashMap<K, B>;

    fn fmap_values<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_1_ref(&f)
    }

    fn try_fmap_values<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_1_ref(&f)
    }
}

impl<K: Eq + Hash, A> Functor1<A> for HashMap<K, A> {
    type Target<B> = HashMap<K, B>;

    fn fmap_1_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }

    fn try_fmap_1_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter()
            .map(|(k, v)| f(v).map(|v| (k, v)))
            .collect()
    }
}


impl<K: Ord, A> Functor<A> for BTreeMap<K, A> {
    type Target<B> = BTreeMap<K, B>;

    /// By default BTreeMaps map their Value generic.
    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_1_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_1_ref(&f)
    }
}

impl<A: Ord, V> FunctorOrdKeys<A> for BTreeMap<A, V> {
    type Target<B: Ord> = BTreeMap<B, V>;

    fn fmap_0_ref<B: Ord>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (f(k), v)).collect()
    }

    fn try_fmap_0_ref<B: Ord, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter()
            .map(|(k, v)| f(k).map(|k| (k, v)))
            .collect()
    }
}

impl<K: Ord, A> FunctorValues<A> for BTreeMap<K, A> {
    type Target<B> = BTreeMap<K, B>;

    fn fmap_values<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_1_ref(&f)
    }

    fn try_fmap_values<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_1_ref(&f)
    }
}

impl<K: Ord, A> Functor1<A> for BTreeMap<K, A> {
    type Target<B> = BTreeMap<K, B>;

    fn fmap_1_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }

    fn try_fmap_1_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.into_iter()
            .map(|(k, v)| f(v).map(|v| (k, v)))
            .collect()
    }
}

impl<const N: usize, A> Functor<A> for [A; N] {
    type Target<B> = [B; N];

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.try_fmap_0_ref(&f)
    }
}

impl<const N: usize, A> Functor0<A> for [A; N] {
    type Target<B> = [B; N];

    fn fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }

    fn try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        // Safety: creates an array of uninits from an uninit array, which is sound since the memory layout is unchanged.
        let mut data: [MaybeUninit<B>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for (i, v) in self.into_iter().enumerate() {
            data[i] = MaybeUninit::new(f(v)?);
        }

        // Safety: We just initialized all elements of the array. We made `data` the same size as `self` so this guaranteed.
        Ok(data.map(|v| unsafe { v.assume_init() }))
    }
}
