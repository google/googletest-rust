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

/// Matches a value where the result of `callable` applied to the value matches the inner matcher.
///
/// The `callable` will be called twice, so make sure it is pure.
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> googletest::Result<()> {
/// #    verify_that!(100, result_of!(|value| value + 1, eq(101)))?; // Passes
/// #    Ok(())
/// # }
///
/// # fn should_fail() -> googletest::Result<()> {
/// #    verify_that!(100, result_of!(|value| value * 2, eq(100)))?; // Fails
/// #    Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __result_of {
    ($($t: tt)*) => { $crate::result_of_internal!($($t)*) };
}

#[macro_export]
macro_rules! result_of_internal {
    (|$param: ident$(: $t: ty)?| $closure_content: expr, $matcher: expr) => {{
        let closure_description = concat!("|", stringify!($param$(: $t)?), "| ", stringify!($closure_content));
        $crate::matchers::__internal_unstable_do_not_depend_on_these::result_of(
            |$param$(: $t)?| $closure_content, $matcher, closure_description)
    }};

    (|$param: ident$(: $t: ty)?| $closure_content: block, $matcher: expr) => {{
        let closure_description = concat!("|", stringify!($param$(: $t)?), "| ", stringify!($closure_content));
        $crate::matchers::__internal_unstable_do_not_depend_on_these::result_of(
            |$param$(: $t)?| $closure_content, $matcher, closure_description)
    }};

    ($function: ident, $matcher: expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::result_of(
            $function, $matcher, stringify!($function))
    }};
}

/// Matches a value where the reference to the result of `callable` applied to the value matches the inner matcher.
///
/// The `callable` will be called twice, so make sure it is pure.
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass_1() -> googletest::Result<()> {
/// #    verify_that!("hello", result_of_ref!(|s: &str| s.to_uppercase(), eq("HELLO")))?; // Passes
/// #    Ok(())
/// # }
///
/// # fn should_pass_2() -> googletest::Result<()> {
/// #    verify_that!(100, result_of_ref!(|value| value + 1, eq(&101)))?; // Passes
/// #    Ok(())
/// # }
///
/// # fn should_fail() -> googletest::Result<()> {
/// #    verify_that!("world", result_of_ref!(|s: &str| s.to_uppercase(), eq("HELLO")))?; // Passes
/// #    Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_pass_2().unwrap();
/// # should_fail().unwrap_err();
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __result_of_ref {
    ($($t: tt)*) => { $crate::result_of_ref_internal!($($t)*)};
}

#[macro_export]
macro_rules! result_of_ref_internal {
    (|$param: ident$(: $t: ty)?| $closure_content: expr, $matcher: expr) => {{
        let closure_description = concat!("|", stringify!($param$(: $t)?), "| ", stringify!($closure_content));
        $crate::matchers::__internal_unstable_do_not_depend_on_these::result_of_ref(
            |$param$(: $t)?| $closure_content, $matcher, closure_description)
    }};

    (|$param: ident$(: $t: ty)?| $closure_content: block, $matcher: expr) => {{
        let closure_description = concat!("|", stringify!($param$(: $t)?), "| ", stringify!($closure_content));
        $crate::matchers::__internal_unstable_do_not_depend_on_these::result_of_ref(
            |$param$(: $t)?| $closure_content, $matcher, closure_description)
    }};

    ($function: ident, $matcher: expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::result_of_ref(
            $function, $matcher, stringify!($function))
    }};
}

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
            self.inner_matcher.describe(matcher_result)
        }

        fn explain_match(&self, actual: I) -> Description {
            let actual_result = (self.callable)(actual);
            format!(
                "where the result of the callable `{}` is {actual_result:?} which is {}",
                self.callable_description,
                self.describe(self.inner_matcher.matches(actual_result))
            )
            .into()
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
            self.inner_matcher.describe(matcher_result)
        }

        fn explain_match(&self, actual: I) -> Description {
            let actual_result = (self.callable)(actual);
            format!(
                "where the result of the callable `{}` is {actual_result:?} which is {}",
                self.callable_description,
                self.describe(self.inner_matcher.matches(&actual_result))
            )
            .into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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
            err(displays_as(contains_substring(
                "where the result of the callable `|value| value - 1` is -1 which is isn't equal to 2"
            )))
        )
    }

    #[test]
    fn result_of_match_with_different_value_block_closure() -> Result<()> {
        let result = verify_that!(0, result_of!(|value| { value - 1 }, eq(2)));
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "where the result of the callable `|value| { value - 1 }` is -1 which is isn't equal to 2"
            )))
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
            err(displays_as(contains_substring(
                "where the result of the callable `|value| { let dec = value - 1; let inc = dec + 1; inc - 2 }` is -2 which is isn't equal to 2"
            )))
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
            err(displays_as(contains_substring(
                "where the result of the callable `dec_by_one` is -1 which is isn't equal to 2"
            )))
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
            err(displays_as(contains_substring(
                r#"where the result of the callable `|s : &str| s.to_uppercase()` is "WORLD" which is isn't equal to "HELLO""#
            )))
        )
    }

    #[test]
    fn result_of_ref_match_with_different_value_block_closure() -> Result<()> {
        let result =
            verify_that!("world", result_of_ref!(|s: &str| { s.to_uppercase() }, eq("HELLO")));
        verify_that!(
            result,
            err(displays_as(contains_substring(
                r#"where the result of the callable `|s : &str| { s.to_uppercase() }` is "WORLD" which is isn't equal to "HELLO""#
            )))
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
            err(displays_as(contains_substring(
                r#"where the result of the callable `to_upper_case` is "WORLD" which is isn't equal to "HELLO""#
            )))
        )
    }
}
