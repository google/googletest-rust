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

use crate::matcher::{MatchExplanation, Matcher, MatcherResult};
use crate::matcher_support::edit_distance;
use std::{fmt::Debug, marker::PhantomData};

/// Matches a value equal (in the sense of `==`) to `expected`.
///
/// The type of `expected` must implement the [`PartialEq`] trait so that the
/// expected and actual values can be compared.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(123, eq(123))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(123, eq(234))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// `expected` to `actual` must be comparable with one another via the
/// [`PartialEq`] trait. In most cases, this means that they must be of the same
/// type. However, there are a few cases where different but closely related
/// types are comparable, for example `String` with `&str`.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(String::from("Some value"), eq("Some value"))?; // Passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// In most cases however, one must convert one of the arguments explicitly.
/// This can be surprising when comparing integer types or references.
///
/// ```compile_fail
/// verify_that!(123u32, eq(123u64))?; // Does not compile
/// verify_that!(123u32 as u64, eq(123u64))?; // Passes
/// ```
///
/// ```ignore
/// let actual: &T = ...;
/// let expected: T = T{...};
/// verify_that(actual, eq(expected))?; // Does not compile
/// verify_that(actual, eq(&expected))?; // Compiles
/// ```
///
/// When matching with string types (`&str` and `String`), one can set more
/// options on how equality is checked through the
/// [`StrMatcherConfigurator`][crate::matchers::str_matcher::StrMatcherConfigurator]
/// extension trait, which is implemented for this matcher.
pub fn eq<A: ?Sized, T>(expected: T) -> EqMatcher<A, T> {
    EqMatcher { expected, phantom: Default::default() }
}

/// A matcher which matches a value equal to `expected`.
///
/// See [`eq`].
pub struct EqMatcher<A: ?Sized, T> {
    pub(crate) expected: T,
    phantom: PhantomData<A>,
}

impl<A: Debug + ?Sized, T: PartialEq<A> + Debug> Matcher for EqMatcher<A, T> {
    type ActualT = A;

    fn matches(&self, actual: &A) -> MatcherResult {
        (self.expected == *actual).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => format!("is equal to {:?}", self.expected),
            MatcherResult::DoesNotMatch => format!("isn't equal to {:?}", self.expected),
        }
    }

    fn explain_match(&self, actual: &A) -> MatchExplanation {
        create_diff(
            &format!("{:#?}", self.expected),
            &format!("{:#?}", actual),
            &self.describe(self.matches(actual)),
        )
    }
}

pub(super) fn create_diff(
    expected_debug: &str,
    actual_debug: &str,
    description: &str,
) -> MatchExplanation {
    if actual_debug.lines().count() < 2 {
        // If the actual debug is only one line, then there is no point in doing a
        // line-by-line diff.
        return MatchExplanation::create(format!("which {description}",));
    }
    let edit_list = edit_distance::edit_list(actual_debug.lines(), expected_debug.lines());

    if edit_list.is_empty() {
        return MatchExplanation::create(format!(
            "which {description}\nNo difference found between debug strings.",
        ));
    }

    MatchExplanation::create(format!(
        "which {description}\nDebug diff:{}",
        edit_list_summary(&edit_list)
    ))
}

fn edit_list_summary(edit_list: &[edit_distance::Edit<&str>]) -> String {
    let mut summary = String::new();
    for edit in edit_list {
        summary.push('\n');
        match edit {
            edit_distance::Edit::Both { left, distance, .. } if *distance == 0.0 => {
                summary.push(' ');
                summary.push_str(left);
            }
            edit_distance::Edit::Both { left, right, .. } => {
                summary.push('+');
                summary.push_str(left);
                summary.push('\n');
                summary.push('-');
                summary.push_str(right);
            }
            edit_distance::Edit::ExtraLeft { left } => {
                summary.push('+');
                summary.push_str(left);
            }
            edit_distance::Edit::ExtraRight { right } => {
                summary.push('-');
                summary.push_str(right);
            }
        }
    }
    summary
}

#[cfg(test)]
mod tests {
    use super::eq;
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn eq_matches_string_reference_with_string_reference() -> Result<()> {
        verify_that!("A string", eq("A string"))
    }

    #[test]
    fn eq_matches_owned_string_with_string_reference() -> Result<()> {
        let value = "A string".to_string();
        verify_that!(value, eq("A string"))
    }

    #[test]
    fn eq_matches_owned_string_reference_with_string_reference() -> Result<()> {
        let value = "A string".to_string();
        verify_that!(&value, eq("A string"))
    }

    #[test]
    fn eq_matches_i32_with_i32() -> Result<()> {
        verify_that!(123, eq(123))
    }

    #[test]
    fn eq_struct_debug_diff() -> Result<()> {
        #[derive(Debug, PartialEq)]
        struct Strukt {
            int: i32,
            string: String,
        }

        let result = verify_that!(
            Strukt { int: 123, string: "something".into() },
            eq(Strukt { int: 321, string: "someone".into() })
        );
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {
            r#"
            Value of: Strukt { int: 123, string: "something".into() }
            Expected: is equal to Strukt { int: 321, string: "someone" }
            Actual: Strukt {
                int: 123,
                string: "something",
            }, which isn't equal to Strukt { int: 321, string: "someone" }
            Debug diff:
             Strukt {
            +    int: 123,
            -    int: 321,
            +    string: "something",
            -    string: "someone",
             }
            "#})))
        )
    }

    #[test]
    fn eq_vec_debug_diff() -> Result<()> {
        let result = verify_that!(vec![1, 2, 3], eq(vec![1, 3, 4]));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {
            r#"
            Value of: vec![1, 2, 3]
            Expected: is equal to [1, 3, 4]
            Actual: [
                1,
                2,
                3,
            ], which isn't equal to [1, 3, 4]
            Debug diff:
             [
                 1,
            +    2,
                 3,
            -    4,
             ]
            "#})))
        )
    }

    #[test]
    fn eq_vec_debug_diff_length_mismatch() -> Result<()> {
        let result = verify_that!(vec![1, 2, 3, 4, 5], eq(vec![1, 3, 5]));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {
            r#"
            Value of: vec![1, 2, 3, 4, 5]
            Expected: is equal to [1, 3, 5]
            Actual: [
                1,
                2,
                3,
                4,
                5,
            ], which isn't equal to [1, 3, 5]
            Debug diff:
             [
                 1,
            +    2,
                 3,
            +    4,
                 5,
             ]
            "#})))
        )
    }

    #[test]
    fn eq_multi_line_string_debug_diff() -> Result<()> {
        let result = verify_that!("One\nTwo\nThree", eq("One\nSix\nThree"));
        // TODO: b/257454450 - Make this more useful, by potentially unescaping the
        // line return.
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {
            r#"
            Value of: "One\nTwo\nThree"
            Expected: is equal to "One\nSix\nThree"
            Actual: "One\nTwo\nThree", which isn't equal to "One\nSix\nThree"
            "#})))
        )
    }
}
