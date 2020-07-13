use std::rc::Rc;
use std::hash::Hash;
use std::collections::HashSet;

#[derive(Debug)]
pub struct RcInterner<T: ?Sized>(HashSet<Rc<T>>);

impl<T: ?Sized> Default for RcInterner<T> {
    fn default() -> RcInterner<T> {
        RcInterner(HashSet::new())
    }
}

impl<T: ?Sized + Hash + Eq> RcInterner<T> {
    pub fn new() -> RcInterner<T> {
        Default::default()
    }

    pub fn try_intern(&self, t: &T) -> Option<Rc<T>> {
        self.0.get(t).cloned()
    }

    pub fn shrink_to_fit(&mut self) {
        self.0.retain(|value| Rc::strong_count(value) > 1);
        self.0.shrink_to_fit();
    }
}

impl<T: Sized + Hash + Eq> RcInterner<T> {
    pub fn intern(&mut self, t: T) -> Rc<T> {
        if let Some(value) = self.0.get(&t) {
            value.clone()
        } else {
            let value = Rc::new(t);
            self.0.insert(value.clone());
            value
        }
    }
}

impl<T: ?Sized + Hash + Eq + Clone> RcInterner<T> {
    pub fn intern_cloned(&mut self, t: &T) -> Rc<T> {
        if let Some(value) = self.0.get(t) {
            value.clone()
        } else {
            let value = Rc::new(t.clone());
            self.0.insert(value.clone());
            value
        }
    }
}

impl<T: ?Sized + Hash + Eq + Clone> RcInterner<[T]> {
    pub fn intern_slice(&mut self, t: &[T]) -> Rc<[T]> {
        if let Some(value) = self.0.get(t) {
            value.clone()
        } else {
            let value: Rc<[T]> = Rc::from(t);
            self.0.insert(value.clone());
            value
        }
    }
}

impl RcInterner<str> {
    pub fn intern_str(&mut self, t: &str) -> Rc<str> {
        if let Some(value) = self.0.get(t) {
            value.clone()
        } else {
            let value: Rc<str> = Rc::from(t);
            self.0.insert(value.clone());
            value
        }
    }
}
