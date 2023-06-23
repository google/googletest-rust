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

use crate::internal::abbreviate::abbreviate;
use crate::matcher::{Matcher, MatcherResult};
use crate::matcher_support::edit_distance;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

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
        let formatted_expected = format!("{:?}", self.expected);
        match matcher_result {
            MatcherResult::Matches => format!("is equal to {}", abbreviate(&formatted_expected)),
            MatcherResult::DoesNotMatch => {
                format!("isn't equal to {}", abbreviate(&formatted_expected))
            }
        }
    }

    fn explain_match(&self, actual: &A) -> String {
        let expected_debug = format!("{:#?}", self.expected);
        let actual_debug = format!("{:#?}", actual);
        let description = self.describe(self.matches(actual));

        let diff = if is_multiline_string_debug(&actual_debug)
            && is_multiline_string_debug(&expected_debug)
        {
            create_diff(
                // The two calls below return None if and only if the strings expected_debug
                // respectively actual_debug are not enclosed in ". The calls to
                // is_multiline_string_debug above ensure that they are. So the calls cannot
                // actually return None and unwrap() should not panic.
                &to_display_output(&expected_debug).unwrap(),
                &to_display_output(&actual_debug).unwrap(),
                edit_distance::Mode::Exact,
            )
        } else {
            create_diff(&expected_debug, &actual_debug, edit_distance::Mode::Exact)
        };

        format!("which {description}{diff}")
    }
}

/// Returns a string describing how the expected and actual lines differ.
///
/// This is included in a match explanation for [`EqMatcher`] and
/// [`crate::matchers::str_matcher::StrMatcher`].
///
/// If the actual value has at most two lines, or the two differ by more than
/// the maximum edit distance, then this returns the empty string. If the two
/// are equal, it returns a simple statement that they are equal. Otherwise,
/// this constructs a unified diff view of the actual and expected values.
pub(super) fn create_diff(
    expected_debug: &str,
    actual_debug: &str,
    diff_mode: edit_distance::Mode,
) -> Cow<'static, str> {
    if actual_debug.lines().count() < 2 {
        // If the actual debug is only one line, then there is no point in doing a
        // line-by-line diff.
        return "".into();
    }
    match edit_distance::edit_list(actual_debug.lines(), expected_debug.lines(), diff_mode) {
        edit_distance::Difference::Equal => "No difference found between debug strings.".into(),
        edit_distance::Difference::Editable(edit_list) => {
            format!("\nDifference:{}", edit_list_summary(&edit_list)).into()
        }
        edit_distance::Difference::Unrelated => "".into(),
    }
}

/// Returns a string describing how the expected and actual differ after
/// reversing the lines in each.
///
/// This is similar to [`create_diff`] except that it first reverses the lines
/// in both the expected and actual values, then reverses the constructed edit
/// list. When `diff_mode` is [`edit_distance::Mode::Prefix`], this becomes a
/// diff of the suffix for use by [`ends_with`][crate::matchers::ends_with].
pub(super) fn create_diff_reversed(
    expected_debug: &str,
    actual_debug: &str,
    diff_mode: edit_distance::Mode,
) -> Cow<'static, str> {
    if actual_debug.lines().count() < 2 {
        // If the actual debug is only one line, then there is no point in doing a
        // line-by-line diff.
        return "".into();
    }
    let mut actual_lines_reversed = actual_debug.lines().collect::<Vec<_>>();
    let mut expected_lines_reversed = expected_debug.lines().collect::<Vec<_>>();
    actual_lines_reversed.reverse();
    expected_lines_reversed.reverse();
    match edit_distance::edit_list(actual_lines_reversed, expected_lines_reversed, diff_mode) {
        edit_distance::Difference::Equal => "No difference found between debug strings.".into(),
        edit_distance::Difference::Editable(mut edit_list) => {
            edit_list.reverse();
            format!("\nDifference:{}", edit_list_summary(&edit_list)).into()
        }
        edit_distance::Difference::Unrelated => "".into(),
    }
}

fn edit_list_summary(edit_list: &[edit_distance::Edit<&str>]) -> String {
    let mut summary = String::new();
    // Use to collect common line and compress them.
    let mut common_line_buffer = vec![];
    for edit in edit_list {
        let (start, line) = match edit {
            edit_distance::Edit::Both(left) => {
                common_line_buffer.push(*left);
                continue;
            }
            edit_distance::Edit::ExtraLeft(left) => ("+", *left),
            edit_distance::Edit::ExtraRight(right) => ("-", *right),
            edit_distance::Edit::AdditionalLeft => ("<---- remaining lines omitted ---->", ""),
        };
        summary.push_str(&compress_common_lines(std::mem::take(&mut common_line_buffer)));

        summary.push('\n');
        summary.push_str(start);
        summary.push_str(line);
    }
    summary.push_str(&compress_common_lines(common_line_buffer));

    summary
}

// The number of the lines kept before and after the compressed lines.
const COMMON_LINES_CONTEXT_SIZE: usize = 2;

fn compress_common_lines(common_lines: Vec<&str>) -> String {
    if common_lines.len() <= 2 * COMMON_LINES_CONTEXT_SIZE + 1 {
        let mut all_lines = String::new();
        for line in common_lines {
            all_lines.push('\n');
            all_lines.push(' ');
            all_lines.push_str(line);
        }
        return all_lines;
    }

    let mut truncated_lines = String::new();

    for line in &common_lines[0..COMMON_LINES_CONTEXT_SIZE] {
        truncated_lines.push('\n');
        truncated_lines.push(' ');
        truncated_lines.push_str(line);
    }

    truncated_lines.push_str(&format!(
        "\n<---- {} common lines omitted ---->",
        common_lines.len() - 2 * COMMON_LINES_CONTEXT_SIZE
    ));

    for line in &common_lines[common_lines.len() - COMMON_LINES_CONTEXT_SIZE..common_lines.len()] {
        truncated_lines.push('\n');
        truncated_lines.push(' ');
        truncated_lines.push_str(line);
    }
    truncated_lines
}

fn is_multiline_string_debug(string: &str) -> bool {
    string.starts_with('"')
        && string.ends_with('"')
        && !string.contains('\n')
        && string.contains("\\n")
}

fn to_display_output(string: &str) -> Option<String> {
    Some(string.strip_prefix('"')?.strip_suffix('"')?.split("\\n").collect::<Vec<_>>().join("\n"))
}

#[cfg(test)]
mod tests {
    use super::eq;
    use crate::prelude::*;
    use indoc::indoc;
    use serial_test::serial;

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
            Actual: Strukt { int: 123, string: "something" },
              which isn't equal to Strukt { int: 321, string: "someone" }
            Difference:
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
            Actual: [1, 2, 3],
              which isn't equal to [1, 3, 4]
            Difference:
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
            Actual: [1, 2, 3, 4, 5],
              which isn't equal to [1, 3, 5]
            Difference:
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
    fn eq_debug_diff_common_lines_omitted() -> Result<()> {
        let result = verify_that!((1..50).collect::<Vec<_>>(), eq((3..52).collect::<Vec<_>>()));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {
            "
            Difference:
             [
            +    1,
            +    2,
                 3,
                 4,
            <---- 43 common lines omitted ---->
                 48,
                 49,
            -    50,
            -    51,
             ]"})))
        )
    }

    #[test]
    fn eq_debug_diff_5_common_lines_not_omitted() -> Result<()> {
        let result = verify_that!((1..8).collect::<Vec<_>>(), eq((3..10).collect::<Vec<_>>()));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {
            "
            Difference:
             [
            +    1,
            +    2,
                 3,
                 4,
                 5,
                 6,
                 7,
            -    8,
            -    9,
             ]"})))
        )
    }

    #[test]
    fn eq_debug_diff_start_common_lines_omitted() -> Result<()> {
        let result = verify_that!((1..50).collect::<Vec<_>>(), eq((1..52).collect::<Vec<_>>()));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {
            "
            Difference:
             [
                 1,
            <---- 46 common lines omitted ---->
                 48,
                 49,
            -    50,
            -    51,
             ]"})))
        )
    }

    #[test]
    fn eq_debug_diff_end_common_lines_omitted() -> Result<()> {
        let result = verify_that!((1..52).collect::<Vec<_>>(), eq((3..52).collect::<Vec<_>>()));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {
            "
            Difference:
             [
            +    1,
            +    2,
                 3,
                 4,
            <---- 46 common lines omitted ---->
                 51,
             ]"})))
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
            Actual: "One\nTwo\nThree",
              which isn't equal to "One\nSix\nThree"
            "#})))
        )
    }

    #[test]
    #[serial] // Another test disables abbreviation by environment variable
    fn description_abbreviates_long_expected_value() -> Result<()> {
        let expected_value = "01234567\n".repeat(7);

        let result = verify_that!("", eq(&expected_value));

        verify_that!(
            result,
            err(displays_as(all![
                not(contains_substring(expected_value.as_str())),
                contains_substring(
                    "Expected: is equal to \"01234567\\n01234567\\n01234567\\n…01234567\\n01234567\\n01234567\\n\""
                )
            ]))
        )
    }

    #[test]
    fn match_explanation_contains_diff_of_strings_if_more_than_one_line() -> Result<()> {
        let result = verify_that!(
            indoc!(
                "
                    First line
                    Second line
                    Third line
                "
            ),
            eq(indoc!(
                "
                    First line
                    Second lines
                    Third line
                "
            ))
        );

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                r#"
                     First line
                    +Second line
                    -Second lines
                     Third line
                "#
            ))))
        )
    }

    #[test]
    fn match_explanation_does_not_show_diff_if_actual_value_is_single_line() -> Result<()> {
        let result = verify_that!(
            "First line",
            eq(indoc!(
                "
                    First line
                    Second line
                    Third line
                "
            ))
        );

        verify_that!(result, err(displays_as(not(contains_substring("Difference:")))))
    }

    #[test]
    fn match_explanation_does_not_show_diff_if_expected_value_is_single_line() -> Result<()> {
        let result = verify_that!(
            indoc!(
                "
                    First line
                    Second line
                    Third line
                "
            ),
            eq("First line")
        );

        verify_that!(result, err(displays_as(not(contains_substring("Difference:")))))
    }
}
