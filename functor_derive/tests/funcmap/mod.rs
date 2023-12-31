//! This test suite was provided by Matthias Stemmler's crate [funcmap_derive](https://crates.io/crates/funcmap_derive) under the MIT license.
//! The tests are translated from tests on commit [031a1b0400abd2f4ddae748ed356a02569ea982c](https://github.com/matthias-stemmler/funcmap/tree/031a1b0400abd2f4ddae748ed356a02569ea982c/funcmap_tests/tests).
//! Care was taken to change the tests minimally, and were commented if no translation was possible.

//! Licensed under either of
//! * Apache License, Version 2.0 (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//! * MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
//! at your option.

#![allow(unused)]

mod decorations;
mod fallible;
mod hygiene;
mod impl_alloc;
mod impl_core;
mod impl_std;
mod lints;
mod multi_param;
mod opts_crate;
mod opts_params;
mod single_param;
mod variants;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
struct T1;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
struct T2;
