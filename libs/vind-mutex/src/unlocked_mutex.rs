use core::ops::{Deref, DerefMut};

use super::locked_mutex::LockedMutex;

#[derive(Debug)]
pub struct UnlockedMutex<'a, T> {
    mutex: &'a LockedMutex<T>,
}

unsafe impl<T> Send for UnlockedMutex<'_, T> {}
unsafe impl<T> Sync for UnlockedMutex<'_, T> {}

impl<T> Deref for UnlockedMutex<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.inner.data.get() }
    }
}

impl<T> DerefMut for UnlockedMutex<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.inner.data.get() }
    }
}

impl<T> Drop for UnlockedMutex<'_, T> {
    fn drop(&mut self) {
        self.mutex.release();
    }
}

impl<T> UnlockedMutex<'_, T> {
    pub fn new(mutex: &LockedMutex<T>) -> UnlockedMutex<T> {
        UnlockedMutex { mutex: &mutex }
    }

    pub fn release(&self) {
        self.mutex.release();
    }
}
