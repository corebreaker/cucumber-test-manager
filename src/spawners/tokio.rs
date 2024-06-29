use super::{TestSpawner, TestSpawnFactory};
use tokio::spawn;
use std::{future::Future, pin::Pin};

pub struct TokioSpawner;

impl TestSpawner for TokioSpawner {
    fn spawn(&self, res: Pin<Box<dyn Future<Output = ()> + Send>>) {
        spawn(res);
    }
}

impl TestSpawnFactory for TokioSpawner {
    fn new() -> Self {
        Self
    }
}

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
        Mutex,
        Condvar,
    };

    #[test]
    fn test_tokio_spawner() {
        let value = Arc::new(AtomicUsize::new(0_usize));

        {
            let condition = Arc::new(Condvar::new());
            let func = {
                let value = Arc::clone(&value);
                let condition = Arc::clone(&condition);

                || async move {
                    value.store(42, Ordering::SeqCst);
                    condition.notify_all();
                }
            };

            let lock = Mutex::new(());
            let rt = Runtime::new().unwrap();

            rt.block_on(async move {
                TokioSpawner.spawn(Box::pin(func()));
                drop(condition.wait(lock.lock().unwrap()));
            });
        }

        assert_eq!(value.load(Ordering::SeqCst), 42);
    }
}
// no-coverage:stop
