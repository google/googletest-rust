// Copyright 2023 Google LLC
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

#[doc(hidden)]
use std::borrow::Cow;
use std::fmt::{Display, Write};

use crate::matcher_support::edit_distance;

/// Environment variable controlling the usage of ansi color in difference
/// summary.
const NO_COLOR_VAR: &str = "GTEST_RUST_NO_COLOR";

/// Returns a string describing how the expected and actual lines differ.
///
/// This is included in a match explanation for [`EqMatcher`] and
/// [`crate::matchers::str_matcher::StrMatcher`].
///
/// If the actual value has less than two lines, or the two differ by more than
/// the maximum edit distance, then this returns the empty string. If the two
/// are equal, it returns a simple statement that they are equal. Otherwise,
/// this constructs a unified diff view of the actual and expected values.
pub(crate) fn create_diff(
    actual_debug: &str,
    expected_debug: &str,
    diff_mode: edit_distance::Mode,
) -> Cow<'static, str> {
    if actual_debug.lines().count() < 2 {
        // If the actual debug is only one line, then there is no point in doing a
        // line-by-line diff.
        return "".into();
    }
    match edit_distance::edit_list(actual_debug.lines(), expected_debug.lines(), diff_mode) {
        edit_distance::Difference::Equal => "No difference found between debug strings.".into(),
        edit_distance::Difference::Editable(edit_list) => format!(
            "\nDifference({} / {}):{}",
            LineStyle::extra_actual_style().style("actual"),
            LineStyle::extra_expected_style().style("expected"),
            edit_list_summary(&edit_list)
        )
        .into(),
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
pub(crate) fn create_diff_reversed(
    actual_debug: &str,
    expected_debug: &str,
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
            format!(
                "\nDifference({} / {}):{}",
                LineStyle::extra_actual_style().style("actual"),
                LineStyle::extra_expected_style().style("expected"),
                edit_list_summary(&edit_list)
            )
            .into()
        }
        edit_distance::Difference::Unrelated => "".into(),
    }
}

fn edit_list_summary(edit_list: &[edit_distance::Edit<&str>]) -> String {
    let mut summary = String::new();
    // Use to collect common line and compress them.
    let mut common_line_buffer = vec![];
    for edit in edit_list {
        let (style, line) = match edit {
            edit_distance::Edit::Both(same) => {
                common_line_buffer.push(*same);
                continue;
            }
            edit_distance::Edit::ExtraActual(actual) => (LineStyle::extra_actual_style(), *actual),
            edit_distance::Edit::ExtraExpected(expected) => {
                (LineStyle::extra_expected_style(), *expected)
            }
            edit_distance::Edit::AdditionalActual => {
                (LineStyle::comment_style(), "<---- remaining lines omitted ---->")
            }
        };
        summary.push_str(&compress_common_lines(std::mem::take(&mut common_line_buffer)));

        write!(&mut summary, "\n{}", style.style(line)).unwrap();
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
            write!(&mut all_lines, "\n{}", LineStyle::unchanged_style().style(line)).unwrap();
        }
        return all_lines;
    }

    let mut truncated_lines = String::new();

    for line in &common_lines[0..COMMON_LINES_CONTEXT_SIZE] {
        write!(&mut truncated_lines, "\n{}", LineStyle::unchanged_style().style(line)).unwrap();
    }

    write!(
        &mut truncated_lines,
        "\n{}",
        LineStyle::comment_style().style(&format!(
            "<---- {} common lines omitted ---->",
            common_lines.len() - 2 * COMMON_LINES_CONTEXT_SIZE
        )),
    )
    .unwrap();

    for line in &common_lines[common_lines.len() - COMMON_LINES_CONTEXT_SIZE..common_lines.len()] {
        write!(&mut truncated_lines, "\n{}", LineStyle::unchanged_style().style(line)).unwrap();
    }
    truncated_lines
}

// See
struct LineStyle {
    ansi_prefix: &'static str,
    ansi_suffix: &'static str,
    header: char,
}

impl LineStyle {
    // Font in red and bold
    fn extra_actual_style() -> Self {
        Self { ansi_prefix: "\x1B[1;31m", ansi_suffix: "\x1B[0m", header: '-' }
    }

    // Font in green and bold
    fn extra_expected_style() -> Self {
        Self { ansi_prefix: "\x1B[1;32m", ansi_suffix: "\x1B[0m", header: '+' }
    }

    // Font in italic
    fn comment_style() -> Self {
        Self { ansi_prefix: "\x1B[3m", ansi_suffix: "\x1B[0m", header: ' ' }
    }

    // No ansi styling
    fn unchanged_style() -> Self {
        Self { ansi_prefix: "", ansi_suffix: "", header: ' ' }
    }

    fn style(self, line: &str) -> StyledLine<'_> {
        StyledLine { style: self, line }
    }
}

struct StyledLine<'a> {
    style: LineStyle,
    line: &'a str,
}

impl<'a> Display for StyledLine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if std::env::var(NO_COLOR_VAR).is_err() {
            write!(
                f,
                "{}{}{}{}",
                self.style.header, self.style.ansi_prefix, self.line, self.style.ansi_suffix
            )
        } else {
            write!(f, "{}{}", self.style.header, self.line)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matcher_support::edit_distance::Mode, prelude::*};
    use indoc::indoc;
    use serial_test::serial;

    #[must_use]
    fn remove_var() -> TempVar {
        let old_value = std::env::var(NO_COLOR_VAR);
        std::env::remove_var(NO_COLOR_VAR);
        TempVar(old_value.ok())
    }

    #[must_use]
    fn set_var(var: &str) -> TempVar {
        let old_value = std::env::var(NO_COLOR_VAR);
        std::env::set_var(NO_COLOR_VAR, var);
        TempVar(old_value.ok())
    }
    struct TempVar(Option<String>);

    impl Drop for TempVar {
        fn drop(&mut self) {
            match &self.0 {
                Some(old_var) => std::env::set_var(NO_COLOR_VAR, old_var),
                None => std::env::remove_var(NO_COLOR_VAR),
            }
        }
    }

    // Make a long text with each element of the iterator on one line.
    // `collection` must contains at least one element.
    fn build_text<T: Display>(mut collection: impl Iterator<Item = T>) -> String {
        let mut text = String::new();
        write!(&mut text, "{}", collection.next().expect("Provided collection without elements"))
            .unwrap();
        for item in collection {
            write!(&mut text, "\n{}", item).unwrap();
        }
        text
    }

    #[test]
    fn create_diff_smaller_than_one_line() -> Result<()> {
        verify_that!(create_diff("One", "Two", Mode::Exact), eq(""))
    }

    #[test]
    fn create_diff_exact_same() -> Result<()> {
        let expected = indoc! {"
            One
            Two
            "};
        let actual = indoc! {"
        One
        Two
        "};
        verify_that!(
            create_diff(expected, actual, Mode::Exact),
            eq("No difference found between debug strings.")
        )
    }

    #[test]
    fn create_diff_exact_unrelated() -> Result<()> {
        verify_that!(create_diff(&build_text(1..500), &build_text(501..1000), Mode::Exact), eq(""))
    }

    #[test]
    #[serial]
    fn create_diff_exact_small_difference() -> Result<()> {
        let _cleanup = remove_var();

        verify_that!(
            create_diff(&build_text(1..50), &build_text(1..51), Mode::Exact),
            eq(indoc! {
                "

                Difference(-\x1B[1;31mactual\x1B[0m / +\x1B[1;32mexpected\x1B[0m):
                 1
                 2
                 \x1B[3m<---- 45 common lines omitted ---->\x1B[0m
                 48
                 49
                +\x1B[1;32m50\x1B[0m"
            })
        )
    }

    #[test]
    #[serial]
    fn create_diff_exact_small_difference_no_color() -> Result<()> {
        let _cleanup = set_var("NO_COLOR");

        verify_that!(
            create_diff(&build_text(1..50), &build_text(1..51), Mode::Exact),
            eq(indoc! {
                "

                Difference(-actual / +expected):
                 1
                 2
                 <---- 45 common lines omitted ---->
                 48
                 49
                +50"
            })
        )
    }
}
