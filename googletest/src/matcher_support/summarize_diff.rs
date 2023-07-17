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

#![doc(hidden)]

use crate::matcher_support::edit_distance;
use crate::matcher_support::line_style::*;
#[rustversion::since(1.70)]
use std::{
    borrow::Cow,
    fmt::{Display, Write},
};

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
            edit_list.into_iter().collect::<BufferedSummary>(),
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
                edit_list.into_iter().collect::<BufferedSummary>(),
            )
            .into()
        }
        edit_distance::Difference::Unrelated => "".into(),
    }
}

// Aggregator collecting the lines to be printed in the difference summary.
//
// This is buffered in order to allow a future line to potentially impact how
// the current line would be printed.
struct BufferedSummary<'a> {
    summary: String,
    buffer: Buffer<'a>,
}

impl<'a> BufferedSummary<'a> {
    // Appends a new line which is common to both actual and expected.
    fn feed_common_lines(&mut self, common_line: &'a str) {
        if let Buffer::CommonLineBuffer(ref mut common_lines) = self.buffer {
            common_lines.push(common_line);
        } else {
            self.flush_buffer();
            self.buffer = Buffer::CommonLineBuffer(vec![common_line]);
        }
    }

    // Appends a new line which is found only in the actual string.
    fn feed_extra_actual(&mut self, extra_actual: &'a str) {
        if let Buffer::ExtraExpectedBuffer(extra_expected) = self.buffer {
            self.print_inline_diffs(extra_actual, extra_expected);
            self.buffer = Buffer::Empty;
        } else {
            self.flush_buffer();
            self.buffer = Buffer::ExtraActualBuffer(extra_actual);
        }
    }

    // Appends a new line which is found only in the expected string.
    fn feed_extra_expected(&mut self, extra_expected: &'a str) {
        if let Buffer::ExtraActualBuffer(extra_actual) = self.buffer {
            self.print_inline_diffs(extra_actual, extra_expected);
            self.buffer = Buffer::Empty;
        } else {
            self.flush_buffer();
            self.buffer = Buffer::ExtraExpectedBuffer(extra_expected);
        }
    }

    // Appends a comment for the additional line at the start or the end of the
    // actual string which should be omitted.
    fn feed_additional_actual(&mut self) {
        self.flush_buffer();
        write!(
            &mut self.summary,
            "\n{}",
            LineStyle::comment_style().style("<---- remaining lines omitted ---->")
        )
        .unwrap();
    }

    fn flush_buffer(&mut self) {
        self.buffer.flush(&mut self.summary).unwrap();
    }

    fn print_inline_diffs(&mut self, actual_line: &str, expected_line: &str) {
        let line_edits = edit_distance::edit_list(
            actual_line.chars(),
            expected_line.chars(),
            edit_distance::Mode::Exact,
        );

        if let edit_distance::Difference::Editable(edit_list) = line_edits {
            let mut actual_summary = HighlightedString::new();
            let mut expected_summary = HighlightedString::new();
            for edit in &edit_list {
                match edit {
                    edit_distance::Edit::ExtraActual(c) => actual_summary.push_highlighted(*c),
                    edit_distance::Edit::ExtraExpected(c) => expected_summary.push_highlighted(*c),
                    edit_distance::Edit::Both(c) => {
                        actual_summary.push(*c);
                        expected_summary.push(*c);
                    }
                    edit_distance::Edit::AdditionalActual => {
                        panic!("This should not happen. This is a bug in gtest_rust")
                    }
                }
            }
            write!(
                &mut self.summary,
                "\n{}",
                LineStyle::extra_actual_style().style_highlighted(actual_summary)
            )
            .unwrap();
            write!(
                &mut self.summary,
                "\n{}",
                LineStyle::extra_expected_style().style_highlighted(expected_summary)
            )
            .unwrap();
        } else {
            write!(&mut self.summary, "\n{}", LineStyle::extra_actual_style().style(actual_line))
                .unwrap();
            write!(
                &mut self.summary,
                "\n{}",
                LineStyle::extra_expected_style().style(expected_line)
            )
            .unwrap();
        }
    }
}

impl<'a> FromIterator<edit_distance::Edit<&'a str>> for BufferedSummary<'a> {
    fn from_iter<T: IntoIterator<Item = edit_distance::Edit<&'a str>>>(iter: T) -> Self {
        let mut buffered_summary =
            BufferedSummary { summary: String::new(), buffer: Buffer::CommonLineBuffer(vec![]) };
        for edit in iter {
            match edit {
                edit_distance::Edit::Both(same) => {
                    buffered_summary.feed_common_lines(same);
                }
                edit_distance::Edit::ExtraActual(actual) => {
                    buffered_summary.feed_extra_actual(actual);
                }
                edit_distance::Edit::ExtraExpected(expected) => {
                    buffered_summary.feed_extra_expected(expected);
                }
                edit_distance::Edit::AdditionalActual => {
                    buffered_summary.feed_additional_actual();
                }
            };
        }
        buffered_summary.flush_buffer();

        buffered_summary
    }
}

impl<'a> Display for BufferedSummary<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !matches!(self.buffer, Buffer::Empty) {
            panic!("Buffer is not empty. This is a bug in gtest_rust.")
        }
        self.summary.fmt(f)
    }
}

enum Buffer<'a> {
    Empty,
    CommonLineBuffer(Vec<&'a str>),
    ExtraActualBuffer(&'a str),
    ExtraExpectedBuffer(&'a str),
}

impl<'a> Buffer<'a> {
    fn flush(&mut self, mut writer: impl std::fmt::Write) -> std::fmt::Result {
        match self {
            Buffer::CommonLineBuffer(common_lines) => {
                Self::flush_common_lines(std::mem::take(common_lines), writer)?
            }
            Buffer::Empty => {}
            Buffer::ExtraActualBuffer(extra_actual) => {
                write!(writer, "\n{}", LineStyle::extra_actual_style().style(extra_actual))?
            }
            Buffer::ExtraExpectedBuffer(extra_expected) => {
                write!(writer, "\n{}", LineStyle::extra_expected_style().style(extra_expected))?
            }
        };
        *self = Buffer::Empty;
        Ok(())
    }

    fn flush_common_lines(
        common_lines: Vec<&'a str>,
        mut writer: impl std::fmt::Write,
    ) -> std::fmt::Result {
        // The number of the lines kept before and after the compressed lines.
        const COMMON_LINES_CONTEXT_SIZE: usize = 2;

        if common_lines.len() <= 2 * COMMON_LINES_CONTEXT_SIZE + 1 {
            for line in common_lines {
                write!(writer, "\n{}", LineStyle::unchanged_style().style(line))?;
            }
            return Ok(());
        }

        let start_context = &common_lines[0..COMMON_LINES_CONTEXT_SIZE];

        for line in start_context {
            write!(writer, "\n{}", LineStyle::unchanged_style().style(line))?;
        }

        write!(
            writer,
            "\n{}",
            LineStyle::comment_style().style(&format!(
                "<---- {} common lines omitted ---->",
                common_lines.len() - 2 * COMMON_LINES_CONTEXT_SIZE
            )),
        )?;

        let end_context =
            &common_lines[common_lines.len() - COMMON_LINES_CONTEXT_SIZE..common_lines.len()];

        for line in end_context {
            write!(writer, "\n{}", LineStyle::unchanged_style().style(line))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher_support::line_style::NO_COLOR_VAR;
    use crate::{matcher_support::edit_distance::Mode, prelude::*};
    use indoc::indoc;

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
    fn create_diff_exact_small_difference() -> Result<()> {
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

    #[test]
    fn create_diff_exact_small_difference_no_color() -> Result<()> {
        std::env::set_var("NO_COLOR", "1");

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
