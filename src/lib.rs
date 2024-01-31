#![no_std]
//! Lock data with mutable access on a `cortex_m` processor.

use core::cell::RefCell;

pub use microbit;
use microbit::hal::pac::{NVIC, Interrupt};


/// This datatype provides a lock with interior mutability
/// for the data inside. The locking is provided through a
/// `cortex_m` critical section.
pub struct LockMut<T>(RefCell<Option<T>>);
unsafe impl<T> core::marker::Sync for LockMut<T> {}

impl<T> LockMut<T> {
    /// Create a new empty `LockMut`.
    pub const fn new() -> Self {
        LockMut(RefCell::new(None))
    }

    /// Initialize a previously-uninitialized `LockMut`.
    ///
    /// # Panics
    ///
    /// Panics if this `LockMut` is to be initialized a second time.
    pub fn init(&self, val: T) {
        NVIC::mask(Interrupt::TIMER1);
        let mut cell = self.0.borrow_mut();
        assert!(cell.is_none(), "lock reinitialized");
        *cell = Some(val);
        unsafe { NVIC::unmask(Interrupt::TIMER1) };
    }

    /// Locks, then runs the closure `f` with a mutable
    /// reference to the locked data.
    ///
    /// # Panics
    ///
    /// Panics if this `LockMut` is uninitialized.
    pub fn with_lock<F: FnOnce(&mut T)>(&self, f: F) {
        NVIC::mask(Interrupt::TIMER1);
        let mut cell = self.0.borrow_mut();
        let val_mut = cell.as_mut().expect("empty lock");
        f(val_mut);
        unsafe { NVIC::unmask(Interrupt::TIMER1) };
    }
}
