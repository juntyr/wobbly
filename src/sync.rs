//! Thread-safe reference-counting pointers. 'Arc' stands for 'Atomically
//! Reference Counted'. This module provides extended functionality for
//! [`std::sync`].
//!
//! The type [`Wobbly<T>`](Wobbly) provides wobbly-shared ownership of a value
//! of type `T`, allocated on the heap. Like [`Weak`], [`Wobbly`] is
//! generally a non-owning (weak) pointer that can be [`upgrade`][upgrade]d to
//! obtain an [`Arc`], but this will return [`None`] if the value stored
//! in the allocation has already been dropped. A [`Wobbly`] pointer can also be
//! [`downgrade`][downgrade]d to obtain a [`Weak`].
//!
//! Unlike [`Weak`], however, one or more [`Wobbly`]s may also together
//! share an owning (strong) pointer, which keeps the stored value alive. When
//! the first [`Wobbly`] sharing an owning (strong) pointer is dropped, the
//! owning (strong) pointer is released as well.
//!
//! A [`Wobbly`] pointer can be created from an owned [`Arc`] using
//! [`Wobbly::new`], which consumes the owning (strong) pointer and creates an
//! additional non-owning (weak) pointer as well. Invoking [`clone`][clone] on
//! [`Wobbly`] produces a new wobbly pointer pointer to same value, which shares
//! the same one owning (strong) pointer. While all [`Wobbly`]s produced by
//! [`clone`][clone]-ing are alive, they keep the value alive even if there are
//! no other owned [`Arc`]s to it. Once the first of the [`Wobbly`]s is
//! dropped, the owned (strong) pointer is released, and if it was the only
//! remaining owned (strong) pointer the value is dropped. Note that multiple
//! calls to [`Wobbly::new`] create multiple independent groups of wobbly
//! pointers which can all keep the value alive on their own.
//!
//! A cycle between [`Wobbly`] pointers where all [`Wobbly`]s that share the
//! same owning (strong) pointer participate in the cycle will never be
//! deallocated. However, the cycle can be broken by storing a [`clone`][clone]
//! of one of the cyclic [`Wobbly`]s that can then be dropped.
//!
//! [clone]: Clone::clone
//! [downgrade]: Wobbly::downgrade
//! [upgrade]: Wobbly::upgrade

#[cfg(not(feature = "std"))]
use alloc::sync::{Arc, Weak};
use core::sync::atomic::{AtomicBool, Ordering};
#[cfg(feature = "std")]
use std::sync::{Arc, Weak};

/// A thread-safe atomically reference-counting pointer, similar to [`Weak`],
/// which provides wobbly-shared ownership of a value of type `T`, allocated on
/// the heap.
///
/// See the [module-level documentation](./index.html) for more details.
///
/// A `Wobbly<T>` pointer is [`Send`](Send) and [`Sync`](Sync) iff `T` is
/// [`Send`](Send) and [`Sync`](Sync), just like [`Weak`]:
///
/// ```
/// # use wobbly::sync::Wobbly;
/// fn check_send_sync<T: Send + Sync>() {}
///
/// check_send_sync::<Wobbly<()>>()
/// ```
///
/// ```compile_fail
/// # use wobbly::sync::Wobbly;
/// fn check_send_sync<T: Send + Sync>() {}
///
/// // `Cell<()>` cannot be shared between threads safely
/// check_send_sync::<Wobbly<std::cell::Cell<()>>>()
/// ```
///
/// ```compile_fail
/// # use wobbly::sync::Wobbly;
/// fn check_send_sync<T: Send + Sync>() {}
///
/// // `MutexGuard<()>` cannot be sent between threads safely
/// check_send_sync::<Wobbly<std::sync::MutexGuard<()>>>()
/// ```
pub struct Wobbly<T: ?Sized> {
    weak: Weak<T>,
    should_decref: Arc<AtomicBool>,
}

impl<T: ?Sized> Wobbly<T> {
    /// Creates a new `Wobbly<T>` from an owning (strong) [`Arc`]
    /// pointer.
    ///
    /// `Wobbly::new` creates a new group of `Wobbly` pointers, which keeps the
    /// inner value alive as long as none of the `Wobbly`s is dropped. To extend
    /// an existing group, [`clone`][clone] one of its `Wobbly` pointers.
    ///
    /// [clone]: Clone::clone
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

    /// Creates a new [`Weak`] pointer to this allocation.
    #[must_use]
    #[inline]
    pub fn downgrade(&self) -> Weak<T> {
        self.weak.clone()
    }

    /// Attempts to upgrade the `Wobbly` pointer to an [`Arc`], delaying
    /// dropping of the inner value if successful.
    ///
    /// Returns [`None`] if the inner value has since been dropped.
    #[must_use]
    #[inline]
    pub fn upgrade(&self) -> Option<Arc<T>> {
        self.weak.upgrade()
    }

    /// Gets the number of weak pointers pointing to this allocation.
    ///
    /// Note that this `Wobbly` counts as one weak pointer.
    #[must_use]
    #[inline]
    pub fn weak_count(&self) -> usize {
        self.weak.weak_count()
    }

    /// Gets the number of strong pointers pointing to this allocation.
    ///
    /// Note that one group of `Wobbly`s that was created only by
    /// [`clone`][clone]-ing counts as one strong pointer as long as none of the
    /// group members has been dropped.
    ///
    /// [clone]: Clone::clone
    #[must_use]
    #[inline]
    pub fn strong_count(&self) -> usize {
        self.weak.strong_count()
    }
}

impl<T: ?Sized> Clone for Wobbly<T> {
    /// Makes a clone of the `Wobbly` pointer that points to the same allocation.
    ///
    /// Cloning a `Wobbly` pointer also extends its group of `Wobbly`s that
    /// share the same one owning pointer. If this owning pointer has not yet
    /// been released, the newly cloned `Wobbly` will release it if it is the
    /// first `Wobbly` pointer of the group to be dropped.
    fn clone(&self) -> Self {
        Self {
            weak: self.weak.clone(),
            should_decref: self.should_decref.clone(),
        }
    }
}

impl<T: ?Sized> Drop for Wobbly<T> {
    /// Drops the `Wobbly` pointer, which releases one non-owning (weak) pointer
    /// to the value. If this `Wobbly` is the first of its group, created only
    /// by [`clone`][clone]-ing, that is dropped, an owning (strong) pointer is
    /// released as well, and the value may be dropped iff this was the last
    /// remaining owning (strong) pointer.
    ///
    /// [clone]: Clone::clone
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
