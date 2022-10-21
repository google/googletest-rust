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
use googletest::matcher::{Describe, Matcher, MatcherResult};
use std::fmt::Debug;
use std::ops::Deref;

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
// N.B. This returns the concrete type rather than an impl Matcher so that it
// can act simultaneously as a Matcher<str> and a Matcher<String>. Otherwise the
// compiler treats it as a Matcher<str> only and the code
//   verify_that!("Some value".to_string(), ends_with("value"))?;
// doesn't compile.
pub fn ends_with<SuffixT: Deref<Target = str>>(suffix: SuffixT) -> EndsWithMatcher<SuffixT> {
    EndsWithMatcher { suffix }
}

/// A matcher matching a string-like type starting with a given suffix.
///
/// Intended only to be used from the function [`ends_with`] only.
/// Should not be referenced by code outside this library.
pub struct EndsWithMatcher<SuffixT: Deref<Target = str>> {
    suffix: SuffixT,
}

impl<SuffixT, ActualT> Matcher<ActualT> for EndsWithMatcher<SuffixT>
where
    SuffixT: Deref<Target = str>,
    ActualT: AsRef<str> + Debug + ?Sized,
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        if actual.as_ref().ends_with(self.suffix.deref()) {
            MatcherResult::Matches
        } else {
            MatcherResult::DoesNotMatch
        }
    }
}

impl<SuffixT: Deref<Target = str>> Describe for EndsWithMatcher<SuffixT> {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => format!("ends with suffix {:#?}", self.suffix.deref()),
            MatcherResult::DoesNotMatch => {
                format!("doesn't end with suffix {:#?}", self.suffix.deref())
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
    use googletest::{google_test, verify_that, Result};
    use matchers::eq;

    #[google_test]
    fn ends_with_matches_string_reference_with_suffix() -> Result<()> {
        let matcher = ends_with("value");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn ends_with_does_not_match_string_without_suffix() -> Result<()> {
        let matcher = ends_with("other value");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn ends_with_does_not_match_string_with_substring_not_at_end() -> Result<()> {
        let matcher = ends_with("Some");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn ends_with_matches_owned_string_with_suffix() -> Result<()> {
        let matcher = ends_with("value");

        let result = matcher.matches(&"Some value".to_string());

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn ends_with_matches_string_reference_with_owned_string_suffix() -> Result<()> {
        let matcher = ends_with("value".to_string());

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn verify_that_works_with_owned_string() -> Result<()> {
        verify_that!("Some value".to_string(), ends_with("value"))
    }

    #[google_test]
    fn ends_with_displays_quoted_debug_of_substring() -> Result<()> {
        let matcher = ends_with("\n");

        verify_that!(matcher.describe(MatcherResult::Matches), eq("ends with suffix \"\\n\""))
    }
}
