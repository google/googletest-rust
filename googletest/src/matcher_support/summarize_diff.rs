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
#[rustversion::since(1.70)]
use std::io::IsTerminal;
use std::{borrow::Cow, fmt::Display};

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
        edit_distance::Difference::Editable(edit_list) => {
            format!("\n{}{}", summary_header(), edit_list.into_iter().collect::<BufferedSummary>(),)
                .into()
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
            format!("\n{}{}", summary_header(), edit_list.into_iter().collect::<BufferedSummary>(),)
                .into()
        }
        edit_distance::Difference::Unrelated => "".into(),
    }
}

// Produces the header, with or without coloring depending on
// stdout_supports_color()
fn summary_header() -> Cow<'static, str> {
    if stdout_supports_color() {
        format!(
            "Difference(-{ACTUAL_ONLY_STYLE}actual{RESET_ALL} / +{EXPECTED_ONLY_STYLE}expected{RESET_ALL}):"
        ).into()
    } else {
        "Difference(-actual / +expected):".into()
    }
}

// Aggregator collecting the lines to be printed in the difference summary.
//
// This is buffered in order to allow a future line to potentially impact how
// the current line would be printed.
#[derive(Default)]
struct BufferedSummary<'a> {
    summary: SummaryBuilder,
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
        if let Buffer::ExtraExpectedLineChunk(extra_expected) = self.buffer {
            self.print_inline_diffs(extra_actual, extra_expected);
            self.buffer = Buffer::Empty;
        } else {
            self.flush_buffer();
            self.buffer = Buffer::ExtraActualLineChunk(extra_actual);
        }
    }

    // Appends a new line which is found only in the expected string.
    fn feed_extra_expected(&mut self, extra_expected: &'a str) {
        if let Buffer::ExtraActualLineChunk(extra_actual) = self.buffer {
            self.print_inline_diffs(extra_actual, extra_expected);
            self.buffer = Buffer::Empty;
        } else {
            self.flush_buffer();
            self.buffer = Buffer::ExtraExpectedLineChunk(extra_expected);
        }
    }

    // Appends a comment for the additional line at the start or the end of the
    // actual string which should be omitted.
    fn feed_additional_actual(&mut self) {
        self.flush_buffer();
        self.summary.new_line();
        self.summary.push_str_as_comment("<---- remaining lines omitted ---->");
    }

    fn flush_buffer(&mut self) {
        self.buffer.flush(&mut self.summary);
    }

    fn print_inline_diffs(&mut self, actual_line: &str, expected_line: &str) {
        let line_edits = edit_distance::edit_list(
            actual_line.chars(),
            expected_line.chars(),
            edit_distance::Mode::Exact,
        );

        if let edit_distance::Difference::Editable(edit_list) = line_edits {
            let mut actual_summary = SummaryBuilder::default();
            actual_summary.new_line_for_actual();
            let mut expected_summary = SummaryBuilder::default();
            expected_summary.new_line_for_expected();
            for edit in &edit_list {
                match edit {
                    edit_distance::Edit::ExtraActual(c) => actual_summary.push_actual_only(*c),
                    edit_distance::Edit::ExtraExpected(c) => {
                        expected_summary.push_expected_only(*c)
                    }
                    edit_distance::Edit::Both(c) => {
                        actual_summary.push_actual_with_match(*c);
                        expected_summary.push_expected_with_match(*c);
                    }
                    edit_distance::Edit::AdditionalActual => {
                        // Calling edit_distance::edit_list(_, _, Mode::Exact) should never return
                        // this enum
                        panic!("This should not happen. This is a bug in gtest_rust")
                    }
                }
            }
            actual_summary.reset_ansi();
            expected_summary.reset_ansi();
            self.summary.push_str(&actual_summary.summary);
            self.summary.push_str(&expected_summary.summary);
        } else {
            self.summary.new_line_for_actual();
            self.summary.push_str_actual_only(actual_line);
            self.summary.new_line_for_expected();
            self.summary.push_str_expected_only(expected_line);
        }
    }
}

impl<'a> FromIterator<edit_distance::Edit<&'a str>> for BufferedSummary<'a> {
    fn from_iter<T: IntoIterator<Item = edit_distance::Edit<&'a str>>>(iter: T) -> Self {
        let mut buffered_summary = BufferedSummary::default();
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
        buffered_summary.summary.reset_ansi();

        buffered_summary
    }
}

impl<'a> Display for BufferedSummary<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !matches!(self.buffer, Buffer::Empty) {
            panic!("Buffer is not empty. This is a bug in gtest_rust.")
        }
        if !self.summary.last_ansi_style.is_empty() {
            panic!("ANSI style has not been reset. This is a bug in gtest_rust.")
        }
        self.summary.summary.fmt(f)
    }
}

enum Buffer<'a> {
    Empty,
    CommonLineBuffer(Vec<&'a str>),
    ExtraActualLineChunk(&'a str),
    ExtraExpectedLineChunk(&'a str),
}

impl<'a> Buffer<'a> {
    fn flush(&mut self, summary: &mut SummaryBuilder) {
        match self {
            Buffer::Empty => {}
            Buffer::CommonLineBuffer(common_lines) => {
                Self::flush_common_lines(std::mem::take(common_lines), summary);
            }
            Buffer::ExtraActualLineChunk(extra_actual) => {
                summary.new_line_for_actual();
                summary.push_str_actual_only(extra_actual);
            }
            Buffer::ExtraExpectedLineChunk(extra_expected) => {
                summary.new_line_for_expected();
                summary.push_str_expected_only(extra_expected);
            }
        };
        *self = Buffer::Empty;
    }

    fn flush_common_lines(common_lines: Vec<&'a str>, summary: &mut SummaryBuilder) {
        // The number of the lines kept before and after the compressed lines.
        const COMMON_LINES_CONTEXT_SIZE: usize = 2;

        if common_lines.len() <= 2 * COMMON_LINES_CONTEXT_SIZE + 1 {
            for line in common_lines {
                summary.new_line();
                summary.push_str(line);
            }
            return;
        }

        let start_context = &common_lines[0..COMMON_LINES_CONTEXT_SIZE];

        for line in start_context {
            summary.new_line();
            summary.push_str(line);
        }

        summary.new_line();
        summary.push_str_as_comment(&format!(
            "<---- {} common lines omitted ---->",
            common_lines.len() - 2 * COMMON_LINES_CONTEXT_SIZE,
        ));

        let end_context =
            &common_lines[common_lines.len() - COMMON_LINES_CONTEXT_SIZE..common_lines.len()];

        for line in end_context {
            summary.new_line();
            summary.push_str(line);
        }
    }
}

impl<'a> Default for Buffer<'a> {
    fn default() -> Self {
        Self::Empty
    }
}

#[rustversion::since(1.70)]
fn stdout_supports_color() -> bool {
    match (is_env_var_set("NO_COLOR"), is_env_var_set("FORCE_COLOR")) {
        (true, _) => false,
        (false, true) => true,
        (false, false) => std::io::stdout().is_terminal(),
    }
}

#[rustversion::not(since(1.70))]
fn stdout_supports_color() -> bool {
    is_env_var_set("FORCE_COLOR")
}

fn is_env_var_set(var: &'static str) -> bool {
    std::env::var(var).map(|s| !s.is_empty()).unwrap_or(false)
}

// Font in italic
const COMMENT_STYLE: &str = "\x1B[3m";
// Font in green and bold
const EXPECTED_ONLY_STYLE: &str = "\x1B[1;32m";
// Font in red and bold
const ACTUAL_ONLY_STYLE: &str = "\x1B[1;31m";
// Font in green onlyh
const EXPECTED_WITH_MATCH_STYLE: &str = "\x1B[32m";
// Font in red only
const ACTUAL_WITH_MATCH_STYLE: &str = "\x1B[31m";
// Reset all ANSI formatting
const RESET_ALL: &str = "\x1B[0m";

#[derive(Default)]
struct SummaryBuilder {
    summary: String,
    last_ansi_style: &'static str,
}

impl SummaryBuilder {
    fn push_str(&mut self, element: &str) {
        self.reset_ansi();
        self.summary.push_str(element);
    }

    fn push_str_as_comment(&mut self, element: &str) {
        self.set_ansi(COMMENT_STYLE);
        self.summary.push_str(element);
    }

    fn push_str_actual_only(&mut self, element: &str) {
        self.set_ansi(ACTUAL_ONLY_STYLE);
        self.summary.push_str(element);
    }

    fn push_str_expected_only(&mut self, element: &str) {
        self.set_ansi(EXPECTED_ONLY_STYLE);
        self.summary.push_str(element);
    }

    fn push_actual_only(&mut self, element: char) {
        self.set_ansi(ACTUAL_ONLY_STYLE);
        self.summary.push(element);
    }

    fn push_expected_only(&mut self, element: char) {
        self.set_ansi(EXPECTED_ONLY_STYLE);
        self.summary.push(element);
    }

    fn push_actual_with_match(&mut self, element: char) {
        self.set_ansi(ACTUAL_WITH_MATCH_STYLE);
        self.summary.push(element);
    }

    fn push_expected_with_match(&mut self, element: char) {
        self.set_ansi(EXPECTED_WITH_MATCH_STYLE);
        self.summary.push(element);
    }

    fn new_line(&mut self) {
        self.reset_ansi();
        self.summary.push_str("\n ");
    }

    fn new_line_for_actual(&mut self) {
        self.reset_ansi();
        self.summary.push_str("\n-");
    }

    fn new_line_for_expected(&mut self) {
        self.reset_ansi();
        self.summary.push_str("\n+");
    }

    fn reset_ansi(&mut self) {
        if !self.last_ansi_style.is_empty() && stdout_supports_color() {
            self.summary.push_str(RESET_ALL);
            self.last_ansi_style = "";
        }
    }

    fn set_ansi(&mut self, ansi_style: &'static str) {
        if !stdout_supports_color() || self.last_ansi_style == ansi_style {
            return;
        }
        if !self.last_ansi_style.is_empty() {
            self.summary.push_str(RESET_ALL);
        }
        self.summary.push_str(ansi_style);
        self.last_ansi_style = ansi_style;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matcher_support::edit_distance::Mode, prelude::*};
    use indoc::indoc;
    use serial_test::{parallel, serial};
    use std::fmt::Write;

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
    #[parallel]
    fn create_diff_smaller_than_one_line() -> Result<()> {
        verify_that!(create_diff("One", "Two", Mode::Exact), eq(""))
    }

    #[test]
    #[parallel]
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
    #[parallel]
    fn create_diff_multiline_diff() -> Result<()> {
        let expected = indoc! {"
            prefix
            Actual#1
            Actual#2
            Actual#3
            suffix"};
        let actual = indoc! {"
            prefix
            Expected@one
            Expected@two
            suffix"};
        // TODO: It would be better to have all the Actual together followed by all the
        // Expected together.
        verify_that!(
            create_diff(expected, actual, Mode::Exact),
            eq(indoc!(
                "

                Difference(-actual / +expected):
                 prefix
                -Actual#1
                +Expected@one
                -Actual#2
                +Expected@two
                -Actual#3
                 suffix"
            ))
        )
    }

    #[test]
    #[parallel]
    fn create_diff_exact_unrelated() -> Result<()> {
        verify_that!(create_diff(&build_text(1..500), &build_text(501..1000), Mode::Exact), eq(""))
    }

    #[test]
    #[parallel]
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

    // Test with color enabled.

    struct ForceColor;

    fn force_color() -> ForceColor {
        std::env::set_var("FORCE_COLOR", "1");
        std::env::remove_var("NO_COLOR");
        ForceColor
    }

    impl Drop for ForceColor {
        fn drop(&mut self) {
            std::env::remove_var("FORCE_COLOR");
            std::env::set_var("NO_COLOR", "1");
        }
    }

    #[test]
    #[serial]
    fn create_diff_exact_small_difference_with_color() -> Result<()> {
        let _keep = force_color();

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
    fn create_diff_exact_difference_with_inline_color() -> Result<()> {
        let _keep = force_color();
        let actual = indoc!(
            "There is a home in Nouvelle Orleans
            They say, it is the rising sons
            And it has been the ruin of many a po'boy"
        );

        let expected = indoc!(
            "There is a house way down in New Orleans
            They call the rising sun
            And it has been the ruin of many a poor boy"
        );

        verify_that!(
            create_diff(actual, expected, Mode::Exact),
            eq(indoc! {
                "

                Difference(-\x1B[1;31mactual\x1B[0m / +\x1B[1;32mexpected\x1B[0m):
                -\x1B[31mThere is a ho\x1B[0m\x1B[1;31mm\x1B[0m\x1B[31me in N\x1B[0m\x1B[1;31mouv\x1B[0m\x1B[31me\x1B[0m\x1B[1;31mlle\x1B[0m\x1B[31m Orleans\x1B[0m
                +\x1B[32mThere is a ho\x1B[0m\x1B[1;32mus\x1B[0m\x1B[32me \x1B[0m\x1B[1;32mway down \x1B[0m\x1B[32min Ne\x1B[0m\x1B[1;32mw\x1B[0m\x1B[32m Orleans\x1B[0m
                -\x1B[31mThey \x1B[0m\x1B[1;31ms\x1B[0m\x1B[31ma\x1B[0m\x1B[1;31my,\x1B[0m\x1B[31m \x1B[0m\x1B[1;31mi\x1B[0m\x1B[31mt\x1B[0m\x1B[1;31m is t\x1B[0m\x1B[31mhe rising s\x1B[0m\x1B[1;31mo\x1B[0m\x1B[31mn\x1B[0m\x1B[1;31ms\x1B[0m
                +\x1B[32mThey \x1B[0m\x1B[1;32mc\x1B[0m\x1B[32ma\x1B[0m\x1B[1;32mll\x1B[0m\x1B[32m the rising s\x1B[0m\x1B[1;32mu\x1B[0m\x1B[32mn\x1B[0m
                -\x1B[31mAnd it has been the ruin of many a po\x1B[0m\x1B[1;31m'\x1B[0m\x1B[31mboy\x1B[0m
                +\x1B[32mAnd it has been the ruin of many a po\x1B[0m\x1B[1;32mor \x1B[0m\x1B[32mboy\x1B[0m"
            })
        )
    }
}
