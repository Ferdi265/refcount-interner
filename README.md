# `refcount_interner`

A simple reference-counted
[interning](https://en.wikipedia.org/wiki/String_interning)
library for strings, slices, and other data.

This crate provides two kinds of owned interners that store the interned
data in the reference-counted types `Rc<T>` or `Arc<T>`. When the
`shrink_to_fit()` method is called on the interner, or when the interner is
dropped, unused interned objects are deallocated.

The two kinds of interners provided by this crate are `RcInterner` and
`ArcInterner`, returning `Rc<T>` and `Arc<T>` objects respectively.
