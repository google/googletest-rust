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

static NEXT_TRACE_ID: AtomicUsize = AtomicUsize::new(0);

/// Information about a scoped trace.
#[derive(Clone, Debug)]
pub struct TraceInfo {
    pub id: usize,
    pub file: &'static str,
    pub line: u32,
    pub message: String,
}

thread_local! {
    static TRACE_STACK: RefCell<Vec<TraceInfo>> = const { RefCell::new(Vec::new()) };
}

/// RAII guard to manage the push and pop of trace information.
///
/// This struct is `!Send` and `!Sync` to prevent it from being held across
/// `.await` points in async tests, which would cause incorrect trace tracking
/// if the task moves between threads.
#[doc(hidden)]
pub struct ScopedTraceGuard {
    id: usize,
    _phantom: std::marker::PhantomData<*mut ()>,
}

impl ScopedTraceGuard {
    #[doc(hidden)]
    #[track_caller]
    pub fn new(message: String) -> Self {
        let caller = std::panic::Location::caller();
        let id = NEXT_TRACE_ID.fetch_add(1, Ordering::Relaxed);
        TRACE_STACK.with(|stack| {
            // Use try_borrow_mut to avoid double panic if called during unwinding.
            if let Ok(mut s) = stack.try_borrow_mut() {
                s.push(TraceInfo { id, file: caller.file(), line: caller.line(), message });
            }
        });
        Self { id, _phantom: std::marker::PhantomData }
    }
}

impl Drop for ScopedTraceGuard {
    fn drop(&mut self) {
        TRACE_STACK.with(|stack| {
            // Use try_borrow_mut to avoid double panic if called during unwinding.
            if let Ok(mut s) = stack.try_borrow_mut() {
                if let Some(pos) = s.iter().rposition(|t| t.id == self.id) {
                    s.remove(pos);
                }
            }
        });
    }
}

/// Retrieves a clone of the current thread's trace stack.
pub fn get_scoped_traces() -> Vec<TraceInfo> {
    TRACE_STACK.with(|stack| stack.try_borrow().map(|s| s.clone()).unwrap_or_default())
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
