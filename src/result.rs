use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub(super) struct TestResult {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl TestResult {
    pub(super) fn new<F: Future<Output = ()> + 'static>(f: F) -> Self {
        TestResult { future: Box::pin(f) }
    }
}

impl Future for TestResult {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.future.as_mut().poll(cx)
    }
}

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    #[test]
    fn test_result() {
        let mut witness = Arc::new(AtomicUsize::new(1));

        let result = {
            let witness = Arc::clone(&mut witness);
            TestResult::new(async move {
                witness.store(42, Ordering::SeqCst);
            })
        };

        block_on(result);
        assert_eq!(witness.load(Ordering::SeqCst), 42);
    }
}
// no-coverage:stop
