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
use googletest::matcher::{MatchExplanation, Matcher, MatcherResult};
use std::fmt::Debug;
use std::ops::Deref;

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
// N.B. This returns the concrete type rather than an impl Matcher so that it
// can act simultaneously as a Matcher<str> and a Matcher<String>. Otherwise the
// compiler treats it as a Matcher<str> only and the code
//   verify_that!("Some value".to_string(), starts_with("Some"))?;
// doesn't compile.
pub fn starts_with<PrefixT: Deref<Target = str>>(prefix: PrefixT) -> StartsWithMatcher<PrefixT> {
    StartsWithMatcher { prefix }
}

/// A matcher matching a string-like type starting with a given prefix.
///
/// Intended only to be used from the function [`starts_with`] only.
/// Should not be referenced by code outside this library.
pub struct StartsWithMatcher<PrefixT: Deref<Target = str>> {
    prefix: PrefixT,
}

impl<PrefixT, ActualT> Matcher<ActualT> for StartsWithMatcher<PrefixT>
where
    PrefixT: Deref<Target = str>,
    ActualT: AsRef<str> + Debug + ?Sized,
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        if actual.as_ref().starts_with(self.prefix.deref()) {
            MatcherResult::Matches
        } else {
            MatcherResult::DoesNotMatch
        }
    }
    fn explain_match(&self, actual: &ActualT) -> MatchExplanation {
        match self.matches(actual) {
            MatcherResult::Matches => {
                MatchExplanation::create(format!("which starts with {:?}", self.prefix.deref()))
            }
            MatcherResult::DoesNotMatch => MatchExplanation::create(format!(
                "which does not start with {:?}",
                self.prefix.deref()
            )),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => format!("starts with prefix {:#?}", self.prefix.deref()),
            MatcherResult::DoesNotMatch => {
                format!("doesn't start with prefix {:#?}", self.prefix.deref())
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
    fn starts_with_matches_string_reference_with_prefix() -> Result<()> {
        let matcher = starts_with("Some");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn starts_with_does_not_match_string_without_prefix() -> Result<()> {
        let matcher = starts_with("Another");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn starts_with_does_not_match_string_with_substring_not_at_beginning() -> Result<()> {
        let matcher = starts_with("value");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn starts_with_matches_owned_string_with_prefix() -> Result<()> {
        let matcher = starts_with("Some");

        let result = matcher.matches(&"Some value".to_string());

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn starts_with_matches_string_reference_with_owned_string_prefix() -> Result<()> {
        let matcher = starts_with("Some".to_string());

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn verify_that_works_with_owned_string() -> Result<()> {
        verify_that!("Some value".to_string(), starts_with("Some"))
    }

    #[google_test]
    fn starts_with_displays_quoted_debug_of_substring() -> Result<()> {
        let matcher = starts_with("\n");

        verify_that!(
            <StartsWithMatcher<&str> as Matcher<&str>>::describe(&matcher, MatcherResult::Matches),
            eq("starts with prefix \"\\n\"")
        )
    }
}
