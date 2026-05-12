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
/// This struct is `Send` to allow it to be used in async tests, but it should
/// not be held across `.await` points directly unless the future is
/// instrumented.
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
        TRACE_STACK.with(|stack| {
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

/// A future that instruments another future with a set of traces.
pub struct InstrumentedFuture<F> {
    inner: F,
    traces: Option<Arc<TraceNode>>,
}

impl<F> InstrumentedFuture<F> {
    pub fn new(inner: F) -> Self {
        Self { inner, traces: None }
    }
}

impl<F: std::future::Future> std::future::Future for InstrumentedFuture<F> {
    type Output = F::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // SAFETY: `InstrumentedFuture` provides structural pinning. If `self` is
        // pinned, it is safe to pin the `inner` future.
        let this = unsafe { self.get_unchecked_mut() };

        struct TraceSwapGuard<'a>(&'a mut Option<Arc<TraceNode>>);
        impl<'a> TraceSwapGuard<'a> {
            fn new(traces: &'a mut Option<Arc<TraceNode>>) -> Self {
                TRACE_STACK.with(|stack| {
                    if let Ok(mut s) = stack.try_borrow_mut() {
                        std::mem::swap(&mut *s, traces);
                    }
                });
                Self(traces)
            }
        }
        impl<'a> Drop for TraceSwapGuard<'a> {
            fn drop(&mut self) {
                TRACE_STACK.with(|stack| {
                    if let Ok(mut s) = stack.try_borrow_mut() {
                        std::mem::swap(&mut *s, self.0);
                    }
                });
            }
        }

        // Swap traces into thread-local, ensuring they are swapped back on
        // return/panic.
        let _guard = TraceSwapGuard::new(&mut this.traces);

        // Poll inner future
        // SAFETY: As explained above, `this.inner` is properly pinned.
        unsafe { std::pin::Pin::new_unchecked(&mut this.inner) }.poll(cx)
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
}
