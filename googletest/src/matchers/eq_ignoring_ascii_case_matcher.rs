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

/// Matches a string equal to the given string, ignoring upper/lower case for
/// ASCII characters.
///
/// This is is identical to the behaviour of [`str::eq_ignore_ascii_case`].
///
/// ```rust
/// verify_that!("Some value", eq_ignoring_ascii_case("SOME VALUE"))?;  // Passes
/// verify_that!("Another value", eq_ignoring_ascii_case("Some value"))?;   // Fails
/// ```
///
/// This does **not** match strings with differing upper/lower case characters
/// outside of the codepoints 0-127 covered by ASCII. For example, the following
/// will *fail*:
///
/// ```rust
/// verify_that!("Söme välüe", eq_ignoring_ascii_case("SÖME VÄLÜE"))?;
/// ```
///
/// Both the actual value and the expected string may be either a `String` or
/// a string reference.
///
/// ```rust
/// verify_that!("Some value".to_string(), eq_ignoring_ascii_case("some value"))?;   // Passes
/// verify_that!("Some value", eq_ignoring_ascii_case("some value".to_string()))?;   // Passes
/// ```
// N.B. This returns the concrete type rather than an impl Matcher so that it
// can act simultaneously as a Matcher<str> and a Matcher<String>. Otherwise the
// compiler treats it as a Matcher<str> only and the code
//   verify_that!("Some value".to_string(), eq_ignoring_ascii_case("some
// value"))?; doesn't compile.
pub fn eq_ignoring_ascii_case<ExpectedT: Deref<Target = str>>(
    expected: ExpectedT,
) -> EqIgnoringCaseMatcher<ExpectedT> {
    EqIgnoringCaseMatcher { expected }
}

/// A matcher matching a string-like type equal to the given string, ignoring
/// case.
///
/// Intended only to be used from the function [`eq_ignoring_ascii_case`] only.
/// Should not be referenced by code outside this library.
pub struct EqIgnoringCaseMatcher<ExpectedT: Deref<Target = str>> {
    expected: ExpectedT,
}

impl<ExpectedT, ActualT> Matcher<ActualT> for EqIgnoringCaseMatcher<ExpectedT>
where
    ExpectedT: Deref<Target = str>,
    ActualT: AsRef<str> + Debug + ?Sized,
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        if actual.as_ref().eq_ignore_ascii_case(self.expected.deref()) {
            MatcherResult::Matches
        } else {
            MatcherResult::DoesNotMatch
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!("is equal to {:#?} (ignoring case)", self.expected.deref())
            }
            MatcherResult::DoesNotMatch => {
                format!("isn't equal to {:#?} (ignoring case)", self.expected.deref())
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
    fn eq_ignoring_ascii_case_matches_string_reference_with_equal_string() -> Result<()> {
        let matcher = eq_ignoring_ascii_case("Some value");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn eq_ignoring_ascii_case_matches_string_reference_with_capitalised_string() -> Result<()> {
        let matcher = eq_ignoring_ascii_case("Some value");

        let result = matcher.matches("SOME VALUE");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn eq_ignoring_ascii_case_matches_string_reference_with_lower_case_string() -> Result<()> {
        let matcher = eq_ignoring_ascii_case("Some value");

        let result = matcher.matches("some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn eq_ignoring_ascii_case_does_not_match_string_with_different_string() -> Result<()> {
        let matcher = eq_ignoring_ascii_case("Another value");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn eq_ignoring_ascii_case_matches_owned_string_with_expected() -> Result<()> {
        let matcher = eq_ignoring_ascii_case("some value");

        let result = matcher.matches(&"Some value".to_string());

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn eq_ignoring_ascii_case_matches_string_reference_with_owned_string() -> Result<()> {
        let matcher = eq_ignoring_ascii_case("some value".to_string());

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn verify_that_works_with_owned_string() -> Result<()> {
        verify_that!("Some value".to_string(), eq_ignoring_ascii_case("Some value"))
    }

    #[google_test]
    fn eq_ignoring_ascii_case_displays_quoted_debug_of_expected_string() -> Result<()> {
        let matcher = eq_ignoring_ascii_case("\n");

        verify_that!(
            <EqIgnoringCaseMatcher<&str> as Matcher<&str>>::describe(
                &matcher,
                MatcherResult::Matches
            ),
            eq("is equal to \"\\n\" (ignoring case)")
        )
    }
}
