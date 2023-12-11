use core::cell::{Cell, RefCell, UnsafeCell};
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::{mem, ptr};
use core::ops::ControlFlow;
use crate::*;

functor_impl!(Option);

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

functor_impl!(PhantomData);

impl<A> Functor0<A> for PhantomData<A> {
    type Target<B> = PhantomData<B>;

    fn __fmap_0_ref<B>(self, _f: &impl Fn(A) -> B) -> Self::Target<B> {
        PhantomData
    }

    fn __try_fmap_0_ref<B, E>(self, _f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        Ok(PhantomData)
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

functor_impl!(Cell);

impl<A> Functor0<A> for Cell<A> {
    type Target<B> = Cell<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        Cell::new(f(self.into_inner()))
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        f(self.into_inner()).map(Cell::new)
    }
}

functor_impl!(RefCell);

impl<A> Functor0<A> for RefCell<A> {
    type Target<B> = RefCell<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        RefCell::new(f(self.into_inner()))
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        f(self.into_inner()).map(RefCell::new)
    }
}

functor_impl!(UnsafeCell);

impl<A> Functor0<A> for UnsafeCell<A> {
    type Target<B> = UnsafeCell<B>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        UnsafeCell::new(f(self.into_inner()))
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        f(self.into_inner()).map(UnsafeCell::new)
    }
}

impl<A, C> Functor<A> for ControlFlow<A,C> {
    type Target<B> = ControlFlow<B,C>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
        self.__fmap_0_ref(&f)
    }

    fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        self.__try_fmap_0_ref(&f)
    }
}

impl<A, C> Functor0<A> for ControlFlow<A, C> {
    type Target<B> = ControlFlow<B, C>;

    fn __fmap_0_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        match self {
            ControlFlow::Continue(c) => ControlFlow::Continue(c),
            ControlFlow::Break(v) => ControlFlow::Break(f(v)),
        }
    }

    fn __try_fmap_0_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        Ok(match self {
            ControlFlow::Continue(c) => ControlFlow::Continue(c),
            ControlFlow::Break(v) => ControlFlow::Break(f(v)?),
        })
    }
}

impl<C, A> Functor1<A> for ControlFlow<C, A> {
    type Target<B> = ControlFlow<C, B>;

    fn __fmap_1_ref<B>(self, f: &impl Fn(A) -> B) -> Self::Target<B> {
        match self {
            ControlFlow::Continue(v) => ControlFlow::Continue(f(v)),
            ControlFlow::Break(c) => ControlFlow::Break(c),
        }
    }

    fn __try_fmap_1_ref<B, E>(self, f: &impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
        Ok(match self {
            ControlFlow::Continue(v) => ControlFlow::Continue(f(v)?),
            ControlFlow::Break(c) => ControlFlow::Break(c),
        })
    }
}
