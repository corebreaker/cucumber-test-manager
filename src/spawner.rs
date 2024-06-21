use futures::{executor::block_on, future::Future};
use std::pin::Pin;

pub trait TestSpawner {
    fn spawn(&self, res: Pin<Box<dyn Future<Output = ()>>>);
}

pub(super) struct DefaultSpawner;

impl TestSpawner for DefaultSpawner {
    fn spawn(&self, res: Pin<Box<dyn Future<Output = ()>>>) {
        block_on(res);
    }
}
