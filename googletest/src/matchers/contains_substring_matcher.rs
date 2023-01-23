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
// N.B. This returns the concrete type rather than an impl Matcher so that it
// can act simultaneously as a Matcher<str> and a Matcher<String>. Otherwise the
// compiler treats it as a Matcher<str> only and the code
//   verify_that!("Some value".to_string(), contains_substring("value"))?;
// doesn't compile.
pub fn contains_substring<SubstringT: Deref<Target = str>>(
    substring: SubstringT,
) -> ContainsSubstringMatcher<SubstringT> {
    ContainsSubstringMatcher { substring }
}

/// A matcher matching a string-like type containing a given substring.
///
/// Intended only to be used from the function [`contains_substring`] only.
/// Should not be referenced by code outside this library.
pub struct ContainsSubstringMatcher<SubstringT: Deref<Target = str>> {
    substring: SubstringT,
}

impl<SubstringT, ActualT> Matcher<ActualT> for ContainsSubstringMatcher<SubstringT>
where
    SubstringT: Deref<Target = str>,
    ActualT: AsRef<str> + Debug + ?Sized,
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        if actual.as_ref().contains(self.substring.deref()) {
            MatcherResult::Matches
        } else {
            MatcherResult::DoesNotMatch
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => format!("contains substring {:#?}", self.substring.deref()),
            MatcherResult::DoesNotMatch => {
                format!("does not contain substring {:#?}", self.substring.deref())
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
    fn contains_substring_matches_string_reference_with_substring() -> Result<()> {
        let matcher = contains_substring("value");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn contains_substring_does_not_match_string_without_substring() -> Result<()> {
        let matcher = contains_substring("Another");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn contains_substring_matches_owned_string_with_substring() -> Result<()> {
        let matcher = contains_substring("value");

        let result = matcher.matches(&"Some value".to_string());

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn contains_substring_matches_string_reference_with_owned_string() -> Result<()> {
        let matcher = contains_substring("value".to_string());

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn verify_that_works_with_owned_string() -> Result<()> {
        verify_that!("Some value".to_string(), contains_substring("value"))
    }

    #[google_test]
    fn contains_substring_displays_quoted_debug_of_substring() -> Result<()> {
        let matcher = contains_substring("\n");

        verify_that!(
            Matcher::<&str>::describe(&matcher, MatcherResult::Matches),
            eq("contains substring \"\\n\"")
        )
    }
}
