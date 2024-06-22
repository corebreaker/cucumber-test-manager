use futures::future::Future;
use std::pin::Pin;

pub trait TestSpawner {
    fn spawn(&self, res: Pin<Box<dyn Future<Output = ()> + Send>>);
}
