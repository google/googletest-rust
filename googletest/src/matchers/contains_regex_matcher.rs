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

#[cfg(not(google3))]
use crate as googletest;
use googletest::matcher::{Matcher, MatcherResult};
use regex::Regex;
use std::fmt::Debug;
use std::ops::Deref;

/// Matches a string containing a substring which matches the given regular
/// expression.
///
/// Both the actual value and the expected regular expression may be either a
/// `String` or a string reference.
///
/// ```
/// verify_that!("Some value", contains_regex("S.*e"))?;  // Passes
/// verify_that!("Another value", contains_regex("Some"))?;   // Fails
/// verify_that!("Some value".to_string(), contains_regex("v.*e"))?;   // Passes
/// verify_that!("Some value", contains_regex("v.*e".to_string()))?;   // Passes
/// ```
///
/// Panics if the given `pattern` is not a syntactically valid regular
/// expression.
// N.B. This returns the concrete type rather than an impl Matcher so that it
// can act simultaneously as a Matcher<str> and a Matcher<String>. Otherwise the
// compiler treats it as a Matcher<str> only and the code
//   verify_that!("Some value".to_string(), contains_regex(".*value"))?;
// doesn't compile.
pub fn contains_regex<PatternT: Deref<Target = str>>(pattern: PatternT) -> ContainsRegexMatcher {
    ContainsRegexMatcher { regex: Regex::new(pattern.deref()).unwrap() }
}

/// A matcher matching a string-like type containing a substring matching a
/// given regular expression.
///
/// Intended only to be used from the function [`contains_regex`] only.
/// Should not be referenced by code outside this library.
pub struct ContainsRegexMatcher {
    regex: Regex,
}

impl<ActualT: AsRef<str> + Debug + ?Sized> Matcher<ActualT> for ContainsRegexMatcher {
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        self.regex.is_match(actual.as_ref()).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!("contains the regular expression {:#?}", self.regex.as_str())
            }
            MatcherResult::DoesNotMatch => {
                format!("doesn't contain the regular expression {:#?}", self.regex.as_str())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(google3))]
    use crate as googletest;
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{google_test, matcher::Matcher, verify_that, Result};
    use matchers::eq;

    #[google_test]
    fn contains_regex_matches_string_reference_with_pattern() -> Result<()> {
        let matcher = contains_regex("S.*val");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn contains_regex_does_not_match_string_without_pattern() -> Result<()> {
        let matcher = contains_regex("Another");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn contains_regex_matches_owned_string_with_pattern() -> Result<()> {
        let matcher = contains_regex("value");

        let result = matcher.matches(&"Some value".to_string());

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn contains_regex_matches_string_reference_with_owned_string() -> Result<()> {
        let matcher = contains_regex("value".to_string());

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn verify_that_works_with_owned_string() -> Result<()> {
        verify_that!("Some value".to_string(), contains_regex("value"))
    }

    #[google_test]
    fn contains_regex_displays_quoted_debug_of_pattern() -> Result<()> {
        let matcher = contains_regex("\n");

        verify_that!(
            <ContainsRegexMatcher as Matcher<&str>>::describe(&matcher, MatcherResult::Matches),
            eq("contains the regular expression \"\\n\"")
        )
    }
}
