use alloc::boxed::Box;
use futures_core::{future::BoxFuture, Future};

impl<T: ?Sized> FutureExt for T where T: Future {}

pub trait FutureExt: Future {
    fn boxed<'a>(self) -> BoxFuture<'a, Self::Output>
    where
        Self: Sized + Send + 'a,
    {
        Box::pin(self)
    }
}
