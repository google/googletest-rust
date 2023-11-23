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

#![doc = include_str!("../crate_docs.md")]

extern crate googletest_macro;

#[cfg(test)]
extern crate quickcheck;

#[macro_use]
pub mod assertions;
pub mod description;
pub mod internal;
pub mod matcher;
pub mod matcher_support;
pub mod matchers;

/// Re-exports of the symbols in this crate which are most likely to be used.
///
/// This includes:
///  * All assertion macros,
///  * Traits and type definitions normally used by tests, and
///  * All built-in matchers.
///
/// Typically, one imports everything in the prelude in one's test module:
///
/// ```
/// mod tests {
///     use googletest::prelude::*;
/// }
/// ```
pub mod prelude {
    pub use super::matcher::Matcher;
    pub use super::matchers::*;
    pub use super::verify_current_test_outcome;
    pub use super::GoogleTestSupport;
    pub use super::IntoTestResult;
    pub use super::Result;
    // Assert macros
    pub use super::{assert_that, expect_pred, expect_that, fail, verify_pred, verify_that};
}

pub use googletest_macro::test;

use internal::test_outcome::{TestAssertionFailure, TestOutcome};

/// A `Result` whose `Err` variant indicates a test failure.
///
/// The assertions [`verify_that!`][crate::verify_that],
/// [`verify_pred!`][crate::verify_pred], and [`fail!`][crate::fail] evaluate
/// to `Result<()>`. A test function may return `Result<()>` in combination with
/// those macros to abort immediately on assertion failure.
///
/// This can be used with subroutines which may cause the test to fatally fail
/// and which return some value needed by the caller. For example:
///
/// ```ignore
/// fn load_file_content_as_string() -> Result<String> {
///     let file_stream = load_file().err_to_test_failure()?;
///     Ok(file_stream.to_string())
/// }
/// ```
///
/// The `Err` variant contains a [`TestAssertionFailure`] which carries the data
/// of the (fatal) assertion failure which generated this result. Non-fatal
/// assertion failures, which log the failure and report the test as having
/// failed but allow it to continue running, are not encoded in this type.
pub type Result<T> = std::result::Result<T, TestAssertionFailure>;

/// Returns a [`Result`] corresponding to the outcome of the currently running
/// test.
///
/// This returns `Result::Err` precisely if the current test has recorded at
/// least one test assertion failure via [`expect_that!`][crate::expect_that],
/// [`expect_pred!`][crate::expect_pred], or
/// [`GoogleTestSupport::and_log_failure`]. It can be used in concert with the
/// `?` operator to continue execution of the test conditionally on there not
/// having been any failure yet.
///
/// This requires the use of the [`#[googletest::test]`][crate::test] attribute
/// macro.
///
/// ```
/// # use googletest::prelude::*;
/// # /* Make sure this also compiles as a doctest.
/// #[googletest::test]
/// # */
/// # fn foo() -> u32 { 1 }
/// # fn bar() -> u32 { 2 }
/// fn should_fail_and_not_execute_last_assertion() -> Result<()> {
/// #   googletest::internal::test_outcome::TestOutcome::init_current_test_outcome();
///     expect_that!(foo(), eq(2));     // May fail, but will not abort the test.
///     expect_that!(bar(), gt(1));     // May fail, but will not abort the test.
///     verify_current_test_outcome()?; // Aborts the test if one of the previous assertions failed.
///     verify_that!(foo(), gt(0))      // Does not execute if the line above aborts.
/// }
/// # verify_that!(should_fail_and_not_execute_last_assertion(), err(displays_as(contains_substring("Test failed")))).unwrap();
/// ```
pub fn verify_current_test_outcome() -> Result<()> {
    TestOutcome::get_current_test_outcome()
}

/// Adds to `Result` support for GoogleTest Rust functionality.
pub trait GoogleTestSupport {
    /// If `self` is a `Result::Err`, writes to `stdout` a failure report
    /// and marks the test failed. Otherwise, does nothing.
    ///
    /// This can be used for non-fatal test assertions, for example:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # use googletest::internal::test_outcome::TestOutcome;
    /// # TestOutcome::init_current_test_outcome();
    /// let actual = 42;
    /// verify_that!(actual, eq(42)).and_log_failure();
    ///                                  // Test still passing; nothing happens
    /// verify_that!(actual, eq(10)).and_log_failure();
    ///                          // Test now fails and failure output to stdout
    /// verify_that!(actual, eq(100)).and_log_failure();
    ///               // Test still fails and new failure also output to stdout
    /// # TestOutcome::close_current_test_outcome::<&str>(Ok(())).unwrap_err();
    /// ```
    fn and_log_failure(self);

    /// Adds `message` to the logged failure message if `self` is a
    /// `Result::Err`. Otherwise, does nothing.
    ///
    /// If this method is called more than once, only `message` from the last
    /// invocation is output.
    ///
    /// For example:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_fail() -> Result<()> {
    /// let actual = 0;
    /// verify_that!(actual, eq(42)).failure_message("Actual was wrong!")?;
    /// # Ok(())
    /// # }
    /// # verify_that!(should_fail(), err(displays_as(contains_substring("Actual was wrong"))))
    /// #     .unwrap();
    /// ```
    ///
    /// results in the following failure message:
    ///
    /// ```text
    /// Expected: actual equal to 42
    ///   but was: 0
    /// Actual was wrong!
    /// ```
    ///
    /// One can pass a `String` too:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_fail() -> Result<()> {
    /// let actual = 0;
    /// verify_that!(actual, eq(42))
    ///    .failure_message(format!("Actual {} was wrong!", actual))?;
    /// # Ok(())
    /// # }
    /// # verify_that!(should_fail(), err(displays_as(contains_substring("Actual 0 was wrong"))))
    /// #     .unwrap();
    /// ```
    ///
    /// However, consider using [`GoogleTestSupport::with_failure_message`]
    /// instead in that case to avoid unnecessary memory allocation when the
    /// message is not needed.
    fn failure_message(self, message: impl Into<String>) -> Self;

    /// Adds the output of the closure `provider` to the logged failure message
    /// if `self` is a `Result::Err`. Otherwise, does nothing.
    ///
    /// This is analogous to [`GoogleTestSupport::failure_message`] but
    /// only executes the closure `provider` if it actually produces the
    /// message, thus saving possible memory allocation.
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_fail() -> Result<()> {
    /// let actual = 0;
    /// verify_that!(actual, eq(42))
    ///    .with_failure_message(|| format!("Actual {} was wrong!", actual))?;
    /// # Ok(())
    /// # }
    /// # verify_that!(should_fail(), err(displays_as(contains_substring("Actual 0 was wrong"))))
    /// #     .unwrap();
    /// ```
    fn with_failure_message(self, provider: impl FnOnce() -> String) -> Self;
}

impl<T> GoogleTestSupport for std::result::Result<T, TestAssertionFailure> {
    fn and_log_failure(self) {
        TestOutcome::ensure_text_context_present();
        if let Err(failure) = self {
            failure.log();
        }
    }

    fn failure_message(mut self, message: impl Into<String>) -> Self {
        if let Err(ref mut failure) = self {
            failure.custom_message = Some(message.into());
        }
        self
    }

    fn with_failure_message(mut self, provider: impl FnOnce() -> String) -> Self {
        if let Err(ref mut failure) = self {
            failure.custom_message = Some(provider());
        }
        self
    }
}

/// Provides an extension method for converting an arbitrary type into a
/// [`Result`].
///
/// A type can implement this trait to provide an easy way to return immediately
/// from a test in conjunction with the `?` operator. This is useful for
/// [`Result`][std::result::Result] types whose `Result::Err` variant does not
/// implement [`std::error::Error`].
///
/// There is an implementation of this trait for [`anyhow::Error`] (which does
/// not implement `std::error::Error`) when the `anyhow` feature is enabled.
/// Importing this trait allows one to easily map [`anyhow::Error`] to a test
/// failure:
///
/// ```ignore
/// #[test]
/// fn should_work() -> Result<()> {
///     let value = something_which_can_fail().into_test_result()?;
///     ...
/// }
///
/// fn something_which_can_fail() -> anyhow::Result<...> { ... }
/// ```
pub trait IntoTestResult<T> {
    /// Converts this instance into a [`Result`].
    ///
    /// Typically, the `Self` type is itself a [`std::result::Result`]. This
    /// method should then map the `Err` variant to a [`TestAssertionFailure`]
    /// and leave the `Ok` variant unchanged.
    fn into_test_result(self) -> Result<T>;
}

#[cfg(feature = "anyhow")]
impl<T> IntoTestResult<T> for std::result::Result<T, anyhow::Error> {
    fn into_test_result(self) -> std::result::Result<T, TestAssertionFailure> {
        self.map_err(|e| TestAssertionFailure::create(format!("{e}")))
    }
}

#[cfg(feature = "proptest")]
impl<OkT, CaseT: std::fmt::Debug> IntoTestResult<OkT>
    for std::result::Result<OkT, proptest::test_runner::TestError<CaseT>>
{
    fn into_test_result(self) -> std::result::Result<OkT, TestAssertionFailure> {
        self.map_err(|e| TestAssertionFailure::create(format!("{e}")))
    }
}
