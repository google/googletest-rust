A rich test assertion library for Rust.

This library provides:

 * A framework for writing matchers which can be combined to make a wide
   range of assertions on data,
 * A rich set of matchers, and
 * A new set of test assertion macros.
## Learning resources

If you're just getting started with `googletest`, consider going through
the first chapter of
["Advanced testing for Rust applications"](https://github.com/mainmatter/rust-advanced-testing-workshop),
a self-guided Rust course: it provides a guided introduction to the library,
with exercises to help you get comfortable with `googletest` macros,
its matchers and its overall philosophy.

## Assertions and matchers

The core of GoogleTest is its *matchers*. Matchers indicate what aspect of an
actual value one is asserting: (in-)equality, containment, regular expression
matching, and so on.

To make an assertion using a matcher, GoogleTest offers three macros:

 * [`assert_that!`] panics if the assertion fails, aborting the test.
 * [`expect_that!`] logs an assertion failure, marking the test as having
   failed, but allows the test to continue running (called a _non-fatal
   assertion_). It requires the use of the [`gtest`]
   attribute macro on the test itself.
 * [`verify_that!`] has no side effects and evaluates to a [`Result`] whose
   `Err` variant describes the assertion failure, if there is one. In
   combination with the
   [`?` operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator),
   this can be used to abort the test on assertion failure without panicking. It
   is also the building block for the other two macros above.

For example:

```
use googletest::prelude::*;

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[test]
# */
fn fails_and_panics() {
    let value = 2;
    assert_that!(value, eq(4));
}

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[gtest]
# */
fn two_logged_failures() {
    let value = 2;
    expect_that!(value, eq(4)); // Test now failed, but continues executing.
    expect_that!(value, eq(5)); // Second failure is also logged.
}

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[test]
# */
fn fails_immediately_without_panic() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(4))?; // Test fails and aborts.
    verify_that!(value, eq(2))?; // Never executes.
    Ok(())
}

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[test]
# */
fn simple_assertion() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(4)) // One can also just return the last assertion.
}
```

Matchers are composable:

```
use googletest::prelude::*;

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[gtest]
# */
fn contains_at_least_one_item_at_least_3() {
# googletest::internal::test_outcome::TestOutcome::init_current_test_outcome();
    let value = vec![1, 2, 3];
    expect_that!(value, contains(ge(&3)));
# googletest::internal::test_outcome::TestOutcome::close_current_test_outcome::<&str>(Ok(()))
#     .unwrap();
}
# contains_at_least_one_item_at_least_3();
```

They can also be logically combined, with methods from [`MatcherBase`]:

```
use googletest::prelude::*;

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[gtest]
# */
fn strictly_between_9_and_11() {
# googletest::internal::test_outcome::TestOutcome::init_current_test_outcome();
    let value = 10;
    expect_that!(value, gt(9).and(not(ge(11))));
# googletest::internal::test_outcome::TestOutcome::close_current_test_outcome::<&str>(Ok(()))
#     .unwrap();
}
# strictly_between_9_and_11();
```

## Available matchers

The following matchers are provided in GoogleTest Rust:

| Matcher              | What it matches                                                          |
|----------------------|--------------------------------------------------------------------------|
| [`all!`]             | Anything matched by all given matchers.                                  |
| [`any!`]             | Anything matched by at least one of the given matchers.                  |
| [`anything`]         | Any input.                                                               |
| [`approx_eq`]        | A floating point number within a standard tolerance of the argument.     |
| [`char_count`]       | A string with a Unicode scalar count matching the argument.              |
| [`container_eq`]     | Same as [`eq`], but for containers (with a better mismatch description). |
| [`contains`]         | A container containing an element matched by the given matcher.          |
| [`contains_each!`]   | A container containing distinct elements each of the arguments match.    |
| [`contains_regex`]   | A string containing a substring matching the given regular expression.   |
| [`contains_substring`] | A string containing the given substring.                               |
| [`derefs_to`]        | A [`Deref`] which `deref()`s to a value that the argument matches.       |
| [`displays_as`]      | A [`Display`] value whose formatted string is matched by the argument.   |
| [`each`]             | A container all of whose elements the given argument matches.            |
| [`elements_are!`]    | A container whose elements the arguments match, in order.                |
| [`ends_with`]        | A string ending with the given suffix.                                   |
| [`eq`]               | A value equal to the argument, in the sense of the [`PartialEq`] trait.  |
| [`err`]              | A [`Result`][std::result::Result] containing an `Err` variant the argument matches. |
| [`field!`]           | A struct or enum with a given field whose value the argument matches.    |
| [`ge`]               | A [`PartialOrd`] value greater than or equal to the given value.         |
| [`gt`]               | A [`PartialOrd`] value strictly greater than the given value.            |
| [`has_entry`]        | A [`HashMap`] containing a given key whose value the argument matches.   |
| [`is_contained_in!`] | A container each of whose elements is matched by some given matcher.     |
| [`is_empty`]         | An empty collection.                                                     |
| [`is_err`]           | Alias for [`err`].                                                       |
| [`is_finite`]        | A floating point number which is neither infinite nor NaN.               |
| [`is_infinite`]      | A floating point number which is positive or negative infinity.          |
| [`is_nan`]           | A floating point number which is NaN.                                    |
| [`is_ok`]            | Alias for [`ok`].                                                        |
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
| [`points_to`]        | A reference `&` which points to a value that the argument matches.       |
| [`pointwise!`]       | A container whose contents the arguments match in a pointwise fashion.   |
| [`predicate`]        | A value on which the given predicate returns true.                       |
| [`some`]             | An [`Option`] containing `Some` whose value the argument matches.        |
| [`starts_with`]      | A string starting with the given prefix.                                 |
| [`subset_of`]        | A container all of whose elements are contained in the argument.         |
| [`superset_of`]      | A container containing all elements of the argument.                     |
| [`unordered_elements_are!`] | A container whose elements the arguments match, in any order.     |

[`all!`]: matchers::all
[`any!`]: matchers::any
[`anything`]: matchers::anything
[`approx_eq`]: matchers::approx_eq
[`char_count`]: matchers::char_count
[`container_eq`]: matchers::container_eq
[`contains`]: matchers::contains
[`contains_each!`]: matchers::contains_each
[`contains_regex`]: matchers::contains_regex
[`contains_substring`]: matchers::contains_substring
[`displays_as`]: matchers::displays_as
[`derefs_to`]: matchers::derefs_to
[`each`]: matchers::each
[`elements_are!`]: matchers::elements_are
[`ends_with`]: matchers::ends_with
[`eq`]: matchers::eq
[`err`]: matchers::err
[`field!`]: matchers::field
[`ge`]: matchers::ge
[`gt`]: matchers::gt
[`has_entry`]: matchers::has_entry
[`is_contained_in!`]: matchers::is_contained_in
[`is_empty`]: matchers::is_empty
[`is_err`]: matchers::is_err
[`is_finite`]: matchers::is_finite
[`is_infinite`]: matchers::is_infinite
[`is_nan`]: matchers::is_nan
[`is_ok`]: matchers::is_ok
[`le`]: matchers::le
[`len`]: matchers::len
[`lt`]: matchers::lt
[`matches_regex`]: matchers::matches_regex
[`matches_pattern!`]: matchers::matches_pattern
[`near`]: matchers::near
[`none`]: matchers::none
[`not`]: matchers::not
[`pat!`]: matchers::pat
[`ok`]: matchers::ok
[`points_to`]: matchers::points_to
[`pointwise!`]: matchers::pointwise
[`predicate`]: matchers::predicate
[`property!`]: matchers::property
[`some`]: matchers::some
[`starts_with`]: matchers::starts_with
[`subset_of`]: matchers::subset_of
[`superset_of`]: matchers::superset_of
[`unordered_elements_are!`]: matchers::unordered_elements_are
[`Copy`]: std::marker::Copy
[`Deref`]: std::ops::Deref
[`Display`]: std::fmt::Display
[`HashMap`]: std::collections::HashMap
[`Option`]: std::option::Option
[`PartialEq`]: std::cmp::PartialEq
[`PartialOrd`]: std::cmp::PartialOrd

## Writing matchers

One can extend the library by writing additional matchers. To do so, create
a struct holding the matcher's data and have it implement the traits
[`Matcher`] and  [`MatcherBase`]:

```no_run
use googletest::{description::Description, matcher::{Matcher, MatcherBase, MatcherResult}};
use std::fmt::Debug;

#[derive(MatcherBase)]
struct MyEqMatcher<T> {
    expected: T,
}

impl<T: PartialEq + Debug> Matcher<T> for MyEqMatcher<T> {
    fn matches(&self, actual: &T) -> MatcherResult {
        if self.expected == *actual {
            MatcherResult::Match
        } else {
            MatcherResult::NoMatch
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("is equal to {:?} the way I define it", self.expected).into()
            }
            MatcherResult::NoMatch => {
                format!("isn't equal to {:?} the way I define it", self.expected).into()
            }
        }
    }
}
```

 It is recommended to expose a function which constructs the matcher:

 ```no_run
 # use googletest::{description::Description, matcher::{Matcher, MatcherBase, MatcherResult}};
 # use std::fmt::Debug;
 # #[derive(MatcherBase)]
 # struct MyEqMatcher<T> {
 #    expected: T,
 # }
 #
 # impl<T: PartialEq + Debug> Matcher<T> for MyEqMatcher<T> {
 #    fn matches(&self, actual: &T) -> MatcherResult {
 #        if self.expected == *actual {
 #            MatcherResult::Match
 #        } else {
 #            MatcherResult::NoMatch
 #        }
 #    }
 #
 #    fn describe(&self, matcher_result: MatcherResult) -> Description {
 #        match matcher_result {
 #            MatcherResult::Match => {
 #                format!("is equal to {:?} the way I define it", self.expected).into()
 #            }
 #            MatcherResult::NoMatch => {
 #                format!("isn't equal to {:?} the way I define it", self.expected).into()
 #            }
 #        }
 #    }
 # }
 #
 pub fn eq_my_way<T: PartialEq + Debug>(expected: T) -> impl Matcher<T> {
    MyEqMatcher { expected }
 }
 ```

 The new matcher can then be used in the assertion macros:

```
# use googletest::prelude::*;
# use googletest::{description::Description, matcher::{Matcher, MatcherBase, MatcherResult}};
# use std::fmt::Debug;
# #[derive(MatcherBase)]
# struct MyEqMatcher<T> {
#    expected: T,
# }
#
# impl<T: PartialEq + Debug> Matcher<T> for MyEqMatcher<T> {
#    fn matches(&self, actual: &T) -> MatcherResult {
#        if self.expected == *actual {
#            MatcherResult::Match
#        } else {
#            MatcherResult::NoMatch
#        }
#    }
#
#    fn describe(&self, matcher_result: MatcherResult) -> Description {
#        match matcher_result {
#            MatcherResult::Match => {
#                format!("is equal to {:?} the way I define it", self.expected).into()
#            }
#            MatcherResult::NoMatch => {
#                format!("isn't equal to {:?} the way I define it", self.expected).into()
#            }
#        }
#    }
# }
#
# pub fn eq_my_way<T: PartialEq + Debug>(expected: T) -> impl Matcher<T> {
#    MyEqMatcher { expected }
# }
# /* The attribute macro would prevent the function from being compiled in a doctest.
#[gtest]
# */
fn should_be_equal_by_my_definition() {
# googletest::internal::test_outcome::TestOutcome::init_current_test_outcome();
    expect_that!(10, eq_my_way(10));
# googletest::internal::test_outcome::TestOutcome::close_current_test_outcome::<&str>(Ok(()))
#     .unwrap();
}
# should_be_equal_by_my_definition();
```

### Matching both values and references

[`Matcher::matches`] receives the actual value by reference, so a matcher does
not need its actual type to be [`Copy`]. Note, however, that `Matcher<T>` and
`Matcher<&T>` are two distinct traits. Which one an assertion needs depends on
where the matcher is used:

* `verify_that!(value, matcher)` needs `Matcher<T>` when `value` is a `T` and
  `Matcher<&T>` when `value` is a `&T`.
* A matcher nested inside another matcher is given whatever its parent hands to
  it. Matchers such as [`matches_pattern!`], [`field!`] and [`property!`] hand
  their inner matchers a reference when the type they destructure is not
  `Copy`.

A matcher which is generic over its actual type, such as `MyEqMatcher` above,
works in both cases with a single implementation, since `T` can itself be a
reference type.

A matcher which names a concrete outer type and delegates to an inner matcher
is different. In some cases one must implement [`Matcher`] twice: once for the
value and once for the reference. Otherwise the matcher compiles when used at
the top level of an assertion on an owned value but fails to compile, with an
error such as ``the trait `Matcher<&Wrapper<_>>` is not implemented``, when it
is used on a reference or nested inside a matcher which hands it a reference.

```
use googletest::prelude::*;
use googletest::{description::Description, matcher::{Matcher, MatcherBase, MatcherResult}};
use std::fmt::Debug;

#[derive(Debug)]
struct Wrapper<T>(T);

#[derive(MatcherBase)]
struct WrapsMatcher<InnerMatcherT> {
    inner: InnerMatcherT,
}

// Matches a `Wrapper<T>`. The inner matcher must match a `T`.
impl<T: Debug, InnerMatcherT: Matcher<T>> Matcher<Wrapper<T>> for WrapsMatcher<InnerMatcherT> {
    fn matches(&self, actual: &Wrapper<T>) -> MatcherResult {
        self.inner.matches(&actual.0)
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        format!("wraps a value which {}", self.inner.describe(matcher_result)).into()
    }
}

// Matches a `&Wrapper<T>`. The inner matcher must match a `&T`.
impl<'a, T: Debug, InnerMatcherT: Matcher<&'a T>> Matcher<&'a Wrapper<T>>
    for WrapsMatcher<InnerMatcherT>
{
    fn matches(&self, actual: &&'a Wrapper<T>) -> MatcherResult {
        self.inner.matches(&&actual.0)
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        format!("wraps a value which {}", self.inner.describe(matcher_result)).into()
    }
}

fn wraps<InnerMatcherT>(inner: InnerMatcherT) -> WrapsMatcher<InnerMatcherT> {
    WrapsMatcher { inner }
}

#[derive(Debug)]
struct Holder {
    wrapper: Wrapper<String>,
}

# fn should_pass() -> Result<()> {
// Uses the implementation for `Wrapper<T>`.
verify_that!(Wrapper(1), wraps(eq(1)))?;
// Both of these use the implementation for `&Wrapper<T>`: the first is given a
// reference directly, the second is handed one by `matches_pattern!` because
// `Holder` is not `Copy`.
verify_that!(&Wrapper(1), wraps(eq(&1)))?;
verify_that!(
    Holder { wrapper: Wrapper("a".to_string()) },
    matches_pattern!(Holder { wrapper: wraps(eq("a")) })
)?;
#     Ok(())
# }
# should_pass().unwrap();
```

The matchers in this crate which wrap a concrete type, such as [`some`],
[`ok`], [`err`], [`field!`] and [`property!`], are implemented in this way.

## Non-fatal assertions

Using non-fatal assertions, a single test is able to log multiple assertion
failures. Any single assertion failure causes the test to be considered
having failed, but execution continues until the test completes or otherwise
aborts.

To make a non-fatal assertion, use the macro [`expect_that!`]. The test must
also be marked with [`gtest`] instead of the Rust-standard `#[test]`.

```no_run
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

```no_run
use googletest::prelude::*;

# /* Make sure this also compiles as a doctest.
#[gtest]
# */
fn failing_non_fatal_assertion() -> Result<()> {
    let value = 2;
    expect_that!(value, eq(3));  // Just marks the test as having failed.
    verify_that!(value, eq(2))?;  // Passes, so does not abort the test.
    Ok(())        // Because of the failing expect_that! call above, the
                  // test fails despite returning Ok(())
}
```

```no_run
use googletest::prelude::*;

#[gtest]
fn failing_fatal_assertion_after_non_fatal_assertion() -> Result<()> {
    let value = 2;
    expect_that!(value, eq(2));  // Passes; test still considered passing.
    verify_that!(value, eq(3))?; // Fails and aborts the test.
    expect_that!(value, eq(3));  // Never executes, since the test already aborted.
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

## Death tests

GoogleTest Rust provides macros to assert that an expression terminates the
current process:

*   [`verify_exit!`] and [`expect_exit!`] assert that an expression terminates
    the process with an exit status matching a given matcher and stderr matching
    a string matcher.
*   [`verify_death!`] and [`expect_death!`] assert that an expression causes the
    process to die (exiting unsuccessfully, either with a non-zero exit code or
    by a signal), with stderr matching a string matcher.

## Conversion from `Result::Err` and `Option::None`

To simplify error management during a test arrangement, [`Result<T>`]
provides a few conversion utilities.

If your setup function returns `std::result::Result<T, E>` where `E: std::error::Error`,
the `std::result::Result<T, E>` can simply be handled with the `?` operator. If an `Err(e)`
is returned, the test will report a failure at the line where the `?` operator has been
applied (or the lowest caller without `#[track_caller]`).

```
# use googletest::prelude::*;
struct PngImage { h: i32, w: i32 /* ... */ }
impl PngImage {
  fn new_from_file(file_name: &str) -> std::result::Result<Self, std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))

   }
  fn rotate(&mut self) { std::mem::swap(&mut self.h, &mut self.w);}
  fn dimensions(&self) -> (i32, i32) { (self.h, self.w)}
}

fn test_png_image_dimensions() -> googletest::Result<()> {
  // Arrange
  let mut png = PngImage::new_from_file("example.png")?;
  verify_eq!(png.dimensions(), (128, 64))?;

  // Act
  png.rotate();

  // Assert
  expect_eq!(png.dimensions(), (64, 128));
  Ok(())
}

# test_png_image_dimensions().unwrap_err();
```

If your setup function returns `Option<T>` or `std::result::Result<T, E>` where
`E: !std::error::Error`, then you can convert these types with `or_fail()`
from the `OrFail` extension trait.

```
# use googletest::prelude::*;
# struct PngImage;
# static PNG_BINARY: [u8;0] = [];

impl PngImage {
  fn new_from_binary(bin: &[u8]) -> std::result::Result<Self, String> {
    Err("Parsing failed".into())
  }
}

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[gtest]
# */
fn test_png_image_binary() -> googletest::Result<()> {
  // Arrange
  let png_image = PngImage::new_from_binary(&PNG_BINARY).or_fail()?;
  /* ... */
  # Ok(())
}
# test_png_image_binary().unwrap_err();

impl PngImage {
  fn new_from_cache(key: u64) -> Option<Self> {
    None
  }
}

# /* The attribute macro would prevent the function from being compiled in a doctest.
#[gtest]
# */
fn test_png_from_cache() -> googletest::Result<()> {
  // Arrange
  let png_image = PngImage::new_from_cache(123).or_fail()?;
  /* ... */
  # Ok(())
}
# test_png_from_cache().unwrap_err();
```


## Multi-threaded tests

Due to internal use of thread-local storage, the `expect_*` family of macros
should only be invoked from the same thread as that running the test itself.
`verify_*` and `assert_*` macros on the other hand may be called from other
threads.

## Integrations with other crates

GoogleTest Rust includes integrations with the
[Proptest](https://crates.io/crates/proptest) crates to simplify turning
GoogleTest assertion failures into Proptest
[`TestCaseError`](https://docs.rs/proptest/latest/proptest/test_runner/enum.TestCaseError.html)
through the `?` operator.

[`and_log_failure()`]: GoogleTestSupport::and_log_failure
[`or_fail()`]: OrFail::or_fail
[`Matcher`]: matcher::Matcher
[`Matcher::matches`]: matcher::Matcher::matches
[`MatcherBase`]: matcher::MatcherBase
