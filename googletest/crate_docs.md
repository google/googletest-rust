A rich test assertion library for Rust.

This library provides:

 * A framework for writing matchers which can be combined to make a wide
   range of assertions on data,
 * A rich set of matchers, and
 * A new set of test assertion macros.

## Assertions and matchers

Most assertions are made through the macro [`verify_that!`]. It takes two
arguments: an actual value to be tested and a [`Matcher`].

Unlike the macros used in other test assertion libraries in Rust,
`verify_that!` does not panic when the test assertion fails. Instead, it
evaluates to [`googletest::Result<()>`][Result], which the caller can choose
to handle by:

 * Returning immediately from the function with the `?` operator (a *fatal*
   assertion), or
 * Logging the failure, marking the test as failed, and allowing execution to
   continue (see [Non-fatal assertions](#non-fatal-assertions) below).

For example, for fatal assertions:

```
use googletest::prelude::*;

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[test]
# */
fn more_than_one_failure() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(4))?;  // Fails and ends execution of the test.
    verify_that!(value, eq(2)) // One can also just return the assertion result.
}
# more_than_one_failure().unwrap_err();
```

> In case one wants behaviour closer to other Rust test libraries, the macro
> [`assert_that!`] has the same parameters as [`verify_that!`] but panics on
> failure.

Matchers are composable:

```
use googletest::prelude::*;

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[test]
# */
fn contains_at_least_one_item_at_least_3() -> Result<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, contains(ge(3)))
}
# contains_at_least_one_item_at_least_3().unwrap();
```

They can also be logically combined:

```
use googletest::prelude::*;

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[test]
# */
fn strictly_between_9_and_11() -> Result<()> {
    let value = 10;
    verify_that!(value, gt(9).and(not(ge(11))))
}
# strictly_between_9_and_11().unwrap();
```

## Available matchers

The following matchers are provided in GoogleTest Rust:

| Matcher              | What it matches                                                          |
|----------------------|--------------------------------------------------------------------------|
| [`all!`]             | Anything matched by all given matchers.                                  |
| [`anything`]         | Any input.                                                               |
| [`approx_eq`]        | A floating point number within a standard tolerance of the argument.     |
| [`char_count`]       | A string with a Unicode scalar count matching the argument.              |
| [`container_eq`]     | Same as [`eq`], but for containers (with a better mismatch description). |
| [`contains`]         | A container containing an element matched by the given matcher.          |
| [`contains_each!`]   | A container containing distinct elements each of the arguments match.    |
| [`contains_regex`]   | A string containing a substring matching the given regular expression.   |
| [`contains_substring`] | A string containing the given substring.                               |
| [`displays_as`]      | A [`Display`] value whose formatted string is matched by the argument.   |
| [`each`]             | A container all of whose elements the given argument matches.            |
| [`elements_are!`]    | A container whose elements the arguments match, in order.                |
| [`empty`]            | An empty collection.                                                     |
| [`ends_with`]        | A string ending with the given suffix.                                   |
| [`eq`]               | A value equal to the argument, in the sense of the [`PartialEq`] trait.  |
| [`eq_deref_of`]      | A value equal to the dereferenced value of the argument.                 |
| [`err`]              | A [`Result`][std::result::Result] containing an `Err` variant the argument matches. |
| [`field!`]           | A struct or enum with a given field whose value the argument matches.    |
| [`ge`]               | A [`PartialOrd`] value greater than or equal to the given value.         |
| [`gt`]               | A [`PartialOrd`] value strictly greater than the given value.            |
| [`has_entry`]        | A [`HashMap`] containing a given key whose value the argument matches.   |
| [`is_contained_in!`] | A container each of whose elements is matched by some given matcher.     |
| [`is_nan`]           | A floating point number which is NaN.                                    |
| [`le`]               | A [`PartialOrd`] value less than or equal to the given value.            |
| [`len`]              | A container whose number of elements the argument matches.               |
| [`lt`]               | A [`PartialOrd`] value strictly less than the given value.               |
| [`matches_pattern!`] | A struct or enum whose fields are matched according to the arguments.    |
| [`matches_regex`]    | A string matched by the given regular expression.                        |
| [`near`]             | A floating point number within a given tolerance of the argument.        |
| [`none`]             | An [`Option`] containing `None`.                                         |
| [`not`]              | Any value the argument does not match.                                   |
| [`ok`]               | A [`Result`][std::result::Result] containing an `Ok` variant the argument matches. |
| [`pat!`]             | Alias for [`matches_pattern!`].                                          |
| [`points_to`]        | Any [`Deref`] such as `&`, `Rc`, etc. whose value the argument matches.  |
| [`pointwise!`]       | A container whose contents the arguments match in a pointwise fashion.   |
| [`predicate`]        | A value on which the given predicate returns true.                       |
| [`some`]             | An [`Option`] containing `Some` whose value the argument matches.        |
| [`starts_with`]      | A string starting with the given prefix.                                 |
| [`subset_of`]        | A container all of whose elements are contained in the argument.         |
| [`superset_of`]      | A container containing all elements of the argument.                     |
| [`unordered_elements_are!`] | A container whose elements the arguments match, in any order.     |

[`anything`]: matchers::anything_matcher::anything
[`approx_eq`]: matchers::near_matcher::approx_eq
[`char_count`]: matchers::char_count_matcher::char_count
[`container_eq`]: matchers::container_eq_matcher::container_eq
[`contains`]: matchers::contains_matcher::contains
[`contains_regex`]: matchers::contains_regex_matcher::contains_regex
[`contains_substring`]: matchers::str_matcher::contains_substring
[`displays_as`]: matchers::display_matcher::displays_as
[`each`]: matchers::each_matcher::each
[`empty`]: matchers::empty_matcher::empty
[`ends_with`]: matchers::str_matcher::ends_with
[`eq`]: matchers::eq_matcher::eq
[`eq_deref_of`]: matchers::eq_deref_of_matcher::eq_deref_of
[`err`]: matchers::err_matcher::err
[`ge`]: matchers::ge_matcher::ge
[`gt`]: matchers::gt_matcher::gt
[`has_entry`]: matchers::has_entry_matcher::has_entry
[`is_nan`]: matchers::is_nan_matcher::is_nan
[`le`]: matchers::le_matcher::le
[`len`]: matchers::len_matcher::len
[`lt`]: matchers::lt_matcher::lt
[`matches_regex`]: matchers::matches_regex_matcher::matches_regex
[`near`]: matchers::near_matcher::near
[`none`]: matchers::none_matcher::none
[`not`]: matchers::not_matcher::not
[`ok`]: matchers::ok_matcher::ok
[`points_to`]: matchers::points_to_matcher::points_to
[`predicate`]: matchers::predicate_matcher::predicate
[`some`]: matchers::some_matcher::some
[`starts_with`]: matchers::str_matcher::starts_with
[`subset_of`]: matchers::subset_of_matcher::subset_of
[`superset_of`]: matchers::superset_of_matcher::superset_of
[`Deref`]: std::ops::Deref
[`Display`]: std::fmt::Display
[`HashMap`]: std::collections::HashMap
[`Option`]: std::option::Option
[`PartialEq`]: std::cmp::PartialEq
[`PartialOrd`]: std::cmp::PartialOrd

## Writing matchers

One can extend the library by writing additional matchers. To do so, create
a struct holding the matcher's data and have it implement the trait
[`Matcher`]:

```no_run
use googletest::matcher::{Matcher, MatcherResult};
use std::fmt::Debug;

struct MyEqMatcher<T> {
    expected: T,
}

impl<T: PartialEq + Debug> Matcher for MyEqMatcher<T> {
    type ActualT = T;

    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
        if self.expected == *actual {
            MatcherResult::Match
        } else {
            MatcherResult::NoMatch
        }
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

 ```no_run
 # use googletest::matcher::{Matcher, MatcherResult};
 # use std::fmt::Debug;
 #
 # struct MyEqMatcher<T> {
 #    expected: T,
 # }
 #
 # impl<T: PartialEq + Debug> Matcher for MyEqMatcher<T> {
 #    type ActualT = T;
 #
 #    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
 #        if self.expected == *actual {
 #            MatcherResult::Match
 #        } else {
 #            MatcherResult::NoMatch
 #        }
 #    }
 #
 #    fn describe(&self, matcher_result: MatcherResult) -> String {
 #        match matcher_result {
 #            MatcherResult::Match => {
 #                format!("is equal to {:?} the way I define it", self.expected)
 #            }
 #            MatcherResult::NoMatch => {
 #                format!("isn't equal to {:?} the way I define it", self.expected)
 #            }
 #        }
 #    }
 # }
 #
 pub fn eq_my_way<T: PartialEq + Debug>(expected: T) -> impl Matcher<ActualT = T> {
    MyEqMatcher { expected }
 }
 ```

 The new matcher can then be used in `verify_that!`:

```
# use googletest::prelude::*;
# use googletest::matcher::{Matcher, MatcherResult};
# use std::fmt::Debug;
#
# struct MyEqMatcher<T> {
#    expected: T,
# }
#
# impl<T: PartialEq + Debug> Matcher for MyEqMatcher<T> {
#    type ActualT = T;
#
#    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
#        if self.expected == *actual {
#            MatcherResult::Match
#        } else {
#            MatcherResult::NoMatch
#        }
#    }
#
#    fn describe(&self, matcher_result: MatcherResult) -> String {
#        match matcher_result {
#            MatcherResult::Match => {
#                format!("is equal to {:?} the way I define it", self.expected)
#            }
#            MatcherResult::NoMatch => {
#                format!("isn't equal to {:?} the way I define it", self.expected)
#            }
#        }
#    }
# }
#
# pub fn eq_my_way<T: PartialEq + Debug>(expected: T) -> impl Matcher<ActualT = T> {
#    MyEqMatcher { expected }
# }
# /* The attribute macro would prevent the function from being compiled in a doctest.
#[test]
# */
fn should_be_equal_by_my_definition() -> Result<()> {
    verify_that!(10, eq_my_way(10))
}
# should_be_equal_by_my_definition().unwrap();
```

## Non-fatal assertions

Using non-fatal assertions, a single test is able to log multiple assertion
failures. Any single assertion failure causes the test to be considered
having failed, but execution continues until the test completes or otherwise
aborts.

To make a non-fatal assertion, use the macro [`expect_that!`]. The test must
also be marked with [`googletest::test`][test] instead of the Rust-standard
`#[test]`. It must return [`Result<()>`].

```no_run
use googletest::prelude::*;

# /* Make sure this also compiles as a doctest.
#[googletest::test]
# */
fn more_than_one_failure() -> Result<()> {
    let value = 2;
    expect_that!(value, eq(3));  // Just marks the test as having failed.
    verify_that!(value, eq(2))?;  // Passes, but the test already failed.
    Ok(())
}
```

## Predicate assertions

The macro [`verify_pred!`] provides predicate assertions analogous to
GoogleTest's `EXPECT_PRED` family of macros. Wrap an invocation of a
predicate in a `verify_pred!` invocation to turn that into a test assertion
which passes precisely when the predicate returns `true`:

```
# use googletest::prelude::*;
fn stuff_is_correct(x: i32, y: i32) -> bool {
    x == y
}

# fn run_test() -> Result<()> {
let x = 3;
let y = 4;
verify_pred!(stuff_is_correct(x, y))?;
# Ok(())
# }
# run_test().unwrap_err();
```

The assertion failure message shows the arguments and the values to which
they evaluate:

```text
stuff_is_correct(x, y) was false with
  x = 3,
  y = 4
```

The `verify_pred!` invocation evaluates to a [`Result<()>`] just like
[`verify_that!`]. There is also a macro [`expect_pred!`] to make a non-fatal
predicaticate assertion.

## Unconditionally generating a test failure

The macro [`fail!`] unconditionally evaluates to a `Result` indicating a
test failure. It can be used analogously to [`verify_that!`] and
[`verify_pred!`] to cause a test to fail, with an optional formatted
message:

```
# use googletest::prelude::*;
# /* The attribute macro would prevent the function from being compiled in a doctest.
#[test]
# */
fn always_fails() -> Result<()> {
    fail!("This test must fail with {}", "today")
}
# always_fails().unwrap_err();
```

## Integrations with other crates

GoogleTest Rust includes integrations with the
[Anyhow](https://crates.io/crates/anyhow) and
[Proptest](https://crates.io/crates/proptest) crates to simplify turning
errors from those crates into test failures.

To use this, activate the `anyhow`, respectively `proptest` feature in
GoogleTest Rust and invoke the extension method [`into_test_result()`] on a
`Result` value in your test. For example:

```
# use googletest::prelude::*;
# #[cfg(feature = "anyhow")]
# use anyhow::anyhow;
# #[cfg(feature = "anyhow")]
# /* The attribute macro would prevent the function from being compiled in a doctest.
#[test]
# */
fn has_anyhow_failure() -> Result<()> {
    Ok(just_return_error().into_test_result()?)
}

# #[cfg(feature = "anyhow")]
fn just_return_error() -> anyhow::Result<()> {
    anyhow::Result::Err(anyhow!("This is an error"))
}
# #[cfg(feature = "anyhow")]
# has_anyhow_failure().unwrap_err();
```

One can convert Proptest test failures into GoogleTest test failures when the
test is invoked with
[`TestRunner::run`](https://docs.rs/proptest/latest/proptest/test_runner/struct.TestRunner.html#method.run):

```
# use googletest::prelude::*;
# #[cfg(feature = "proptest")]
# use proptest::test_runner::{Config, TestRunner};
# #[cfg(feature = "proptest")]
# /* The attribute macro would prevent the function from being compiled in a doctest.
#[test]
# */
fn numbers_are_greater_than_zero() -> Result<()> {
    let mut runner = TestRunner::new(Config::default());
    runner.run(&(1..100i32), |v| Ok(verify_that!(v, gt(0))?)).into_test_result()
}
# #[cfg(feature = "proptest")]
# numbers_are_greater_than_zero().unwrap();
```

Similarly, when the `proptest` feature is enabled, GoogleTest assertion failures
can automatically be converted into Proptest
[`TestCaseError`](https://docs.rs/proptest/latest/proptest/test_runner/enum.TestCaseError.html)
through the `?` operator as the example above shows.

[`and_log_failure()`]: GoogleTestSupport::and_log_failure
[`into_test_result()`]: IntoTestResult::into_test_result
[`Matcher`]: matcher::Matcher
