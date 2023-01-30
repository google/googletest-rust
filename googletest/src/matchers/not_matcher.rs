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

/// Matches the actual value exactly when the inner matcher does _not_ match.
///
/// ```rust
/// verify_that!(0, not(eq(1)))?; // Passes
/// verify_that!(0, not(eq(0)))?; // Fails
/// ```
pub fn not<T: Debug, InnerMatcherT: Matcher<T>>(inner: InnerMatcherT) -> impl Matcher<T> {
    NotMatcher { inner }
}

struct NotMatcher<InnerMatcherT> {
    inner: InnerMatcherT,
}

impl<T: Debug, InnerMatcherT: Matcher<T>> Matcher<T> for NotMatcher<InnerMatcherT> {
    fn matches(&self, actual: &T) -> MatcherResult {
        match self.inner.matches(actual) {
            MatcherResult::Matches => MatcherResult::DoesNotMatch,
            MatcherResult::DoesNotMatch => MatcherResult::Matches,
        }
    }

    fn explain_match(&self, actual: &T) -> MatchExplanation {
        self.inner.explain_match(actual)
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        self.inner
            .describe(matcher_result.pick(MatcherResult::DoesNotMatch, MatcherResult::Matches))
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
    use matchers::{container_eq, contains_substring, displays_as, eq, err};

    #[google_test]
    fn matches_when_inner_matcher_does_not_match() -> Result<()> {
        let matcher = not(eq(1));

        let result = matcher.matches(&0);

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn does_not_match_when_inner_matcher_matches() -> Result<()> {
        let matcher = not(eq(1));

        let result = matcher.matches(&1);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn match_explanation_references_actual_value() -> Result<()> {
        let result = verify_that!(*&[1], not(container_eq([1])));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "\
Actual: [
    1,
], which contains all the elements
"
            )))
        )
    }
}
