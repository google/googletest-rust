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
    static CURRENT_TEST_OUTCOME: RefCell<TestOutcome> =
        RefCell::new(TestOutcome::Success);
}

impl TestOutcome {
    /// Resets the current test's [`TestOutcome`].
    ///
    /// This is intended only for use by the attribute macro `#[google_test]`.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn init_current_test_outcome() {
        Self::with_current_test_outcome(|mut current_test_outcome| {
            *current_test_outcome = TestOutcome::Success;
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
    pub fn close_current_test_outcome<E: Display>(result: Result<(), E>) -> Result<(), ()> {
        TestOutcome::with_current_test_outcome(|outcome| match &*outcome {
            TestOutcome::Success => match result {
                Ok(()) => Ok(()),
                Err(f) => {
                    print!("{}", f);
                    Err(())
                }
            },
            TestOutcome::Failure => Err(()),
        })
    }

    /// Records that the currently running test has failed.
    fn fail_current_test() {
        TestOutcome::with_current_test_outcome(|mut outcome| {
            *outcome = TestOutcome::Failure;
        })
    }

    /// Runs `action` with the [`TestOutcome`] for the currently running test.
    ///
    /// This is primarily intended for use by assertion macros like
    /// `verify_that!`.
    fn with_current_test_outcome<T>(action: impl FnOnce(RefMut<TestOutcome>) -> T) -> T {
        CURRENT_TEST_OUTCOME.with(|current_test_outcome| action(current_test_outcome.borrow_mut()))
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
        print!("{}", self);
    }

    pub(crate) fn from_error<T: std::fmt::Display>(err: T) -> TestAssertionFailure {
        TestAssertionFailure::create(format!("{}", err))
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
