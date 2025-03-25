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
    // specialized to sequences:
    ($actual:expr, [$($expecteds:expr),+ $(,)?]) => {
        {
            use $crate::assertions::internal::Subject as _;
            $actual.check(
                $crate::matchers::elements_are![$($expecteds),+],
                stringify!($actual),
            )
        }
    };

    // specialized to unordered sequences:
    ($actual:expr, {$($expecteds:expr),+ $(,)?}) => {
        {
            use $crate::assertions::internal::Subject as  _;
            $actual.check(
                $crate::matchers::unordered_elements_are![$($expecteds),+],
                stringify!($actual),
            )
        }
    };

    // general case:
    ($actual:expr, $expected:expr $(,)?) => {
        {
            use $crate::assertions::internal::Subject as  _;
            $actual.check(
                $expected,
                stringify!($actual),
            )
        }
    };
}
pub use verify_that;

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
///     fn b(_x: i32) -> i32 { 7 }
///     verify_pred!(equals_modulo(a, b(b(2)), 2 + 3))?;
///     Ok(())
/// }
/// # verify_that!(
/// #     test(),
/// #     err(displays_as(contains_substring("equals_modulo(a, b(b(2)), 2 + 3) was false with")))
/// # ).unwrap();
/// ```
///
/// This results in the following message:
///
/// ```text
/// equals_modulo(a, b(b(2)), 2 + 3) was false with
///   a = 1,
///   b(b(2)) = 7,
///   2 + 3 = 5,
/// ```
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
/// The expression passed to this macro must return `bool`. In the most general
/// case, it prints out each of the `.`-separated parts of the expression and
/// the arguments of all top-level method calls as long as they implement
/// `Debug`. It evaluates every value (including the method receivers) exactly
/// once. Effectively, for `verify_pred!((a + 1).b.c(x + y, &mut z, 2))`, it
/// generates code analogous to the following, which allows printing accurate
/// intermediate values even if they are subsequently consumed (moved out) or
/// mutated in-place by the expression:
///
/// ```ignore
/// let mut error_message = "(a + 1).b.c(x + y, 2) was false with".to_string();
/// let mut x1 = (a + 1);
/// write!(error_message, "\n  (a + 1) = {:?},", x1);
/// write!(error_message, "\n  (a + 1).b = {:?},", x1.b);
/// let mut x2 = x + y;
/// write!(error_message, "\n  x + y = {:?},", x2);
/// let mut x3 = &mut z;
/// write!(error_message, "\n  & mut z = {:?},", x3);
/// let mut x4 = x1.b.c(x2, x3, 2);
/// if (x4) {
///   Ok(())
/// } else {
///   Err(error_message)
/// }
/// ```
///
/// Wrapping the passed-in expression in parens or curly braces will prevent the
/// detailed printing of the expression.
///
/// ```ignore
/// verify_pred!((a.foo()).bar())?;
/// ```
///
/// would not print `a`, but would print `(a.foo())` and `(a.foo()).bar()` on
/// error.
#[macro_export]
macro_rules! verify_pred {
    ($expr:expr $(,)?) => {
        $crate::assertions::internal::__googletest_macro_verify_pred!($expr)
    };
}
pub use verify_pred;

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
    ($($message:expr),+ $(,)?) => {{
        $crate::assertions::internal::create_fail_result(
            format!($($message),*),
        )
    }};

    () => { $crate::fail!("Test failed") };
}
pub use fail;

/// Generates a success. This **does not** make the overall test succeed. A test
/// is only considered successful if none of its assertions fail during its
/// execution.
///
/// The succeed!() assertion is purely documentary. The only user visible output
/// is a stdout with information on where the success was generated from.
///
/// ```ignore
/// fn test_to_be_implemented() {
///     succeed!();
/// }
/// ```
///
/// One may include formatted arguments in the success message:
///
/// ```ignore
/// fn test_to_be_implemented() {
///     succeed!("I am just a fake test: {}", "a fake test indeed");
/// }
/// ```
#[macro_export]
macro_rules! succeed {
    ($($message:expr),+ $(,)?) => {{
        println!(
            "{}\n at {}:{}:{}",
            format!($($message),*),
            file!(), line!(), column!()
        );
    }};

    () => {
        $crate::succeed!("Success")
    };
}
pub use succeed;

/// Generates a failure marking the test as failed but continue execution.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail_but_not_abort() {
///     add_failure!();
/// }
/// ```
///
/// One may include formatted arguments in the failure message:
///
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail_but_not_abort() {
///     add_failure!("I am just a fake test: {}", "a fake test indeed");
/// }
/// ```
#[macro_export]
macro_rules! add_failure {
    ($($message:expr),+ $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure(
        $crate::assertions::internal::create_fail_result(
            format!($($message),*),
        ));
    }};

    () => {
        add_failure!("Failed")
    };
}
pub use add_failure;

/// Generates a failure at specified location marking the test as failed but
/// continue execution.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail_but_not_abort() {
///     add_failure_at!("src/my_file.rs", 32, 12);
/// }
/// ```
///
/// One may include formatted arguments in the failure message:
///
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail_but_not_abort() {
///     add_failure_at!(
///         "src/my_file.rs",
///         32,
///         12,
///         "I am just a fake test: {}", "a fake test indeed",
///     );
/// }
/// ```
#[macro_export]
macro_rules! add_failure_at {
    ($file:expr, $line:expr, $column:expr, $($message:expr),+ $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure(
        $crate::assertions::internal::create_fail_result(
            format!($($message),*),
        ).map_err(|e| e.with_fake_location($file, $line, $column)));
    }};

    ($file:expr, $line:expr, $column:expr $(,)?) => {
        add_failure_at!($file, $line, $column, "Failed")
    };
}
pub use add_failure_at;

/// Verify if the condition evaluates to true and returns `Result`.
///
/// Evaluates to `Result::Ok(())` if the condition is true and
/// `Result::Err(TestAssertionFailure)` if it evaluates to false. The caller
/// must then decide how to handle the `Err` variant. It has a few options:
///   * Abort the current function with the `?` operator. This requires that the
///     function return a suitable `Result`.
///   * Log the failure and continue by calling the method `and_log_failure`.
///
/// Of course, one can also use all other standard methods on `Result`.
///
/// **Invoking this macro by itself does not cause a test failure to be recorded
/// or output.** The resulting `Result` must be handled as described above to
/// cause the test to be recorded as a failure.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[test]
/// fn should_fail() -> Result<()> {
///     verify_true!(2 + 2 == 5)
/// }
/// ```
#[macro_export]
macro_rules! verify_true {
    ($condition:expr) => {{
        use $crate::assertions::internal::Subject as _;
        ($condition).check($crate::matchers::eq(true), stringify!($condition))
    }};
}
pub use verify_true;

/// Marks test as failed and continue execution if the expression evaluates to
/// false.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     expect_true!(2 + 2 == 5);
///     println!("This will print");
/// }
/// ```
///
/// One may optionally add arguments which will be formatted and appended to a
/// failure message. For example:
///
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let extra_information = "Some additional information";
///     expect_true!(false, "Test failed. Extra information: {extra_information}.");
/// }
/// ```
///
/// The output is as follows:
///
/// ```text
/// Value of: false
/// Expected: is equal to true
/// Actual: false,
///   which isn't equal to true
/// Test failed. Extra information: Some additional information.
/// ```
#[macro_export]
macro_rules! expect_true {
    ($condition:expr) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_true!($condition))
    }};
    ($condition:expr, $($format_args:expr),* $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_true!($condition)
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
}
pub use expect_true;

/// Verify if the condition evaluates to false and returns `Result`.
///
/// Evaluates to `Result::Ok(())` if the condition is false and
/// `Result::Err(TestAssertionFailure)` if it evaluates to true. The caller
/// must then decide how to handle the `Err` variant. It has a few options:
///   * Abort the current function with the `?` operator. This requires that the
///     function return a suitable `Result`.
///   * Log the failure and continue by calling the method `and_log_failure`.
///
/// Of course, one can also use all other standard methods on `Result`.
///
/// **Invoking this macro by itself does not cause a test failure to be recorded
/// or output.** The resulting `Result` must be handled as described above to
/// cause the test to be recorded as a failure.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[test]
/// fn should_fail() -> Result<()> {
///     verify_false!(2 + 2 == 4)
/// }
/// ```
#[macro_export]
macro_rules! verify_false {
    ($condition:expr) => {{
        use $crate::assertions::internal::Subject as _;
        ($condition).check($crate::matchers::eq(false), stringify!($condition))
    }};
}
pub use verify_false;

/// Marks test as failed and continue execution if the expression evaluates to
/// true.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     expect_false!(2 + 2 == 4);
///     println!("This will print");
/// }
/// ```
///
/// One may optionally add arguments which will be formatted and appended to a
/// failure message. For example:
///
/// ``` ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let extra_information = "Some additional information";
///     expect_false!(true, "Test failed. Extra information: {extra_information}.");
/// }
/// ```
///
/// The output is as follows:
///
/// ```text
/// Value of: true
/// Expected: is equal to false
/// Actual: true,
///   which isn't equal to false
/// Test failed. Extra information: Some additional information.
/// ```
#[macro_export]
macro_rules! expect_false {
    ($condition:expr) => {{
        $crate::GoogleTestSupport::and_log_failure(($crate::verify_false!($condition)))
    }};
    ($condition:expr, $($format_args:expr),* $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_false!($condition)
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
}
pub use expect_false;

/// Checks whether the second argument is equal to the first argument.
///
/// Evaluates to `Result::Ok(())` if they are equal and
/// `Result::Err(TestAssertionFailure)` if they are not. The caller must then
/// decide how to handle the `Err` variant. It has a few options:
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
/// ```ignore
/// use googletest::prelude::*;
///
/// #[test]
/// fn should_fail() -> Result<()> {
///     verify_eq!(2, 1)
/// }
/// ```
///
/// This macro has special support for matching against container. Namely:
///  * `verify_eq!(actual, [e1, e2, ...])` is equivalent to
///    `verify_that!(actual, elements_are![eq(e1), eq(e2), ...])`
///  * `verify_eq!(actual, {e1, e2, ...})` is equivalent to
///    `verify_that!(actual, unordered_elements_are![eq(e1), eq(e2), ...])`
#[macro_export]
macro_rules! verify_eq {
    // Specialization for ordered sequences of tuples:
    ($actual:expr, [ $( ( $($tuple_elt:expr),* ) ),+ $(,)? ] $(,)?) => {
        $crate::verify_that!(&$actual, [
            $(
                // tuple matching
                (
                    $(
                        $crate::matchers::eq(&$tuple_elt)
                    ),*
                )
            ),*
        ])
    };

    // Specialization for unordered sequences of tuples:
    ($actual:expr, { $( ( $($tuple_elt:expr),* ) ),+ $(,)?} $(,)?) => {
        $crate::verify_that!(&$actual, {
            $(
                // tuple matching
                (
                    $(
                        $crate::matchers::eq(&$tuple_elt)
                    ),*
                )
            ),*
        })
    };

    // Ordered sequences:
    ($actual:expr, [$($expected:expr),+ $(,)?] $(,)?) => {
        $crate::verify_that!(&$actual, [$($crate::matchers::eq(&$expected)),*])
    };

    // Unordered sequences:
    ($actual:expr, {$($expected:expr),+ $(,)?} $(,)?) => {
        $crate::verify_that!(&$actual, {$($crate::matchers::eq(&$expected)),*})
    };

    // General case:
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::verify_that!(&$actual, $crate::matchers::eq(&$expected))
    };
}
pub use verify_eq;

/// Marks test as failed and continues execution if the second argument is not
/// equal to first argument.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     expect_eq!(2, 1);
///     println!("This will print!");
/// }
/// ```
///
/// This macro has special support for matching against container. Namely:
///  * `expect_eq!(actual, [e1, e2, ...])` for checking actual contains "e1, e2,
///    ..." in order.
///  * `expect_eq!(actual, {e1, e2, ...})` for checking actual contains "e1, e2,
///    ..." in any order.
///
/// One may include formatted arguments in the failure message:
///```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let argument = "argument"
///     expect_eq!(2, 1, "custom failure message: {argument}");
///     println!("This will print!");
/// }
/// ```
#[macro_export]
macro_rules! expect_eq {
    ($actual:expr, [$($expected:expr),+ $(,)?] $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_eq!($actual, [$($expected),*]));
    }};
    ($actual:expr, [$($expected:expr),+ $(,)?], $($format_args:expr),* $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_eq!($actual, [$($expected),*])
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
    ($actual:expr, {$($expected:expr),+ $(,)?} $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_eq!($actual, {$($expected),*}));
    }};
    ($actual:expr, {$($expected:expr),+ $(,)?}, $($format_args:expr),* $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_eq!($actual, {$($expected),*})
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
    ($actual:expr, $expected:expr $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_eq!($actual, $expected));
    }};
    ($actual:expr, $expected:expr, $($format_args:expr),* $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_eq!($actual, $expected)
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
}
pub use expect_eq;

/// Checks whether the second argument is not equal to the first argument.
///
/// Evaluates to `Result::Ok(())` if they are not equal and
/// `Result::Err(TestAssertionFailure)` if they are equal. The caller must then
/// decide how to handle the `Err` variant. It has a few options:
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
/// ```ignore
/// use googletest::prelude::*;
///
/// #[test]
/// fn should_fail() -> Result<()> {
///     verify_ne!(1, 1)
/// }
/// ```
#[macro_export]
macro_rules! verify_ne {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::verify_that!(&$actual, $crate::matchers::not($crate::matchers::eq(&$expected)))
    };
}
pub use verify_ne;

/// Marks test as failed and continues execution if the second argument is
/// equal to first argument.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     expect_ne!(1, 1);
///     println!("This will print!");
/// }
/// ```
///
/// One may include formatted arguments in the failure message:
///```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let argument = "argument"
///     expect_ne!(1, 1, "custom failure message: {argument}");
///     println!("This will print!");
/// }
/// ```
#[macro_export]
macro_rules! expect_ne {
    ($actual:expr, $expected:expr, $($format_args:expr),+ $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_ne!($actual, $expected)
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
    ($actual:expr, $expected:expr $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_ne!($actual, $expected));
    }};
}
pub use expect_ne;

/// Checks whether the first argument is less than second argument.
///
/// Evaluates to `Result::Ok(())` if the first argument is less than the second
/// and `Result::Err(TestAssertionFailure)` if it is greater or equal. The
/// caller must then decide how to handle the `Err` variant. It has a few
/// options:
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
/// ```ignore
/// use googletest::prelude::*;
///
/// #[test]
/// fn should_fail() -> Result<()> {
///     verify_lt!(2, 1)
/// }
#[macro_export]
macro_rules! verify_lt {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::verify_that!($actual, $crate::matchers::lt($expected))
    };
}
pub use verify_lt;

/// Marks test as failed and continues execution if the first argument is
/// greater or equal to second argument.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     expect_lt!(2, 1);
///     println!("This will print!");
/// }
/// ```
///
/// One may include formatted arguments in the failure message:
///```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let argument = "argument"
///     expect_lt!(1, 1, "custom failure message: {argument}");
///     println!("This will print!");
/// }
/// ```
#[macro_export]
macro_rules! expect_lt {
    ($actual:expr, $expected:expr, $($format_args:expr),+ $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_lt!($actual, $expected)
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
    ($actual:expr, $expected:expr $(,)?) => {{
       $crate::GoogleTestSupport::and_log_failure($crate::verify_lt!($actual, $expected));
    }};
}
pub use expect_lt;

/// Checks whether the first argument is less than or equal to the second
/// argument.
///
/// Evaluates to `Result::Ok(())` if the first argument is less than or equal to
/// the second and `Result::Err(TestAssertionFailure)` if it is greater. The
/// caller must then decide how to handle the `Err` variant. It has a few
/// options:
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
/// ```ignore
/// use googletest::prelude::*;
///
/// #[test]
/// fn should_fail() -> Result<()> {
///     verify_le!(2, 1)
/// }
#[macro_export]
macro_rules! verify_le {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::verify_that!($actual, $crate::matchers::le($expected))
    };
}
pub use verify_le;

/// Marks test as failed and continues execution if the first argument is
/// greater than the second argument.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     expect_le!(2, 1);
///     println!("This will print!");
/// }
/// ```
///
/// One may include formatted arguments in the failure message:
///```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let argument = "argument"
///     expect_le!(2, 1, "custom failure message: {argument}");
///     println!("This will print!");
/// }
/// ```
#[macro_export]
macro_rules! expect_le {
    ($actual:expr, $expected:expr, $($format_args:expr),+ $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_le!($actual, $expected)
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
    ($actual:expr, $expected:expr $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_le!($actual, $expected));
    }};
}
pub use expect_le;

/// Checks whether the first argument is greater than the second argument.
///
/// Evaluates to `Result::Ok(())` if the first argument is greater than
/// the second and `Result::Err(TestAssertionFailure)` if it is not. The
/// caller must then decide how to handle the `Err` variant. It has a few
/// options:
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
/// ```ignore
/// use googletest::prelude::*;
///
/// #[test]
/// fn should_fail() -> Result<()> {
///     verify_gt!(1, 2)
/// }
#[macro_export]
macro_rules! verify_gt {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::verify_that!($actual, $crate::matchers::gt($expected))
    };
}
pub use verify_gt;

/// Marks test as failed and continues execution if the first argument is
/// not greater than the second argument.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     expect_gt!(1, 2);
///     println!("This will print!");
/// }
/// ```
///
/// One may include formatted arguments in the failure message:
///```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let argument = "argument"
///     expect_gt!(1, 2, "custom failure message: {argument}");
///     println!("This will print!");
/// }
/// ```
#[macro_export]
macro_rules! expect_gt {
    ($actual:expr, $expected:expr, $($format_args:expr),+ $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_gt!($actual, $expected)
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
    ($actual:expr, $expected:expr $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_gt!($actual, $expected));
    }};
}
pub use expect_gt;

/// Checks whether the first argument is greater than or equal to the second
/// argument.
///
/// Evaluates to `Result::Ok(())` if the first argument is greater than or equal
/// to the second and `Result::Err(TestAssertionFailure)` if it is not. The
/// caller must then decide how to handle the `Err` variant. It has a few
/// options:
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
/// ```ignore
/// use googletest::prelude::*;
///
/// #[test]
/// fn should_fail() -> Result<()> {
///     verify_ge!(1, 2)
/// }
/// ```
#[macro_export]
macro_rules! verify_ge {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::verify_that!($actual, $crate::matchers::ge($expected))
    };
}
pub use verify_ge;

/// Marks test as failed and continues execution if the first argument is
/// not greater than or equal to the second argument.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     expect_ge!(1, 2);
///     println!("This will print!");
/// }
/// ```
///
/// One may include formatted arguments in the failure message:
///```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let argument = "argument"
///     expect_ge!(1, 2, "custom failure message: {argument}");
///     println!("This will print!");
/// }
/// ```
#[macro_export]
macro_rules! expect_ge {
    ($actual:expr, $expected:expr, $($format_args:expr),+ $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_ge!($actual, $expected)
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
    ($actual:expr, $expected:expr $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_ge!($actual, $expected));
    }};
}
pub use expect_ge;

/// Checks whether the float given by first argument is approximately
/// equal to second argument.
///
/// This automatically computes a tolerance from the magnitude of `expected` and
/// matches any actual value within this tolerance of the expected value. The
/// tolerance is chosen to account for the inaccuracies in most ordinary
/// floating point calculations. To see details of how the tolerance is
/// calculated look at the implementation of
/// [`googletest::approx_eq`][crate::matchers::approx_eq].
///
/// Evaluates to `Result::Ok(())` if the first argument is approximately equal
/// to the second and `Result::Err(TestAssertionFailure)` if it is not. The
/// caller must then decide how to handle the `Err` variant. It has a few
/// options:
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
/// ```ignore
/// use googletest::prelude::*;
///
/// #[test]
/// fn should_fail() -> Result<()> {
///     verify_float_eq!(1.0, 2.0)
/// }
/// ```
#[macro_export]
macro_rules! verify_float_eq {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::verify_that!($actual, $crate::matchers::approx_eq($expected))
    };
}
pub use verify_float_eq;

/// Marks test as failed and continues execution if the float given by the first
/// argument is not approximately equal to the float given by the second
/// argument.
///
/// This automatically computes a tolerance from the magnitude of `expected` and
/// matches any actual value within this tolerance of the expected value. The
/// tolerance is chosen to account for the inaccuracies in most ordinary
/// floating point calculations. To see details of how the tolerance is
/// calculated look at the implementation of
/// [`googletest::approx_eq`][crate::matchers::approx_eq].
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     expect_float_eq!(1.0, 2.0);
///     println!("This will print!");
/// }
/// ```
///
/// One may include formatted arguments in the failure message:
///```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let argument = "argument"
///     expect_float_eq!(1.0, 2.0, "custom failure message: {argument}");
///     println!("This will print!");
/// }
/// ```
#[macro_export]
macro_rules! expect_float_eq {
    ($actual:expr, $expected:expr, $($format_args:expr),+ $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_float_eq!($actual, $expected)
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
    ($actual:expr, $expected:expr $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_float_eq!($actual, $expected));
    }};
}
pub use expect_float_eq;

/// Checks whether the float given by first argument is equal to second argument
/// with error tolerance of max_abs_error.
///
/// Evaluates to `Result::Ok(())` if the first argument is approximately equal
/// to the second and `Result::Err(TestAssertionFailure)` if it is not. The
/// caller must then decide how to handle the `Err` variant. It has a few
/// options:
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
/// ```ignore
/// use googletest::prelude::*;
///
/// #[test]
/// fn should_fail() -> Result<()> {
///     verify_near!(1.12345, 1.12346, 1e-6)
/// }
/// ```
#[macro_export]
macro_rules! verify_near {
    ($actual:expr, $expected:expr, $max_abs_error:expr $(,)?) => {
        $crate::verify_that!($actual, $crate::matchers::near($expected, $max_abs_error))
    };
}
pub use verify_near;

/// Marks the test as failed and continues execution if the float given by first
/// argument is not equal to second argument with error tolerance of
/// max_abs_error.
///
/// This is a **not-fatal** failure. The test continues execution even after the
/// macro execution.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The failure must be generated
/// in the same thread as that running the test itself.
///
/// Example:
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     expect_near!(1.12345, 1.12346, 1e-6);
///     println!("This will print!");
/// }
/// ```
///
/// One may include formatted arguments in the failure message:
///```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let argument = "argument"
///     expect_near!(1.12345, 1.12346, 1e-6, "custom failure message: {argument}");
///     println!("This will print!");
/// }
/// ```
#[macro_export]
macro_rules! expect_near {
    ($actual:expr, $expected:expr, $max_abs_error:expr, $($format_args:expr),+ $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_near!($actual, $expected, $max_abs_error)
            .with_failure_message(|| format!($($format_args),*))
            );
    }};
    ($actual:expr, $expected:expr, $max_abs_error:expr $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_near!($actual, $expected, $max_abs_error));
    }};
}
pub use expect_near;

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
    // specialized to sequence:
    ($actual:expr, [ $($expected:expr),* ] $(,)?) => {
        match $crate::verify_that!($actual, [ $($expected),* ]) {
            Ok(_) => {}
            Err(e) => {
                // The extra newline before the assertion failure message makes the failure a
                // bit easier to read when there's some generic boilerplate from the panic.
                panic!("\n{}", e);
            }
        }
    };

    // specialized to unordered sequence
    ($actual:expr, { $($expected:expr),* } $(,)?) => {
        match $crate::verify_that!($actual, { $($expected),* }) {
            Ok(_) => {}
            Err(e) => {
                // The extra newline before the assertion failure message makes the failure a
                // bit easier to read when there's some generic boilerplate from the panic.
                panic!("\n{}", e);
            }
        }
    };

    // w/ format args, specialized to sequence:
    ($actual:expr, [ $($expected:expr),* ], $($format_args:expr),* $(,)?) => {
        match $crate::verify_that!($actual, [ $($expected),* ])
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

    // w/ format args, specialized to unordered sequence:
    ($actual:expr, { $($expected:expr),* }, $($format_args:expr),* $(,)?) => {
        match $crate::verify_that!($actual, { $($expected),* })
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

    // general case:
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

    // w/ format args, general case:
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
pub use assert_that;

/// Asserts that the given predicate applied to the given arguments returns
/// true, panicking if it does not.
///
/// One may optionally add arguments which will be formatted and appended to a
/// failure message. For example:
///
/// ```should_panic
/// # use googletest::prelude::*;
/// # fn should_fail() {
///     let extra_information = "Some additional information";
///     assert_pred!(1 == 2, "Test failed. Extra information: {extra_information}.");
/// # }
/// # should_fail();
/// ```
///
/// The output is as follows:
///
/// ```text
/// 1 == 2 was false with
/// Test failed. Extra information: Some additional information.
/// ```
///
/// **Note for users of [GoogleTest for C++](http://google.github.io/googletest/):**
/// This differs from the `ASSERT_PRED*` family of macros in that it panics
/// rather than triggering an early return from the invoking function. To get
/// behaviour equivalent to `ASSERT_PRED*`, use [`verify_pred!`] with the `?`
/// operator.
#[macro_export]
macro_rules! assert_pred {
    ($content:expr $(,)?) => {
        match $crate::verify_pred!($content) {
            Ok(_) => {}
            Err(e) => {
                // The extra newline before the assertion failure message makes the failure a
                // bit easier to read when there's some generic boilerplate from the panic.
                panic!("\n{}", e);
            }
        }
    };

    // w/ format args
    ($content:expr $(,)?, $($format_args:expr),* $(,)?) => {
        match $crate::verify_pred!($content)
            .with_failure_message(|| format!($($format_args),*)) {
            Ok(_) => {}
            Err(e) => {
                // The extra newline before the assertion failure message makes the failure a
                // bit easier to read when there's some generic boilerplate from the panic.
                panic!("\n{}", e);
            }
        }
    };
}
pub use assert_pred;

/// Matches the given value against the given matcher, marking the test as
/// failed but continuing execution if it does not match.
///
/// This is a *non-fatal* assertion: the test continues
/// execution in the event of assertion failure.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The assertion must
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
    // specialized to sequence:
    ($actual:expr, [$($expected:expr),*] $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_that!($actual, [$($expected),*]));
    }};

    // specialized to unordered sequence:
    ($actual:expr, {$($expected:expr),*} $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_that!($actual, {$($expected),*}));
    }};

    // w/ format args, specialized to sequence:
    ($actual:expr, [$($expected:expr),*], $($format_args:expr),* $(,)?) => {
        $crate::GoogleTestSupport::and_log_failure($crate::verify_that!($actual, [$($expected),*])
            .with_failure_message(|| format!($($format_args),*))
            )
    };

    // w/ format args, specialized to unordered sequence:
    ($actual:expr, {$($expected:expr),*}, $($format_args:expr),* $(,)?) => {
        $crate::GoogleTestSupport::and_log_failure($crate::verify_that!($actual, {$($expected),*})
            .with_failure_message(|| format!($($format_args),*))
            )
    };

    // general case:
    ($actual:expr, $expected:expr $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_that!($actual, $expected));
    }};

    // w/ format args, general case:
    ($actual:expr, $expected:expr, $($format_args:expr),* $(,)?) => {
        $crate::GoogleTestSupport::and_log_failure($crate::verify_that!($actual, $expected)
            .with_failure_message(|| format!($($format_args),*))
            )
    };
}
pub use expect_that;

/// Asserts that the given predicate applied to the given arguments returns
/// true, failing the test but continuing execution if not.
///
/// This is a *non-fatal* predicate assertion: the test
/// continues execution in the event of assertion failure.
///
/// This can only be invoked inside tests with the
/// [`gtest`][crate::gtest] attribute. The assertion must
/// occur in the same thread as that running the test itself.
///
/// Invoking this macro is equivalent to using
/// [`and_log_failure`](crate::GoogleTestSupport::and_log_failure) as follows:
///
/// ```ignore
/// verify_pred!(predicate(...)).and_log_failure()
/// ```
///
/// One may optionally add arguments which will be formatted and appended to a
/// failure message. For example:
///
/// ```ignore
/// use googletest::prelude::*;
///
/// #[gtest]
/// fn should_fail() {
///     let extra_information = "Some additional information";
///     expect_pred!(1 == 2, "Test failed. Extra information: {extra_information}.");
/// }
/// ```
///
/// The output is as follows:
///
/// ```text
/// 1 == 2 was false with
/// Test failed. Extra information: Some additional information.
/// ```
#[macro_export]
macro_rules! expect_pred {
    ($content:expr $(,)?) => {{
        $crate::GoogleTestSupport::and_log_failure($crate::verify_pred!($content));
    }};
    // w/ format args
    ($content:expr $(,)?, $($format_args:expr),* $(,)?) => {
        $crate::GoogleTestSupport::and_log_failure($crate::verify_pred!($content)
            .with_failure_message(|| format!($($format_args),*))
            )
    };
}
pub use expect_pred;

/// Functions for use only by the procedural macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::{
        internal::test_outcome::TestAssertionFailure,
        matcher::{create_assertion_failure, Matcher, MatcherResult},
    };
    use std::fmt::Debug;

    pub use ::googletest_macro::__googletest_macro_verify_pred;

    /// Extension trait to perform autoref through method lookup in the
    /// assertion macros. With this trait, the subject can be either a value
    /// or a reference. For example, this trait makes the following code
    /// compile and work:
    /// ```
    /// # use googletest::prelude::*;
    /// # fn would_not_compile_without_autoref() -> Result<()> {
    /// let not_copyable = vec![1,2,3];
    /// verify_that!(not_copyable, empty())?;
    /// # Ok(())
    /// # }
    /// ```
    /// See [Method Lookup](https://rustc-dev-guide.rust-lang.org/method-lookup.html)
    pub trait Subject: Copy + Debug {
        /// Checks whether the matcher `expected` matches the `Subject `self`,
        /// adding a test failure report if it does not match.
        ///
        /// Returns `Ok(())` if the value matches and `Err(_)` if it does not
        /// match.
        ///
        /// **For internal use only. API stablility is not guaranteed!**
        #[must_use = "The assertion result must be evaluated to affect the test result."]
        #[track_caller]
        fn check(
            self,
            expected: impl Matcher<Self>,
            actual_expr: &'static str,
        ) -> Result<(), TestAssertionFailure> {
            match expected.matches(self) {
                MatcherResult::Match => Ok(()),
                MatcherResult::NoMatch => {
                    Err(create_assertion_failure(&expected, self, actual_expr))
                }
            }
        }
    }

    impl<T: Copy + Debug> Subject for T {}

    /// Creates a failure at specified location.
    ///
    /// **For internal use only. API stability is not guaranteed!**
    #[must_use = "The assertion result must be evaluated to affect the test result."]
    #[track_caller]
    pub fn create_fail_result(message: String) -> crate::Result<()> {
        Err(crate::internal::test_outcome::TestAssertionFailure::create(message))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self as googletest,
        assertions::{verify_eq, verify_that},
        matchers::{anything, err},
        test, Result as TestResult,
    };

    #[test]
    fn verify_of_hash_maps_with_str_string_matching() -> TestResult<()> {
        let hash_map: std::collections::HashMap<String, String> =
            std::collections::HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
        verify_eq!(hash_map, {("a", "A"), ("b", "B")})
    }

    #[test]
    fn verify_of_hash_maps_with_ad_hoc_struct() -> TestResult<()> {
        #[derive(PartialEq, Debug)]
        struct Greek(String);

        let hash_map: std::collections::HashMap<String, Greek> = std::collections::HashMap::from([
            ("a".into(), Greek("Alpha".into())),
            ("b".into(), Greek("Beta".into())),
        ]);
        verify_eq!(hash_map, {
            ("b", Greek("Beta".into())),
            ("a", Greek("Alpha".into())),
        })
    }

    #[test]
    fn verify_of_hash_maps_with_i32s() -> TestResult<()> {
        let hash_map: std::collections::HashMap<i32, i32> =
            std::collections::HashMap::from([(1, 1), (2, 4), (-1, 1), (-3, 9)]);
        verify_eq!(hash_map, {
            (-3, 9),
            (-1, 1),
            (1, 1),
            (2, 4),
        })
    }

    #[test]
    fn verify_eq_of_unordered_pairs() -> TestResult<()> {
        verify_eq!(vec![(1, 2), (2, 3)], {(1, 2), (2, 3)})?;
        verify_eq!(vec![(1, 2), (2, 3)], {(2, 3), (1, 2)})
    }

    #[test]
    fn verify_eq_of_unordered_structs() -> TestResult<()> {
        #[derive(PartialEq, Debug)]
        struct P(i32, i32);

        verify_eq!(vec![P(1, 1), P(1, 2), P(3, 7)],
                  {P(1, 1), P(1, 2), P(3, 7)})?;
        verify_eq!(vec![P(1, 1), P(1, 2), P(3, 7)],
                  {P(3,7), P(1, 1), P(1, 2)})
    }

    #[test]
    fn verify_eq_of_ordered_pairs() -> TestResult<()> {
        verify_eq!(vec![(1, 2), (2, 3)], [(1, 2), (2, 3)])
    }

    #[test]
    fn verify_eq_of_ordered_structs() -> TestResult<()> {
        #[derive(PartialEq, Debug)]
        struct P(i32, i32);

        verify_eq!(vec![P(1, 1), P(1, 2), P(3, 7)], [P(1, 1), P(1, 2), P(3, 7)])
    }

    #[test]
    fn verify_eq_of_ordered_pairs_order_matters() -> TestResult<()> {
        let result = googletest::verify_eq!(vec![(1, 2), (2, 3)], [(2, 3), (1, 2)]);
        verify_that!(result, err(anything()))
    }

    #[test]
    fn verify_eq_of_ordered_structs_order_matters() -> TestResult<()> {
        #[derive(PartialEq, Debug)]
        struct P(i32, i32);

        let result = verify_eq!(vec![P(1, 1), P(1, 2), P(3, 7)], [P(3, 7), P(1, 1), P(1, 2)]);
        verify_that!(result, err(anything()))
    }
}
