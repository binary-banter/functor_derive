pub mod impl_alloc;
pub mod impl_core;
pub mod impl_std;

use paste::paste;

// Re-export derive macro.
pub use functor_derive_lib::*;
pub use impl_alloc::*;
#[allow(unused)]
pub use impl_core::*;
pub use impl_std::*;

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

#[macro_export]
macro_rules! functor_impl {
    ($typ:ident) => {
        paste::paste! {
            impl<A> Functor<A> for $typ<A> {
                type Target<B> = $typ<B>;

                fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::Target<B> {
                    self.[<__fmap_0_ref>](&f)
                }

                fn try_fmap<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Self::Target<B>, E> {
                    self.[<__try_fmap_0_ref>](&f)
                }
            }
        }
    };
}
