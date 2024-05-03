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
use std::thread_local;

/// The outcome hitherto of running a test.
///
/// This is kept as a running record as the test progresses. One can access it
/// with `TestOutcome::with_current_test_outcome`.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub enum TestOutcome {
    /// The test ran or is currently running and no assertions have failed.
    Success,
    /// The test ran or is currently running and at least one assertion has
    /// failed.
    Failure,
}

thread_local! {
    static CURRENT_TEST_OUTCOME: RefCell<Option<TestOutcome>> = const { RefCell::new(None) };
}

impl TestOutcome {
    /// Resets the current test's [`TestOutcome`].
    ///
    /// This is intended only for use by the attribute macro
    /// `#[googletest::test]`.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn init_current_test_outcome() {
        Self::with_current_test_outcome(|mut current_test_outcome| {
            *current_test_outcome = Some(TestOutcome::Success);
        })
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
        inner_result: Result<(), E>,
    ) -> Result<(), TestFailure> {
        TestOutcome::with_current_test_outcome(|mut outcome| {
            let outer_result = match &*outcome {
                Some(TestOutcome::Success) => match inner_result {
                    Ok(()) => Ok(()),
                    Err(_) => Err(TestFailure),
                },
                Some(TestOutcome::Failure) => Err(TestFailure),
                None => {
                    panic!("No test context found. This indicates a bug in GoogleTest.")
                }
            };
            if let Err(fatal_assertion_failure) = inner_result {
                println!("{fatal_assertion_failure}");
            }
            *outcome = None;
            outer_result
        })
    }

    /// Returns a `Result` corresponding to the outcome of the currently running
    /// test.
    pub(crate) fn get_current_test_outcome() -> Result<(), TestAssertionFailure> {
        TestOutcome::with_current_test_outcome(|mut outcome| {
            let outcome = outcome
                .as_mut()
                .expect("No test context found. This indicates a bug in GoogleTest.");
            match outcome {
                TestOutcome::Success => Ok(()),
                TestOutcome::Failure => Err(TestAssertionFailure::create("Test failed".into())),
            }
        })
    }

    /// Records that the currently running test has failed.
    fn fail_current_test() {
        TestOutcome::with_current_test_outcome(|mut outcome| {
            let outcome = outcome
                .as_mut()
                .expect("No test context found. This indicates a bug in GoogleTest.");
            *outcome = TestOutcome::Failure;
        })
    }

    /// Runs `action` with the [`TestOutcome`] for the currently running test.
    ///
    /// This is primarily intended for use by assertion macros like
    /// `expect_that!`.
    fn with_current_test_outcome<T>(action: impl FnOnce(RefMut<Option<TestOutcome>>) -> T) -> T {
        CURRENT_TEST_OUTCOME.with(|current_test_outcome| action(current_test_outcome.borrow_mut()))
    }

    /// Ensure that there is a test context present and panic if there is not.
    pub(crate) fn ensure_test_context_present() {
        TestOutcome::with_current_test_outcome(|outcome| {
            outcome.as_ref().expect(
                "
No test context found.
 * Did you annotate the test with googletest::test?
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
}

impl TestAssertionFailure {
    /// Creates a new instance with the given `description`.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    pub fn create(description: String) -> Self {
        Self { description, custom_message: None }
    }

    pub(crate) fn log(&self) {
        TestOutcome::fail_current_test();
        println!("{}", self);
    }
}

impl Display for TestAssertionFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{}", self.description)?;
        if let Some(custom_message) = &self.custom_message {
            writeln!(f, "{}", custom_message)?;
        }
        Ok(())
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
