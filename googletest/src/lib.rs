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

extern crate googletest_macro;

#[macro_use]
pub mod assertions;
pub mod internal;
pub mod matcher;
#[cfg(not(google3))]
pub mod matchers;

pub use googletest_macro::google_test;

use internal::test_outcome::TestAssertionFailure;

/// A `Result` whose `Err` variant indicates a test failure.
///
/// All test functions should return `Result<()>`.
///
/// This can be used with subroutines which may cause the test to fatally fail
/// and which return some value needed by the caller. For example:
///
/// ```rust
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

/// Adds to `Result` support for GoogleTest Rust functionality.
pub trait GoogleTestSupport {
    /// If `self` is a `Result::Err`, writes to `stdout` a failure report
    /// and marks the test failed. Otherwise, does nothing.
    ///
    /// This can be used for non-fatal test assertions, for example:
    ///
    /// ```rust
    /// let actual = 42;
    /// verify_that!(actual, eq(42)).and_log_failure();
    ///                                  // Test still passing; nothing happens
    /// verify_that!(actual, eq(10)).and_log_failure();
    ///                          // Test now fails and failure output to stdout
    /// verify_that!(actual, eq(100)).and_log_failure();
    ///               // Test still fails and new failure also output to stdout
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
    /// ```rust
    /// let actual = 0;
    /// verify_that!(actual, eq(42)).failure_message("Actual was wrong!")?;
    /// ```
    ///
    /// results in the following failure message:
    ///
    /// ```
    /// Expected: actual equal to 42
    ///   but was: 0
    /// Actual was wrong!
    /// ```
    ///
    /// One can pass a `String` too:
    ///
    /// ```rust
    /// let actual = 0;
    /// verify_that!(actual, eq(42))
    ///    .failure_message(format!("Actual {} was wrong!", actual))?;
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
    /// ```rust
    /// let actual = 0;
    /// verify_that!(actual, eq(42))
    ///    .with_failure_message(|| format!("Actual {} was wrong!", actual))?;
    /// ```
    fn with_failure_message(self, provider: impl FnOnce() -> String) -> Self;
}

impl<T> GoogleTestSupport for std::result::Result<T, TestAssertionFailure> {
    fn and_log_failure(self) {
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
