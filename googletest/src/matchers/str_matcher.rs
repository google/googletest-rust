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
use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::Deref;

/// Matches a string containing a given substring.
///
/// Both the actual value and the expected substring may be either a `String` or
/// a string reference.
///
/// ```
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
/// ```
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
/// ```
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
pub trait StrMatcherConfigurator<ExpectedT> {
    /// Configures the matcher to ignore any leading whitespace in either the
    /// actual or the expected value.
    ///
    /// Whitespace is defined as in [`str::trim_start`].
    ///
    /// ```
    /// verify_that!("A string", eq("   A string").ignoring_leading_whitespace())?; // Passes
    /// verify_that!("   A string", eq("A string").ignoring_leading_whitespace())?; // Passes
    /// ```
    ///
    /// When all other configuration options are left as the defaults, this is
    /// equivalent to invoking [`str::trim_start`] on both the expected and
    /// actual value.
    ///
    /// [`str::trim_start`]: https://doc.rust-lang.org/std/primitive.str.html#method.trim_start
    fn ignoring_leading_whitespace(self) -> StrMatcher<ExpectedT>;

    /// Configures the matcher to ignore any trailing whitespace in either the
    /// actual or the expected value.
    ///
    /// Whitespace is defined as in [`str::trim_end`].
    ///
    /// ```
    /// verify_that!("A string", eq("A string   ").ignoring_trailing_whitespace())?; // Passes
    /// verify_that!("A string   ", eq("A string").ignoring_trailing_whitespace())?; // Passes
    /// ```
    ///
    /// When all other configuration options are left as the defaults, this is
    /// equivalent to invoking [`str::trim_end`] on both the expected and
    /// actual value.
    ///
    /// [`str::trim_end`]: https://doc.rust-lang.org/std/primitive.str.html#method.trim_end
    fn ignoring_trailing_whitespace(self) -> StrMatcher<ExpectedT>;

    /// Configures the matcher to ignore both leading and trailing whitespace in
    /// either the actual or the expected value.
    ///
    /// Whitespace is defined as in [`str::trim`].
    ///
    /// ```
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
    ///
    /// [`str::trim`]: https://doc.rust-lang.org/std/primitive.str.html#method.trim
    fn ignoring_outer_whitespace(self) -> StrMatcher<ExpectedT>;

    /// Configures the matcher to ignore ASCII case when comparing values.
    ///
    /// This uses the same rules for case as [`str::eq_ignore_ascii_case`].
    ///
    /// ```
    /// verify_that!("Some value", eq("SOME VALUE").ignoring_ascii_case())?;  // Passes
    /// verify_that!("Another value", eq("Some value").ignoring_ascii_case())?;   // Fails
    /// ```
    ///
    /// This is **not guaranteed** to match strings with differing upper/lower
    /// case characters outside of the codepoints 0-127 covered by ASCII.
    ///
    /// [`str::eq_ignore_ascii_case`]: https://doc.rust-lang.org/std/primitive.str.html#method.eq_ignore_ascii_case
    fn ignoring_ascii_case(self) -> StrMatcher<ExpectedT>;

    /// Configures the matcher to match only strings which otherwise satisfy the
    /// conditions a number times matched by the matcher `times`.
    ///
    /// ```
    /// verify_that!("Some value\nSome value", contains_substring("value").times(eq(2)))?; // Passes
    /// verify_that!("Some value", contains_substring("value").times(eq(2)))?; // Fails
    /// ```
    ///
    /// The matched substrings must be disjoint from one another to be counted.
    /// For example:
    ///
    /// ```
    /// // Fails: substrings distinct but not disjoint!
    /// verify_that!("ababab", contains_substring("abab").times(eq(2)))?;
    /// ```
    ///
    /// This is only meaningful when the matcher was constructed with
    /// [`contains_substring`]. This method will panic when it is used with any
    /// other matcher construction.
    fn times(self, times: impl Matcher<usize> + 'static) -> StrMatcher<ExpectedT>;
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
pub struct StrMatcher<ExpectedT> {
    expected: ExpectedT,
    configuration: Configuration,
}

impl<ExpectedT, ActualT> Matcher<ActualT> for StrMatcher<ExpectedT>
where
    ExpectedT: Deref<Target = str> + Debug,
    ActualT: AsRef<str> + Debug + ?Sized,
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        self.configuration.do_strings_match(self.expected.deref(), actual.as_ref()).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        self.configuration.describe(matcher_result, self.expected.deref())
    }
}

impl<ExpectedT, MatcherT: Into<StrMatcher<ExpectedT>>> StrMatcherConfigurator<ExpectedT>
    for MatcherT
{
    fn ignoring_leading_whitespace(self) -> StrMatcher<ExpectedT> {
        let existing = self.into();
        StrMatcher {
            configuration: existing.configuration.ignoring_leading_whitespace(),
            ..existing
        }
    }

    fn ignoring_trailing_whitespace(self) -> StrMatcher<ExpectedT> {
        let existing = self.into();
        StrMatcher {
            configuration: existing.configuration.ignoring_trailing_whitespace(),
            ..existing
        }
    }

    fn ignoring_outer_whitespace(self) -> StrMatcher<ExpectedT> {
        let existing = self.into();
        StrMatcher { configuration: existing.configuration.ignoring_outer_whitespace(), ..existing }
    }

    fn ignoring_ascii_case(self) -> StrMatcher<ExpectedT> {
        let existing = self.into();
        StrMatcher { configuration: existing.configuration.ignoring_ascii_case(), ..existing }
    }

    fn times(self, times: impl Matcher<usize> + 'static) -> StrMatcher<ExpectedT> {
        let existing = self.into();
        if !matches!(existing.configuration.mode, MatchMode::Contains) {
            panic!("The times() configurator is only meaningful with contains_substring().");
        }
        StrMatcher { configuration: existing.configuration.times(times), ..existing }
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
#[derive(Default)]
struct Configuration {
    mode: MatchMode,
    ignore_leading_whitespace: bool,
    ignore_trailing_whitespace: bool,
    case_policy: CasePolicy,
    times: Option<Box<dyn Matcher<usize>>>,
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
enum CasePolicy {
    #[default]
    Respect,
    IgnoreAscii,
}

impl Configuration {
    // The entry point for all string matching. StrMatcher::matches redirects
    // immediately to this function.
    fn do_strings_match(&self, expected: &str, actual: &str) -> bool {
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
                CasePolicy::Respect => self.does_containment_match(actual, expected),
                CasePolicy::IgnoreAscii => self.does_containment_match(
                    actual.to_ascii_lowercase().as_str(),
                    expected.to_ascii_lowercase().as_str(),
                ),
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

    // Returns whether actual contains expected a number of times matched by the
    // matcher self.times. Does not take other configuration into account.
    fn does_containment_match(&self, actual: &str, expected: &str) -> bool {
        if let Some(times) = self.times.as_ref() {
            // Split returns an iterator over the "boundaries" left and right of the
            // substring to be matched, of which there is one more than the number of
            // substrings.
            matches!(times.matches(&(actual.split(expected).count() - 1)), MatcherResult::Matches)
        } else {
            actual.contains(expected)
        }
    }

    // StrMatcher::describe redirects immediately to this function.
    fn describe(&self, matcher_result: MatcherResult, expected: &str) -> String {
        let mut addenda: Vec<Cow<'static, str>> = Vec::with_capacity(3);
        match (self.ignore_leading_whitespace, self.ignore_trailing_whitespace) {
            (true, true) => addenda.push("ignoring leading and trailing whitespace".into()),
            (true, false) => addenda.push("ignoring leading whitespace".into()),
            (false, true) => addenda.push("ignoring trailing whitespace".into()),
            (false, false) => {}
        }
        match self.case_policy {
            CasePolicy::Respect => {}
            CasePolicy::IgnoreAscii => addenda.push("ignoring ASCII case".into()),
        }
        if let Some(times) = self.times.as_ref() {
            addenda.push(format!("count {}", times.describe(matcher_result)).into());
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

    fn ignoring_ascii_case(self) -> Self {
        Self { case_policy: CasePolicy::IgnoreAscii, ..self }
    }

    fn times(self, times: impl Matcher<usize> + 'static) -> Self {
        Self { times: Some(Box::new(times)), ..self }
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
    use matchers::{eq, gt, not};

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
    fn matches_string_containing_expected_value_in_contains_mode() -> Result<()> {
        verify_that!("Some string", contains_substring("str"))
    }

    #[google_test]
    fn matches_string_containing_expected_value_in_contains_mode_while_ignoring_ascii_case()
    -> Result<()> {
        verify_that!("Some string", contains_substring("STR").ignoring_ascii_case())
    }

    #[google_test]
    fn contains_substring_matches_correct_number_of_substrings() -> Result<()> {
        verify_that!("Some string", contains_substring("str").times(eq(1)))
    }

    #[google_test]
    fn contains_substring_does_not_match_incorrect_number_of_substrings() -> Result<()> {
        verify_that!("Some string\nSome string", not(contains_substring("string").times(eq(1))))
    }

    #[google_test]
    fn contains_substring_does_not_match_when_substrings_overlap() -> Result<()> {
        verify_that!("ababab", not(contains_substring("abab").times(eq(2))))
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
    fn describes_itself_with_count_number() -> Result<()> {
        let matcher = contains_substring("A string").times(gt(2));
        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("contains a substring \"A string\" (count is greater than 2)")
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
