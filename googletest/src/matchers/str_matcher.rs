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
}

/// A matcher which matches equality or containment of a string-like value in a
/// configurable way.
///
/// Use the matcher methods [`eq`][googletest::matchers::eq_matcher::eq] et al.
/// to instantiate this.
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
    ignore_leading_whitespace: bool,
    ignore_trailing_whitespace: bool,
}

impl Configuration {
    // The entry point for all string matching. StrMatcher::matches redirects
    // immediately to this function.
    fn do_strings_match(&self, expected: &str, actual: &str) -> bool {
        let (expected, actual) =
            match (self.ignore_leading_whitespace, self.ignore_trailing_whitespace) {
                (true, _) => (expected.trim_start(), actual.trim_start()),
                (false, true) => (expected.trim_end(), actual.trim_end()),
                (false, false) => (expected.deref(), actual),
            };
        expected == actual
    }

    // StrMatcher::describe redirects immediately to this function.
    fn describe(&self, matcher_result: MatcherResult, expected: &str) -> String {
        let extra = match (self.ignore_leading_whitespace, self.ignore_trailing_whitespace) {
            (true, _) => " (ignoring leading whitespace)",
            (false, true) => " (ignoring trailing whitespace)",
            (false, false) => "",
        };
        match matcher_result {
            MatcherResult::Matches => format!("is equal to {expected:?}{extra}"),
            MatcherResult::DoesNotMatch => format!("isn't equal to {expected:?}{extra}"),
        }
    }

    fn ignoring_leading_whitespace(self) -> Self {
        Self { ignore_leading_whitespace: true, ..self }
    }

    fn ignoring_trailing_whitespace(self) -> Self {
        Self { ignore_trailing_whitespace: true, ..self }
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
    fn allows_ignoring_leading_whitespace_from_eq() -> Result<()> {
        verify_that!("A string", eq(" \n\tA string").ignoring_leading_whitespace())
    }

    #[google_test]
    fn allows_ignoring_trailing_whitespace_from_eq() -> Result<()> {
        verify_that!("A string", eq("A string \n\t").ignoring_trailing_whitespace())
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
}
