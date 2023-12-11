use paste::paste;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::{mem, ptr};
use std::cell::{Cell, RefCell, UnsafeCell};

// Re-export derive macro.
pub use functor_derive_lib::*;

pub trait Functor<A>: Sized {
    type Target<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B>;

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E>;
}

#[macro_export]
macro_rules! functor_n {
    ($n:expr) => {
        paste! {
        #[doc(hidden)]
        pub trait [<Functor $n>]<A>: Sized {
            type Target<B>;

            fn [<__fmap_ $n _ref>]<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;

            fn [<__try_fmap_ $n _ref>]<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E>;
        }
        }
    };
}

functor_n!(0);
functor_n!(1);
functor_n!(2);
functor_n!(3);
functor_n!(4);
functor_n!(5);
functor_n!(6);
functor_n!(7);
functor_n!(8);
functor_n!(9);
functor_n!(10);
functor_n!(11);
functor_n!(12);
functor_n!(13);
functor_n!(14);
functor_n!(15);
functor_n!(16);
functor_n!(17);
functor_n!(18);
functor_n!(19);
// please don't use more than 20 generics.

#[doc(hidden)]
pub trait FunctorValues<A>: Sized {
    type Target<B>;

    fn fmap_values<B>(self, f: impl Fn(A) -> B) -> Self::Target<B>;

    fn try_fmap_values<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E>;
}

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

#[doc(hidden)]
pub trait FunctorBTreeSet<A: Ord>: Sized {
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

#[doc(hidden)]
pub trait FunctorOrdKeys<A: Ord>: Sized {
    type Target<B: Ord>;

    fn fmap_keys<B: Ord>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap_keys<B: Ord, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }

    fn __fmap_0_ref<B: Ord>(self, f: &impl Fn(A) -> B) -> Self::Target<B>;

    fn __try_fmap_0_ref<B: Ord, E>(
        self,
        f: &impl Fn(A) -> Result<B, E>,
    ) -> Result<Self::Target<B>, E>;
}

impl<A> Functor<A> for Option<A> {
    type Target<B> = Option<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for Option<A> {
    type Target<B> = Option<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.map(f).transpose()
    }
}

impl<A, E> Functor<A> for Result<A, E> {
    type Target<B> = Result<B, E>;

    /// By default Results map their Ok generic.
    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E2>(self, f: impl Fn(A) -> Result<B, E2>) -> Result<Self::Target<B>, E2> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A, E> Functor0<A> for Result<A, E> {
    type Target<B> = Result<B, E>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }

    fn __try_fmap_0_ref<B, E2>(
        self,
        f: &impl Fn(A) -> Result<B, E2>,
    ) -> Result<Self::Target<B>, E2> {
        match self.map(f) {
            Ok(Ok(v)) => Ok(Ok(v)),
            Ok(Err(e)) => Err(e),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl<O, A> Functor1<A> for Result<O, A> {
    type Target<B> = Result<O, B>;

    fn __fmap_1_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map_err(f)
    }

    fn __try_fmap_1_ref<B, E2>(
        self,
        f: &impl Fn(A) -> Result<B, E2>,
    ) -> Result<Self::Target<B>, E2> {
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

impl<A> Functor<A> for PhantomData<A> {
    type Target<B> = PhantomData<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for PhantomData<A> {
    type Target<B> = PhantomData<B>;

    fn __fmap_0_ref<B>(self, _f: &impl Fn(A) -> B) -> Self::Target<B> {
        PhantomData
    }

    fn __try_fmap_0_ref<B, E>(self, _f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        Ok(PhantomData)
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

impl<K: Eq + Hash, A> FunctorValues<A> for HashMap<K, A> {
    type Target<B> = HashMap<K, B>;

    fn fmap_values<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_1_ref(&f)
    }

    fn try_fmap_values<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_1_ref(&f)
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

impl<A: Ord> FunctorBTreeSet<A> for BTreeSet<A> {
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

impl<const N: usize, A> Functor<A> for [A; N] {
    type Target<B> = [B; N];

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<const N: usize, A> Functor0<A> for [A; N] {
    type Target<B> = [B; N];

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        self.map(f)
    }


    // This implementation was provided by Matthias Stemmler's crate [funcmap_derive](https://crates.io/crates/funcmap_derive) under the MIT license.
    //
    // Licensed under either of
    // * Apache License, Version 2.0 (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
    // * MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
    // at your option.
    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        // This guards the target array, making sure the part of it that has already
        // been filled is dropped if `f` returns `Err(_)` or panics
        struct Guard<'a, T, const N: usize> {
            // mutable borrow of the target array
            array_mut: &'a mut [MaybeUninit<T>; N],
            // index in ..=N up to which (exclusive) `array_mut` is initialized
            init_until_idx: usize,
        }

        impl<T, const N: usize> Drop for Guard<'_, T, N> {
            fn drop(&mut self) {
                // - `self.init_until_idx <= N` is always satisfied
                // - if `self.init_until_idx == N`, the target array is fully
                //   initialized and hence the guard must not be dropped
                debug_assert!(self.init_until_idx < N);

                // SAFETY: as `self.init_until_idx <= N`, the range is within bounds of `self.array_mut`
                let init_slice = unsafe { self.array_mut.get_unchecked_mut(..self.init_until_idx) };

                // SAFETY: by definition of `init_until_idx`, `init_slice` is fully initialized
                let init_slice = unsafe { &mut *(init_slice as *mut [MaybeUninit<T>]).cast::<T>() };

                // SAFETY:
                // - `init_slice` is valid for dropping
                // - `self.array_mut` (and hence `init_slice`) is not used after `self` is dropped
                unsafe { ptr::drop_in_place(init_slice) };
            }
        }

        // This can be replaced with a call to `MaybeUninit::uninit_array` once that is stabilized
        //
        // SAFETY: an array of `MaybeUninit<_>` is always initialized
        let mut mapped: [MaybeUninit<B>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut guard = Guard {
            array_mut: &mut mapped,
            init_until_idx: 0,
        };

        for value in self {
            // SAFETY: the iterator yields exactly `N` elements,
            // so `guard.init_until_idx` has been increased at most `N - 1` times
            // and hence is within bounds of `guard.array_mut`
            unsafe {
                guard
                    .array_mut
                    .get_unchecked_mut(guard.init_until_idx)
                    // if `f` returns `Err(_)` or panics, then `guard` is dropped
                    .write(f(value)?);
            }

            guard.init_until_idx += 1;
        }

        // now `guard.init_until_idx == N` and the target array is fully initialized,
        // so make sure the guard isn't dropped
        mem::forget(guard);

        // SAFETY: `mapped` is fully initialized
        let mapped = unsafe { ptr::addr_of!(mapped).cast::<[B; N]>().read() };

        Ok(mapped)
    }
}

impl<A> Functor<A> for Cell<A> {
    type Target<B> = Cell<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for Cell<A> {
    type Target<B> = Cell<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        Cell::new(f(self.into_inner()))
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        f(self.into_inner()).map(Cell::new)
    }
}

impl<A> Functor<A> for RefCell<A> {
    type Target<B> = RefCell<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for RefCell<A> {
    type Target<B> = RefCell<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        RefCell::new(f(self.into_inner()))
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        f(self.into_inner()).map(RefCell::new)
    }
}

impl<A> Functor<A> for UnsafeCell<A> {
    type Target<B> = UnsafeCell<B>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A> Functor0<A> for UnsafeCell<A> {
    type Target<B> = UnsafeCell<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        UnsafeCell::new(f(self.into_inner()))
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        f(self.into_inner()).map(UnsafeCell::new)
    }
}
