// Copyright 2020 Ferdinand Bachmann
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A simple reference-counted
//! [interning](https://en.wikipedia.org/wiki/String_interning)
//! library for strings, slices, and other data.
//!
//! This crate provides two kinds of owned interners that store the interned
//! data in the reference-counted types `Rc<T>` or `Arc<T>`. When the
//! `shrink_to_fit()` method is called on the interner, or when the interner is
//! dropped, unused interned objects are deallocated.
//!
//! The two kinds of interners provided by this crate are `RcInterner` and
//! `ArcInterner`, returning `Rc<T>` and `Arc<T>` objects respectively.

mod rc_interner;
mod arc_interner;

pub use rc_interner::RcInterner;
pub use arc_interner::ArcInterner;
