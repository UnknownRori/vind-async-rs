use core::hint::spin_loop;

use super::{inner::Inner, unlocked_mutex::UnlockedMutex};

#[derive(Debug)]
pub struct LockedMutex<T> {
    pub inner: Inner<T>,
}

unsafe impl<T> Send for LockedMutex<T> {}
unsafe impl<T> Sync for LockedMutex<T> {}

impl<T> LockedMutex<T> {
    pub fn new(data: T) -> LockedMutex<T> {
        LockedMutex {
            inner: Inner::new(data),
        }
    }

    pub fn lock(&self) -> UnlockedMutex<T> {
        self.acquire()
    }

    pub fn acquire(&self) -> UnlockedMutex<T> {
        while !self.inner.try_unlock() {
            spin_loop();
        }

        UnlockedMutex::new(&self)
    }

    pub fn release(&self) {
        self.inner.lock();
    }
}
