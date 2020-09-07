use std::sync::Arc;
use std::hash::Hash;
use std::collections::HashSet;

/// An interner returning atomically reference-counted pointers to the interned
/// data
///
/// Interned objects will be deallocated when there are no references to them
/// any more and `shrink_to_fit()` is called on the interner
///
/// # Example
/// ```rust
/// # use std::sync::Arc;
/// use refcount_interner::ArcInterner;
///
/// let mut interner = ArcInterner::new();
///
/// let x = interner.intern(42);
/// let y = interner.intern(1337);
///
/// assert_eq!(*x, 42);
/// assert_ne!(x, y);
/// assert!(Arc::ptr_eq(&x, &interner.intern(42)));
/// ```
#[derive(Debug)]
pub struct ArcInterner<T: ?Sized>(HashSet<Arc<T>>);

impl<T: ?Sized> Default for ArcInterner<T> {
    fn default() -> ArcInterner<T> {
        ArcInterner(HashSet::new())
    }
}

impl<T: ?Sized + Hash + Eq> ArcInterner<T> {
    /// Create a new, empty interner.
    ///
    /// # Example
    /// ```rust
    /// # use refcount_interner::ArcInterner;
    /// let mut interner = ArcInterner::new();
    /// # let x = interner.intern(42);
    /// ```
    pub fn new() -> ArcInterner<T> {
        Default::default()
    }

    /// Attempt to get a reference to an already interned object.
    ///
    /// If the object has already been interned, an option containing a
    /// reference to the already interned object will be returned.
    ///
    /// If the object has not yet been interned, `None` will be returned.
    ///
    /// # Example
    /// ```rust
    /// # use std::sync::Arc;
    /// # use refcount_interner::ArcInterner;
    /// let mut interner = ArcInterner::new();
    ///
    /// let x = interner.intern(42);
    /// assert_eq!(interner.try_intern(&42), Some(Arc::new(42)));
    /// assert_eq!(interner.try_intern(&1337), None);
    /// ```
    pub fn try_intern(&self, t: &T) -> Option<Arc<T>> {
        self.0.get(t).cloned()
    }

    /// Intern a boxed object
    ///
    /// This method must be used to intern unsized types, since unsized types
    /// cannot be passed to `intern()`. The two most common unsized types,
    /// `&[T]` and `&str` can be interned with `intern_slice()` and
    /// `intern_str()` as well.
    ///
    /// If the object has already been interned, the passed object will be
    /// dropped and deallocated, and a reference to the already interned object
    /// will be returned.
    ///
    /// If the object has not yet been interned, the passed object will be moved
    /// into an `Arc<T>`, remembered for future calls to `intern()`, and
    /// returned.
    ///
    /// # Example
    /// ```rust
    /// # use refcount_interner::ArcInterner;
    /// let mut interner = ArcInterner::new();
    ///
    /// let x = Box::new(42);
    /// let y = interner.intern_boxed(x);
    ///
    /// assert_eq!(*y, 42);
    /// ```
    pub fn intern_boxed(&mut self, t: Box<T>) -> Arc<T> {
        if let Some(value) = self.0.get(t.as_ref()) {
            value.clone()
        } else {
            let value: Arc<T> = Arc::from(t);
            self.0.insert(value.clone());
            value
        }
    }

    /// Deallocate all interned objects that are no longer referenced and shrink
    /// the internal storage to fit.
    ///
    /// # Example
    /// ```rust
    /// # use std::sync::Arc;
    /// # use refcount_interner::ArcInterner;
    /// let mut interner = ArcInterner::new();
    ///
    /// let x = interner.intern(42);
    /// let y = interner.intern(1337);
    /// let z = y.clone();
    ///
    /// drop(x);
    /// drop(y);
    ///
    /// interner.shrink_to_fit();
    /// assert_eq!(interner.try_intern(&42), None);
    /// assert_eq!(interner.try_intern(&1337), Some(Arc::new(1337)));
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.0.retain(|value| Arc::strong_count(value) > 1);
        self.0.shrink_to_fit();
    }
}

impl<T: Sized + Hash + Eq> ArcInterner<T> {
    /// Intern an owned object
    ///
    /// If the object has already been interned, the passed object will be
    /// dropped, and a reference to the already interned object will be
    /// returned.
    ///
    /// If the object has not yet been interned, the passed object will be moved
    /// into an `Arc<T>`, remembered for future calls to `intern()`, and
    /// returned.
    ///
    /// # Example
    /// ```rust
    /// # use std::sync::Arc;
    /// # use refcount_interner::ArcInterner;
    /// let mut interner = ArcInterner::new();
    ///
    /// let x = interner.intern(42);
    /// let y = interner.intern(1337);
    ///
    /// assert_eq!(*x, 42);
    /// assert_ne!(x, y);
    /// assert!(Arc::ptr_eq(&x, &interner.intern(42)));
    /// ```
    pub fn intern(&mut self, t: T) -> Arc<T> {
        if let Some(value) = self.0.get(&t) {
            value.clone()
        } else {
            let value = Arc::new(t);
            self.0.insert(value.clone());
            value
        }
    }
}

impl<T: Sized + Hash + Eq + Clone> ArcInterner<T> {
    /// Intern a borrowed object, cloning if it has not yet been interned
    ///
    /// If the object has already been interned, a reference to the already
    /// interned object will be returned.
    ///
    /// If the object has not yet been interned, the passed object will be moved
    /// into an `Arc<T>`, remembered for future calls to `intern()`, and
    /// returned.
    ///
    /// # Example
    /// ```rust
    /// # use refcount_interner::ArcInterner;
    /// let mut interner = ArcInterner::new();
    ///
    /// let x = 42;
    /// let y = interner.intern_cloned(&x);
    ///
    /// assert_eq!(x, *y);
    /// ```
    pub fn intern_cloned(&mut self, t: &T) -> Arc<T> {
        if let Some(value) = self.0.get(t) {
            value.clone()
        } else {
            let value = Arc::new(t.clone());
            self.0.insert(value.clone());
            value
        }
    }
}

impl<T: Sized + Hash + Eq + Clone> ArcInterner<[T]> {
    /// Intern a slice object
    ///
    /// This method can be used to intern slices without boxing them.
    ///
    /// If the slice has already been interned, a reference to the already
    /// interned slice will be returned.
    ///
    /// If the slice has not yet been interned, the passed object will be
    /// cloned into an `Arc<[T]>`, remembered for future calls to `intern()`,
    /// and returned.
    ///
    /// # Example
    /// ```rust
    /// # use refcount_interner::ArcInterner;
    /// let mut interner = ArcInterner::new();
    ///
    /// let x = interner.intern_slice(&[1, 2, 3]);
    ///
    /// assert_eq!(x.as_ref(), &[1, 2, 3]);
    /// ```
    pub fn intern_slice(&mut self, t: &[T]) -> Arc<[T]> {
        if let Some(value) = self.0.get(t) {
            value.clone()
        } else {
            let value: Arc<[T]> = Arc::from(t);
            self.0.insert(value.clone());
            value
        }
    }

    /// Intern an owned vector
    ///
    /// If the slice behind the vector has already been interned, a reference
    /// to the already / interned slice will be returned.
    ///
    /// If the slice has not yet been interned, the passed vector will be moved
    /// into an `Arc<[T]>`, remembered for future calls to `intern()`, and
    /// returned.
    ///
    /// # Example
    /// ```rust
    ///
    /// # use refcount_interner::ArcInterner;
    /// let mut interner = ArcInterner::new();
    ///
    /// let v = vec![1, 2, 3];
    /// let x = interner.intern_vec(v);
    ///
    /// assert_eq!(x.as_ref(), &[1, 2, 3]);
    /// ```
    pub fn intern_vec(&mut self, t: Vec<T>) -> Arc<[T]> {
        self.intern_boxed(t.into_boxed_slice())
    }
}

impl ArcInterner<str> {
    /// Intern a string slice
    ///
    /// This method can be used to intern string slices without boxing them.
    ///
    /// If the string slice has already been interned, a reference to the
    /// already interned string slice will be returned.
    ///
    /// If the string slice has not yet been interned, the passed object will be
    /// cloned into an `Arc<str>`, remembered for future calls to `intern()`,
    /// and returned.
    ///
    /// # Example
    /// ```rust
    /// # use refcount_interner::ArcInterner;
    /// let mut interner = ArcInterner::new();
    ///
    /// let x = interner.intern_str("hello");
    ///
    /// assert_eq!(x.as_ref(), "hello");
    /// ```
    pub fn intern_str(&mut self, t: &str) -> Arc<str> {
        if let Some(value) = self.0.get(t) {
            value.clone()
        } else {
            let value: Arc<str> = Arc::from(t);
            self.0.insert(value.clone());
            value
        }
    }

    /// Intern an owned string
    ///
    /// If the string has already been interned, a reference to the already
    /// interned string slice will be returned.
    ///
    /// If the string has not yet been interned, the passed string will be moved
    /// into an `Arc<str>`, remembered for future calls to `intern()`, and
    /// returned.
    ///
    /// # Example
    /// ```rust
    ///
    /// # use refcount_interner::ArcInterner;
    /// let mut interner = ArcInterner::new();
    ///
    /// let s = String::from("hello");
    /// let x = interner.intern_string(s);
    ///
    /// assert_eq!(x.as_ref(), "hello");
    /// ```
    pub fn intern_string(&mut self, t: String) -> Arc<str> {
        self.intern_boxed(t.into_boxed_str())
    }
}
