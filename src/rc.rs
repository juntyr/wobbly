#[cfg(not(feature = "std"))]
use alloc::rc::{Rc, Weak};
use core::cell::Cell;
#[cfg(feature = "std")]
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Wobbly<T: ?Sized> {
    weak: Weak<T>,
    should_decref: Rc<Cell<bool>>,
}

// TODO: check that Wobbly is !Send and !Sync

impl<T: ?Sized> Wobbly<T> {
    #[must_use]
    pub fn new(strong: Rc<T>) -> Self {
        let weak = Rc::downgrade(&strong);

        // leak one strong reference count
        core::mem::forget(strong);

        Self {
            weak,
            should_decref: Rc::new(Cell::new(true)),
        }
    }

    #[must_use]
    #[inline]
    pub fn downgrade(&self) -> Weak<T> {
        self.weak.clone()
    }

    #[must_use]
    #[inline]
    pub fn upgrade(&self) -> Option<Rc<T>> {
        self.weak.upgrade()
    }

    #[must_use]
    #[inline]
    pub fn weak_count(&self) -> usize {
        self.weak.weak_count()
    }

    #[must_use]
    #[inline]
    pub fn strong_count(&self) -> usize {
        self.weak.strong_count()
    }
}

impl<T: ?Sized> Drop for Wobbly<T> {
    fn drop(&mut self) {
        if self.should_decref.replace(false) {
            // Safety:
            // - Wobbly leaks one strong reference upon construction
            // - we have just obtained the unique permission to free it
            unsafe { Rc::decrement_strong_count(self.weak.as_ptr()) };
        }
    }
}
