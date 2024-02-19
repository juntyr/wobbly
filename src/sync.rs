#[cfg(not(feature = "std"))]
use alloc::sync::{Arc, Weak};
use core::sync::atomic::{AtomicBool, Ordering};
#[cfg(feature = "std")]
use std::sync::{Arc, Weak};

#[derive(Clone)]
pub struct Wobbly<T: ?Sized> {
    weak: Weak<T>,
    should_decref: Arc<AtomicBool>,
}

// TODO: check that Wobbly is Send iff T: Send + Sync and Sync iff T: Send + Sync

impl<T: ?Sized> Wobbly<T> {
    #[must_use]
    pub fn new(strong: Arc<T>) -> Self {
        let weak = Arc::downgrade(&strong);

        // leak one strong reference count
        core::mem::forget(strong);

        Self {
            weak,
            should_decref: Arc::new(AtomicBool::new(true)),
        }
    }

    #[must_use]
    #[inline]
    pub fn downgrade(&self) -> Weak<T> {
        self.weak.clone()
    }

    #[must_use]
    #[inline]
    pub fn upgrade(&self) -> Option<Arc<T>> {
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
        if self
            .should_decref
            .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            // Safety:
            // - Wobbly leaks one strong reference upon construction
            // - we have just obtained the unique permission to free it
            unsafe { Arc::decrement_strong_count(self.weak.as_ptr()) };
        }
    }
}
