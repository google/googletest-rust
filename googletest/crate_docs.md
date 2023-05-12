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
| [`and`]              | Anything matched by both matchers.                                       |
| [`approx_eq`]        | A floating point number within a standard tolerance of the argument.     |
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
| [`or`]               | Anything matched by either of the two given matchers.                    |
| [`pat!`]             | Alias for [`matches_pattern!`].                                          |
| [`points_to`]        | Any [`Deref`] such as `&`, `Rc`, etc. whose value the argument matches.  |
| [`pointwise!`]       | A container whose contents the arguments match in a pointwise fashion.   |
| [`predicate`]        | A value on which the given predicate returns true.                       |
| [`some`]             | An [`Option`] containing `Some` whose value the argument matches.        |
| [`starts_with`]      | A string starting with the given prefix.                                 |
| [`subset_of`]        | A container all of whose elements are contained in the argument.         |
| [`superset_of`]      | A container containing all elements of the argument.                     |
| [`tuple!`]           | A tuple whose elements the arguments match.                              |
| [`unordered_elements_are!`] | A container whose elements the arguments match, in any order.     |

[`anything`]: matchers::anything
[`and`]: matchers::AndMatcherExt::and
[`approx_eq`]: matchers::approx_eq
[`container_eq`]: matchers::container_eq
[`contains`]: matchers::contains
[`contains_regex`]: matchers::contains_regex
[`contains_substring`]: matchers::contains_substring
[`displays_as`]: matchers::displays_as
[`each`]: matchers::each
[`empty`]: matchers::empty
[`ends_with`]: matchers::ends_with
[`eq`]: matchers::eq
[`eq_deref_of`]: matchers::eq_deref_of
[`err`]: matchers::err
[`ge`]: matchers::ge
[`gt`]: matchers::gt
[`has_entry`]: matchers::has_entry
[`is_nan`]: matchers::is_nan
[`le`]: matchers::le
[`len`]: matchers::len
[`lt`]: matchers::lt
[`matches_regex`]: matchers::matches_regex
[`near`]: matchers::near
[`none`]: matchers::none
[`not`]: matchers::not
[`ok`]: matchers::ok
[`or`]: matchers::OrMatcherExt::or
[`points_to`]: matchers::points_to
[`predicate`]: matchers::predicate
[`some`]: matchers::some
[`starts_with`]: matchers::starts_with
[`subset_of`]: matchers::subset_of
[`superset_of`]: matchers::superset_of
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
            MatcherResult::Matches
        } else {
            MatcherResult::DoesNotMatch
        }
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
 #            MatcherResult::Matches
 #        } else {
 #            MatcherResult::DoesNotMatch
 #        }
 #    }
 #
 #    fn describe(&self, matcher_result: MatcherResult) -> String {
 #        match matcher_result {
 #            MatcherResult::Matches => {
 #                format!("is equal to {:?} the way I define it", self.expected)
 #            }
 #            MatcherResult::DoesNotMatch => {
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
#            MatcherResult::Matches
#        } else {
#            MatcherResult::DoesNotMatch
#        }
#    }
#
#    fn describe(&self, matcher_result: MatcherResult) -> String {
#        match matcher_result {
#            MatcherResult::Matches => {
#                format!("is equal to {:?} the way I define it", self.expected)
#            }
#            MatcherResult::DoesNotMatch => {
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

[`and_log_failure()`]: GoogleTestSupport::and_log_failure
[`Matcher`]: matcher::Matcher
