#![no_std]
//! Lock data with mutable access on a `cortex_m` processor.

use core::cell::RefCell;

pub use cortex_m;
use cortex_m::interrupt::Mutex;

/// This datatype provides a lock with interior mutability
/// for the data inside. The locking is provided through a
/// `cortex_m` critical section.
pub struct LockMut<T>(Mutex<RefCell<Option<T>>>);

impl<T> LockMut<T> {
    /// Create a new empty `LockMut`.
    pub const fn new() -> Self {
        LockMut(Mutex::new(RefCell::new(None)))
    }

    /// Initialize a previously-uninitialized `LockMut`.
    ///
    /// # Panics
    ///
    /// Panics if this `LockMut` is to be initialized a second time.
    pub fn init(&self, val: T) {
        cortex_m::interrupt::free(|cs| {
            let mut cell = self.0.borrow(cs).borrow_mut();
            assert!(cell.is_none(), "lock reinitialized");
            *cell = Some(val);
        });
    }

    /// Locks, then runs the closure `f` with a mutable
    /// reference to the locked data.
    ///
    /// # Panics
    ///
    /// Panics if this `LockMut` is uninitialized.
    pub fn with_lock<F: FnOnce(&mut T)>(&self, f: F) {
        cortex_m::interrupt::free(|cs| {
            // &LockMut<T> → &Mutex<RefCell<Option<T>>> →
            // &RefCell<Option<T>> → &mut Option<T>
            let mut cell = self.0.borrow(cs).borrow_mut();
            // &mut Option<T> → Option<&mut T> → &mut T
            let val_mut = cell.as_mut().expect("empty lock");
            f(val_mut);
        });
    }
}
