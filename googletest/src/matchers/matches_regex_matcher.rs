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

use crate::description::Description;
use crate::matcher::{Matcher, MatcherBase, MatcherResult};
use regex::Regex;
use std::fmt::Debug;
use std::ops::Deref;

/// Matches a string the entirety of which which matches the given regular
/// expression.
///
/// This is similar to [`contains_regex`][crate::matchers::contains_regex],
/// except that the match must cover the whole string and not a substring.
///
/// Both the actual value and the expected regular expression may be either a
/// `String` or a string reference.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass_1() -> Result<()> {
/// verify_that!("Some value", matches_regex("S.*e"))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!("Another value", matches_regex("Some"))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!("Some value", matches_regex("Some"))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_pass_2() -> Result<()> {
/// verify_that!("Some value".to_string(), matches_regex(".*v.*e"))?;   // Passes
/// verify_that!("Some value", matches_regex(".*v.*e".to_string()))?;   // Passes
/// #     Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_pass_2().unwrap();
/// ```
///
/// Panics if the given `pattern` is not a syntactically valid regular
/// expression.
// N.B. This returns the concrete type rather than an impl Matcher so that it
// can act simultaneously as a Matcher<str> and a Matcher<String>. Otherwise the
// compiler treats it as a Matcher<str> only and the code
//   verify_that!("Some value".to_string(), matches_regex(".*value"))?;
// doesn't compile.
pub fn matches_regex<PatternT: Deref<Target = str>>(
    pattern: PatternT,
) -> MatchesRegexMatcher<PatternT> {
    let adjusted_pattern = format!("^{}$", pattern.deref());
    let regex = Regex::new(adjusted_pattern.as_str()).unwrap();
    MatchesRegexMatcher { regex, pattern, _adjusted_pattern: adjusted_pattern }
}

/// A matcher matching a string-like type matching a given regular expression.
///
/// Intended only to be used from the function [`matches_regex`] only.
/// Should not be referenced by code outside this library.
#[derive(MatcherBase)]
pub struct MatchesRegexMatcher<PatternT: Deref<Target = str>> {
    regex: Regex,
    pattern: PatternT,
    _adjusted_pattern: String,
}

impl<PatternT, ActualT> Matcher<ActualT> for MatchesRegexMatcher<PatternT>
where
    PatternT: Deref<Target = str>,
    ActualT: AsRef<str> + Debug + Copy,
{
    fn matches(&self, actual: ActualT) -> MatcherResult {
        self.regex.is_match(actual.as_ref()).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("matches the regular expression {:#?}", self.pattern.deref()).into()
            }
            MatcherResult::NoMatch => {
                format!("doesn't match the regular expression {:#?}", self.pattern.deref()).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::MatcherResult;
    use crate::prelude::*;
    use crate::Result;

    #[test]
    fn matches_regex_matches_string_reference_with_pattern() -> Result<()> {
        let matcher = matches_regex("S.*e");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn matches_regex_does_not_match_string_without_pattern() -> Result<()> {
        let matcher = matches_regex("Another");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn matches_regex_does_not_match_string_only_beginning_of_which_matches() -> Result<()> {
        let matcher = matches_regex("Some");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn matches_regex_does_not_match_string_only_end_of_which_matches() -> Result<()> {
        let matcher = matches_regex("value");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn matches_regex_matches_owned_string_with_pattern() -> Result<()> {
        let matcher = matches_regex(".*value");

        let result = matcher.matches(&"Some value".to_string());

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn matches_regex_matches_string_when_regex_has_beginning_of_string_marker() -> Result<()> {
        let matcher = matches_regex("^Some value");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn matches_regex_matches_string_when_regex_has_end_of_string_marker() -> Result<()> {
        let matcher = matches_regex("Some value$");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn matches_regex_matches_string_when_regex_has_both_end_markers() -> Result<()> {
        let matcher = matches_regex("^Some value$");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn matches_regex_matches_string_reference_with_owned_string() -> Result<()> {
        let matcher = matches_regex(".*value".to_string());

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn verify_that_works_with_owned_string() -> Result<()> {
        verify_that!("Some value".to_string(), matches_regex(".*value"))
    }

    #[test]
    fn matches_regex_displays_quoted_debug_of_pattern() -> Result<()> {
        let matcher = matches_regex("\n");

        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Match),
            displays_as(eq("matches the regular expression \"\\n\""))
        )
    }
}
