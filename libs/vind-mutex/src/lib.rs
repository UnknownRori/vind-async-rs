#![no_std]

use self::{inner::Inner, locked_mutex::LockedMutex};

mod inner;
mod locked_mutex;
mod unlocked_mutex;

pub use locked_mutex::LockedMutex as Mutex;
