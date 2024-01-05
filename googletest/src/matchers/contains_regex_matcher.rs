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
use crate::matcher::{Matcher, MatcherResult};
use regex::Regex;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

/// Matches a string containing a substring which matches the given regular
/// expression.
///
/// Both the actual value and the expected regular expression may be either a
/// `String` or a string reference.
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
/// Panics if the given `pattern` is not a syntactically valid regular
/// expression.
// N.B. This returns the concrete type rather than an impl Matcher so that it
// can act simultaneously as a Matcher<str> and a Matcher<String>. Otherwise the
// compiler treats it as a Matcher<str> only and the code
//   verify_that!("Some value".to_string(), contains_regex(".*value"))?;
// doesn't compile.
pub fn contains_regex<ActualT: ?Sized, PatternT: Deref<Target = str>>(
    pattern: PatternT,
) -> ContainsRegexMatcher<ActualT> {
    ContainsRegexMatcher {
        regex: Regex::new(pattern.deref()).unwrap(),
        phantom: Default::default(),
    }
}

/// A matcher matching a string-like type containing a substring matching a
/// given regular expression.
///
/// Intended only to be used from the function [`contains_regex`] only.
/// Should not be referenced by code outside this library.
pub struct ContainsRegexMatcher<ActualT: ?Sized> {
    regex: Regex,
    phantom: PhantomData<ActualT>,
}

impl<ActualT: AsRef<str> + Debug + ?Sized> Matcher for ContainsRegexMatcher<ActualT> {
    type ActualT = ActualT;

    fn matches(&self, actual: &ActualT) -> MatcherResult {
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
    use super::{contains_regex, ContainsRegexMatcher};
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;

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
        let matcher: ContainsRegexMatcher<&str> = contains_regex("\n");

        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("contains the regular expression \"\\n\""))
        )
    }
}
