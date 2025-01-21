// future.rs - Future implementation for Prism runtime
// Created by: isdood
// Date: 2025-01-21 11:06:35 UTC

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

use crate::types::{PrismError, PrismResult, TaskHandle, TaskStatus};

/// Future state container
pub(crate) struct FutureState<T> {
    result: Option<PrismResult<T>>,
    waker: Option<Waker>,
    status: TaskStatus,
    start_time: Option<Instant>,
    timeout: Option<Duration>,
}

/// Async future for Prism tasks
pub struct PrismFuture<T> {
    handle: TaskHandle,
    state: Arc<Mutex<FutureState<T>>>,
}

impl<T> PrismFuture<T> {
    /// Create a new future
    pub(crate) fn new(handle: TaskHandle, timeout: Option<Duration>) -> Self {
        Self {
            handle,
            state: Arc::new(Mutex::new(FutureState {
                result: None,
                waker: None,
                status: TaskStatus::Ready,
                start_time: None,
                timeout,
            })),
        }
    }

    /// Set the result and wake the future
    pub(crate) fn set_result(&self, result: PrismResult<T>) {
        let mut state = self.state.lock().unwrap();
        state.result = Some(result);
        state.status = match &state.result {
            Some(Ok(_)) => TaskStatus::Completed,
            Some(Err(_)) => TaskStatus::Failed,
            None => TaskStatus::Running,
        };
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }

    /// Get the current status
    pub fn status(&self) -> TaskStatus {
        self.state.lock().unwrap().status
    }

    /// Get the task handle
    pub fn handle(&self) -> TaskHandle {
        self.handle
    }

    /// Check if the future has timed out
    fn check_timeout(&self) -> PrismResult<()> {
        let state = self.state.lock().unwrap();
        if let (Some(timeout), Some(start_time)) = (state.timeout, state.start_time) {
            if start_time.elapsed() > timeout {
                return Err(PrismError::Timeout);
            }
        }
        Ok(())
    }
}

impl<T> Future for PrismFuture<T> {
    type Output = PrismResult<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        // Set start time on first poll
        if state.start_time.is_none() {
            state.start_time = Some(Instant::now());
        }

        // Check timeout
        if let Err(e) = self.check_timeout() {
            return Poll::Ready(Err(e));
        }

        match state.result.take() {
            Some(result) => Poll::Ready(result),
            None => {
                state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

/// Extension trait for futures
pub trait FutureExt: Future + Sized {
    /// Add timeout to the future
    fn timeout(self, duration: Duration) -> TimeoutFuture<Self>;
    
    /// Chain multiple futures
    fn chain<F, U>(self, f: F) -> ChainFuture<Self, F>
    where
        F: FnOnce(Self::Output) -> U,
        U: Future;
}

impl<F: Future> FutureExt for F {
    fn timeout(self, duration: Duration) -> TimeoutFuture<Self> {
        TimeoutFuture {
            future: self,
            timeout: duration,
            start_time: None,
        }
    }

    fn chain<Fn, U>(self, f: Fn) -> ChainFuture<Self, Fn>
    where
        Fn: FnOnce(Self::Output) -> U,
        U: Future,
    {
        ChainFuture {
            future: self,
            next: Some(f),
            state: ChainState::First,
        }
    }
}

/// Future with timeout
pub struct TimeoutFuture<F: Future> {
    future: F,
    timeout: Duration,
    start_time: Option<Instant>,
}

impl<F: Future> Future for TimeoutFuture<F> {
    type Output = PrismResult<F::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        if let Some(start_time) = self.start_time {
            if start_time.elapsed() >= self.timeout {
                return Poll::Ready(Err(PrismError::Timeout));
            }
        }

        // Safe to use pin projection as fields implement Unpin
        let future = unsafe { self.as_mut().map_unchecked_mut(|s| &mut s.future) };
        match future.poll(cx) {
            Poll::Ready(output) => Poll::Ready(Ok(output)),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// State of chained futures
enum ChainState<T, U> {
    First,
    Second(U),
    Done(T),
}

/// Chained futures
pub struct ChainFuture<F1: Future, F2> {
    future: F1,
    next: Option<F2>,
    state: ChainState<F1::Output, F2::Future>,
}

impl<F1, F2, U> Future for ChainFuture<F1, F2>
where
    F1: Future,
    F2: FnOnce(F1::Output) -> U,
    U: Future,
{
    type Output = U::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match &mut self.state {
                ChainState::First => {
                    // Safe to use pin projection as fields implement Unpin
                    let future = unsafe { self.as_mut().map_unchecked_mut(|s| &mut s.future) };
                    match future.poll(cx) {
                        Poll::Ready(output) => {
                            let next = self.next.take().unwrap()(output);
                            self.state = ChainState::Second(next);
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
                ChainState::Second(future) => {
                    // Safe to poll as future implements Unpin
                    match Pin::new(future).poll(cx) {
                        Poll::Ready(output) => return Poll::Ready(output),
                        Poll::Pending => return Poll::Pending,
                    }
                }
                ChainState::Done(_) => panic!("Future polled after completion"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_future_completion() {
        let handle = TaskHandle::new();
        let future = PrismFuture::<i32>::new(handle, None);
        
        // Simulate async completion
        thread::spawn({
            let future = future.clone();
            move || {
                thread::sleep(Duration::from_millis(10));
                future.set_result(Ok(42));
            }
        });

        let result = futures::executor::block_on(future);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_future_timeout() {
        let handle = TaskHandle::new();
        let future = PrismFuture::<i32>::new(handle, Some(Duration::from_millis(10)));
        
        // Simulate slow task
        thread::spawn({
            let future = future.clone();
            move || {
                thread::sleep(Duration::from_millis(50));
                future.set_result(Ok(42));
            }
        });

        let result = futures::executor::block_on(future);
        assert!(matches!(result, Err(PrismError::Timeout)));
    }

    #[test]
    fn test_future_chaining() {
        let handle = TaskHandle::new();
        let future1 = PrismFuture::<i32>::new(handle, None);
        
        let chained = future1.chain(|result| async move {
            result.map(|x| x * 2)
        });

        // Simulate async completion
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            future1.set_result(Ok(21));
        });

        let result = futures::executor::block_on(chained);
        assert_eq!(result.unwrap(), 42);
    }
}
