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
//! ```rust
//! use googletest::{matchers::eq, verify_that, Result};
//!
//! #[test]
//! fn more_than_one_failure() -> Result<()> {
//!    let value = 2;
//!    verify_that!(value, eq(4))?;  // Fails and ends execution of the test.
//!    verify_that!(value, eq(2)) // One can also just return the assertion result.
//! }
//! ```
//!
//! > In case one wants behaviour closer to other Rust test libraries, the macro
//! > [`assert_that!`] has the same parameters as [`verify_that!`] but panics on
//! > failure.
//!
//! Matchers are composable:
//!
//! ```rust
//! use googletest::{matchers::{contains, ge}, verify_that, Result};
//!
//! #[test]
//! fn contains_at_least_one_item_at_least_3() -> Result<()> {
//!    let value = vec![1, 2, 3];
//!    verify_that!(value, contains(ge(3)))
//! }
//! ```
//!
//! They can also be logically combined:
//!
//! ```rust
//! use googletest::{matchers::{gt, lt, not, AndMatcherExt}, verify_that, Result};
//!
//! #[test]
//! fn strictly_between_9_and_11() -> Result<()> {
//!    let value = 10;
//!    verify_that!(value, gt(9).and(not(ge(11))))
//! }
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
//! [`all!`]: macro.all.html
//! [`anything`]: matchers/anything_matcher/fn.anything.html
//! [`and`]: matchers/conjunction_matcher/trait.AndMatcherExt.html#method.and
//! [`approx_eq`]: matchers/near_matcher/fn.approx_eq.html
//! [`container_eq`]: matchers/container_eq_matcher/fn.container_eq.html
//! [`contains`]: matchers/contains_matcher/fn.contains.html
//! [`contains_each!`]: macro.contains_each.html
//! [`contains_regex`]: matchers/contains_regex_matcher/fn.contains_regex.html
//! [`contains_substring`]: matchers/str_matcher/fn.contains_substring.html
//! [`displays_as`]: matchers/display_matcher/fn.displays_as.html
//! [`each`]: matchers/each_matcher/fn.each.html
//! [`elements_are!`]: macro.elements_are.html
//! [`empty`]: matchers/empty_matcher/fn.empty.html
//! [`ends_with`]: matchers/str_matcher/fn.ends_with.html
//! [`eq`]: matchers/eq_matcher/fn.eq.html
//! [`err`]: matchers/err_matcher/fn.err.html
//! [`field!`]: macro.field.html
//! [`ge`]: matchers/ge_matcher/fn.ge.html
//! [`gt`]: matchers/gt_matcher/fn.gt.html
//! [`has_entry`]: matchers/has_entry_matcher/fn.has_entry.html
//! [`is_contained_in!`]: macro.is_contained_in.html
//! [`is_nan`]: matchers/is_nan_matcher/fn.is_nan.html
//! [`le`]: matchers/le_matcher/fn.le.html
//! [`lt`]: matchers/lt_matcher/fn.lt.html
//! [`matches_pattern!`]: macro.matches_pattern.html
//! [`matches_regex`]: matchers/matches_regex_matcher/fn.matches_regex.html
//! [`near`]: matchers/near_matcher/fn.near.html
//! [`none`]: matchers/none_matcher/fn.none.html
//! [`not`]: matchers/not_matcher/fn.not.html
//! [`ok`]: matchers/ok_matcher/fn.ok.html
//! [`or`]: matchers/disjunction_matcher/trait.OrMatcherExt.html#method.or
//! [`pat!`]: macro.pat.html
//! [`points_to`]: matchers/points_to_matcher/fn.points_to.html
//! [`pointwise!`]: macro.pointwise.html
//! [`predicate`]: matchers/predicate_matcher/fn.predicate.html
//! [`size`]: matchers/size_matcher/fn.size.html
//! [`some`]: matchers/some_matcher/fn.some.html
//! [`starts_with`]: matchers/str_matcher/fn.starts_with.html
//! [`subset_of`]: matchers/subset_of_matcher/fn.subset_of.html
//! [`superset_of`]: matchers/superset_of_matcher/fn.superset_of.html
//! [`tuple!`]: macro.tuple.html
//! [`unordered_elements_are!`]: macro.unordered_elements_are.html
//!
//! [`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
//! [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
//! [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
//! [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
//! [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
//! [`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
//! [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
//!
//! ## Writing matchers
//!
//! One can extend the library by writing additional matchers. To do so, create
//! a struct holding the matcher's data and have it implement the trait
//! [`Matcher`]:
//!
//! ```rust
//! struct MyEqMatcher<T> {
//!    expected: T,
//! }
//!
//! impl<T: PartialEq + Debug> Matcher<T> for MyEqMatcher<T> {
//!    fn matches(&self, actual: &A) -> MatcherResult {
//!        if self.expected == *actual { MatcherResult::Matches } else { MatcherResult::DoesNotMatch }
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
//! ```rust
//! pub fn eq_my_way<T: PartialEq + Debug>(expected: T) -> impl Matcher<T> {
//!    MyEqMatcher { expected }
//! }
//! ```
//!
//! The new matcher can then be used in `verify_that!`:
//!
//! ```rust
//! #[test]
//! fn should_be_equal_by_my_definition() -> Result<()> {
//!    verify_that!(10, eq_my_way(10))
//! }
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
//! also be marked with [`google_test`] instead of the Rust-standard `#[test]`.
//! It must return [`Result<()>`].
//!
//! ```rust
//! use googletest::{
//!    expect_that, google_test, matchers::eq, Result
//! };
//!
//! #[google_test]
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
//! ```rust
//! fn stuff_is_correct(x: i32, y: i32) -> bool {
//!    x == y
//! }
//!
//! let x = 3;
//! let y = 4;
//! verify_pred!(stuff_is_correct(x, y))?;
//! ```
//!
//! The assertion failure message shows the arguments and the values to which
//! they evaluate:
//!
//! ```
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
//! ```rust
//! #[test]
//! fn always_fails() -> Result<()> {
//!    fail!("This test must fail with {}", "today")
//! }
//! ```
//!
//! [`and_log_failure()`]: trait.GoogleTestSupport.html#tymethod.and_log_failure
//! [`assert_that!`]: macro.assert_that.html
//! [`expect_pred!`]: macro.expect_pred.html
//! [`expect_that!`]: macro.expect_that.html
//! [`fail!`]: macro.fail.html
//! [`google_test`]: attr.google_test.html
//! [`matches_pattern!`]: macro.matches_pattern.html
//! [`verify_pred!`]: macro.verify_pred.html
//! [`verify_that!`]: macro.verify_that.html
//! [`Matcher`]: matcher/trait.Matcher.html
//! [`Result<()>`]: type.Result.html

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
