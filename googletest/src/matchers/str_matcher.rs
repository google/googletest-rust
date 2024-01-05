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

use crate::{
    description::Description,
    matcher::{Matcher, MatcherResult},
    matcher_support::{
        edit_distance,
        summarize_diff::{create_diff, create_diff_reversed},
    },
    matchers::{eq_deref_of_matcher::EqDerefOfMatcher, eq_matcher::EqMatcher},
};
use std::borrow::Cow;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

/// Matches a string containing a given substring.
///
/// Both the actual value and the expected substring may be either a `String` or
/// a string reference.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass_1() -> Result<()> {
/// verify_that!("Some value", contains_substring("Some"))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!("Another value", contains_substring("Some"))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_pass_2() -> Result<()> {
/// verify_that!("Some value".to_string(), contains_substring("value"))?;   // Passes
/// verify_that!("Some value", contains_substring("value".to_string()))?;   // Passes
/// #     Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_fail().unwrap_err();
/// # should_pass_2().unwrap();
/// ```
///
/// See the [`StrMatcherConfigurator`] extension trait for more options on how
/// the string is matched.
///
/// > Note on memory use: In most cases, this matcher does not allocate memory
/// > when matching strings. However, it must allocate copies of both the actual
/// > and expected values when matching strings while
/// > [`ignoring_ascii_case`][StrMatcherConfigurator::ignoring_ascii_case] is
/// > set.
pub fn contains_substring<A: ?Sized, T>(expected: T) -> StrMatcher<A, T> {
    StrMatcher {
        configuration: Configuration { mode: MatchMode::Contains, ..Default::default() },
        expected,
        phantom: Default::default(),
    }
}

/// Matches a string which starts with the given prefix.
///
/// Both the actual value and the expected prefix may be either a `String` or
/// a string reference.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass_1() -> Result<()> {
/// verify_that!("Some value", starts_with("Some"))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!("Another value", starts_with("Some"))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!("Some value", starts_with("value"))?;  // Fails
/// #     Ok(())
/// # }
/// # fn should_pass_2() -> Result<()> {
/// verify_that!("Some value".to_string(), starts_with("Some"))?;   // Passes
/// verify_that!("Some value", starts_with("Some".to_string()))?;   // Passes
/// #     Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_pass_2().unwrap();
/// ```
///
/// See the [`StrMatcherConfigurator`] extension trait for more options on how
/// the string is matched.
pub fn starts_with<A: ?Sized, T>(expected: T) -> StrMatcher<A, T> {
    StrMatcher {
        configuration: Configuration { mode: MatchMode::StartsWith, ..Default::default() },
        expected,
        phantom: Default::default(),
    }
}

/// Matches a string which ends with the given suffix.
///
/// Both the actual value and the expected suffix may be either a `String` or
/// a string reference.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass_1() -> Result<()> {
/// verify_that!("Some value", ends_with("value"))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!("Some value", ends_with("other value"))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!("Some value", ends_with("Some"))?;  // Fails
/// #     Ok(())
/// # }
/// # fn should_pass_2() -> Result<()> {
/// verify_that!("Some value".to_string(), ends_with("value"))?;   // Passes
/// verify_that!("Some value", ends_with("value".to_string()))?;   // Passes
/// #     Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_pass_2().unwrap();
/// ```
///
/// See the [`StrMatcherConfigurator`] extension trait for more options on how
/// the string is matched.
pub fn ends_with<A: ?Sized, T>(expected: T) -> StrMatcher<A, T> {
    StrMatcher {
        configuration: Configuration { mode: MatchMode::EndsWith, ..Default::default() },
        expected,
        phantom: Default::default(),
    }
}

/// Extension trait to configure [`StrMatcher`].
///
/// Matchers which match against string values and, through configuration,
/// specialise to [`StrMatcher`] implement this trait. That includes
/// [`EqMatcher`] and [`StrMatcher`].
pub trait StrMatcherConfigurator<ActualT: ?Sized, ExpectedT> {
    /// Configures the matcher to ignore any leading whitespace in either the
    /// actual or the expected value.
    ///
    /// Whitespace is defined as in [`str::trim_start`].
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_pass() -> Result<()> {
    /// verify_that!("A string", eq("   A string").ignoring_leading_whitespace())?; // Passes
    /// verify_that!("   A string", eq("A string").ignoring_leading_whitespace())?; // Passes
    /// #     Ok(())
    /// # }
    /// # should_pass().unwrap();
    /// ```
    ///
    /// When all other configuration options are left as the defaults, this is
    /// equivalent to invoking [`str::trim_start`] on both the expected and
    /// actual value.
    fn ignoring_leading_whitespace(self) -> StrMatcher<ActualT, ExpectedT>;

    /// Configures the matcher to ignore any trailing whitespace in either the
    /// actual or the expected value.
    ///
    /// Whitespace is defined as in [`str::trim_end`].
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_pass() -> Result<()> {
    /// verify_that!("A string", eq("A string   ").ignoring_trailing_whitespace())?; // Passes
    /// verify_that!("A string   ", eq("A string").ignoring_trailing_whitespace())?; // Passes
    /// #     Ok(())
    /// # }
    /// # should_pass().unwrap();
    /// ```
    ///
    /// When all other configuration options are left as the defaults, this is
    /// equivalent to invoking [`str::trim_end`] on both the expected and
    /// actual value.
    fn ignoring_trailing_whitespace(self) -> StrMatcher<ActualT, ExpectedT>;

    /// Configures the matcher to ignore both leading and trailing whitespace in
    /// either the actual or the expected value.
    ///
    /// Whitespace is defined as in [`str::trim`].
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_pass() -> Result<()> {
    /// verify_that!("A string", eq("   A string   ").ignoring_outer_whitespace())?; // Passes
    /// verify_that!("   A string   ", eq("A string").ignoring_outer_whitespace())?; // Passes
    /// #     Ok(())
    /// # }
    /// # should_pass().unwrap();
    /// ```
    ///
    /// This is equivalent to invoking both
    /// [`ignoring_leading_whitespace`][StrMatcherConfigurator::ignoring_leading_whitespace] and
    /// [`ignoring_trailing_whitespace`][StrMatcherConfigurator::ignoring_trailing_whitespace].
    ///
    /// When all other configuration options are left as the defaults, this is
    /// equivalent to invoking [`str::trim`] on both the expected and actual
    /// value.
    fn ignoring_outer_whitespace(self) -> StrMatcher<ActualT, ExpectedT>;

    /// Configures the matcher to ignore ASCII case when comparing values.
    ///
    /// This uses the same rules for case as [`str::eq_ignore_ascii_case`].
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_pass() -> Result<()> {
    /// verify_that!("Some value", eq("SOME VALUE").ignoring_ascii_case())?;  // Passes
    /// #     Ok(())
    /// # }
    /// # fn should_fail() -> Result<()> {
    /// verify_that!("Another value", eq("Some value").ignoring_ascii_case())?;   // Fails
    /// #     Ok(())
    /// # }
    /// # should_pass().unwrap();
    /// # should_fail().unwrap_err();
    /// ```
    ///
    /// This is **not guaranteed** to match strings with differing upper/lower
    /// case characters outside of the codepoints 0-127 covered by ASCII.
    fn ignoring_ascii_case(self) -> StrMatcher<ActualT, ExpectedT>;

    /// Configures the matcher to match only strings which otherwise satisfy the
    /// conditions a number times matched by the matcher `times`.
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_pass() -> Result<()> {
    /// verify_that!("Some value\nSome value", contains_substring("value").times(eq(2)))?; // Passes
    /// #     Ok(())
    /// # }
    /// # fn should_fail() -> Result<()> {
    /// verify_that!("Some value", contains_substring("value").times(eq(2)))?; // Fails
    /// #     Ok(())
    /// # }
    /// # should_pass().unwrap();
    /// # should_fail().unwrap_err();
    /// ```
    ///
    /// The matched substrings must be disjoint from one another to be counted.
    /// For example:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_fail() -> Result<()> {
    /// // Fails: substrings distinct but not disjoint!
    /// verify_that!("ababab", contains_substring("abab").times(eq(2)))?;
    /// #     Ok(())
    /// # }
    /// # should_fail().unwrap_err();
    /// ```
    ///
    /// This is only meaningful when the matcher was constructed with
    /// [`contains_substring`]. This method will panic when it is used with any
    /// other matcher construction.
    fn times(
        self,
        times: impl Matcher<ActualT = usize> + 'static,
    ) -> StrMatcher<ActualT, ExpectedT>;
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
pub struct StrMatcher<ActualT: ?Sized, ExpectedT> {
    expected: ExpectedT,
    configuration: Configuration,
    phantom: PhantomData<ActualT>,
}

impl<ExpectedT, ActualT> Matcher for StrMatcher<ActualT, ExpectedT>
where
    ExpectedT: Deref<Target = str> + Debug,
    ActualT: AsRef<str> + Debug + ?Sized,
{
    type ActualT = ActualT;

    fn matches(&self, actual: &ActualT) -> MatcherResult {
        self.configuration.do_strings_match(self.expected.deref(), actual.as_ref()).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.configuration.describe(matcher_result, self.expected.deref())
    }

    fn explain_match(&self, actual: &ActualT) -> Description {
        self.configuration.explain_match(self.expected.deref(), actual.as_ref())
    }
}

impl<ActualT: ?Sized, ExpectedT, MatcherT: Into<StrMatcher<ActualT, ExpectedT>>>
    StrMatcherConfigurator<ActualT, ExpectedT> for MatcherT
{
    fn ignoring_leading_whitespace(self) -> StrMatcher<ActualT, ExpectedT> {
        let existing = self.into();
        StrMatcher {
            configuration: existing.configuration.ignoring_leading_whitespace(),
            ..existing
        }
    }

    fn ignoring_trailing_whitespace(self) -> StrMatcher<ActualT, ExpectedT> {
        let existing = self.into();
        StrMatcher {
            configuration: existing.configuration.ignoring_trailing_whitespace(),
            ..existing
        }
    }

    fn ignoring_outer_whitespace(self) -> StrMatcher<ActualT, ExpectedT> {
        let existing = self.into();
        StrMatcher { configuration: existing.configuration.ignoring_outer_whitespace(), ..existing }
    }

    fn ignoring_ascii_case(self) -> StrMatcher<ActualT, ExpectedT> {
        let existing = self.into();
        StrMatcher { configuration: existing.configuration.ignoring_ascii_case(), ..existing }
    }

    fn times(
        self,
        times: impl Matcher<ActualT = usize> + 'static,
    ) -> StrMatcher<ActualT, ExpectedT> {
        let existing = self.into();
        if !matches!(existing.configuration.mode, MatchMode::Contains) {
            panic!("The times() configurator is only meaningful with contains_substring().");
        }
        StrMatcher { configuration: existing.configuration.times(times), ..existing }
    }
}

impl<A: ?Sized, T: Deref<Target = str>> From<EqMatcher<A, T>> for StrMatcher<A, T> {
    fn from(value: EqMatcher<A, T>) -> Self {
        Self::with_default_config(value.expected)
    }
}

impl<A: ?Sized, T: Deref<Target = str>> From<EqDerefOfMatcher<A, T>> for StrMatcher<A, T> {
    fn from(value: EqDerefOfMatcher<A, T>) -> Self {
        Self::with_default_config(value.expected)
    }
}

impl<A: ?Sized, T> StrMatcher<A, T> {
    /// Returns a [`StrMatcher`] with a default configuration to match against
    /// the given expected value.
    ///
    /// This default configuration is sensitive to whitespace and case.
    fn with_default_config(expected: T) -> Self {
        Self { expected, configuration: Default::default(), phantom: Default::default() }
    }
}

// Holds all the information on how the expected and actual strings are to be
// compared. Its associated functions perform the actual matching operations
// on string references. The struct and comparison methods therefore need not be
// parameterised, saving compilation time and binary size on monomorphisation.
//
// The default value represents exact equality of the strings.
struct Configuration {
    mode: MatchMode,
    ignore_leading_whitespace: bool,
    ignore_trailing_whitespace: bool,
    case_policy: CasePolicy,
    times: Option<Box<dyn Matcher<ActualT = usize>>>,
}

#[derive(Clone)]
enum MatchMode {
    Equals,
    Contains,
    StartsWith,
    EndsWith,
}

impl MatchMode {
    fn to_diff_mode(&self) -> edit_distance::Mode {
        match self {
            MatchMode::StartsWith | MatchMode::EndsWith => edit_distance::Mode::Prefix,
            MatchMode::Contains => edit_distance::Mode::Contains,
            MatchMode::Equals => edit_distance::Mode::Exact,
        }
    }
}

#[derive(Clone)]
enum CasePolicy {
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
            matches!(times.matches(&(actual.split(expected).count() - 1)), MatcherResult::Match)
        } else {
            actual.contains(expected)
        }
    }

    // StrMatcher::describe redirects immediately to this function.
    fn describe(&self, matcher_result: MatcherResult, expected: &str) -> Description {
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
                MatcherResult::Match => "is equal to",
                MatcherResult::NoMatch => "isn't equal to",
            },
            MatchMode::Contains => match matcher_result {
                MatcherResult::Match => "contains a substring",
                MatcherResult::NoMatch => "does not contain a substring",
            },
            MatchMode::StartsWith => match matcher_result {
                MatcherResult::Match => "starts with prefix",
                MatcherResult::NoMatch => "does not start with",
            },
            MatchMode::EndsWith => match matcher_result {
                MatcherResult::Match => "ends with suffix",
                MatcherResult::NoMatch => "does not end with",
            },
        };
        format!("{match_mode_description} {expected:?}{extra}").into()
    }

    fn explain_match(&self, expected: &str, actual: &str) -> Description {
        let default_explanation = format!(
            "which {}",
            self.describe(self.do_strings_match(expected, actual).into(), expected)
        )
        .into();
        if !expected.contains('\n') || !actual.contains('\n') {
            return default_explanation;
        }

        if self.ignore_leading_whitespace {
            // TODO - b/283448414 : Support StrMatcher with ignore_leading_whitespace.
            return default_explanation;
        }

        if self.ignore_trailing_whitespace {
            // TODO - b/283448414 : Support StrMatcher with ignore_trailing_whitespace.
            return default_explanation;
        }

        if self.times.is_some() {
            // TODO - b/283448414 : Support StrMatcher with times.
            return default_explanation;
        }
        if matches!(self.case_policy, CasePolicy::IgnoreAscii) {
            // TODO - b/283448414 : Support StrMatcher with ignore ascii case policy.
            return default_explanation;
        }
        if self.do_strings_match(expected, actual) {
            // TODO - b/283448414 : Consider supporting debug difference if the
            // strings match. This can be useful when a small contains is found
            // in a long string.
            return default_explanation;
        }

        let diff = match self.mode {
            MatchMode::Equals | MatchMode::StartsWith | MatchMode::Contains => {
                // TODO(b/287632452): Also consider improving the output in MatchMode::Contains
                // when the substring begins or ends in the middle of a line of the actual
                // value.
                create_diff(actual, expected, self.mode.to_diff_mode())
            }
            MatchMode::EndsWith => create_diff_reversed(actual, expected, self.mode.to_diff_mode()),
        };

        format!("{default_explanation}\n{diff}").into()
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

    fn times(self, times: impl Matcher<ActualT = usize> + 'static) -> Self {
        Self { times: Some(Box::new(times)), ..self }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            mode: MatchMode::Equals,
            ignore_leading_whitespace: false,
            ignore_trailing_whitespace: false,
            case_policy: CasePolicy::Respect,
            times: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{contains_substring, ends_with, starts_with, StrMatcher, StrMatcherConfigurator};
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn matches_string_reference_with_equal_string_reference() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!("A string", matcher)
    }

    #[test]
    fn does_not_match_string_reference_with_non_equal_string_reference() -> Result<()> {
        let matcher = StrMatcher::with_default_config("Another string");
        verify_that!("A string", not(matcher))
    }

    #[test]
    fn matches_owned_string_with_string_reference() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        let value = "A string".to_string();
        verify_that!(value, matcher)
    }

    #[test]
    fn matches_owned_string_reference_with_string_reference() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        let value = "A string".to_string();
        verify_that!(&value, matcher)
    }

    #[test]
    fn ignores_leading_whitespace_in_expected_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config(" \n\tA string");
        verify_that!("A string", matcher.ignoring_leading_whitespace())
    }

    #[test]
    fn ignores_leading_whitespace_in_actual_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!(" \n\tA string", matcher.ignoring_leading_whitespace())
    }

    #[test]
    fn does_not_match_unequal_remaining_string_when_ignoring_leading_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config(" \n\tAnother string");
        verify_that!("A string", not(matcher.ignoring_leading_whitespace()))
    }

    #[test]
    fn remains_sensitive_to_trailing_whitespace_when_ignoring_leading_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string \n\t");
        verify_that!("A string", not(matcher.ignoring_leading_whitespace()))
    }

    #[test]
    fn ignores_trailing_whitespace_in_expected_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string \n\t");
        verify_that!("A string", matcher.ignoring_trailing_whitespace())
    }

    #[test]
    fn ignores_trailing_whitespace_in_actual_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!("A string \n\t", matcher.ignoring_trailing_whitespace())
    }

    #[test]
    fn does_not_match_unequal_remaining_string_when_ignoring_trailing_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config("Another string \n\t");
        verify_that!("A string", not(matcher.ignoring_trailing_whitespace()))
    }

    #[test]
    fn remains_sensitive_to_leading_whitespace_when_ignoring_trailing_whitespace() -> Result<()> {
        let matcher = StrMatcher::with_default_config(" \n\tA string");
        verify_that!("A string", not(matcher.ignoring_trailing_whitespace()))
    }

    #[test]
    fn ignores_leading_and_trailing_whitespace_in_expected_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config(" \n\tA string \n\t");
        verify_that!("A string", matcher.ignoring_outer_whitespace())
    }

    #[test]
    fn ignores_leading_and_trailing_whitespace_in_actual_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!(" \n\tA string \n\t", matcher.ignoring_outer_whitespace())
    }

    #[test]
    fn respects_ascii_case_by_default() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!("A STRING", not(matcher))
    }

    #[test]
    fn ignores_ascii_case_when_requested() -> Result<()> {
        let matcher = StrMatcher::with_default_config("A string");
        verify_that!("A STRING", matcher.ignoring_ascii_case())
    }

    #[test]
    fn allows_ignoring_leading_whitespace_from_eq() -> Result<()> {
        verify_that!("A string", eq(" \n\tA string").ignoring_leading_whitespace())
    }

    #[test]
    fn allows_ignoring_trailing_whitespace_from_eq() -> Result<()> {
        verify_that!("A string", eq("A string \n\t").ignoring_trailing_whitespace())
    }

    #[test]
    fn allows_ignoring_outer_whitespace_from_eq() -> Result<()> {
        verify_that!("A string", eq(" \n\tA string \n\t").ignoring_outer_whitespace())
    }

    #[test]
    fn allows_ignoring_ascii_case_from_eq() -> Result<()> {
        verify_that!("A string", eq("A STRING").ignoring_ascii_case())
    }

    #[test]
    fn allows_ignoring_ascii_case_from_eq_deref_of_str_slice() -> Result<()> {
        verify_that!("A string", eq_deref_of("A STRING").ignoring_ascii_case())
    }

    #[test]
    fn allows_ignoring_ascii_case_from_eq_deref_of_owned_string() -> Result<()> {
        verify_that!("A string", eq_deref_of("A STRING".to_string()).ignoring_ascii_case())
    }

    #[test]
    fn matches_string_containing_expected_value_in_contains_mode() -> Result<()> {
        verify_that!("Some string", contains_substring("str"))
    }

    #[test]
    fn matches_string_containing_expected_value_in_contains_mode_while_ignoring_ascii_case()
    -> Result<()> {
        verify_that!("Some string", contains_substring("STR").ignoring_ascii_case())
    }

    #[test]
    fn contains_substring_matches_correct_number_of_substrings() -> Result<()> {
        verify_that!("Some string", contains_substring("str").times(eq(1)))
    }

    #[test]
    fn contains_substring_does_not_match_incorrect_number_of_substrings() -> Result<()> {
        verify_that!("Some string\nSome string", not(contains_substring("string").times(eq(1))))
    }

    #[test]
    fn contains_substring_does_not_match_when_substrings_overlap() -> Result<()> {
        verify_that!("ababab", not(contains_substring("abab").times(eq(2))))
    }

    #[test]
    fn starts_with_matches_string_reference_with_prefix() -> Result<()> {
        verify_that!("Some value", starts_with("Some"))
    }

    #[test]
    fn starts_with_matches_string_reference_with_prefix_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some value", starts_with("SOME").ignoring_ascii_case())
    }

    #[test]
    fn starts_with_does_not_match_wrong_prefix_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some value", not(starts_with("OTHER").ignoring_ascii_case()))
    }

    #[test]
    fn ends_with_does_not_match_short_string_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some", not(starts_with("OTHER").ignoring_ascii_case()))
    }

    #[test]
    fn starts_with_does_not_match_string_without_prefix() -> Result<()> {
        verify_that!("Some value", not(starts_with("Another")))
    }

    #[test]
    fn starts_with_does_not_match_string_with_substring_not_at_beginning() -> Result<()> {
        verify_that!("Some value", not(starts_with("value")))
    }

    #[test]
    fn ends_with_matches_string_reference_with_suffix() -> Result<()> {
        verify_that!("Some value", ends_with("value"))
    }

    #[test]
    fn ends_with_matches_string_reference_with_suffix_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some value", ends_with("VALUE").ignoring_ascii_case())
    }

    #[test]
    fn ends_with_does_not_match_wrong_suffix_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some value", not(ends_with("OTHER").ignoring_ascii_case()))
    }

    #[test]
    fn ends_with_does_not_match_too_short_string_ignoring_ascii_case() -> Result<()> {
        verify_that!("Some", not(ends_with("OTHER").ignoring_ascii_case()))
    }

    #[test]
    fn ends_with_does_not_match_string_without_suffix() -> Result<()> {
        verify_that!("Some value", not(ends_with("other value")))
    }

    #[test]
    fn ends_with_does_not_match_string_with_substring_not_at_end() -> Result<()> {
        verify_that!("Some value", not(ends_with("Some")))
    }

    #[test]
    fn describes_itself_for_matching_result() -> Result<()> {
        let matcher: StrMatcher<&str, _> = StrMatcher::with_default_config("A string");
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("is equal to \"A string\""))
        )
    }

    #[test]
    fn describes_itself_for_non_matching_result() -> Result<()> {
        let matcher: StrMatcher<&str, _> = StrMatcher::with_default_config("A string");
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::NoMatch),
            displays_as(eq("isn't equal to \"A string\""))
        )
    }

    #[test]
    fn describes_itself_for_matching_result_ignoring_leading_whitespace() -> Result<()> {
        let matcher: StrMatcher<&str, _> =
            StrMatcher::with_default_config("A string").ignoring_leading_whitespace();
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("is equal to \"A string\" (ignoring leading whitespace)"))
        )
    }

    #[test]
    fn describes_itself_for_non_matching_result_ignoring_leading_whitespace() -> Result<()> {
        let matcher: StrMatcher<&str, _> =
            StrMatcher::with_default_config("A string").ignoring_leading_whitespace();
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::NoMatch),
            displays_as(eq("isn't equal to \"A string\" (ignoring leading whitespace)"))
        )
    }

    #[test]
    fn describes_itself_for_matching_result_ignoring_trailing_whitespace() -> Result<()> {
        let matcher: StrMatcher<&str, _> =
            StrMatcher::with_default_config("A string").ignoring_trailing_whitespace();
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("is equal to \"A string\" (ignoring trailing whitespace)"))
        )
    }

    #[test]
    fn describes_itself_for_matching_result_ignoring_leading_and_trailing_whitespace() -> Result<()>
    {
        let matcher: StrMatcher<&str, _> =
            StrMatcher::with_default_config("A string").ignoring_outer_whitespace();
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("is equal to \"A string\" (ignoring leading and trailing whitespace)"))
        )
    }

    #[test]
    fn describes_itself_for_matching_result_ignoring_ascii_case() -> Result<()> {
        let matcher: StrMatcher<&str, _> =
            StrMatcher::with_default_config("A string").ignoring_ascii_case();
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("is equal to \"A string\" (ignoring ASCII case)"))
        )
    }

    #[test]
    fn describes_itself_for_matching_result_ignoring_ascii_case_and_leading_whitespace()
    -> Result<()> {
        let matcher: StrMatcher<&str, _> = StrMatcher::with_default_config("A string")
            .ignoring_leading_whitespace()
            .ignoring_ascii_case();
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq(
                "is equal to \"A string\" (ignoring leading whitespace, ignoring ASCII case)"
            ))
        )
    }

    #[test]
    fn describes_itself_for_matching_result_in_contains_mode() -> Result<()> {
        let matcher: StrMatcher<&str, _> = contains_substring("A string");
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("contains a substring \"A string\""))
        )
    }

    #[test]
    fn describes_itself_for_non_matching_result_in_contains_mode() -> Result<()> {
        let matcher: StrMatcher<&str, _> = contains_substring("A string");
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::NoMatch),
            displays_as(eq("does not contain a substring \"A string\""))
        )
    }

    #[test]
    fn describes_itself_with_count_number() -> Result<()> {
        let matcher: StrMatcher<&str, _> = contains_substring("A string").times(gt(2));
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("contains a substring \"A string\" (count is greater than 2)"))
        )
    }

    #[test]
    fn describes_itself_for_matching_result_in_starts_with_mode() -> Result<()> {
        let matcher: StrMatcher<&str, _> = starts_with("A string");
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("starts with prefix \"A string\""))
        )
    }

    #[test]
    fn describes_itself_for_non_matching_result_in_starts_with_mode() -> Result<()> {
        let matcher: StrMatcher<&str, _> = starts_with("A string");
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::NoMatch),
            displays_as(eq("does not start with \"A string\""))
        )
    }

    #[test]
    fn describes_itself_for_matching_result_in_ends_with_mode() -> Result<()> {
        let matcher: StrMatcher<&str, _> = ends_with("A string");
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("ends with suffix \"A string\""))
        )
    }

    #[test]
    fn describes_itself_for_non_matching_result_in_ends_with_mode() -> Result<()> {
        let matcher: StrMatcher<&str, _> = ends_with("A string");
        verify_that!(
            Matcher::describe(&matcher, MatcherResult::NoMatch),
            displays_as(eq("does not end with \"A string\""))
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
            starts_with(indoc!(
                "
                    First line
                    Second lines
                    Third line
                "
            ))
        );

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "\
   First line
  -Second line
  +Second lines
   Third line"
            )))
        )
    }

    #[test]
    fn match_explanation_for_starts_with_ignores_trailing_lines_in_actual_string() -> Result<()> {
        let result = verify_that!(
            indoc!(
                "
                    First line
                    Second line
                    Third line
                    Fourth line
                "
            ),
            starts_with(indoc!(
                "
                    First line
                    Second lines
                    Third line
                "
            ))
        );

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "
   First line
  -Second line
  +Second lines
   Third line
   <---- remaining lines omitted ---->"
            )))
        )
    }

    #[test]
    fn match_explanation_for_starts_with_includes_both_versions_of_differing_last_line()
    -> Result<()> {
        let result = verify_that!(
            indoc!(
                "
                    First line
                    Second line
                    Third line
                "
            ),
            starts_with(indoc!(
                "
                    First line
                    Second lines
                "
            ))
        );

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "\
   First line
  -Second line
  +Second lines
   <---- remaining lines omitted ---->"
            )))
        )
    }

    #[test]
    fn match_explanation_for_ends_with_ignores_leading_lines_in_actual_string() -> Result<()> {
        let result = verify_that!(
            indoc!(
                "
                    First line
                    Second line
                    Third line
                    Fourth line
                "
            ),
            ends_with(indoc!(
                "
                    Second line
                    Third lines
                    Fourth line
                "
            ))
        );

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "
  Difference(-actual / +expected):
   <---- remaining lines omitted ---->
   Second line
  -Third line
  +Third lines
   Fourth line"
            )))
        )
    }

    #[test]
    fn match_explanation_for_contains_substring_ignores_outer_lines_in_actual_string() -> Result<()>
    {
        let result = verify_that!(
            indoc!(
                "
                    First line
                    Second line
                    Third line
                    Fourth line
                    Fifth line
                "
            ),
            contains_substring(indoc!(
                "
                    Second line
                    Third lines
                    Fourth line
                "
            ))
        );

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "
  Difference(-actual / +expected):
   <---- remaining lines omitted ---->
   Second line
  -Third line
  +Third lines
   Fourth line
   <---- remaining lines omitted ---->"
            )))
        )
    }

    #[test]
    fn match_explanation_for_contains_substring_shows_diff_when_first_and_last_line_are_incomplete()
    -> Result<()> {
        let result = verify_that!(
            indoc!(
                "
                    First line
                    Second line
                    Third line
                    Fourth line
                    Fifth line
                "
            ),
            contains_substring(indoc!(
                "
                    line
                    Third line
                    Foorth line
                    Fifth"
            ))
        );

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "
  Difference(-actual / +expected):
   <---- remaining lines omitted ---->
  -Second line
  +line
   Third line
  -Fourth line
  +Foorth line
  -Fifth line
  +Fifth
   <---- remaining lines omitted ---->"
            )))
        )
    }

    #[test]
    fn match_explanation_for_eq_does_not_ignore_trailing_lines_in_actual_string() -> Result<()> {
        let result = verify_that!(
            indoc!(
                "
                    First line
                    Second line
                    Third line
                    Fourth line
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
            err(displays_as(contains_substring(
                "\
   First line
  -Second line
  +Second lines
   Third line
  -Fourth line"
            )))
        )
    }

    #[test]
    fn match_explanation_does_not_show_diff_if_actual_value_is_single_line() -> Result<()> {
        let result = verify_that!(
            "First line",
            starts_with(indoc!(
                "
                    Second line
                    Third line
                "
            ))
        );

        verify_that!(
            result,
            err(displays_as(not(contains_substring("Difference(-actual / +expected):"))))
        )
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
            starts_with("Second line")
        );

        verify_that!(
            result,
            err(displays_as(not(contains_substring("Difference(-actual / +expected):"))))
        )
    }
}
