#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;

pub struct LockMut<T>(Mutex<RefCell<Option<T>>>);

impl<T> LockMut<T> {
    pub const fn empty() -> Self {
        LockMut(Mutex::new(RefCell::new(None)))
    }

    pub fn set(&self, val: T) {
        cortex_m::interrupt::free(|cs| {
            let mut cell = self.0.borrow(cs).borrow_mut();
            *cell = Some(val);
        });
    }

    pub fn lock<F: FnOnce(&mut T)>(&self, f: F) {
        cortex_m::interrupt::free(|cs| {
            let mut cell = self.0.borrow(cs).borrow_mut();
            let val_mut = cell.as_mut().expect("empty lock");
            f(val_mut);
        });
    }
}
