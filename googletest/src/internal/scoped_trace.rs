// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

static NEXT_TRACE_ID: AtomicUsize = AtomicUsize::new(0);

/// Information about a scoped trace.
#[derive(Clone, Debug)]
pub struct TraceInfo {
    pub id: usize,
    pub file: &'static str,
    pub line: u32,
    pub message: String,
}

struct TraceNode {
    info: TraceInfo,
    parent: Option<Arc<TraceNode>>,
}

thread_local! {
    static TRACE_STACK: RefCell<Option<Arc<TraceNode>>> = const { RefCell::new(None) };
}

/// RAII guard to manage the push and pop of trace information.
///
/// This struct is `Send` so that `async` blocks holding `scoped_trace!` across
/// `.await` points remain `Send` for multi-threaded executors. When a future is
/// wrapped in [`instrument_future`] (which `#[gtest]` applies automatically to
/// async tests), [`TraceSwapGuard`] swaps the task's trace stack out of the
/// yielding thread's `TRACE_STACK` on `Poll::Pending` and into the resuming
/// thread's `TRACE_STACK` before the next `poll()`. Thus, if a task yields on
/// one thread and resumes or drops on another, `TRACE_STACK` follows the task
/// (or is dropped with `instrument_future`'s state if cancelled while
/// suspended). It should not be held across `.await` points in un-instrumented
/// futures.
#[doc(hidden)]
pub struct ScopedTraceGuard {
    id: usize,
}

impl ScopedTraceGuard {
    #[doc(hidden)]
    #[track_caller]
    pub fn new(message: String) -> Self {
        let caller = std::panic::Location::caller();
        let id = NEXT_TRACE_ID.fetch_add(1, Ordering::Relaxed);
        let info = TraceInfo { id, file: caller.file(), line: caller.line(), message };

        TRACE_STACK.with(|stack| {
            // Use try_borrow_mut to avoid double panic if called during
            // unwinding.
            if let Ok(mut s) = stack.try_borrow_mut() {
                let prev = s.clone();
                *s = Some(Arc::new(TraceNode { info, parent: prev }));
            }
        });
        Self { id }
    }
}

impl Drop for ScopedTraceGuard {
    fn drop(&mut self) {
        let _ = TRACE_STACK.try_with(|stack| {
            // Use try_borrow_mut to avoid double panic if called during
            // unwinding.
            if let Ok(mut s) = stack.try_borrow_mut() {
                *s = remove_from_list(s.clone(), self.id);
            }
        });
    }
}

/// Removes a node with the given `id` from the trace stack list.
///
/// Because the trace stack is structured as a persistent, reverse-linked list
/// pointing to parent nodes, removing a node from the middle involves
/// rebuilding the chain from the removed node up to the current head.
fn remove_from_list(head: Option<Arc<TraceNode>>, id: usize) -> Option<Arc<TraceNode>> {
    let head = head?;
    if head.info.id == id {
        return head.parent.clone();
    }

    let mut current = head.parent.clone();
    let mut nodes_to_recreate = vec![head.info.clone()];

    while let Some(node) = current {
        if node.info.id == id {
            let mut new_head = node.parent.clone();
            for info in nodes_to_recreate.into_iter().rev() {
                new_head = Some(Arc::new(TraceNode { info, parent: new_head }));
            }
            return new_head;
        }
        nodes_to_recreate.push(node.info.clone());
        current = node.parent.clone();
    }
    Some(head)
}

/// Retrieves a clone of the current thread's trace stack.
pub fn get_scoped_traces() -> Vec<TraceInfo> {
    let mut traces = Vec::new();
    let mut current = TRACE_STACK.with(|stack| stack.try_borrow().ok().and_then(|s| s.clone()));
    while let Some(node) = current {
        traces.push(node.info.clone());
        current = node.parent.clone();
    }
    traces.reverse();
    traces
}

/// RAII guard created on each `poll()` of an [`instrument_future`] that swaps
/// the future's captured `traces` stack into the current polling thread's
/// thread-local `TRACE_STACK` on entry ([`TraceSwapGuard::new`]) and swaps the
/// (potentially updated) thread-local `TRACE_STACK` back into the future's
/// `traces` state on exit ([`Drop`]), whether `poll()` returns `Poll::Ready`,
/// yields `Poll::Pending`, or unwinds from a panic.
///
/// This ensures that:
/// 1. Any [`ScopedTraceGuard`] created or dropped during `poll()` (even after
///    the task migrates across threads between `.await` points) operates on the
///    task's own trace stack in the current thread's `TRACE_STACK`.
/// 2. The outer/executor thread's prior `TRACE_STACK` is always restored as
///    soon as `poll()` yields, returns, or panics.
struct TraceSwapGuard<'a>(&'a mut Option<Arc<TraceNode>>);

impl<'a> TraceSwapGuard<'a> {
    fn new(traces: &'a mut Option<Arc<TraceNode>>) -> Self {
        // Use `try_with` to avoid panicking during thread teardown.
        let _ = TRACE_STACK.try_with(|stack| {
            if let Ok(mut s) = stack.try_borrow_mut() {
                std::mem::swap(&mut *s, traces);
            }
        });
        Self(traces)
    }
}

impl<'a> Drop for TraceSwapGuard<'a> {
    fn drop(&mut self) {
        // Use `try_with` to avoid panicking during thread teardown.
        let _ = TRACE_STACK.try_with(|stack| {
            if let Ok(mut s) = stack.try_borrow_mut() {
                std::mem::swap(&mut *s, self.0);
            }
        });
    }
}

/// Wraps `inner` so the current thread's scoped trace stack is captured at
/// creation time and swapped in across every `.await` poll point without any
/// `unsafe` or heap allocation.
pub fn instrument_future<F: std::future::Future>(
    inner: F,
) -> impl std::future::Future<Output = F::Output> {
    // Capture the trace stack eagerly at creation time to propagate it across
    // threads.
    let mut traces = TRACE_STACK.with(|stack| stack.try_borrow().ok().and_then(|s| s.clone()));
    async move {
        let mut inner = std::pin::pin!(inner);
        std::future::poll_fn(|cx| {
            let _guard = TraceSwapGuard::new(&mut traces);
            inner.as_mut().poll(cx)
        })
        .await
    }
}

// Test-only state and helpers, hidden from production API.
#[cfg(test)]
pub(crate) mod test_helpers {
    use super::*;
    use std::cell::Cell;

    thread_local! {
        pub static CAPTURED_TRACES_IN_HOOK: RefCell<Vec<TraceInfo>> = const { RefCell::new(Vec::new()) };
        pub static USE_CAPTURE_HOOK: Cell<bool> = const { Cell::new(false) };
    }

    pub fn enable_capture_in_hook(enable: bool) {
        USE_CAPTURE_HOOK.with(|v| v.set(enable));
    }

    pub fn get_captured_traces_in_hook() -> Vec<TraceInfo> {
        CAPTURED_TRACES_IN_HOOK.with(|v| v.borrow().clone())
    }

    pub fn clear_captured_traces_in_hook() {
        CAPTURED_TRACES_IN_HOOK.with(|v| v.borrow_mut().clear());
    }
}

#[cfg(test)]
mod tests {
    use super::test_helpers::*;
    use super::*;

    // Simple safe block_on executor for tests to avoid external runtime
    // dependencies.
    fn block_on<F: std::future::Future>(future: F) -> F::Output {
        use std::pin::pin;
        use std::task::{Context, Poll, Waker};

        let mut future = pin!(future);
        let mut cx = Context::from_waker(Waker::noop());

        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(val) => return val,
                Poll::Pending => {
                    std::thread::yield_now();
                }
            }
        }
    }

    #[test]
    fn test_instrument_future_send_spec() {
        fn assert_send<T: Send>(_: &T) {}

        let fut = instrument_future(async {});
        assert_send(&fut);
    }

    #[test]
    fn test_scoped_trace_fatal() {
        enable_capture_in_hook(true);
        clear_captured_traces_in_hook();

        // Ensure hook is installed
        crate::internal::test_outcome::TestOutcome::init_current_test_outcome();

        let _ = std::panic::catch_unwind(|| {
            let _guard = ScopedTraceGuard::new("Second trace".to_string());
            panic!("Intentional panic");
        });

        let captured = get_captured_traces_in_hook();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].message, "Second trace");

        enable_capture_in_hook(false);
    }

    #[test]
    fn test_async_scoped_trace_cross_thread() {
        // 1. Create a scoped trace on the main thread (Thread A).
        let _guard = ScopedTraceGuard::new("Trace from Thread A".to_string());

        // 2. Create the instrumented future.
        // It should capture the active trace context here (Thread A).
        let future = instrument_future(async {
            // 3. This will run on Thread B.
            get_scoped_traces()
        });

        // 4. Spawn Thread B and poll the future there.
        let handle = std::thread::spawn(move || block_on(future));
        let traces_on_thread_b = handle.join().unwrap();

        // 5. Assert that the trace from Thread A was successfully propagated to
        //    Thread B.
        assert_eq!(traces_on_thread_b.len(), 1);
        assert_eq!(traces_on_thread_b[0].message, "Trace from Thread A");
    }

    #[test]
    fn test_async_scoped_trace_yield_resume_and_drop_on_different_thread() {
        use std::future::Future;
        use std::pin::Pin;
        use std::task::{Context, Poll, Waker};

        struct PendingOnce(bool);
        impl Future for PendingOnce {
            type Output = ();
            fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
                if self.0 {
                    Poll::Ready(())
                } else {
                    self.0 = true;
                    Poll::Pending
                }
            }
        }

        let mut fut = Box::pin(instrument_future(async {
            let traces_during_resume = {
                let _guard = ScopedTraceGuard::new("Trace across yield".to_string());
                PendingOnce(false).await;
                // Resumed on Thread B while `_guard` is still live.
                get_scoped_traces()
            };
            // `_guard` is dropped here on Thread B inside `poll()`.
            let traces_after_guard_drop = get_scoped_traces();
            (traces_during_resume, traces_after_guard_drop)
        }));

        // 1. Poll once on Thread A: creates `_guard` and yields
        //    `Poll::Pending`.
        let mut cx = Context::from_waker(Waker::noop());
        assert!(matches!(fut.as_mut().poll(&mut cx), Poll::Pending));
        assert!(get_scoped_traces().is_empty());

        // 2. Move the suspended future to Thread B and poll it to completion
        //    there.
        let (traces_during_resume, traces_after_guard_drop, traces_on_thread_b_after_poll) =
            std::thread::spawn(move || {
                let (during, after_drop) = block_on(fut);
                let after_poll = get_scoped_traces();
                (during, after_drop, after_poll)
            })
            .join()
            .unwrap();

        assert_eq!(traces_during_resume.len(), 1);
        assert_eq!(traces_during_resume[0].message, "Trace across yield");
        assert!(traces_after_guard_drop.is_empty());
        assert!(traces_on_thread_b_after_poll.is_empty());
    }

    #[test]
    fn test_instrument_future_drop_mid_yield_preserves_outer_traces() {
        use std::future::Future;
        use std::pin::{pin, Pin};
        use std::task::{Context, Poll, Waker};

        struct PendingOnce(bool);
        impl Future for PendingOnce {
            type Output = ();
            fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
                if self.0 {
                    Poll::Ready(())
                } else {
                    self.0 = true;
                    Poll::Pending
                }
            }
        }

        let _outer = ScopedTraceGuard::new("Outer executor trace".to_string());
        {
            let mut fut = pin!(instrument_future(async {
                let _inner = ScopedTraceGuard::new("Inner future trace".to_string());
                PendingOnce(false).await;
            }));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(matches!(fut.as_mut().poll(&mut cx), Poll::Pending));
            // `fut` is dropped here while suspended with `_inner` still live
            // inside it.
        }

        let traces = get_scoped_traces();
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].message, "Outer executor trace");
    }

    #[test]
    fn test_instrument_future_panic_restores_outer_traces() {
        let _outer = ScopedTraceGuard::new("Outer trace before panic".to_string());

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            block_on(instrument_future(async {
                let _inner = ScopedTraceGuard::new("Inner trace before panic".to_string());
                panic!("Intentional async panic");
            }));
        }));

        let traces = get_scoped_traces();
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].message, "Outer trace before panic");
    }
}
