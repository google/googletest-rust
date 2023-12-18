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

// There are no visible documentation elements in this module; the declarative
// macros are documented at the top level.
#![doc(hidden)]

/// Checks whether the `Matcher` given by the second argument matches the first
/// argument.
///
/// Evaluates to `Result::Ok(())` if the matcher matches and
/// `Result::Err(TestAssertionFailure)` if it does not. The caller must then
/// decide how to handle the `Err` variant. It has a few options:
///
///  * Abort the current function with the `?` operator. This requires that the
///    function return a suitable `Result`.
///  * Log the test failure and continue by calling the method
///    `and_log_failure`.
///
/// Of course, one can also use all other standard methods on `Result`.
///
/// **Invoking this macro by itself does not cause a test failure to be recorded
/// or output.** The resulting `Result` must be handled as described above to
/// cause the test to be recorded as a failure.
///
/// Example:
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(42, eq(42))?; // This will pass.
/// # Ok(())
/// # }
/// # should_pass().unwrap();
/// # fn should_fail() -> Result<()> {
/// # googletest::internal::test_outcome::TestOutcome::init_current_test_outcome();
/// verify_that!(42, eq(123)).and_log_failure();
///             // This will log a test failure and allow execution to continue.
/// let _ = verify_that!(42, eq(123)); // This will do nothing.
/// verify_that!(42, eq(123))?; // This will fail, returning immediately.
/// verify_that!(42, eq(0))?; // This will not run.
/// # googletest::internal::test_outcome::TestOutcome::close_current_test_outcome::<&str>(Ok(()))
/// #     .unwrap_err();
/// # Ok(())
/// # }
/// # verify_that!(should_fail(), err(displays_as(contains_substring("Expected: is equal to 123"))))
/// #     .unwrap();
/// ```
///
/// This macro has special support for matching against container. Namely:
///   * `verify_that!(actual, [m1, m2, ...])` is equivalent to
///     `verify_that!(actual, elements_are![m1, m2, ...])`
///   * `verify_that!(actual, {m1, m2, ...})` is equivalent to
///     `verify_that!(actual, unordered_elements_are![m1, m2, ...])`
///
/// ## Matching against tuples
///
/// One can match against a tuple by constructing a tuple of matchers as
/// follows:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!((123, 456), (eq(123), eq(456)))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!((123, 456), (eq(123), eq(0)))?; // Fails: second matcher does not match
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// This also works with composed matchers:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!((123, 456), not((eq(456), eq(123))))?; // Passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Matchers must correspond to the actual tuple in count and type. Otherwise
/// the test will fail to compile.
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// # fn should_not_compile() -> Result<()> {
/// verify_that!((123, 456), (eq(123),))?; // Does not compile: wrong tuple size
/// verify_that!((123, "A string"), (eq(123), eq(456)))?; // Does not compile: wrong type
/// #     Ok(())
/// # }
/// ```
///
/// All fields must be covered by matchers. Use
/// [`anything`][crate::matchers::anything] for fields which are not relevant
/// for the test.
///
/// ```
/// # use googletest::prelude::*;
/// verify_that!((123, 456), (eq(123), anything()))
/// #     .unwrap();
/// ```
///
/// This supports tuples of up to 12 elements. Tuples longer than that do not
/// automatically inherit the `Debug` trait from their members, so are generally
/// not supported; see [Rust by Example](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html#tuples).
#[macro_export]
macro_rules! verify_that {
    ($actual:expr, [$($expecteds:expr),+ $(,)?]) => {
        $crate::assertions::internal::check_matcher(
            &$actual,
            $crate::matchers::elements_are![$($expecteds),+],
            stringify!($actual),
            $crate::internal::source_location::SourceLocation::new(file!(), line!(), column!()),
        )
    };
    ($actual:expr, {$($expecteds:expr),+ $(,)?}) => {
        $crate::assertions::internal::check_matcher(
            &$actual,
            $crate::matchers::unordered_elements_are![$($expecteds),+],
            stringify!($actual),
            $crate::internal::source_location::SourceLocation::new(file!(), line!(), column!()),
        )
    };
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::assertions::internal::check_matcher(
            &$actual,
            $expected,
            stringify!($actual),
            $crate::internal::source_location::SourceLocation::new(file!(), line!(), column!()),
        )
    };
}

/// Asserts that the given predicate applied to the given arguments returns
/// true.
///
/// Similarly to [`verify_that`], this evaluates to a `Result` whose `Ok`
/// variant indicates that the given predicate returned true and whose `Err`
/// variant indicates that it returned false.
///
/// The failure message contains detailed information about the arguments. For
/// example:
///
/// ```
/// # use googletest::prelude::*;
/// fn equals_modulo(a: i32, b: i32, n: i32) -> bool { a % n == b % n }
///
/// # /* The attribute macro would prevent the function from being compiled in a doctest.
/// #[test]
/// # */
/// fn test() -> Result<()> {
///     let a = 1;
///     let b = 7;
///     let n = 5;
///     verify_pred!(equals_modulo(a, b, n))?;
///     Ok(())
/// }
/// # verify_that!(
/// #     test(),
/// #     err(displays_as(contains_substring("equals_modulo(a, b, n) was false with")))
/// # ).unwrap();
/// ```
///
/// This results in the following message:
///
/// ```text
/// equals_modulo(a, b, n) was false with
///   a = 1,
///   b = 7,
///   n = 5
/// ```
///
/// The function passed to this macro must return `bool`. Each of the arguments
/// must evaluate to a type implementing [`std::fmt::Debug`]. The debug output
/// is used to construct the failure message.
///
/// The predicate can also be a method on a struct, e.g.:
///
/// ```ignore
/// struct AStruct {};
///
/// impl AStruct {
///   fn equals_modulo(...) {...}
/// }
///
/// verify_pred!((AStruct {}).equals_modulo(a, b, n))?;
/// ```
///
/// **Warning:** This macro assumes that the arguments passed to the predicate
/// are either *variables* or *calls to pure functions*. If two subsequent
/// invocations to any of the expresssions passed as arguments result in
/// different values, then the output message of a test failure will deviate
/// from the values actually passed to the predicate. For this reason, *always
/// assign the outputs of non-pure functions to variables before using them in
/// this macro. For example:
///
/// ```ignore
/// let output = generate_random_number();  // Assigned outside of verify_pred.
/// verify_pred!(is_sufficiently_random(output))?;
/// ```
#[macro_export]
macro_rules! verify_pred {
    ([$($predicate:tt)*]($($arg:tt),* $(,)?)) => {
        if !$($predicate)*($($arg),*) {
            $crate::assertions::internal::report_failed_predicate(
                concat!(stringify!($($predicate)*), stringify!(($($arg),*))),
                vec![$((format!(concat!(stringify!($arg), " = {:?}"), $arg))),*],
                $crate::internal::source_location::SourceLocation::new(
                    file!(),
                    line!(),
                    column!(),
                ),
            )
        } else {
            Ok(())
        }
    };

    ([$($predicate:tt)*] $first:tt $($rest:tt)*) => {
        $crate::verify_pred!([$($predicate)* $first] $($rest)*)
    };

    ($first:tt $($rest:tt)*) => {
        $crate::verify_pred!([$first] $($rest)*)
    };
}

/// Evaluates to a `Result` which contains an `Err` variant with the given test
/// failure message.
///
/// This can be used to force the test to fail if its execution reaches a
/// particular point. For example:
///
/// ```ignore
/// match some_value {
///     ExpectedVariant => {...}
///     UnwantedVaraint => {
///         fail!("This thing which should not happen happened anyway")?;
///     }
/// }
/// ```
///
/// One may include formatted arguments in the failure message:
///
/// ```ignore
/// match some_value {
///     ExpectedVariant => {...}
///     UnwantedVaraint => {
///         fail!("This thing which should not happen happened anyway: {}", some_value)?;
///     }
/// }
/// ```
///
/// One may also omit the message, in which case the test failure message will
/// be generic:
///
/// ```ignore
/// match some_value {
///     ExpectedVariant => {...}
///     UnwantedVaraint => {
///         fail!()?;
///     }
/// }
/// ```
///
/// Unlike `panic!` but analogously to [`verify_that`] and [`verify_pred`], this
/// macro has no effect on the flow of control but instead returns a `Result`
/// which must be handled by the invoking function. This can be done with the
/// question mark operator (as above) or the method
/// [`and_log_failure`](crate::GoogleTestSupport::and_log_failure).
#[macro_export]
macro_rules! fail {
    ($($message:expr),+) => {{
        // We wrap this in a function so that we can annotate it with the must_use attribute.
        // must_use on expressions is still experimental.
        #[must_use = "The assertion result must be evaluated to affect the test result."]
        fn create_fail_result(message: String) -> $crate::Result<()> {
            Err($crate::internal::test_outcome::TestAssertionFailure::create(format!(
                "{}\n{}",
                message,
                $crate::internal::source_location::SourceLocation::new(
                    file!(),
                    line!(),
                    column!(),
                ),
            )))
        }
        create_fail_result(format!($($message),*))
    }};

    () => { fail!("Test failed") };
}

/// Matches the given value against the given matcher, panicking if it does not
/// match.
///
/// ```should_panic
/// # use googletest::prelude::*;
/// # fn should_fail() {
/// let value = 2;
/// assert_that!(value, eq(3));  // Fails and panics.
/// # }
/// # should_fail();
/// ```
///
/// This is analogous to assertions in most Rust test libraries, where a failed
/// assertion causes a panic.
///
/// One may optionally add arguments which will be formatted and appended to a
/// failure message. For example:
///
/// ```should_panic
/// # use googletest::prelude::*;
/// # fn should_fail() {
/// let value = 2;
/// let extra_information = "Some additional information";
/// assert_that!(value, eq(3), "Test failed. Extra information: {extra_information}.");
/// # }
/// # should_fail();
/// ```
///
/// This is output as follows:
///
/// ```text
/// Value of: value
/// Expected: is equal to 3
/// Actual: 2,
///   which isn't equal to 3
///   at ...
/// Test failed. Extra information: Some additional information.
/// ```
///
/// **Note for users of [GoogleTest for C++](http://google.github.io/googletest/):**
/// This differs from the `ASSERT_THAT` macro in that it panics rather
/// than triggering an early return from the invoking function. To get behaviour
/// equivalent to `ASSERT_THAT`, use [`verify_that!`] with the `?` operator.
#[macro_export]
macro_rules! assert_that {
    ($actual:expr, $expected:expr $(,)?) => {
        match $crate::verify_that!($actual, $expected) {
            Ok(_) => {}
            Err(e) => {
                // The extra newline before the assertion failure message makes the failure a
                // bit easier to read when there's some generic boilerplate from the panic.
                panic!("\n{}", e);
            }
        }
    };

    ($actual:expr, $expected:expr, $($format_args:expr),* $(,)?) => {
        match $crate::verify_that!($actual, $expected)
            .with_failure_message(|| format!($($format_args),*))
        {
            Ok(_) => {}
            Err(e) => {
                // The extra newline before the assertion failure message makes the failure a
                // bit easier to read when there's some generic boilerplate from the panic.
                panic!("\n{}", e);
            }
        }
    };
}

/// Asserts that the given predicate applied to the given arguments returns
/// true, panicking if it does not.
///
/// **Note for users of [GoogleTest for C++](http://google.github.io/googletest/):**
/// This differs from the `ASSERT_PRED*` family of macros in that it panics
/// rather than triggering an early return from the invoking function. To get
/// behaviour equivalent to `ASSERT_PRED*`, use [`verify_pred!`] with the `?`
/// operator.
#[macro_export]
macro_rules! assert_pred {
    ($($content:tt)*) => {
        match $crate::verify_pred!($($content)*) {
            Ok(_) => {}
            Err(e) => {
                // The extra newline before the assertion failure message makes the failure a
                // bit easier to read when there's some generic boilerplate from the panic.
                panic!("\n{}", e);
            }
        }
    };
}

/// Matches the given value against the given matcher, marking the test as
/// failed but continuing execution if it does not match.
///
/// This is a *non-fatal* assertion: the test continues
/// execution in the event of assertion failure.
///
/// This can only be invoked inside tests with the
/// [`googletest::test`][crate::test] attribute. The assertion must
/// occur in the same thread as that running the test itself.
///
/// Invoking this macro is equivalent to using
/// [`and_log_failure`](crate::GoogleTestSupport::and_log_failure) as follows:
///
/// ```ignore
/// verify_that!(actual, expected).and_log_failure()
/// ```
///
/// One may optionally add arguments which will be formatted and appended to a
/// failure message. For example:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_fail() -> std::result::Result<(), googletest::internal::test_outcome::TestFailure> {
/// # googletest::internal::test_outcome::TestOutcome::init_current_test_outcome();
/// let value = 2;
/// let extra_information = "Some additional information";
/// expect_that!(value, eq(3), "Test failed. Extra information: {extra_information}.");
/// # googletest::internal::test_outcome::TestOutcome::close_current_test_outcome::<&str>(Ok(()))
/// # }
/// # should_fail().unwrap_err();
/// ```
///
/// This is output as follows:
///
/// ```text
/// Value of: value
/// Expected: is equal to 3
/// Actual: 2,
///   which isn't equal to 3
///   at ...
/// Test failed. Extra information: Some additional information.
/// ```
#[macro_export]
macro_rules! expect_that {
    ($actual:expr, $expected:expr $(,)?) => {{
        use $crate::GoogleTestSupport;
        $crate::verify_that!($actual, $expected).and_log_failure();
    }};

    ($actual:expr, $expected:expr, $($format_args:expr),* $(,)?) => {
        $crate::verify_that!($actual, $expected)
            .with_failure_message(|| format!($($format_args),*))
            .and_log_failure()
    };
}

/// Asserts that the given predicate applied to the given arguments returns
/// true, failing the test but continuing execution if not.
///
/// This is a *non-fatal* predicate assertion: the test
/// continues execution in the event of assertion failure.
///
/// This can only be invoked inside tests with the
/// [`googletest::test`][crate::test] attribute. The assertion must
/// occur in the same thread as that running the test itself.
///
/// Invoking this macro is equivalent to using
/// [`and_log_failure`](crate::GoogleTestSupport::and_log_failure) as follows:
///
/// ```ignore
/// verify_pred!(predicate(...)).and_log_failure()
/// ```
#[macro_export]
macro_rules! expect_pred {
    ($($content:tt)*) => {{
        use $crate::GoogleTestSupport;
        $crate::verify_pred!($($content)*).and_log_failure();
    }};
}

/// Functions for use only by the procedural macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::{
        internal::{source_location::SourceLocation, test_outcome::TestAssertionFailure},
        matcher::{create_assertion_failure, Matcher, MatcherResult},
    };
    use std::fmt::Debug;

    /// Checks whether the matcher `expected` matches the value `actual`, adding
    /// a test failure report if it does not match.
    ///
    /// Returns `Ok(())` if the value matches and `Err(())` if it does not
    /// match.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[must_use = "The assertion result must be evaluated to affect the test result."]
    pub fn check_matcher<T: Debug + ?Sized>(
        actual: &T,
        expected: impl Matcher<ActualT = T>,
        actual_expr: &'static str,
        source_location: SourceLocation,
    ) -> Result<(), TestAssertionFailure> {
        match expected.matches(actual) {
            MatcherResult::Match => Ok(()),
            MatcherResult::NoMatch => {
                Err(create_assertion_failure(&expected, actual, actual_expr, source_location))
            }
        }
    }

    /// Constructs a `Result::Err(TestAssertionFailure)` for a predicate failure
    /// as produced by the macro [`crate::verify_pred`].
    ///
    /// This intended only for use by the macro [`crate::verify_pred`].
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[must_use = "The assertion result must be evaluated to affect the test result."]
    pub fn report_failed_predicate(
        actual_expr: &'static str,
        formatted_arguments: Vec<String>,
        source_location: SourceLocation,
    ) -> Result<(), TestAssertionFailure> {
        Err(TestAssertionFailure::create(format!(
            "{} was false with\n  {}\n{}",
            actual_expr,
            formatted_arguments.join(",\n  "),
            source_location,
        )))
    }
}
