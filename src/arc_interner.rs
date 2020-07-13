use std::sync::Arc;
use std::hash::Hash;
use std::collections::HashSet;

#[derive(Debug)]
pub struct ArcInterner<T: ?Sized>(HashSet<Arc<T>>);

impl<T: ?Sized> Default for ArcInterner<T> {
    fn default() -> ArcInterner<T> {
        ArcInterner(HashSet::new())
    }
}

impl<T: ?Sized + Hash + Eq> ArcInterner<T> {
    pub fn new() -> ArcInterner<T> {
        Default::default()
    }

    pub fn try_intern(&self, t: &T) -> Option<Arc<T>> {
        self.0.get(t).cloned()
    }

    pub fn shrink_to_fit(&mut self) {
        self.0.retain(|value| Arc::strong_count(value) > 1);
        self.0.shrink_to_fit();
    }
}

impl<T: Sized + Hash + Eq> ArcInterner<T> {
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

impl<T: ?Sized + Hash + Eq + Clone> ArcInterner<T> {
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

impl<T: ?Sized + Hash + Eq + Clone> ArcInterner<[T]> {
    pub fn intern_slice(&mut self, t: &[T]) -> Arc<[T]> {
        if let Some(value) = self.0.get(t) {
            value.clone()
        } else {
            let value: Arc<[T]> = Arc::from(t);
            self.0.insert(value.clone());
            value
        }
    }
}

impl ArcInterner<str> {
    pub fn intern_str(&mut self, t: &str) -> Arc<str> {
        if let Some(value) = self.0.get(t) {
            value.clone()
        } else {
            let value: Arc<str> = Arc::from(t);
            self.0.insert(value.clone());
            value
        }
    }
}
