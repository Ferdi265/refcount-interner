# `refcount-interner`

A simple reference-counted
[interning](https://en.wikipedia.org/wiki/String_interning)
library for strings, slices, and other data.

This crate provides two kinds of owned interners that store the interned
data in the reference-counted types `Rc<T>` or `Arc<T>`. When the
`shrink_to_fit()` method is called on the interner, or when the interner is
dropped, unused interned objects are deallocated.

The two kinds of interners provided by this crate are `RcInterner` and
`ArcInterner`, returning `Rc<T>` and `Arc<T>` objects respectively.


## Example

```rust
use std::rc::Rc;
use refcount_interner::RcInterner;

let mut interner = RcInterner::new();

let hello = interner.intern_str("hello");
let world = interner.intern_str("world");

assert!(Rc::ptr_eq(&hello, &interner.intern_str("hello")));
```

## Documentation

Documentation is provided via rustdoc, and can be built with `cargo doc`, or
viewed online at
[docs.rs/refcount-interner/](https://docs.rs/refcount-interner/).

## License

Licensed under either of

- Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
