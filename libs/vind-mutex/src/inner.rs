use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug)]
pub struct Inner<T: ?Sized> {
    is_acquirable: AtomicBool,
    pub data: UnsafeCell<T>,
}

impl<T> Inner<T> {
    pub fn new(data: T) -> Inner<T> {
        Inner {
            is_acquirable: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) {
        self.is_acquirable.store(false, Ordering::Release);
    }

    pub fn try_unlock(&self) -> bool {
        self.is_acquirable
            .compare_exchange_weak(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
    }
}
