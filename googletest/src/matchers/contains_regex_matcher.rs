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

/// Matches a string containing a substring which matches the given regular
/// expression.
///
/// Both the actual value and the expected regular expression may be either a
/// `String` or a string reference.
///
/// # Regular Expression Syntax
///
/// The regular expression engine used is the Rust [`regex`](https://docs.rs/regex) crate,
/// which supports **RE2 syntax** (guaranteeing linear time matching without
/// backtracking). For a full reference on accepted syntax, see the
/// [RE2 Syntax Wiki](https://github.com/google/re2/wiki/Syntax).
///
/// # Examples
///
/// Basic substring matching:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass_1() -> Result<()> {
/// verify_that!("Some value", contains_regex("S.*e"))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!("Another value", contains_regex("Some"))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_pass_2() -> Result<()> {
/// verify_that!("Some value".to_string(), contains_regex("v.*e"))?;   // Passes
/// verify_that!("Some value", contains_regex("v.*e".to_string()))?;   // Passes
/// #     Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_fail().unwrap_err();
/// # should_pass_2().unwrap();
/// ```
///
/// ## Complex Syntax and Inline Flags
///
/// Inline flags can be used to modify matching behavior, such as `(?m)` for
/// multi-line mode (where `^` and `$` match line boundaries instead of string
/// boundaries) and `(?s)` so that `.` matches newlines:
///
/// ```
/// # use googletest::prelude::*;
/// # fn multi_line_example() -> Result<()> {
/// let log = "INFO: Starting\nERROR: Failed to connect\nINFO: Retrying";
///
/// // Use (?m) to enable multi-line matching of anchors (^ and $):
/// verify_that!(log, contains_regex("(?m)^ERROR:.*$"))?;
///
/// // Use (?s) to allow '.' to match across newline boundaries:
/// verify_that!(log, contains_regex("(?s)Starting.*Retrying"))?;
/// #     Ok(())
/// # }
/// # multi_line_example().unwrap();
/// ```
///
/// ## Matching Multiple Occurrences and Match Positions
///
/// While `contains_regex` verifies that a pattern appears at least once, you
/// can also use complex regex syntax (like non-capturing groups `(?:...)` and
/// repetition quantifiers `{n}`) to verify multiple occurrences across a
/// string.
///
/// If you need to inspect the exact byte or character positions of all matches
/// of a substring in a string containing multiple occurrences, you can use
/// [`regex::Regex::find_iter`] combined with container matchers such as
/// [`elements_are!`][crate::matchers::elements_are]:
///
/// ```
/// # use googletest::prelude::*;
/// # use regex::Regex;
/// # fn multiple_occurrences_example() -> Result<()> {
/// let text = "token at index 0, token at index 18, token at index 37";
///
/// // Verify that the pattern occurs at least 3 times using inline flags and repetition:
/// verify_that!(text, contains_regex("(?s)(?:.*token){3}"))?;
///
/// // Extract and verify the exact byte positions of all matches of the substring:
/// let re = Regex::new("token").unwrap();
/// let positions: Vec<usize> = re.find_iter(text).map(|m| m.start()).collect();
/// verify_that!(positions, elements_are![eq(&0), eq(&18), eq(&37)])?;
/// #     Ok(())
/// # }
/// # multiple_occurrences_example().unwrap();
/// ```
///
/// Panics if the given `pattern` is not a syntactically valid regular
/// expression.
#[track_caller]
pub fn contains_regex<PatternT: Deref<Target = str>>(pattern: PatternT) -> ContainsRegexMatcher {
    ContainsRegexMatcher { regex: Regex::new(pattern.deref()).unwrap() }
}

/// A matcher matching a string-like type containing a substring matching a
/// given regular expression.
///
/// Intended only to be used from the function [`contains_regex`] only.
/// Should not be referenced by code outside this library.
#[derive(MatcherBase)]
pub struct ContainsRegexMatcher {
    regex: Regex,
}

impl<ActualT: AsRef<str> + Debug + Copy> Matcher<ActualT> for ContainsRegexMatcher {
    fn matches(&self, actual: ActualT) -> MatcherResult {
        self.regex.is_match(actual.as_ref()).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("contains the regular expression {:#?}", self.regex.as_str()).into()
            }
            MatcherResult::NoMatch => {
                format!("doesn't contain the regular expression {:#?}", self.regex.as_str()).into()
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
    fn contains_regex_matches_string_reference_with_pattern() -> Result<()> {
        let matcher = contains_regex("S.*val");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_regex_does_not_match_string_without_pattern() -> Result<()> {
        let matcher = contains_regex("Another");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_regex_matches_owned_string_with_pattern() -> Result<()> {
        let matcher = contains_regex("value");

        let result = matcher.matches(&"Some value".to_string());

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_regex_matches_string_reference_with_owned_string() -> Result<()> {
        let matcher = contains_regex("value");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn verify_that_works_with_owned_string() -> Result<()> {
        verify_that!("Some value".to_string(), contains_regex("value"))
    }

    #[test]
    fn contains_regex_displays_quoted_debug_of_pattern() -> Result<()> {
        let matcher = contains_regex("\n");

        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Match),
            displays_as(eq("contains the regular expression \"\\n\""))
        )
    }

    #[test]
    fn contains_regex_matches_multi_line() -> Result<()> {
        let matcher = contains_regex("(?m)^Second line$");
        let result = matcher.matches("First line\nSecond line\nThird line");
        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_regex_matches_across_newlines() -> Result<()> {
        let matcher = contains_regex("(?s)Starting.*Retrying");
        let result = matcher.matches("INFO: Starting\nERROR: Failed to connect\nINFO: Retrying");
        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_regex_matches_multiple_occurrences() -> Result<()> {
        let matcher = contains_regex("(?s)(?:.*token){3}");
        let result = matcher.matches("token at index 0, token at index 18, token at index 37");
        verify_that!(result, eq(MatcherResult::Match))
    }
}
