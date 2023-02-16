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

#[cfg(not(google3))]
use crate as googletest;
use eq_matcher::EqMatcher;
use googletest::matcher::{Matcher, MatcherResult};
#[cfg(not(google3))]
use googletest::matchers::eq_matcher;
use std::fmt::Debug;
use std::ops::Deref;

/// Matches a string containing a given substring.
///
/// Both the actual value and the expected substring may be either a `String` or
/// a string reference.
///
/// ```rust
/// verify_that!("Some value", contains_substring("Some"))?;  // Passes
/// verify_that!("Another value", contains_substring("Some"))?;   // Fails
/// verify_that!("Some value".to_string(), contains_substring("value"))?;   // Passes
/// verify_that!("Some value", contains_substring("value".to_string()))?;   // Passes
/// ```
///
/// > Note on memory use: In most cases, this matcher does not allocate memory
/// > when matching strings. However, it must allocate copies of both the actual
/// > and expected values when matching strings while
/// > [`ignoring_ascii_case`][StrMatcherConfigurator::ignoring_ascii_case] is
/// > set.
pub fn contains_substring<T>(expected: T) -> StrMatcher<T> {
    StrMatcher {
        configuration: Configuration { mode: MatchMode::Contains, ..Default::default() },
        expected,
    }
}

/// Matches a string which starts with the given prefix.
///
/// Both the actual value and the expected prefix may be either a `String` or
/// a string reference.
///
/// ```rust
/// verify_that!("Some value", starts_with("Some"))?;  // Passes
/// verify_that!("Another value", starts_with("Some"))?;   // Fails
/// verify_that!("Some value", starts_with("value"))?;  // Fails
/// verify_that!("Some value".to_string(), starts_with("Some"))?;   // Passes
/// verify_that!("Some value", starts_with("Some".to_string()))?;   // Passes
/// ```
pub fn starts_with<T>(expected: T) -> StrMatcher<T> {
    StrMatcher {
        configuration: Configuration { mode: MatchMode::StartsWith, ..Default::default() },
        expected,
    }
}

/// Matches a string which ends with the given suffix.
///
/// Both the actual value and the expected suffix may be either a `String` or
/// a string reference.
///
/// ```rust
/// verify_that!("Some value", ends_with("value"))?;  // Passes
/// verify_that!("Some value", ends_with("other value"))?;   // Fails
/// verify_that!("Some value", ends_with("Some"))?;  // Fails
/// verify_that!("Some value".to_string(), ends_with("value"))?;   // Passes
/// verify_that!("Some value", ends_with("value".to_string()))?;   // Passes
/// ```
pub fn ends_with<T>(expected: T) -> StrMatcher<T> {
    StrMatcher {
        configuration: Configuration { mode: MatchMode::EndsWith, ..Default::default() },
        expected,
    }
}

/// Extension trait to configure [StrMatcher].
///
/// Matchers which match against string values and, through configuration,
/// specialise to [StrMatcher] implement this trait. Currently that only
/// includes [EqMatcher] and [StrMatcher].
pub trait StrMatcherConfigurator<T> {
    /// Configures the matcher to ignore any leading whitespace in either the
    /// actual or the expected value.
    ///
    /// Whitespace is defined as in [`str::trim_start`][https://doc.rust-lang.org/std/primitive.str.html#method.trim_start].
    ///
    /// ```rust
    /// verify_that!("A string", eq("   A string").ignoring_leading_whitespace())?; // Passes
    /// verify_that!("   A string", eq("A string").ignoring_leading_whitespace())?; // Passes
    /// ```
    ///
    /// When all other configuration options are left as the defaults, this is
    /// equivalent to invoking [`str::trim_start`] on both the expected and
    /// actual value.
    fn ignoring_leading_whitespace(self) -> StrMatcher<T>;

    /// Configures the matcher to ignore any trailing whitespace in either the
    /// actual or the expected value.
    ///
    /// Whitespace is defined as in [`str::trim_end`][https://doc.rust-lang.org/std/primitive.str.html#method.trim_end].
    ///
    /// ```rust
    /// verify_that!("A string", eq("A string   ").ignoring_trailing_whitespace())?; // Passes
    /// verify_that!("A string   ", eq("A string").ignoring_trailing_whitespace())?; // Passes
    /// ```
    ///
    /// When all other configuration options are left as the defaults, this is
    /// equivalent to invoking [`str::trim_end`] on both the expected and
    /// actual value.
    fn ignoring_trailing_whitespace(self) -> StrMatcher<T>;

    /// Configures the matcher to ignore both leading and trailing whitespace in
    /// either the actual or the expected value.
    ///
    /// Whitespace is defined as in [`str::trim`][https://doc.rust-lang.org/std/primitive.str.html#method.trim].
    ///
    /// ```rust
    /// verify_that!("A string", eq("   A string   ").ignoring_outer_whitespace())?; // Passes
    /// verify_that!("   A string   ", eq("A string").ignoring_outer_whitespace())?; // Passes
    /// ```
    ///
    /// This is equivalent to invoking both
    /// [`ignoring_leading_whitespace`][StrMatcherConfigurator::ignoring_leading_whitespace] and
    /// [`ignoring_trailing_whitespace`][StrMatcherConfigurator::ignoring_trailing_whitespace].
    ///
    /// When all other configuration options are left as the defaults, this is
    /// equivalent to invoking [`str::trim`] on both the expected and actual
    /// value.
    fn ignoring_outer_whitespace(self) -> StrMatcher<T>;

    /// Configures the matcher to ignore uniform indentation in both the
    /// expected and actual values.
    ///
    /// Uniform indentation is defined as the maximimal prefix consisting only
    /// of ordinary space (' ') characters common to every line. The actual and
    /// expected value may additionally have one empty initial line and one
    /// final line consisting only of space characters which are both
    /// ignored. This is relaxed further to include all leading and trailing
    /// whitespace if additionally
    /// [`ignoring_leading_whitespace`][Self::ignoring_leading_whitespace],
    /// respectively
    /// [`ignore_trailing_whitespace`][Self::ignore_trailing_whitespace] is set.
    ///
    /// Lines are as defined in
    /// [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines) and can be
    /// terminated in the Unix or DOS styles.
    ///
    /// ```rust
    /// // Passes:
    /// let value = "
    /// Some text
    /// Some more text
    ///    Some indented text";
    /// verify_that!(
    ///     value,
    //      eq("
    ///         Some text
    ///         Some more text
    ///            Some indented text
    ///     ").ignoring_uniform_indentation()
    /// )?;
    ///
    /// // Passes:
    /// let value = "
    ///    Some text
    ///    Some more text
    ///       Some indented text
    /// ";
    /// verify_that!(
    ///     value,
    //      eq("
    /// Some text
    /// Some more text
    ///    Some indented text
    ///     ").ignoring_uniform_indentation()
    /// )?;
    ///
    /// // Fails due to inconsistent indentation:
    /// let value = "
    ///   Some text
    ///     Some more text
    ///       Some indented text";
    /// verify_that!(
    ///     value,
    //      eq("
    ///            Some text
    ///         Some more text
    ///            Some indented text").ignoring_uniform_indentation()
    /// )?;
    /// ```
    fn ignoring_uniform_indentation(self) -> StrMatcher<T>;

    /// Configures the matcher to ignore ASCII case when comparing values.
    ///
    /// This uses the same rules for case as
    /// [`str::eq_ignore_ascii_case`][https://doc.rust-lang.org/std/primitive.str.html#method.eq_ignore_ascii_case].
    ///
    /// ```rust
    /// verify_that!("Some value", eq_ignoring_ascii_case("SOME VALUE"))?;  // Passes
    /// verify_that!("Another value", eq_ignoring_ascii_case("Some value"))?;   // Fails
    /// ```
    ///
    /// This is **not guaranteed** to match strings with differing upper/lower
    /// case characters outside of the codepoints 0-127 covered by ASCII.
    fn ignoring_ascii_case(self) -> StrMatcher<T>;
}

/// A matcher which matches equality or containment of a string-like value in a
/// configurable way.
///
/// The following matcher methods instantiate this:
///
///  * [`eq`][crate::matchers::eq_matcher::eq],
///  * [`contains_substring`],
///  * [`starts_with`],
///  * [`ends_with`].
pub struct StrMatcher<T> {
    expected: T,
    configuration: Configuration,
}

impl<ExpectedT, ActualT> Matcher<ActualT> for StrMatcher<ExpectedT>
where
    ExpectedT: Deref<Target = str> + Debug,
    ActualT: AsRef<str> + Debug + ?Sized,
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        if self.configuration.do_strings_match(self.expected.deref(), actual.as_ref()) {
            MatcherResult::Matches
        } else {
            MatcherResult::DoesNotMatch
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        self.configuration.describe(matcher_result, self.expected.deref())
    }
}

impl<T, MatcherT: Into<StrMatcher<T>>> StrMatcherConfigurator<T> for MatcherT {
    fn ignoring_leading_whitespace(self) -> StrMatcher<T> {
        let existing = self.into();
        StrMatcher {
            configuration: existing.configuration.ignoring_leading_whitespace(),
            ..existing
        }
    }

    fn ignoring_trailing_whitespace(self) -> StrMatcher<T> {
        let existing = self.into();
        StrMatcher {
            configuration: existing.configuration.ignoring_trailing_whitespace(),
            ..existing
        }
    }

    fn ignoring_outer_whitespace(self) -> StrMatcher<T> {
        let existing = self.into();
        StrMatcher { configuration: existing.configuration.ignoring_outer_whitespace(), ..existing }
    }

    fn ignoring_uniform_indentation(self) -> StrMatcher<T> {
        let existing = self.into();
        StrMatcher {
            configuration: existing.configuration.ignoring_uniform_indentation(),
            ..existing
        }
    }

    fn ignoring_ascii_case(self) -> StrMatcher<T> {
        let existing = self.into();
        StrMatcher { configuration: existing.configuration.ignoring_ascii_case(), ..existing }
    }
}

impl<T: Deref<Target = str>> From<EqMatcher<T>> for StrMatcher<T> {
    fn from(value: EqMatcher<T>) -> Self {
        Self::with_default_config(value.expected)
    }
}

impl<T> StrMatcher<T> {
    /// Returns a [`StrMatcher`] with a default configuration to match against
    /// the given expected value.
    ///
    /// This default configuration is sensitive to whitespace and case.
    fn with_default_config(expected: T) -> Self {
        Self { expected, configuration: Default::default() }
    }
}

// Holds all the information on how the expected and actual strings are to be
// compared. Its associated functions perform the actual matching operations
// on string references. The struct and comparison methods therefore need not be
// parameterised, saving compilation time and binary size on monomorphisation.
//
// The default value represents exact equality of the strings.
#[derive(Default, Clone)]
struct Configuration {
    mode: MatchMode,
    ignore_leading_whitespace: bool,
    ignore_trailing_whitespace: bool,
    indentation_policy: IndentationPolicy,
    case_policy: CasePolicy,
}

#[derive(Default, Clone)]
enum MatchMode {
    #[default]
    Equals,
    Contains,
    StartsWith,
    EndsWith,
}

#[derive(Default, Clone)]
enum IndentationPolicy {
    #[default]
    Respect,
    IgnoreUniform,
}

#[derive(Default, Clone)]
enum CasePolicy {
    #[default]
    Respect,
    IgnoreAscii,
}

impl Configuration {
    // The entry point for all string matching. StrMatcher::matches redirects
    // immediately to this function.
    //
    // This takes into account all aspects including IndentationPolicy.
    fn do_strings_match(&self, expected: &str, actual: &str) -> bool {
        match self.indentation_policy {
            IndentationPolicy::Respect => self.are_strings_equivalent(expected, actual),
            IndentationPolicy::IgnoreUniform => {
                self.compare_strings_ignoring_uniform_ident(expected, actual)
            }
        }
    }

    // This takes into account leading and trailing whitespace, CasePolicy, and
    // MatchMode. It ignores IndentationPolicy.
    fn are_strings_equivalent(&self, expected: &str, actual: &str) -> bool {
        let (expected, actual) =
            match (self.ignore_leading_whitespace, self.ignore_trailing_whitespace) {
                (true, true) => (expected.trim(), actual.trim()),
                (true, false) => (expected.trim_start(), actual.trim_start()),
                (false, true) => (expected.trim_end(), actual.trim_end()),
                (false, false) => (expected, actual),
            };
        match self.mode {
            MatchMode::Equals => match self.case_policy {
                CasePolicy::Respect => expected == actual,
                CasePolicy::IgnoreAscii => expected.eq_ignore_ascii_case(actual),
            },
            MatchMode::Contains => match self.case_policy {
                CasePolicy::Respect => actual.contains(expected),
                CasePolicy::IgnoreAscii => {
                    actual.to_ascii_lowercase().contains(&expected.to_ascii_lowercase())
                }
            },
            MatchMode::StartsWith => match self.case_policy {
                CasePolicy::Respect => actual.starts_with(expected),
                CasePolicy::IgnoreAscii => {
                    actual.len() >= expected.len()
                        && actual[..expected.len()].eq_ignore_ascii_case(expected)
                }
            },
            MatchMode::EndsWith => match self.case_policy {
                CasePolicy::Respect => actual.ends_with(expected),
                CasePolicy::IgnoreAscii => {
                    actual.len() >= expected.len()
                        && actual[actual.len() - expected.len()..].eq_ignore_ascii_case(expected)
                }
            },
        }
    }

    fn compare_strings_ignoring_uniform_ident(&self, expected: &str, actual: &str) -> bool {
        // TODO(b/266919284): This behaves incorrectly when self.mode is not Equals,
        // since it would apply the mode separately to each line.
        let expected = self.strip_initial_and_final_blank_lines(expected);
        let actual = self.strip_initial_and_final_blank_lines(actual);

        let expected_lines = expected.lines();
        let actual_lines = actual.lines();

        if expected_lines.clone().count() != actual_lines.clone().count() {
            return false;
        }

        let expected_lines_prefix_len = Self::common_indentation_prefix_len(expected_lines.clone());
        let actual_lines_prefix_len = Self::common_indentation_prefix_len(actual_lines.clone());
        let mut expected_actual = expected_lines.zip(actual_lines).peekable();
        let Some((expected, actual)) = expected_actual.next() else {
            return true;
        };
        match expected_actual.peek() {
            Some(_) => {
                // There are multiple lines. The first one is compared with the initial
                // configuration which is always sensitive to trailing whitespace, etc. The
                // middle ones are compared with the middle configuration which is always
                // sensitive to leading and trailing whitespace etc. The last one is compared
                // with the final configuration which is always sensitive to leading whitespace,
                // etc.
                let initial_configuration = self.to_initial_configuration();
                if !initial_configuration.are_strings_equivalent(
                    &expected[expected_lines_prefix_len..],
                    &actual[actual_lines_prefix_len..],
                ) {
                    return false;
                }
                let mut current_configuration = self.to_middle_configuration();
                while let Some((expected, actual)) = expected_actual.next() {
                    if expected_actual.peek().is_none() {
                        current_configuration = self.to_final_configuration();
                    }
                    if !current_configuration.are_strings_equivalent(
                        &expected[expected_lines_prefix_len..],
                        &actual[actual_lines_prefix_len..],
                    ) {
                        return false;
                    }
                }
                true
            }

            // There is one line, so just use this configuration to compare.
            None => self.are_strings_equivalent(
                &expected[expected_lines_prefix_len..],
                &actual[actual_lines_prefix_len..],
            ),
        }
    }

    fn common_indentation_prefix_len<'a>(lines: impl IntoIterator<Item = &'a str>) -> usize {
        lines.into_iter().filter_map(|l| l.find(|c: char| c != ' ')).min().unwrap_or(0)
    }

    fn strip_initial_and_final_blank_lines<'a>(&self, mut value: &'a str) -> &'a str {
        if self.ignore_leading_whitespace {
            // Strip all leading lines which consist only of whitespace. Any leading space
            // characters on the first line with non-whitespace characters must be included
            // in calculating the uniform indentation, so we can't use trim_start(). The
            // initial configuration from to_initial_configuration() will handle the rest.

            // N.B. This handles both Unix and DOS-style newlines since the \n character
            // terminates the newline in both cases.
            while let Some(first_newline_index) = value.find('\n') {
                if value[..first_newline_index].trim() != "" {
                    break;
                }
                value = &value[first_newline_index + 1..];
            }
        }
        if self.ignore_trailing_whitespace {
            value.trim_end()
        } else {
            let value = value.strip_prefix('\n').unwrap_or(value);
            value.trim_end_matches(' ').strip_suffix('\n').unwrap_or(value)
        }
    }

    fn to_initial_configuration(&self) -> Self {
        Self { ignore_trailing_whitespace: false, ..self.clone() }
    }

    fn to_middle_configuration(&self) -> Self {
        Self { ignore_leading_whitespace: false, ignore_trailing_whitespace: false, ..self.clone() }
    }

    fn to_final_configuration(&self) -> Self {
        Self { ignore_leading_whitespace: false, ..self.clone() }
    }

    // StrMatcher::describe redirects immediately to this function.
    fn describe(&self, matcher_result: MatcherResult, expected: &str) -> String {
        let mut addenda = Vec::with_capacity(3);
        match (self.ignore_leading_whitespace, self.ignore_trailing_whitespace) {
            (true, true) => addenda.push("ignoring leading and trailing whitespace"),
            (true, false) => addenda.push("ignoring leading whitespace"),
            (false, true) => addenda.push("ignoring trailing whitespace"),
            (false, false) => {}
        }
        match self.indentation_policy {
            IndentationPolicy::Respect => {}
            IndentationPolicy::IgnoreUniform => addenda.push("ignoring uniform indentation"),
        }
        match self.case_policy {
            CasePolicy::Respect => {}
            CasePolicy::IgnoreAscii => addenda.push("ignoring ASCII case"),
        }
        let extra =
            if !addenda.is_empty() { format!(" ({})", addenda.join(", ")) } else { "".into() };
        let match_mode_description = match self.mode {
            MatchMode::Equals => match matcher_result {
                MatcherResult::Matches => "is equal to",
                MatcherResult::DoesNotMatch => "isn't equal to",
            },
            MatchMode::Contains => match matcher_result {
                MatcherResult::Matches => "contains a substring",
                MatcherResult::DoesNotMatch => "does not contain a substring",
            },
            MatchMode::StartsWith => match matcher_result {
                MatcherResult::Matches => "starts with prefix",
                MatcherResult::DoesNotMatch => "does not start with",
            },
            MatchMode::EndsWith => match matcher_result {
                MatcherResult::Matches => "ends with suffix",
                MatcherResult::DoesNotMatch => "does not end with",
            },
        };
        format!("{match_mode_description} {expected:?}{extra}")
    }

    fn ignoring_leading_whitespace(self) -> Self {
        Self { ignore_leading_whitespace: true, ..self }
    }

    fn ignoring_trailing_whitespace(self) -> Self {
        Self { ignore_trailing_whitespace: true, ..self }
    }

    fn ignoring_outer_whitespace(self) -> Self {
        Self { ignore_leading_whitespace: true, ignore_trailing_whitespace: true, ..self }
    }

    fn ignoring_uniform_indentation(self) -> Self {
        Self { indentation_policy: IndentationPolicy::IgnoreUniform, ..self }
    }

    fn ignoring_ascii_case(self) -> Self {
        Self { case_policy: CasePolicy::IgnoreAscii, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(google3))]
    use crate as googletest;
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{google_test, verify_that, Result};
    use matchers::{eq, not};

    #[google_test]
    fn matches_string_reference_with_equal_string_reference() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!("A string", matcher)
    }

    #[google_test]
    fn does_not_match_string_reference_with_non_equal_string_reference() -> Result<()> {
        let matcher = StrMatcher::with_default_config("Another string");
        verify_that!("A string", not(matcher))
    }

    #[google_test]
    fn matches_owned_string_with_string_reference() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        let value = "A string".to_string();
        verify_that!(value, matcher)
    }

    #[google_test]
    fn matches_owned_string_reference_with_string_reference() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        let value = "A string".to_string();
        verify_that!(&value, matcher)
    }

    #[google_test]
    fn ignores_leading_whitespace_in_expected_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config(" \n\tA string");
        verify_that!("A string", matcher.ignoring_leading_whitespace())
    }

    #[google_test]
    fn ignores_leading_whitespace_in_actual_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!(" \n\tA string", matcher.ignoring_leading_whitespace())
    }

    #[google_test]
    fn does_not_match_unequal_remaining_string_when_ignoring_leading_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config(" \n\tAnother string");
        verify_that!("A string", not(matcher.ignoring_leading_whitespace()))
    }

    #[google_test]
    fn remains_sensitive_to_trailing_whitespace_when_ignoring_leading_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string \n\t");
        verify_that!("A string", not(matcher.ignoring_leading_whitespace()))
    }

    #[google_test]
    fn ignores_trailing_whitespace_in_expected_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string \n\t");
        verify_that!("A string", matcher.ignoring_trailing_whitespace())
    }

    #[google_test]
    fn ignores_trailing_whitespace_in_actual_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!("A string \n\t", matcher.ignoring_trailing_whitespace())
    }

    #[google_test]
    fn does_not_match_unequal_remaining_string_when_ignoring_trailing_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config("Another string \n\t");
        verify_that!("A string", not(matcher.ignoring_trailing_whitespace()))
    }

    #[google_test]
    fn remains_sensitive_to_leading_whitespace_when_ignoring_trailing_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config(" \n\tA string");
        verify_that!("A string", not(matcher.ignoring_trailing_whitespace()))
    }

    #[google_test]
    fn ignores_leading_and_trailing_whitespace_in_expected_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config(" \n\tA string \n\t");
        verify_that!("A string", matcher.ignoring_outer_whitespace())
    }

    #[google_test]
    fn ignores_leading_and_trailing_whitespace_in_actual_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!(" \n\tA string \n\t", matcher.ignoring_outer_whitespace())
    }

    #[google_test]
    fn ignores_uniform_indent_in_expected_when_requested() -> Result<()> {
        let value = "
Some text
Some more text
";
        let matcher = StrMatcher::with_default_config(
            "
                Some text
                Some more text
            ",
        );
        verify_that!(value, matcher.ignoring_uniform_indentation())
    }

    #[google_test]
    fn ignores_uniform_indent_in_actual_when_requested() -> Result<()> {
        let value = "
            Some text
            Some more text
               Some indented text
        ";
        let matcher = StrMatcher::with_default_config(
            "
Some text
Some more text
   Some indented text
            ",
        );
        verify_that!(value, matcher.ignoring_uniform_indentation())
    }

    #[google_test]
    fn remains_sensitive_to_nonuniform_indent_when_ignoring_uniform_indent() -> Result<()> {
        let value = "
    Some text
  Some more text
   Some indented text
        ";
        let matcher = StrMatcher::with_default_config(
            "
             Some text
                Some more text
                   Some indented text
            ",
        );
        verify_that!(value, not(matcher.ignoring_uniform_indentation()))
    }

    #[google_test]
    fn ignores_initial_and_final_empty_lines_when_ignoring_uniform_indent() -> Result<()> {
        let value = "Some text
Some more text";
        let matcher = StrMatcher::with_default_config(
            "
                Some text
                Some more text
            ",
        );
        verify_that!(value, matcher.ignoring_uniform_indentation())
    }

    #[google_test]
    fn does_not_match_values_with_differing_number_of_lines_when_ignoring_uniform_indent()
    -> Result<()> {
        let value = "
            Some text
            Some text
            Some text
        ";
        let matcher = StrMatcher::with_default_config(
            "
                Some text
                Some text
            ",
        );
        verify_that!(value, not(matcher.ignoring_uniform_indentation()))
    }

    #[google_test]
    fn ignores_uniform_indent_and_outer_whitespace_on_same_line_when_requested() -> Result<()> {
        let value = "
Some text
Some more text
Some other more text
";
        let matcher = StrMatcher::with_default_config(
            "
                \tSome text
                Some more text
                Some other more text\t
            ",
        );
        verify_that!(value, matcher.ignoring_uniform_indentation().ignoring_outer_whitespace())
    }

    #[google_test]
    fn considers_only_uniform_indent_when_ignoring_both_uniform_indent_and_outer_whitespace()
    -> Result<()> {
        let value = "
Some text
  Some more text
Some other more text
";
        let matcher = StrMatcher::with_default_config(
            "
                Some text
                Some more text
                Some other more text
            ",
        );
        verify_that!(value, not(matcher.ignoring_uniform_indentation().ignoring_outer_whitespace()))
    }

    #[google_test]
    fn sensitive_to_trailing_whitsepace_on_first_line_when_ignoring_both_uniform_indent_and_outer_whitespace()
    -> Result<()> {
        let value = "
Some text\t
Some more text
Some other more text
";
        let matcher = StrMatcher::with_default_config(
            "
                Some text
                Some more text
                Some other more text
            ",
        );
        verify_that!(value, not(matcher.ignoring_uniform_indentation().ignoring_outer_whitespace()))
    }

    #[google_test]
    fn sensitive_to_trailing_whitsepace_on_middle_lines_when_ignoring_both_uniform_indent_and_outer_whitespace()
    -> Result<()> {
        let value = "
Some text
Some more text\t
Some other more text
";
        let matcher = StrMatcher::with_default_config(
            "
                Some text
                Some more text
                Some other more text
            ",
        );
        verify_that!(value, not(matcher.ignoring_uniform_indentation().ignoring_outer_whitespace()))
    }

    #[google_test]
    fn sensitive_to_leading_whitsepace_on_last_line_when_ignoring_both_uniform_indent_and_outer_whitespace()
    -> Result<()> {
        let value = "
Some text
Some more text
\tSome other more text
";
        let matcher = StrMatcher::with_default_config(
            "
                Some text
                Some more text
                Some other more text
            ",
        );
        verify_that!(value, not(matcher.ignoring_uniform_indentation().ignoring_outer_whitespace()))
    }

    #[google_test]
    fn ignores_uniform_indent_and_outer_whitespace_on_other_lines_when_requested() -> Result<()> {
        let value = "

Some text
Some more text
Some other more text

";
        let matcher = StrMatcher::with_default_config(
            "
                Some text
                Some more text
                Some other more text
            ",
        );
        verify_that!(value, matcher.ignoring_uniform_indentation().ignoring_outer_whitespace())
    }

    #[google_test]
    fn ignores_initial_dos_style_blank_lines_when_ignoring_uniform_indent_and_leading_whitespace()
    -> Result<()> {
        let value = "\r
        \r
Some text
";
        let matcher = StrMatcher::with_default_config(
            "
                Some text
            ",
        );
        verify_that!(value, matcher.ignoring_uniform_indentation().ignoring_leading_whitespace())
    }

    #[google_test]
    fn ignores_trailing_whitespace_when_requested_with_ignore_uniform_indent_and_value_has_one_line()
    -> Result<()> {
        let value = "Some text\t";
        let matcher = StrMatcher::with_default_config(
            "
                Some text
            ",
        );
        verify_that!(value, matcher.ignoring_uniform_indentation().ignoring_outer_whitespace())
    }

    #[google_test]
    fn respects_ascii_case_by_default() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!("A STRING", not(matcher))
    }

    #[google_test]
    fn ignores_ascii_case_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!("A STRING", matcher.ignoring_ascii_case())
    }

    #[google_test]
    fn allows_ignoring_leading_whitespace_from_eq() -> Result<()> {
        verify_that!("A string", eq(" \n\tA string").ignoring_leading_whitespace())
    }

    #[google_test]
    fn allows_ignoring_trailing_whitespace_from_eq() -> Result<()> {
        verify_that!("A string", eq("A string \n\t").ignoring_trailing_whitespace())
    }

    #[google_test]
    fn allows_ignoring_outer_whitespace_from_eq() -> Result<()> {
        verify_that!("A string", eq(" \n\tA string \n\t").ignoring_outer_whitespace())
    }

    #[google_test]
    fn allows_ignoring_ascii_case_from_eq() -> Result<()> {
        verify_that!("A string", eq("A STRING").ignoring_ascii_case())
    }

    #[google_test]
    fn allows_ignoring_uniform_indent_from_eq() -> Result<()> {
        verify_that!("Some text", eq("   Some text").ignoring_uniform_indentation())
    }

    #[google_test]
    fn allows_ignoring_uniform_indent_and_ignoring_ascii_case() -> Result<()> {
        verify_that!(
            "Some text",
            eq("   SOME TEXT").ignoring_uniform_indentation().ignoring_ascii_case()
        )
    }

    #[google_test]
    fn allows_ignoring_uniform_indent_and_ignoring_ascii_case_with_multiple_lines() -> Result<()> {
        let value = "
            Some text
            Some more text
            Some other text
        ";
        verify_that!(
            value,
            eq("
                SOME TEXT
                SOME MORE TEXT
                SOME OTHER TEXT
            ")
            .ignoring_uniform_indentation()
            .ignoring_ascii_case()
        )
    }

    #[google_test]
    fn matches_string_containing_expected_value_in_contains_mode() -> Result<()> {
        verify_that!("Some string", contains_substring("str"))
    }

    #[google_test]
    fn matches_string_containing_expected_value_in_contains_mode_while_ignoring_ascii_case()
    -> Result<()> {
        verify_that!("Some string", contains_substring("STR").ignoring_ascii_case())
    }

    #[google_test]
    fn starts_with_matches_string_reference_with_prefix() -> Result<()> {
        verify_that!("Some value", starts_with("Some"))
    }

    #[google_test]
    fn starts_with_matches_string_reference_with_prefix_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some value", starts_with("SOME").ignoring_ascii_case())
    }

    #[google_test]
    fn starts_with_does_not_match_wrong_prefix_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some value", not(starts_with("OTHER").ignoring_ascii_case()))
    }

    #[google_test]
    fn ends_with_does_not_match_short_string_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some", not(starts_with("OTHER").ignoring_ascii_case()))
    }

    #[google_test]
    fn starts_with_does_not_match_string_without_prefix() -> Result<()> {
        verify_that!("Some value", not(starts_with("Another")))
    }

    #[google_test]
    fn starts_with_does_not_match_string_with_substring_not_at_beginning() -> Result<()> {
        verify_that!("Some value", not(starts_with("value")))
    }

    #[google_test]
    fn ends_with_matches_string_reference_with_suffix() -> Result<()> {
        verify_that!("Some value", ends_with("value"))
    }

    #[google_test]
    fn ends_with_matches_string_reference_with_suffix_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some value", ends_with("VALUE").ignoring_ascii_case())
    }

    #[google_test]
    fn ends_with_does_not_match_wrong_suffix_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some value", not(ends_with("OTHER").ignoring_ascii_case()))
    }

    #[google_test]
    fn ends_with_does_not_match_too_short_string_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some", not(ends_with("OTHER").ignoring_ascii_case()))
    }

    #[google_test]
    fn ends_with_does_not_match_string_without_suffix() -> Result<()> {
        verify_that!("Some value", not(ends_with("other value")))
    }

    #[google_test]
    fn ends_with_does_not_match_string_with_substring_not_at_end() -> Result<()> {
        verify_that!("Some value", not(ends_with("Some")))
    }

    #[google_test]
    fn describes_itself_for_matching_result() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("is equal to \"A string\"")
        )
    }

    #[google_test]
    fn describes_itself_for_non_matching_result() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::DoesNotMatch),
            eq("isn't equal to \"A string\"")
        )
    }

    #[google_test]
    fn describes_itself_for_matching_result_ignoring_leading_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string").ignoring_leading_whitespace();
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("is equal to \"A string\" (ignoring leading whitespace)")
        )
    }

    #[google_test]
    fn describes_itself_for_non_matching_result_ignoring_leading_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string").ignoring_leading_whitespace();
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::DoesNotMatch),
            eq("isn't equal to \"A string\" (ignoring leading whitespace)")
        )
    }

    #[google_test]
    fn describes_itself_for_matching_result_ignoring_trailing_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string").ignoring_trailing_whitespace();
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("is equal to \"A string\" (ignoring trailing whitespace)")
        )
    }

    #[google_test]
    fn describes_itself_for_matching_result_ignoring_leading_and_trailing_whitespace() -> Result<()>
    {
        let matcher = StrMatcher::with_default_config("A string").ignoring_outer_whitespace();
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("is equal to \"A string\" (ignoring leading and trailing whitespace)")
        )
    }

    #[google_test]
    fn describes_itself_for_matching_result_ignoring_ascii_case() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string").ignoring_ascii_case();
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("is equal to \"A string\" (ignoring ASCII case)")
        )
    }

    #[google_test]
    fn describes_itself_for_matching_result_ignoring_ascii_case_and_leading_whitespace()
    -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string")
            .ignoring_leading_whitespace()
            .ignoring_ascii_case();
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("is equal to \"A string\" (ignoring leading whitespace, ignoring ASCII case)")
        )
    }

    #[google_test]
    fn describes_itself_for_matching_result_ignoring_uniform_indentation() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string").ignoring_uniform_indentation();
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("is equal to \"A string\" (ignoring uniform indentation)")
        )
    }

    #[google_test]
    fn describes_itself_for_matching_result_ignoring_ascii_case_and_uniform_indentation()
    -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string")
            .ignoring_uniform_indentation()
            .ignoring_ascii_case();
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("is equal to \"A string\" (ignoring uniform indentation, ignoring ASCII case)")
        )
    }

    #[google_test]
    fn describes_itself_for_matching_result_in_contains_mode() -> Result<()> {
        let matcher = contains_substring("A string");
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("contains a substring \"A string\"")
        )
    }

    #[google_test]
    fn describes_itself_for_non_matching_result_in_contains_mode() -> Result<()> {
        let matcher = contains_substring("A string");
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::DoesNotMatch),
            eq("does not contain a substring \"A string\"")
        )
    }

    #[google_test]
    fn describes_itself_for_matching_result_in_starts_with_mode() -> Result<()> {
        let matcher = starts_with("A string");
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("starts with prefix \"A string\"")
        )
    }

    #[google_test]
    fn describes_itself_for_non_matching_result_in_starts_with_mode() -> Result<()> {
        let matcher = starts_with("A string");
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::DoesNotMatch),
            eq("does not start with \"A string\"")
        )
    }

    #[google_test]
    fn describes_itself_for_matching_result_in_ends_with_mode() -> Result<()> {
        let matcher = ends_with("A string");
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("ends with suffix \"A string\"")
        )
    }

    #[google_test]
    fn describes_itself_for_non_matching_result_in_ends_with_mode() -> Result<()> {
        let matcher = ends_with("A string");
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::DoesNotMatch),
            eq("does not end with \"A string\"")
        )
    }
}
