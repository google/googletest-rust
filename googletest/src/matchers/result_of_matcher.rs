// Copyright 2024 Google LLC
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

/// Matches a value where the result of `callable` applied to the value matches
/// the inner matcher.
///
/// The `callable` will be called twice, so make sure it is pure.
/// ```
/// use googletest::prelude::*;
/// fn should_pass() -> googletest::Result<()> {
///    verify_that!(100, result_of!(|value| value + 1, eq(101)))?; // Passes
///    Ok(())
/// }
///
/// fn should_fail() -> googletest::Result<()> {
///    verify_that!(100, result_of!(|value| value * 2, eq(100)))?; // Fails
///    Ok(())
/// }
/// should_pass().unwrap();
/// should_fail().unwrap_err();
/// ```
#[macro_export]
macro_rules! __result_of {
    ($function: expr, $matcher: expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::result_of(
            $function,
            $matcher,
            stringify!($function),
        )
    }};
}

/// Matches a value where the reference to the result of `callable` applied to
/// the value matches the inner matcher.
///
/// The `callable` will be called twice, so make sure it is pure.
/// ```
/// use googletest::prelude::*;
/// fn should_pass_1() -> googletest::Result<()> {
///    verify_that!("hello", result_of_ref!(|s: &str| s.to_uppercase(), eq("HELLO")))?; // Passes
///    Ok(())
/// }
///
/// fn should_pass_2() -> googletest::Result<()> {
///    verify_that!(100, result_of_ref!(|value| value + 1, eq(&101)))?; // Passes
///    Ok(())
/// }
///
/// fn should_fail() -> googletest::Result<()> {
///    verify_that!("world", result_of_ref!(|s: &str| s.to_uppercase(), eq("HELLO")))?; // Passes
///    Ok(())
/// }
/// should_pass_1().unwrap();
/// should_pass_2().unwrap();
/// should_fail().unwrap_err();
/// ```
#[macro_export]
macro_rules! __result_of_ref {
    ($function: expr, $matcher: expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::result_of_ref(
            $function,
            $matcher,
            stringify!($function),
        )
    }};
}

/// Items for use only by the declarative macros in this module.
///
/// **For internal use only. API stability is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::description::Description;
    use crate::matcher::{Matcher, MatcherBase, MatcherResult};
    use std::fmt::Debug;

    pub fn result_of<Callable, InnerMatcher>(
        callable: Callable,
        inner_matcher: InnerMatcher,
        callable_description: &'static str,
    ) -> ResultOfMatcher<Callable, InnerMatcher> {
        ResultOfMatcher { callable, inner_matcher, callable_description }
    }

    #[derive(MatcherBase)]
    pub struct ResultOfMatcher<Callable, InnerMatcher> {
        callable: Callable,
        inner_matcher: InnerMatcher,
        callable_description: &'static str,
    }

    impl<I: Copy + Debug, T: Debug + Copy, CallableT: Fn(I) -> T, InnerMatcherT: Matcher<T>>
        Matcher<I> for ResultOfMatcher<CallableT, InnerMatcherT>
    {
        fn matches(&self, actual: I) -> MatcherResult {
            self.inner_matcher.matches((self.callable)(actual))
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            Description::new()
                .text(format!("by applying {},", self.callable_description))
                .nested(self.inner_matcher.describe(matcher_result))
        }

        fn explain_match(&self, actual: I) -> Description {
            let actual_result = (self.callable)(actual);
            Description::new().text(format!("which, results into {actual_result:?}")).nested(
                Description::new()
                    .text(format!("by applying {},", self.callable_description))
                    .nested(self.inner_matcher.explain_match(actual_result)),
            )
        }
    }

    pub fn result_of_ref<Callable, InnerMatcher>(
        callable: Callable,
        inner_matcher: InnerMatcher,
        callable_description: &'static str,
    ) -> ResultOfRefMatcher<Callable, InnerMatcher> {
        ResultOfRefMatcher { callable, inner_matcher, callable_description }
    }
    #[derive(MatcherBase)]
    pub struct ResultOfRefMatcher<Callable, InnerMatcher> {
        callable: Callable,
        inner_matcher: InnerMatcher,
        callable_description: &'static str,
    }

    impl<
            I: Copy + Debug,
            T: Debug,
            Callable: Fn(I) -> T,
            InnerMatcherT: for<'a> Matcher<&'a T>,
        > Matcher<I> for ResultOfRefMatcher<Callable, InnerMatcherT>
    {
        fn matches(&self, actual: I) -> MatcherResult {
            self.inner_matcher.matches(&(self.callable)(actual))
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            Description::new()
                .text(format!("by applying {},", self.callable_description))
                .nested(self.inner_matcher.describe(matcher_result))
        }

        fn explain_match(&self, actual: I) -> Description {
            let actual_result = (self.callable)(actual);
            Description::new().text(format!("which, results into {actual_result:?}")).nested(
                Description::new()
                    .text(format!("by applying {},", self.callable_description))
                    .nested(self.inner_matcher.explain_match(&actual_result)),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::Result;
    use indoc::indoc;

    #[test]
    fn result_of_match_with_value() -> Result<()> {
        verify_that!(1, result_of!(|value| value + 1, eq(2)))
    }

    #[test]
    fn result_of_match_with_value_function() -> Result<()> {
        fn inc_by_one(value: i32) -> i32 {
            value + 1
        }
        verify_that!(1, result_of!(inc_by_one, eq(2)))
    }

    #[test]
    fn result_of_match_with_different_value() -> Result<()> {
        let result = verify_that!(0, result_of!(|value| value - 1, eq(2)));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: 0
                Expected: by applying |value| value - 1,
                  is equal to 2
                Actual: 0,
                  which, results into -1
                    by applying |value| value - 1,
                      which isn't equal to 2
                "
            ))))
        )
    }

    #[test]
    fn result_of_match_with_different_value_block_closure() -> Result<()> {
        let result = verify_that!(0, result_of!(|value| { value - 1 }, eq(2)));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: 0
                Expected: by applying |value| { value - 1 },
                  is equal to 2
                Actual: 0,
                  which, results into -1
                    by applying |value| { value - 1 },
                      which isn't equal to 2
                "
            ))))
        )
    }

    #[test]
    fn result_of_match_with_different_value_multiline_closure() -> Result<()> {
        let result = verify_that!(
            0,
            result_of!(
                |value| {
                    let dec = value - 1;
                    let inc = dec + 1;
                    inc - 2
                },
                eq(2)
            )
        );
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: 0
                Expected: by applying |value| { let dec = value - 1; let inc = dec + 1; inc - 2 },
                  is equal to 2
                Actual: 0,
                  which, results into -2
                    by applying |value| { let dec = value - 1; let inc = dec + 1; inc - 2 },
                      which isn't equal to 2
                "
            ))))
        )
    }

    #[test]
    fn result_of_match_with_different_value_function() -> Result<()> {
        fn dec_by_one(value: i32) -> i32 {
            value - 1
        }
        let result = verify_that!(0, result_of!(dec_by_one, eq(2)));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: 0
                Expected: by applying dec_by_one,
                  is equal to 2
                Actual: 0,
                  which, results into -1
                    by applying dec_by_one,
                      which isn't equal to 2
                "
            ))))
        )
    }

    #[test]
    fn result_of_ref_match_with_string_reference() -> Result<()> {
        verify_that!("hello", result_of_ref!(|s: &str| s.to_uppercase(), eq("HELLO")))
    }

    #[test]
    fn result_of_ref_match_with_string_reference_function() -> Result<()> {
        fn to_upper_case<S: AsRef<str>>(s: S) -> String {
            s.as_ref().to_uppercase()
        }
        verify_that!("hello", result_of_ref!(to_upper_case, eq("HELLO")))
    }

    #[test]
    fn result_of_ref_match_with_copy_types() -> Result<()> {
        verify_that!(100, result_of_ref!(|value| value + 1, eq(&101)))
    }

    #[test]
    fn result_of_ref_match_with_different_value() -> Result<()> {
        let result = verify_that!("world", result_of_ref!(|s: &str| s.to_uppercase(), eq("HELLO")));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                r#"
                Value of: "world"
                Expected: by applying |s: &str| s.to_uppercase(),
                  is equal to "HELLO"
                Actual: "world",
                  which, results into "WORLD"
                    by applying |s: &str| s.to_uppercase(),
                      which isn't equal to "HELLO""#
            ))))
        )
    }

    #[test]
    fn result_of_ref_match_with_different_value_block_closure() -> Result<()> {
        let result =
            verify_that!("world", result_of_ref!(|s: &str| { s.to_uppercase() }, eq("HELLO")));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                r#"
            Value of: "world"
            Expected: by applying |s: &str| { s.to_uppercase() },
              is equal to "HELLO"
            Actual: "world",
              which, results into "WORLD"
                by applying |s: &str| { s.to_uppercase() },
                  which isn't equal to "HELLO"
            "#
            ))))
        )
    }

    #[test]
    fn result_of_ref_match_with_different_value_function() -> Result<()> {
        fn to_upper_case<S: AsRef<str>>(s: S) -> String {
            s.as_ref().to_uppercase()
        }
        let result = verify_that!("world", result_of_ref!(to_upper_case, eq("HELLO")));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                r#"
            Value of: "world"
            Expected: by applying to_upper_case,
              is equal to "HELLO"
            Actual: "world",
              which, results into "WORLD"
                by applying to_upper_case,
                  which isn't equal to "HELLO"
            "#
            ))))
        )
    }

    #[test]
    fn result_of_ref_match_different_with_closure_variable() -> Result<()> {
        let to_upper_case = |s: &str| s.to_uppercase();
        let result = verify_that!("world", result_of_ref!(to_upper_case, eq("HELLO")));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                r#"
                Value of: "world"
                Expected: by applying to_upper_case,
                  is equal to "HELLO"
                Actual: "world",
                  which, results into "WORLD"
                    by applying to_upper_case,
                      which isn't equal to "HELLO"
            "#
            ))))
        )
    }

    #[test]
    fn result_of_ref_match_different_with_method_literal() -> Result<()> {
        let result = verify_that!("world", result_of_ref!(str::to_uppercase, eq("HELLO")));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                r#"
                Value of: "world"
                Expected: by applying str::to_uppercase,
                  is equal to "HELLO"
                Actual: "world",
                  which, results into "WORLD"
                    by applying str::to_uppercase,
                      which isn't equal to "HELLO"
            "#
            ))))
        )
    }

    #[test]
    fn result_of_ref_match_different_with_function_return_closure() -> Result<()> {
        fn upper_case() -> impl Fn(&str) -> String {
            |s: &str| s.to_uppercase()
        }
        let result = verify_that!("world", result_of_ref!(upper_case(), eq("HELLO")));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                r#"
            Value of: "world"
            Expected: by applying upper_case(),
              is equal to "HELLO"
            Actual: "world",
              which, results into "WORLD"
                by applying upper_case(),
                  which isn't equal to "HELLO"
            "#
            ))))
        )
    }

    #[test]
    fn test_describe_simple() -> Result<()> {
        let matcher = result_of!(|x| x + 1, eq(2));
        let description = matcher.describe(matcher.matches(0));
        verify_that!(
            description,
            displays_as(eq(indoc!(
                r#"
        by applying |x| x + 1,
          isn't equal to 2"#
            )))
        )
    }

    #[test]
    fn test_describe_complicated() -> Result<()> {
        let matcher = result_of_ref!(
            |s: &str| s.chars().collect::<Vec<_>>(),
            each(predicate(char::is_ascii_alphabetic))
        );
        let description = matcher.describe(matcher.matches("A quick brown fox"));
        verify_that!(
            description,
            displays_as(eq(indoc!(
                r#"
        by applying |s: &str| s.chars().collect::<Vec<_>>(),
          contains no element that matches"#
            )))
        )
    }
}
