# GoogleTest Rust

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![Apache licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/googletest.svg
[crates-url]: https://crates.io/crates/googletest
[docs-badge]: https://img.shields.io/badge/docs.rs-googletest-66c2a5
[docs-url]: https://docs.rs/googletest/*/googletest/
[license-badge]: https://img.shields.io/badge/license-Apache-blue.svg
[license-url]: https://github.com/google/googletest-rust/blob/main/LICENSE
[actions-badge]: https://github.com/google/googletest-rust/workflows/CI/badge.svg
[actions-url]: https://github.com/google/googletest-rust/actions?query=workflow%3ACI+branch%3Amain

This library brings the rich assertion types of Google's C++ testing library
[GoogleTest](https://github.com/google/googletest) to Rust. It provides:

 * A framework for writing matchers which can be combined to make a wide range
   of assertions on data,
 * A rich set of matchers providing similar functionality to those included in
   [GoogleTest](https://google.github.io/googletest/reference/matchers.html),
   and
 * A new set of assertion macros offering similar functionality to those of
   [GoogleTest](https://google.github.io/googletest/primer.html#assertions).

**The minimum supported Rust version is 1.59**. We recommend using at least
version 1.66 for the best developer experience.

> :warning: The API is not fully stable and may still be changed until we
> publish version 1.0.

## Assertions and matchers

Most assertions are made through the macro [`verify_that!`] which evaluates to a
[`Result<()>`]. It takes two arguments: an actual value to be tested and a
[`Matcher`].

Unlike the macros used in other test assertion libraries in Rust, `verify_that!`
does not panic when the test assertion fails. Instead, it evaluates to `Result`,
which the caller can choose to handle by:

 * Returning immediately from the function with the `?` operator (a *fatal*
   assertion), or
 * Logging the failure, marking the test as failed, and allowing execution to
   continue (see [Non-fatal assertions](#non-fatal-assertions) below).

Fatal assertions are analogous to the `ASSERT_*` family of macros in GoogleTest.

For example, for fatal assertions:

```rust
use googletest::prelude::*;

#[test]
fn more_than_one_failure() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(4))?;  // Fails and ends execution of the test.
    verify_that!(value, eq(2)) // One can also just return the assertion result.
}
```

> In case one wants behaviour closer to other Rust test libraries, the macro
> [`assert_that!`] has the same parameters as [`verify_that!`] but panics on
> failure.

This library includes a rich set of matchers, covering:

 * Equality, numeric inequality, and approximate equality;
 * Strings and regular expressions;
 * Containers and set-theoretic matching.

Matchers are composable:

```rust
use googletest::prelude::*;

#[test]
fn contains_at_least_one_item_at_least_3() -> Result<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, contains(ge(3)))
}
```

They can also be logically combined:

```rust
use googletest::prelude::*;

#[test]
fn strictly_between_9_and_11() -> Result<()> {
    let value = 10;
    verify_that!(value, gt(9).and(not(ge(11))))
}
```

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
fn struct_has_expected_values() -> Result<()> {
    let value = AStruct {
        a_field: 10,
        another_field: 100,
        a_third_field: "A correct value",
    };
    verify_that!(value, matches_pattern!(AStruct {
        a_field: eq(10),
        another_field: gt(50),
        a_third_field: contains_substring("correct"),
    }))
}
```

## Writing matchers

One can extend the library by writing additional matchers. To do so, create a
struct holding the matcher's data and have it implement the trait [`Matcher`]:

```rust
struct MyEqMatcher<T> {
    expected: T,
}

impl<T: PartialEq + Debug> Matcher for MyEqMatcher<T> {
    type ActualT = T;

    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
         (self.expected == *actual).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!("is equal to {:?} the way I define it", self.expected)
            }
            MatcherResult::DoesNotMatch => {
                format!("isn't equal to {:?} the way I define it", self.expected)
            }
        }
    }
}
```

It is recommended to expose a function which constructs the matcher:

```rust
pub fn eq_my_way<T: PartialEq + Debug>(expected: T) -> impl Matcher<ActualT = T> {
    MyEqMatcher { expected }
}
```

The new matcher can then be used in `verify_that!`:

```rust
#[test]
fn should_be_equal_by_my_definition() -> Result<()> {
    verify_that!(10, eq_my_way(10))
}
```

## Non-fatal assertions

Using non-fatal assertions, a single test is able to log multiple assertion
failures. Any single assertion failure causes the test to be considered having
failed, but execution continues until the test completes or otherwise aborts.

This is analogous to the `EXPECT_*` family of macros in GoogleTest.

To make a non-fatal assertion, use the macro [`expect_that!`]. The test must
also be marked with [`googletest::test`] instead of the Rust-standard `#[test]`.
It must return [`Result<()>`].

```rust
use googletest::prelude::*;

#[googletest::test]
fn more_than_one_failure() -> Result<()> {
    let value = 2;
    expect_that!(value, eq(3));  // Just marks the test as having failed.
    verify_that!(value, eq(2))?;  // Passes, but the test already failed.
    Ok(())
}
```

### Interoperability

You can use the `#[googletest::test]` macro together with many other libraries
such as [rstest](https://crates.io/crates/rstest). Just apply both attribute
macros to the test:

```rust
#[googletest::test]
#[rstest]
#[case(1)]
#[case(2)]
#[case(3)]
fn rstest_works_with_google_test(#[case] value: u32) -> Result<()> {
   verify_that!(value, gt(0))
}
```

Make sure to put `#[googletest::test]` *before* `#[rstest]`. Otherwise the
annotated test will run twice, since both macros will attempt to register a test
with the Rust test harness.

The macro also works together with
[async tests with Tokio](https://docs.rs/tokio/latest/tokio/attr.test.html) in
the same way:

```rust
#[googletest::test]
#[tokio::test]
async fn should_work_with_tokio() -> Result<()> {
    verify_that!(3, gt(0))
}
```

There is one caveat when running async tests: test failure reporting through
`and_log_failure` will not work properly if the assertion occurs on a different
thread than runs the test.

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
predicaticate assertion.

## Unconditionally generating a test failure

The macro [`fail!`] unconditionally evaluates to a `Result` indicating a test
failure. It can be used analogously to [`verify_that!`] and [`verify_pred!`] to
cause a test to fail, with an optional formatted message:

```rust
#[test]
fn always_fails() -> Result<()> {
    fail!("This test must fail with {}", "today")
}
```

## Configuration

This library is configurable through environment variables. Since the
configuration does not impact whether a test fails or not but how a failure is
displayed, we recommend setting those variables in the personal
`~/.cargo/config.toml` instead of in the project-scoped `Cargo.toml`.

## Contributing Changes

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute
to this project.

[`and_log_failure()`]: https://docs.rs/googletest/*/googletest/trait.GoogleTestSupport.html#tymethod.and_log_failure
[`assert_that!`]: https://docs.rs/googletest/*/googletest/macro.assert_that.html
[`expect_pred!`]: https://docs.rs/googletest/*/googletest/macro.expect_pred.html
[`expect_that!`]: https://docs.rs/googletest/*/googletest/macro.expect_that.html
[`fail!`]: https://docs.rs/googletest/*/googletest/macro.fail.html
[`googletest::test`]: https://docs.rs/googletest/*/googletest/attr.test.html
[`matches_pattern!`]: https://docs.rs/googletest/*/googletest/macro.matches_pattern.html
[`verify_pred!`]: https://docs.rs/googletest/*/googletest/macro.verify_pred.html
[`verify_that!`]: https://docs.rs/googletest/*/googletest/macro.verify_that.html
[`Describe`]: https://docs.rs/googletest/*/googletest/matcher/trait.Describe.html
[`Matcher`]: https://docs.rs/googletest/*/googletest/matcher/trait.Matcher.html
[`Result<()>`]: https://docs.rs/googletest/*/googletest/type.Result.html
