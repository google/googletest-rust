# GoogleTest Rust

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![Apache licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]
[![OpenSSF Best Practices][openssf-badge]][openssf-url]

[crates-badge]: https://img.shields.io/crates/v/googletest.svg
[crates-url]: https://crates.io/crates/googletest
[docs-badge]: https://img.shields.io/badge/docs.rs-googletest-66c2a5
[docs-url]: https://docs.rs/googletest/*/googletest/
[license-badge]: https://img.shields.io/badge/license-Apache-blue.svg
[license-url]: https://github.com/google/googletest-rust/blob/main/LICENSE
[actions-badge]: https://github.com/google/googletest-rust/workflows/CI/badge.svg
[actions-url]: https://github.com/google/googletest-rust/actions?query=workflow%3ACI+branch%3Amain
[openssf-badge]: https://www.bestpractices.dev/projects/10037/badge
[openssf-url]: https://www.bestpractices.dev/projects/10037

This library brings the rich assertion types of Google's C++ testing library
[GoogleTest](https://github.com/google/googletest) to Rust. It provides:

 * A framework for writing matchers which can be combined to make a wide range
   of assertions on data,
 * A rich set of matchers providing similar functionality to those included in
   [GoogleTest](https://google.github.io/googletest/reference/matchers.html),
   and
 * A new set of assertion macros offering similar functionality to those of
   [GoogleTest](https://google.github.io/googletest/primer.html#assertions).

**The minimum supported Rust version is 1.85**.

> :warning: The API is not fully stable and may still be changed until we
> publish version 1.0.
>
> Moreover, any items or modules starting with `__` (double underscores) must
> not be used directly. Those items or modules are only for internal uses and
> their API may change without a major version update.

## Learning resources

If you're just getting started with `googletest`, consider going through:

*   The first chapter of
    ["Advanced testing for Rust applications"](https://github.com/mainmatter/rust-advanced-testing-workshop),
    a self-guided Rust course. It provides a guided introduction to the library,
    with exercises to help you get comfortable with `googletest` macros,
    its matchers and its overall philosophy.
*   [docs.rs API Documentation](https://docs.rs/googletest/latest/googletest/):
    Comprehensive, automatically generated API documentation for all macros,
    matchers, and types.

## Assertions and matchers

The core of GoogleTest is its *matchers*. Matchers indicate what aspect of an
actual value one is asserting: (in-)equality, containment, regular expression
matching, and so on.

To make an assertion using a matcher, GoogleTest offers three macros:

 * [`expect_that!`] logs an assertion failure, marking the test as having
   failed, but allows the test to continue running (called a _non-fatal
   assertion_). It requires the use of the [`gtest`] attribute macro
   on the test itself.
 * [`verify_that!`] has no side effects and evaluates to a [`Result<()>`] whose
   `Err` variant describes the assertion failure, if there is one. In
   combination with the
   [`?` operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator),
   this can be used to abort the test on assertion failure without panicking. It
   is also the building block for `assert_that!` and `expect_that!`.
 * [`assert_that!`] panics if the assertion fails, aborting the test and any
   further execution.

For example:

```rust
use googletest::prelude::*;

#[test]
fn fails_and_panics() {
    let value = 2;
    assert_that!(value, eq(4));
}

#[gtest]
fn two_logged_failures() {
    let value = 2;
    expect_that!(value, eq(4)); // Test now failed, but continues executing.
    expect_that!(value, eq(5)); // Second failure is also logged.
}

#[test]
fn fails_immediately_without_panic() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(4))?; // Test fails and aborts.
    verify_that!(value, eq(2))?; // Never executes.
    Ok(())
}

#[test]
fn simple_assertion() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(4)) // One can also just return the last assertion.
}
```

These macros also support shorthand syntax for matching containers:

 * `verify_that!(actual, [m1, m2, ...])` is equivalent to `verify_that!(actual, elements_are![m1, m2, ...])`
 * `verify_that!(actual, {m1, m2, ...})` is equivalent to `verify_that!(actual, unordered_elements_are![m1, m2, ...])`

This library includes a rich set of matchers, covering:

 * Equality, numeric inequality, and approximate equality;
 * Strings and regular expressions;
 * Containers and set-theoretic matching.

Matchers are composable:

```rust
use googletest::prelude::*;

#[gtest]
fn contains_at_least_one_item_at_least_3() {
    let value = vec![1, 2, 3];
    expect_that!(value, contains(ge(3)));
}
```

They can also be logically combined:

```rust
use googletest::prelude::*;

#[gtest]
fn strictly_between_9_and_11() {
    let value = 10;
    expect_that!(value, gt(9).and(not(ge(11))));
}
```

## Available matchers

GoogleTest includes an extensive set of built-in matchers. For complete API
documentation, type signatures, and examples for each matcher, see the
[docs.rs matchers documentation](https://docs.rs/googletest/latest/googletest/matchers/index.html).

Common matchers include:

*   **Equality and comparison**: [`eq`], [`ne`], [`gt`], [`ge`], [`lt`], [`le`], [`approx_eq`], [`near`]
*   **Booleans**: [`is_true`], [`is_false`]
*   **Options and Results**: [`some`], [`none`], [`is_ok`], [`is_err`] (aliases
    [`ok`], [`err`]; `is_ok` and `is_err` are preferred over `ok`/`err` to avoid
    name collisions with `status::ok`/`status::err`)
*   **Strings and regular expressions**: [`contains_substring`], [`starts_with`], [`ends_with`], [`matches_regex`], [`contains_regex`], [`displays_as`], [`is_utf8_string`]
*   **Containers and collections**: [`contains`], [`elements_are!`], 
    [`unordered_elements_are!`], [`contains_each!`], [`is_contained_in!`],
    [`container_eq`], [`is_empty`], [`has_entry`], [`len`], [`subset_of`],
    [`superset_of`]
*   **Pointers and references**: [`points_to`], [`derefs_to`], [`ptr_eq`]
*   **Process exit and death**: [`died`], [`exited_with_code`]
*   **Members and pattern matching**: [`matches_pattern!`] (alias [`pat!`]), [`field!`], [`property!`], [`property_ref!`], [`result_of!`]
*   **Logical combinators**: [`all!`], [`any!`], [`not`], [`anything`], [`predicate`]


## Pattern-matching

One can use the macro [`matches_pattern!`] to create a composite matcher for a
struct or enum that matches fields with other matchers:

```rust
use googletest::prelude::*;

struct AStruct {
    a_field: i32,
    another_field: i32,
    a_third_field: &'static str,
}

#[test]
fn struct_has_expected_values() {
    let value = AStruct {
        a_field: 10,
        another_field: 100,
        a_third_field: "A correct value",
    };
    expect_that!(value, matches_pattern!(AStruct {
        a_field: eq(10),
        another_field: gt(50),
        a_third_field: contains_substring("correct"),
    }));
}
```

## Writing matchers

One can extend the library by writing additional matchers. To do so, create a
struct holding the matcher's data and have it implement the trait [`Matcher`]:

```rust
struct MyEqMatcher<T> {
    expected: T,
}

impl<T: PartialEq + Debug + Copy> Matcher<T> for MyEqMatcher<T> {
    fn matches(&self, actual: T) -> MatcherResult {
         (self.expected == actual).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Match => {
                format!("is equal to {:?} the way I define it", self.expected)
            }
            MatcherResult::NoMatch => {
                format!("isn't equal to {:?} the way I define it", self.expected)
            }
        }
    }
}
```

It is recommended to expose a function which constructs the matcher:

```rust
pub fn eq_my_way<T: PartialEq + Debug>(expected: T) -> impl Matcher<T> {
    MyEqMatcher { expected }
}
```

The new matcher can then be used in the assertion macros:

```rust
#[gtest]
fn should_be_equal_by_my_definition() {
    expect_that!(10, eq_my_way(10));
}
```

## Non-fatal assertions

Using non-fatal assertions, a single test is able to log multiple assertion
failures. Any single assertion failure causes the test to be considered having
failed, but execution continues until the test completes or otherwise aborts.

This is analogous to the `EXPECT_*` family of macros in GoogleTest.

To make a non-fatal assertion, use the macro [`expect_that!`]. The test must
also be marked with [`gtest`] instead of the Rust-standard `#[test]`.

```rust
use googletest::prelude::*;

#[gtest]
fn three_non_fatal_assertions() {
    let value = 2;
    expect_that!(value, eq(2));  // Passes; test still considered passing.
    expect_that!(value, eq(3));  // Fails; logs failure and marks the test failed.
    expect_that!(value, eq(4));  // A second failure, also logged.
}
```

This can be used in the same tests as `verify_that!`, in which case the test
function must also return [`Result<()>`]:

```rust
use googletest::prelude::*;

#[gtest]
fn failing_non_fatal_assertion() -> Result<()> {
    let value = 2;
    expect_that!(value, eq(3));  // Just marks the test as having failed.
    verify_that!(value, eq(2))?;  // Passes, so does not abort the test.
    Ok(())        // Because of the failing expect_that! call above, the
                  // test fails despite returning Ok(())
}
```

```rust
use googletest::prelude::*;

#[gtest]
fn failing_fatal_assertion_after_non_fatal_assertion() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(3))?; // Fails and aborts the test.
    expect_that!(value, eq(3));  // Never executes, since the test already aborted.
    Ok(())
}
```

### Interoperability

You can use the `#[gtest]` macro together with many other libraries
such as [rstest](https://crates.io/crates/rstest). Just apply both attribute
macros to the test:

```rust
#[gtest]
#[rstest]
#[case(1)]
#[case(2)]
#[case(3)]
fn rstest_works_with_google_test(#[case] value: u32) -> Result<()> {
   verify_that!(value, gt(0))
}
```

Make sure to put `#[gtest]` *before* `#[rstest]`. Otherwise the
annotated test will run twice, since both macros will attempt to register a test
with the Rust test harness.

The macro also works together with
[async tests with Tokio](https://docs.rs/tokio/latest/tokio/attr.gtest.html) in
the same way:

```rust
#[gtest]
#[tokio::test]
async fn should_work_with_tokio() -> Result<()> {
    verify_that!(3, gt(0))
}
```

There is one caveat when running async tests: test failure reporting through
`and_log_failure` will not work properly if the assertion occurs on a different
thread than runs the test.

## Common assertion macros

In addition to `*_that!` matcher assertions, GoogleTest offers specialized
macros for common test assertions. Most macros follow the three-variant pattern:

* `verify_*!` evaluates to a [`Result<()>`]. Use with `?` for fatal assertions
    without panicking.
* `expect_*!` logs an assertion failure (non-fatal) and continues execution.
* `assert_*!` panics immediately upon assertion failure, aborting the test and
    any further execution.

### Boolean assertions

* [`verify_true!(condition)`][`verify_true!`] / [`verify_false!(condition)`][`verify_false!`]
* [`expect_true!(condition)`][`expect_true!`] / [`expect_false!(condition)`][`expect_false!`]
* [`assert_true!(condition)`][`assert_true!`] / [`assert_false!(condition)`][`assert_false!`]

```rust
#[gtest]
fn test_booleans() -> Result<()> {
    expect_true!(2 + 2 == 4);
    verify_false!(2 + 2 == 5)?;
    assert_true!(true);
    Ok(())
}
```

### Result assertions

Assert that an expression evaluates to an `Ok(_)` variant:

* [`verify_ok!(expression)`][`verify_ok!`]
* [`expect_ok!(expression)`][`expect_ok!`]
* [`assert_ok!(expression)`][`assert_ok!`]

```rust
#[gtest]
fn test_results() -> Result<()> {
    let outcome: std::result::Result<i32, &str> = Ok(42);
    verify_ok!(outcome)?;
    expect_ok!(outcome);
    assert_ok!(outcome);
    Ok(())
}
```

### Direct comparison assertions

GoogleTest provides macros for direct binary comparisons without needing
explicit matcher syntax:

| Comparison | Non-fatal macro | Result-returning macro |
| :--- | :--- | :--- |
| Equality | `expect_eq!(a, b)` | `verify_eq!(a, b)` |
| Inequality | `expect_ne!(a, b)` | `verify_ne!(a, b)` |
| Less than (`a < b`) | `expect_lt!(a, b)` | `verify_lt!(a, b)` |
| Less than or equal (`a <=b `) | `expect_le!(a, b)` | `verify_le!(a, b)` |
| Greater than (`a > b`) | `expect_gt!(a, b)` | `verify_gt!(a, b)` |
| Greater than or equal (`a >= b`) | `expect_ge!(a, b)` | `verify_ge!(a, b)` |
| Float approximate equality | `expect_float_eq!(a, b)` | `verify_float_eq!(a, b)` |
| Float within tolerance | `expect_near!(a, b, tol)` | `verify_near!(a, b, tol)` |

`verify_eq!` and `expect_eq!` also support container shorthand:

```rust
verify_eq!(vec![1, 2, 3], [1, 2, 3])?; // Ordered sequence
verify_eq!(vec![3, 1, 2], {1, 2, 3})?; // Unordered sequence
```

### Custom failure messages

All `verify_*!`, `expect_*!`, and `assert_*!` macros accept optional format
strings and arguments:

```rust
expect_true!(value > 0, "Expected positive value, got {value}");
assert_ok!(result, "Failed to load config for user {}", user_id);
```

## Death tests

Death tests verify that a statement causes the process to exit or terminate
abnormally. GoogleTest runs the statement by spawning a child process that
re-executes the test up to the assertion point, capturing only `stderr` for
output matching.

* [`verify_death!(statement, output_matcher)`][`verify_death!`] and
[`expect_death!(statement, output_matcher)`][`expect_death!`] assert that
`statement` terminates abnormally (killed by signal or non-zero exit code) and 
captured `stderr` matches `output_matcher`.
* [`verify_exit!(statement, code_matcher, output_matcher)`][`verify_exit!`] and [`expect_exit!(statement, code_matcher, output_matcher)`][`expect_exit!`]
assert that `statement` exits with a status matching `code_matcher` (e.g. [`exited_with_code(1)`][`exited_with_code`]) and captured `stderr` matches `output_matcher`.

```rust
#[gtest]
fn test_death_and_exit() -> Result<()> {
    verify_death!(std::process::exit(1), anything())?;
    expect_death!(panic!("crash"), contains_substring("crash"));
    verify_exit!(std::process::exit(0), exited_with_code(0), anything())?;
    Ok(())
}
```

## Predicate assertions

The macro [`verify_pred!`] provides predicate assertions analogous to
GoogleTest's `EXPECT_PRED` family of macros. Wrap an invocation of a predicate
in a `verify_pred!` invocation to turn that into a test assertion which passes
precisely when the predicate returns `true`:

```rust
fn stuff_is_correct(x: i32, y: i32) -> bool {
    x == y
}

let x = 3;
let y = 4;
verify_pred!(stuff_is_correct(x, y))?;
```

The assertion failure message shows the arguments and the values to which they
evaluate:

```
stuff_is_correct(x, y) was false with
  x = 3,
  y = 4
```

The `verify_pred!` invocation evaluates to a [`Result<()>`] just like
[`verify_that!`]. There is also a macro [`expect_pred!`] to make a non-fatal
predicate assertion, and [`assert_pred!`] to make a fatal (panicking) assertion.

## Explicit success and failure

GoogleTest provides macros to explicitly record successes or failures:

* [`fail!("message")`][`fail!`] unconditionally evaluates to a `Result` indicating failure (use with `?`).
* [`succeed!("message")`][`succeed!`] generates a documentary success message to `stdout`.
* [`add_failure!("message")`][`add_failure!`] records a non-fatal failure and continues execution (requires `#[gtest]`).
* [`add_failure_at!("file", line, col, "message")`][`add_failure_at!`] records a non-fatal failure attributed to a specific file and line.

```rust
#[test]
fn test_explicit_fail() -> Result<()> {
    match some_value {
        ExpectedVariant => succeed!("Handled expected case"),
        UnwantedVariant => fail!("Unexpected variant: {:?}", some_value)?,
    }
    Ok(())
}
```

## Configuration

This library is configurable through environment variables. Since the
configuration does not impact whether a test fails or not but how a failure is
displayed, we recommend setting those variables in the personal
`~/.cargo/config.toml` instead of in the project-scoped `Cargo.toml`.

### Configuration variable list

| Variable name | Description                                             |
| ------------- | ------------------------------------------------------- |
| NO_COLOR      | Disables colored output. See <https://no-color.org/>.   |
| FORCE_COLOR   | Forces colors even when the output is piped to a file.  |

## Contributing Changes

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute
to this project.

[`add_failure!`]: https://docs.rs/googletest/*/googletest/macro.add_failure.html
[`add_failure_at!`]: https://docs.rs/googletest/*/googletest/macro.add_failure_at.html
[`all!`]: https://docs.rs/googletest/*/googletest/matchers/macro.all.html
[`any!`]: https://docs.rs/googletest/*/googletest/matchers/macro.any.html
[`anything`]: https://docs.rs/googletest/*/googletest/matchers/fn.anything.html
[`approx_eq`]: https://docs.rs/googletest/*/googletest/matchers/fn.approx_eq.html
[`assert_false!`]: https://docs.rs/googletest/*/googletest/macro.assert_false.html
[`assert_ok!`]: https://docs.rs/googletest/*/googletest/macro.assert_ok.html
[`assert_pred!`]: https://docs.rs/googletest/*/googletest/macro.assert_pred.html
[`assert_that!`]: https://docs.rs/googletest/*/googletest/macro.assert_that.html
[`assert_true!`]: https://docs.rs/googletest/*/googletest/macro.assert_true.html
[`container_eq`]: https://docs.rs/googletest/*/googletest/matchers/fn.container_eq.html
[`contains`]: https://docs.rs/googletest/*/googletest/matchers/fn.contains.html
[`contains_each!`]: https://docs.rs/googletest/*/googletest/matchers/macro.contains_each.html
[`contains_regex`]: https://docs.rs/googletest/*/googletest/matchers/fn.contains_regex.html
[`contains_substring`]: https://docs.rs/googletest/*/googletest/matchers/fn.contains_substring.html
[`derefs_to`]: https://docs.rs/googletest/*/googletest/matchers/fn.derefs_to.html
[`died`]: https://docs.rs/googletest/*/googletest/matchers/fn.died.html
[`displays_as`]: https://docs.rs/googletest/*/googletest/matchers/fn.displays_as.html
[`elements_are!`]: https://docs.rs/googletest/*/googletest/matchers/macro.elements_are.html
[`ends_with`]: https://docs.rs/googletest/*/googletest/matchers/fn.ends_with.html
[`eq`]: https://docs.rs/googletest/*/googletest/matchers/fn.eq.html
[`err`]: https://docs.rs/googletest/*/googletest/matchers/fn.err.html
[`exited_with_code`]: https://docs.rs/googletest/*/googletest/matchers/fn.exited_with_code.html
[`expect_death!`]: https://docs.rs/googletest/*/googletest/macro.expect_death.html
[`expect_exit!`]: https://docs.rs/googletest/*/googletest/macro.expect_exit.html
[`expect_false!`]: https://docs.rs/googletest/*/googletest/macro.expect_false.html
[`expect_ok!`]: https://docs.rs/googletest/*/googletest/macro.expect_ok.html
[`expect_pred!`]: https://docs.rs/googletest/*/googletest/macro.expect_pred.html
[`expect_that!`]: https://docs.rs/googletest/*/googletest/macro.expect_that.html
[`expect_true!`]: https://docs.rs/googletest/*/googletest/macro.expect_true.html
[`fail!`]: https://docs.rs/googletest/*/googletest/macro.fail.html
[`field!`]: https://docs.rs/googletest/*/googletest/matchers/macro.field.html
[`ge`]: https://docs.rs/googletest/*/googletest/matchers/fn.ge.html
[`gtest`]: https://docs.rs/googletest/*/googletest/attr.gtest.html
[`gt`]: https://docs.rs/googletest/*/googletest/matchers/fn.gt.html
[`has_entry`]: https://docs.rs/googletest/*/googletest/matchers/fn.has_entry.html
[`is_contained_in!`]: https://docs.rs/googletest/*/googletest/matchers/macro.is_contained_in.html
[`is_empty`]: https://docs.rs/googletest/*/googletest/matchers/fn.is_empty.html
[`is_err`]: https://docs.rs/googletest/*/googletest/matchers/fn.is_err.html
[`is_false`]: https://docs.rs/googletest/*/googletest/matchers/fn.is_false.html
[`is_ok`]: https://docs.rs/googletest/*/googletest/matchers/fn.is_ok.html
[`is_true`]: https://docs.rs/googletest/*/googletest/matchers/fn.is_true.html
[`is_utf8_string`]: https://docs.rs/googletest/*/googletest/matchers/fn.is_utf8_string.html
[`le`]: https://docs.rs/googletest/*/googletest/matchers/fn.le.html
[`len`]: https://docs.rs/googletest/*/googletest/matchers/fn.len.html
[`lt`]: https://docs.rs/googletest/*/googletest/matchers/fn.lt.html
[`matches_pattern!`]: https://docs.rs/googletest/*/googletest/macro.matches_pattern.html
[`matches_regex`]: https://docs.rs/googletest/*/googletest/matchers/fn.matches_regex.html
[`ne`]: https://docs.rs/googletest/*/googletest/matchers/fn.ne.html
[`near`]: https://docs.rs/googletest/*/googletest/matchers/fn.near.html
[`none`]: https://docs.rs/googletest/*/googletest/matchers/fn.none.html
[`not`]: https://docs.rs/googletest/*/googletest/matchers/fn.not.html
[`ok`]: https://docs.rs/googletest/*/googletest/matchers/fn.ok.html
[`pat!`]: https://docs.rs/googletest/*/googletest/matchers/macro.pat.html
[`points_to`]: https://docs.rs/googletest/*/googletest/matchers/fn.points_to.html
[`predicate`]: https://docs.rs/googletest/*/googletest/matchers/fn.predicate.html
[`property!`]: https://docs.rs/googletest/*/googletest/matchers/macro.property.html
[`property_ref!`]: https://docs.rs/googletest/*/googletest/matchers/macro.property_ref.html
[`ptr_eq`]: https://docs.rs/googletest/*/googletest/matchers/fn.ptr_eq.html
[`result_of!`]: https://docs.rs/googletest/*/googletest/matchers/macro.result_of.html
[`some`]: https://docs.rs/googletest/*/googletest/matchers/fn.some.html
[`starts_with`]: https://docs.rs/googletest/*/googletest/matchers/fn.starts_with.html
[`subset_of`]: https://docs.rs/googletest/*/googletest/matchers/fn.subset_of.html
[`succeed!`]: https://docs.rs/googletest/*/googletest/macro.succeed.html
[`superset_of`]: https://docs.rs/googletest/*/googletest/matchers/fn.superset_of.html
[`unordered_elements_are!`]: https://docs.rs/googletest/*/googletest/matchers/macro.unordered_elements_are.html
[`verify_death!`]: https://docs.rs/googletest/*/googletest/macro.verify_death.html
[`verify_exit!`]: https://docs.rs/googletest/*/googletest/macro.verify_exit.html
[`verify_false!`]: https://docs.rs/googletest/*/googletest/macro.verify_false.html
[`verify_ok!`]: https://docs.rs/googletest/*/googletest/macro.verify_ok.html
[`verify_pred!`]: https://docs.rs/googletest/*/googletest/macro.verify_pred.html
[`verify_that!`]: https://docs.rs/googletest/*/googletest/macro.verify_that.html
[`verify_true!`]: https://docs.rs/googletest/*/googletest/macro.verify_true.html
[`Matcher`]: https://docs.rs/googletest/*/googletest/matcher/trait.Matcher.html
[`Result<()>`]: https://docs.rs/googletest/*/googletest/type.Result.html
