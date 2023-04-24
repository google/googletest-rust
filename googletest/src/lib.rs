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

//! A rich test assertion library for Rust.
//!
//! This library provides:
//!
//! * A framework for writing matchers which can be combined to make a wide
//!   range of assertions on data,
//! * A rich set of matchers, and
//! * A new set of test assertion macros.
//!
//! ## Assertions and matchers
//!
//! Most assertions are made through the macro [`verify_that!`] which evaluates
//! to a [`Result<()>`]. It takes two arguments: an actual value to be tested
//! and a [`Matcher`].
//!
//! Unlike the macros used in other test assertion libraries in Rust,
//! `verify_that!` does not panic when the test assertion fails. Instead, it
//! evaluates to `Result`, which the caller can choose to handle by:
//!
//! * Returning immediately from the function with the `?` operator (a *fatal*
//!   assertion), or
//! * Logging the failure, marking the test as failed, and allowing execution to
//!   continue (see [Non-fatal assertions](#non-fatal-assertions) below).
//!
//! For example, for fatal assertions:
//!
//! ```
//! use googletest::{matchers::eq, verify_that, Result};
//!
//! # /* The attribute macro would prevent the function from being compiled in a doctest.
//! #[test]
//! # */
//! fn more_than_one_failure() -> Result<()> {
//!    let value = 2;
//!    verify_that!(value, eq(4))?;  // Fails and ends execution of the test.
//!    verify_that!(value, eq(2)) // One can also just return the assertion result.
//! }
//! # more_than_one_failure().unwrap_err();
//! ```
//!
//! > In case one wants behaviour closer to other Rust test libraries, the macro
//! > [`assert_that!`] has the same parameters as [`verify_that!`] but panics on
//! > failure.
//!
//! Matchers are composable:
//!
//! ```
//! use googletest::{matchers::{contains, ge}, verify_that, Result};
//!
//! # /* The attribute macro would prevent the function from being compiled in a doctest.
//! #[test]
//! # */
//! fn contains_at_least_one_item_at_least_3() -> Result<()> {
//!    let value = vec![1, 2, 3];
//!    verify_that!(value, contains(ge(3)))
//! }
//! # contains_at_least_one_item_at_least_3().unwrap();
//! ```
//!
//! They can also be logically combined:
//!
//! ```
//! use googletest::{matchers::{ge, gt, not, AndMatcherExt}, verify_that, Result};
//!
//! # /* The attribute macro would prevent the function from being compiled in a doctest.
//! #[test]
//! # */
//! fn strictly_between_9_and_11() -> Result<()> {
//!    let value = 10;
//!    verify_that!(value, gt(9).and(not(ge(11))))
//! }
//! # strictly_between_9_and_11().unwrap();
//! ```
//!
//! ## Available matchers
//!
//! The following matchers are provided in GoogleTest Rust:
//!
//! | Matcher              | What it matches                                                          |
//! |----------------------|--------------------------------------------------------------------------|
//! | [`all!`]             | Anything matched by all given matchers.                                  |
//! | [`anything`]         | Any input.                                                               |
//! | [`and`]              | Anything matched by both matchers.                                       |
//! | [`approx_eq`]        | A floating point number within a standard tolerance of the argument.     |
//! | [`container_eq`]     | Same as [`eq`], but for containers (with a better mismatch description). |
//! | [`contains`]         | A container containing an element matched by the given matcher.          |
//! | [`contains_each!`]   | A container containing distinct elements each of the arguments match.    |
//! | [`contains_regex`]   | A string containing a substring matching the given regular expression.   |
//! | [`contains_substring`] | A string containing the given substring.                               |
//! | [`displays_as`]      | A [`Display`] value whose formatted string is matched by the argument.   |
//! | [`each`]             | A container all of whose elements the given argument matches.            |
//! | [`elements_are!`]    | A container whose elements the arguments match, in order.                |
//! | [`empty`]            | An empty collection.                                                     |
//! | [`ends_with`]        | A string ending with the given suffix.                                   |
//! | [`eq`]               | A value equal to the argument, in the sense of the [`PartialEq`] trait.  |
//! | [`err`]              | A [`Result`] containing an `Err` variant the argument matches.           |
//! | [`field!`]           | A struct or enum with a given field whose value the argument matches.    |
//! | [`ge`]               | A [`PartialOrd`] value greater than or equal to the given value.         |
//! | [`gt`]               | A [`PartialOrd`] value strictly greater than the given value.            |
//! | [`has_entry`]        | A [`HashMap`] containing a given key whose value the argument matches.   |
//! | [`is_contained_in!`] | A container each of whose elements is matched by some given matcher.     |
//! | [`is_nan`]           | A floating point number which is NaN.                                    |
//! | [`le`]               | A [`PartialOrd`] value less than or equal to the given value.            |
//! | [`lt`]               | A [`PartialOrd`] value strictly less than the given value.               |
//! | [`matches_pattern!`] | A struct or enum whose fields are matched according to the arguments.    |
//! | [`matches_regex`]    | A string matched by the given regular expression.                        |
//! | [`near`]             | A floating point number within a given tolerance of the argument.        |
//! | [`none`]             | An [`Option`] containing `None`.                                         |
//! | [`not`]              | Any value the argument does not match.                                   |
//! | [`ok`]               | A [`Result`] containing an `Ok` variant the argument matches.            |
//! | [`or`]               | Anything matched by either of the two given matchers.                    |
//! | [`pat!`]             | Alias for [`matches_pattern!`].                                          |
//! | [`points_to`]        | Any [`Deref`] such as `&`, `Rc`, etc. whose value the argument matches.  |
//! | [`pointwise!`]       | A container whose contents the arguments match in a pointwise fashion.   |
//! | [`predicate`]        | A value on which the given predicate returns true.                       |
//! | [`size`]             | A container whose size the argument matches.                             |
//! | [`some`]             | An [`Option`] containing `Some` whose value the argument matches.        |
//! | [`starts_with`]      | A string starting with the given prefix.                                 |
//! | [`subset_of`]        | A container all of whose elements are contained in the argument.         |
//! | [`superset_of`]      | A container containing all elements of the argument.                     |
//! | [`tuple!`]           | A tuple whose elements the arguments match.                              |
//! | [`unordered_elements_are!`] | A container whose elements the arguments match, in any order.     |
//!
//! [`anything`]: matchers::anything
//! [`and`]: matchers::AndMatcherExt::and
//! [`approx_eq`]: matchers::approx_eq
//! [`container_eq`]: matchers::container_eq
//! [`contains`]: matchers::contains
//! [`contains_regex`]: matchers::contains_regex
//! [`contains_substring`]: matchers::contains_substring
//! [`displays_as`]: matchers::displays_as
//! [`each`]: matchers::each
//! [`empty`]: matchers::empty
//! [`ends_with`]: matchers::ends_with
//! [`eq`]: matchers::eq
//! [`err`]: matchers::err
//! [`ge`]: matchers::ge
//! [`gt`]: matchers::gt
//! [`has_entry`]: matchers::has_entry
//! [`is_nan`]: matchers::is_nan
//! [`le`]: matchers::le
//! [`lt`]: matchers::lt
//! [`matches_regex`]: matchers::matches_regex
//! [`near`]: matchers::near
//! [`none`]: matchers::none
//! [`not`]: matchers::not
//! [`ok`]: matchers::ok
//! [`or`]: matchers::OrMatcherExt::or
//! [`points_to`]: matchers::points_to
//! [`predicate`]: matchers::predicate
//! [`size`]: matchers::size
//! [`some`]: matchers::some
//! [`starts_with`]: matchers::starts_with
//! [`subset_of`]: matchers::subset_of
//! [`superset_of`]: matchers::superset_of
//!
//! [`Deref`]: std::ops::Deref
//! [`Display`]: std::fmt::Display
//! [`HashMap`]: std::collections::HashMap
//! [`Option`]: std::option::Option
//! [`PartialEq`]: std::cmp::PartialEq
//! [`PartialOrd`]: std::cmp::PartialOrd
//! [`Result`]: std::result::Result
//!
//! ## Writing matchers
//!
//! One can extend the library by writing additional matchers. To do so, create
//! a struct holding the matcher's data and have it implement the trait
//! [`Matcher`]:
//!
//! ```no_run
//! use googletest::matcher::{Matcher, MatcherResult};
//! use std::fmt::Debug;
//!
//! struct MyEqMatcher<T> {
//!    expected: T,
//! }
//!
//! impl<T: PartialEq + Debug> Matcher<T> for MyEqMatcher<T> {
//!    fn matches(&self, actual: &T) -> MatcherResult {
//!        if self.expected == *actual {
//!            MatcherResult::Matches
//!        } else {
//!            MatcherResult::DoesNotMatch
//!        }
//!    }
//!
//!    fn describe(&self, matcher_result: MatcherResult) -> String {
//!        match matcher_result {
//!            MatcherResult::Matches => {
//!                format!("is equal to {:?} the way I define it", self.expected)
//!            }
//!            MatcherResult::DoesNotMatch => {
//!                format!("isn't equal to {:?} the way I define it", self.expected)
//!            }
//!        }
//!    }
//! }
//! ```
//!
//! It is recommended to expose a function which constructs the matcher:
//!
//! ```no_run
//! # use googletest::matcher::{Matcher, MatcherResult};
//! # use std::fmt::Debug;
//! #
//! # struct MyEqMatcher<T> {
//! #    expected: T,
//! # }
//! #
//! # impl<T: PartialEq + Debug> Matcher<T> for MyEqMatcher<T> {
//! #    fn matches(&self, actual: &T) -> MatcherResult {
//! #        if self.expected == *actual {
//! #            MatcherResult::Matches
//! #        } else {
//! #            MatcherResult::DoesNotMatch
//! #        }
//! #    }
//! #
//! #    fn describe(&self, matcher_result: MatcherResult) -> String {
//! #        match matcher_result {
//! #            MatcherResult::Matches => {
//! #                format!("is equal to {:?} the way I define it", self.expected)
//! #            }
//! #            MatcherResult::DoesNotMatch => {
//! #                format!("isn't equal to {:?} the way I define it", self.expected)
//! #            }
//! #        }
//! #    }
//! # }
//! #
//! pub fn eq_my_way<T: PartialEq + Debug>(expected: T) -> impl Matcher<T> {
//!    MyEqMatcher { expected }
//! }
//! ```
//!
//! The new matcher can then be used in `verify_that!`:
//!
//! ```
//! # use googletest::{matcher::{Matcher, MatcherResult}, verify_that, Result};
//! # use std::fmt::Debug;
//! #
//! # struct MyEqMatcher<T> {
//! #    expected: T,
//! # }
//! #
//! # impl<T: PartialEq + Debug> Matcher<T> for MyEqMatcher<T> {
//! #    fn matches(&self, actual: &T) -> MatcherResult {
//! #        if self.expected == *actual {
//! #            MatcherResult::Matches
//! #        } else {
//! #            MatcherResult::DoesNotMatch
//! #        }
//! #    }
//! #
//! #    fn describe(&self, matcher_result: MatcherResult) -> String {
//! #        match matcher_result {
//! #            MatcherResult::Matches => {
//! #                format!("is equal to {:?} the way I define it", self.expected)
//! #            }
//! #            MatcherResult::DoesNotMatch => {
//! #                format!("isn't equal to {:?} the way I define it", self.expected)
//! #            }
//! #        }
//! #    }
//! # }
//! #
//! # pub fn eq_my_way<T: PartialEq + Debug>(expected: T) -> impl Matcher<T> {
//! #    MyEqMatcher { expected }
//! # }
//! # /* The attribute macro would prevent the function from being compiled in a doctest.
//! #[test]
//! # */
//! fn should_be_equal_by_my_definition() -> Result<()> {
//!    verify_that!(10, eq_my_way(10))
//! }
//! # should_be_equal_by_my_definition().unwrap();
//! ```
//!
//! ## Non-fatal assertions
//!
//! Using non-fatal assertions, a single test is able to log multiple assertion
//! failures. Any single assertion failure causes the test to be considered
//! having failed, but execution continues until the test completes or otherwise
//! aborts.
//!
//! To make a non-fatal assertion, use the macro [`expect_that!`]. The test must
//! also be marked with [`googletest::test`][test] instead of the Rust-standard
//! `#[test]`. It must return [`Result<()>`].
//!
//! ```no_run
//! use googletest::{expect_that, verify_that, matchers::eq, Result};
//!
//! # /* Make sure this also compiles as a doctest.
//! #[googletest::test]
//! # */
//! fn more_than_one_failure() -> Result<()> {
//!    let value = 2;
//!    expect_that!(value, eq(3));  // Just marks the test as having failed.
//!    verify_that!(value, eq(2))?;  // Passes, but the test already failed.
//!    Ok(())
//! }
//! ```
//!
//! ## Predicate assertions
//!
//! The macro [`verify_pred!`] provides predicate assertions analogous to
//! GoogleTest's `EXPECT_PRED` family of macros. Wrap an invocation of a
//! predicate in a `verify_pred!` invocation to turn that into a test assertion
//! which passes precisely when the predicate returns `true`:
//!
//! ```
//! # use googletest::{verify_pred, Result};
//! fn stuff_is_correct(x: i32, y: i32) -> bool {
//!    x == y
//! }
//!
//! # fn run_test() -> Result<()> {
//! let x = 3;
//! let y = 4;
//! verify_pred!(stuff_is_correct(x, y))?;
//! # Ok(())
//! # }
//! # run_test().unwrap_err();
//! ```
//!
//! The assertion failure message shows the arguments and the values to which
//! they evaluate:
//!
//! ```text
//! stuff_is_correct(x, y) was false with
//!  x = 3,
//!  y = 4
//! ```
//!
//! The `verify_pred!` invocation evaluates to a [`Result<()>`] just like
//! [`verify_that!`]. There is also a macro [`expect_pred!`] to make a non-fatal
//! predicaticate assertion.
//!
//! ## Unconditionally generating a test failure
//!
//! The macro [`fail!`] unconditionally evaluates to a `Result` indicating a
//! test failure. It can be used analogously to [`verify_that!`] and
//! [`verify_pred!`] to cause a test to fail, with an optional formatted
//! message:
//!
//! ```
//! # use googletest::{fail, Result};
//! # /* The attribute macro would prevent the function from being compiled in a doctest.
//! #[test]
//! # */
//! fn always_fails() -> Result<()> {
//!    fail!("This test must fail with {}", "today")
//! }
//! # always_fails().unwrap_err();
//! ```
//!
//! [`and_log_failure()`]: GoogleTestSupport::and_log_failure
//! [`Matcher`]: matcher::Matcher

extern crate googletest_macro;

#[macro_use]
pub mod assertions;
pub mod internal;
pub mod matcher;
#[cfg(not(google3))]
pub mod matchers;

pub use googletest_macro::test;

// For backwards compatibility.
#[deprecated(since = "0.5.0", note = "Use googletest::test instead")]
pub use googletest_macro::test as google_test;

use internal::test_outcome::TestAssertionFailure;

/// A `Result` whose `Err` variant indicates a test failure.
///
/// All test functions should return `Result<()>`.
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

/// Adds to `Result` support for GoogleTest Rust functionality.
pub trait GoogleTestSupport {
    /// If `self` is a `Result::Err`, writes to `stdout` a failure report
    /// and marks the test failed. Otherwise, does nothing.
    ///
    /// This can be used for non-fatal test assertions, for example:
    ///
    /// ```
    /// # use googletest::{matchers::eq, verify_that, GoogleTestSupport, Result};
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
    /// ```
    /// # use googletest::{
    /// #     matchers::{contains_substring, displays_as, err, eq},
    /// #     verify_that,
    /// #     GoogleTestSupport,
    /// #     Result
    /// # };
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
    /// # use googletest::{
    /// #     matchers::{contains_substring, displays_as, err, eq},
    /// #     verify_that,
    /// #     GoogleTestSupport,
    /// #     Result
    /// # };
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
    /// # use googletest::{
    /// #     matchers::{contains_substring, displays_as, err, eq},
    /// #     verify_that,
    /// #     GoogleTestSupport,
    /// #     Result
    /// # };
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
