// Copyright 2022 Google LLC
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

use std::cell::{RefCell, RefMut};
use std::fmt::{Debug, Display, Error, Formatter};
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::sync::Arc;
use std::thread_local;

/// The outcome hitherto of running a test.
///
/// This is kept as a running record as the test progresses. One can access it
/// with `TestOutcome::with_current_test_outcome`.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub struct TestOutcome {
    is_success: AtomicBool,
}

impl Default for TestOutcome {
    fn default() -> Self {
        Self::new()
    }
}

impl TestOutcome {
    pub fn new() -> Self {
        Self { is_success: AtomicBool::new(true) }
    }
    pub fn fail(&self) {
        self.is_success.store(false, AtomicOrdering::Relaxed)
    }
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.is_success.load(AtomicOrdering::Relaxed)
    }
    #[must_use]
    pub fn is_failed(&self) -> bool {
        self.is_success.load(AtomicOrdering::Relaxed)
    }
}

thread_local! {
    // Whether or not the current test has failed.
    //
    // If inside a `#[gtest]` function, this value will initially be set to a new `TestOutcome`.
    // Upon assertion failure (e.g. `expect_that!` failing), the `TestOutcome` will be updated to
    // indicate failure.
    //
    // The `Arc` is used to share the `TestOutcome` across threads that have been spawned by the
    // `#[gtest]` function, which can then set it to fail upon an assertion failure in a thread.
    static CURRENT_TEST_OUTCOME: RefCell<Option<Arc<TestOutcome>>> = const { RefCell::new(None) };
    #[cfg(feature = "unstable_thread_spawn_hook")]
    static HAS_SET_SPAWN_HOOK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

impl TestOutcome {
    /// Resets the current test's [`TestOutcome`].
    ///
    /// This is intended only for use by the attribute macro `#[gtest]`.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn init_current_test_outcome() {
        Self::with_current_test_outcome(|mut current_test_outcome| {
            *current_test_outcome = Some(Arc::new(TestOutcome::new()));
        });

        #[cfg(feature = "unstable_thread_spawn_hook")]
        if !HAS_SET_SPAWN_HOOK.get() {
            // Ensure that the spawn hook is only set once so that we don't accumulate spawn
            // hooks for threads that run multiple tests.
            HAS_SET_SPAWN_HOOK.set(true);
            std::thread::add_spawn_hook(|_thread| {
                let outcome: Option<Arc<TestOutcome>> =
                    Self::with_current_test_outcome(|current_test_outcome| {
                        current_test_outcome.clone()
                    });
                move || {
                    Self::with_current_test_outcome(|mut current_test_outcome| {
                        *current_test_outcome = outcome;
                    });
                }
            })
        }
    }

    /// Evaluates the current test's [`TestOutcome`], producing a suitable
    /// `Result`.
    ///
    /// The parameter `result` is the value returned by the test function
    /// itself. This returns `Result::Err` with a `Display`-formatted string of
    /// the error if `result` is `Result::Err`.
    ///
    /// Otherwise, this returns `Result::Err` precisely when a test failure has
    /// been recorded with
    /// [`and_log_failure`](crate::GoogleTestSupport::and_log_failure).
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn close_current_test_outcome<E: Display>(
        test_return_value: Result<(), E>,
    ) -> Result<(), TestFailure> {
        TestOutcome::with_current_test_outcome(|mut outcome_arc| {
            let Some(outcome) = outcome_arc.as_ref() else {
                panic!("No test context found. This indicates a bug in GoogleTest.")
            };
            let outer_result = if outcome.is_success() && test_return_value.is_ok() {
                Ok(())
            } else {
                Err(TestFailure)
            };
            if let Err(fatal_assertion_failure) = test_return_value {
                println!("{fatal_assertion_failure}");
            }
            *outcome_arc = None;
            outer_result
        })
    }

    /// Returns a `Result` corresponding to the outcome of the currently running
    /// test.
    #[track_caller]
    pub(crate) fn get_current_test_outcome() -> Result<(), TestAssertionFailure> {
        TestOutcome::with_current_test_outcome(|mut outcome| {
            let is_success = outcome
                .as_mut()
                .expect("No test context found. This indicates a bug in GoogleTest.")
                .is_success();
            if is_success {
                Ok(())
            } else {
                Err(TestAssertionFailure::create("Test failed".into()))
            }
        })
    }

    /// Records that the currently running test has failed.
    fn fail_current_test() {
        TestOutcome::with_current_test_outcome(|mut outcome| {
            outcome
                .as_mut()
                .expect("No test context found. This indicates a bug in GoogleTest.")
                .fail();
        })
    }

    /// Runs `action` with the [`TestOutcome`] for the currently running test.
    ///
    /// This is primarily intended for use by assertion macros like
    /// `expect_that!`.
    fn with_current_test_outcome<T>(
        action: impl FnOnce(RefMut<Option<Arc<TestOutcome>>>) -> T,
    ) -> T {
        CURRENT_TEST_OUTCOME.with(|current_test_outcome| action(current_test_outcome.borrow_mut()))
    }

    /// Ensure that there is a test context present and panic if there is not.
    pub(crate) fn ensure_test_context_present() {
        TestOutcome::with_current_test_outcome(|outcome| {
            outcome.as_ref().expect(
                "
No test context found.
 * Did you annotate the test with gtest?
 * Is the assertion running in the original test thread?
",
            );
        })
    }
}

/// A marking struct indicating that a test has failed.
///
/// This exists to implement the [Error][std::error::Error] trait. It displays
/// to a message indicating that the actual test assertion failure messages are
/// in the text above.
pub struct TestFailure;

impl std::error::Error for TestFailure {}

impl std::fmt::Debug for TestFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "See failure output above")?;
        Ok(())
    }
}

impl std::fmt::Display for TestFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "See failure output above")?;
        Ok(())
    }
}

/// A report that a single test assertion failed.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
#[derive(Clone)]
pub struct TestAssertionFailure {
    /// A human-readable formatted string describing the error.
    pub description: String,
    pub custom_message: Option<String>,
    location: Location,
}

/// A code location.
///
/// `std::panic::Location` does not provide a constructor, hence we cannot
/// construct a fake value.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
#[derive(Clone)]
enum Location {
    Real(&'static std::panic::Location<'static>),
    Fake { file: &'static str, line: u32, column: u32 },
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Real(l) => write!(f, "{l}"),
            Location::Fake { file, line, column } => write!(f, "{file}:{line}:{column}"),
        }
    }
}

impl TestAssertionFailure {
    /// Creates a new instance with the given `description`.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[track_caller]
    pub fn create(description: String) -> Self {
        Self {
            description,
            custom_message: None,
            location: Location::Real(std::panic::Location::caller()),
        }
    }

    /// Set `location`` to a fake value.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    pub fn with_fake_location(mut self, file: &'static str, line: u32, column: u32) -> Self {
        self.location = Location::Fake { file, line, column };
        self
    }

    pub(crate) fn log(&self) {
        TestOutcome::fail_current_test();
        println!("{self}");
    }
}

impl Display for TestAssertionFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{}", self.description)?;
        if let Some(custom_message) = &self.custom_message {
            writeln!(f, "{custom_message}")?;
        }
        writeln!(f, "  at {}", self.location)
    }
}

// The standard Rust test harness outputs the TestAssertionFailure with the
// Debug trait. We want the output to be formatted, so we use a custom Debug
// implementation which defers to Display.
impl Debug for TestAssertionFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(self, f)
    }
}

impl<T: std::error::Error> From<T> for TestAssertionFailure {
    #[track_caller]
    fn from(value: T) -> Self {
        TestAssertionFailure::create(format!("{value}"))
    }
}

#[cfg(feature = "proptest")]
impl From<TestAssertionFailure> for proptest::test_runner::TestCaseError {
    fn from(value: TestAssertionFailure) -> Self {
        proptest::test_runner::TestCaseError::Fail(format!("{value}").into())
    }
}
