use super::{TestSpawner, TestSpawnFactory};
use futures::executor::block_on;
use std::{future::Future, pin::Pin};

pub(in super::super) struct DefaultSpawner;

impl TestSpawner for DefaultSpawner {
    fn spawn(&self, res: Pin<Box<dyn Future<Output = ()> + Send>>) {
        block_on(res);
    }
}

impl TestSpawnFactory for DefaultSpawner {
    fn new() -> Self {
        Self
    }
}
