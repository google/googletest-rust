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

use crate::matcher::{MatchExplanation, Matcher, MatcherResult};
#[cfg(google3)]
use googletest::*;
use std::fmt::Debug;

/// Matches an `Option` containing a value matched by `inner`.
///
/// ```
/// verify_that!(Some("Some value"), some(eq("Some value")))?;  // Passes
/// verify_that!(None, some(eq("Some value")))?;   // Fails
/// verify_that!(Some("Some value"), some(eq("Some other value")))?;   // Fails
/// ```
pub fn some<T: Debug>(inner: impl Matcher<T>) -> impl Matcher<Option<T>> {
    SomeMatcher { inner }
}

struct SomeMatcher<InnerMatcherT> {
    inner: InnerMatcherT,
}

impl<T: Debug, InnerMatcherT: Matcher<T>> Matcher<Option<T>> for SomeMatcher<InnerMatcherT> {
    fn matches(&self, actual: &Option<T>) -> MatcherResult {
        actual.as_ref().map(|v| self.inner.matches(v)).unwrap_or(MatcherResult::DoesNotMatch)
    }

    fn explain_match(&self, actual: &Option<T>) -> MatchExplanation {
        match (self.matches(actual), actual) {
            (_, Some(t)) => MatchExplanation::create(format!(
                "which has a value {}",
                self.inner.explain_match(t)
            )),
            (_, None) => MatchExplanation::create("which is None".to_string()),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!("has a value which {}", self.inner.describe(MatcherResult::Matches))
            }
            MatcherResult::DoesNotMatch => {
                format!(
                    "is None or has a value which {}",
                    self.inner.describe(MatcherResult::DoesNotMatch)
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::some;
    #[cfg(not(google3))]
    use crate::matchers;
    use crate::{
        matcher::{Matcher, MatcherResult},
        verify_that, Result,
    };
    use matchers::{contains_substring, displays_as, eq, err};

    #[test]
    fn some_matches_option_with_value() -> Result<()> {
        let matcher = some(eq(1));

        let result = matcher.matches(&Some(1));

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[test]
    fn some_does_not_match_option_with_wrong_value() -> Result<()> {
        let matcher = some(eq(1));

        let result = matcher.matches(&Some(0));

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[test]
    fn some_does_not_match_option_with_none() -> Result<()> {
        let matcher = some(eq(1));

        let result = matcher.matches(&None);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[test]
    fn some_full_error_message() -> Result<()> {
        let result = verify_that!(Some(2), some(eq(1)));
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "\
Value of: Some(2)
Expected: has a value which is equal to 1
Actual: Some(
    2,
), which has a value which isn't equal to 1
"
            )))
        )
    }

    #[test]
    fn some_describe_matches() -> Result<()> {
        verify_that!(
            some(eq(1)).describe(MatcherResult::Matches),
            eq("has a value which is equal to 1")
        )
    }

    #[test]
    fn some_describe_does_not_match() -> Result<()> {
        verify_that!(
            some(eq(1)).describe(MatcherResult::DoesNotMatch),
            eq("is None or has a value which isn't equal to 1")
        )
    }

    #[test]
    fn some_explain_match_with_none() -> Result<()> {
        verify_that!(some(eq(1)).explain_match(&None), displays_as(eq("which is None")))
    }

    #[test]
    fn some_explain_match_with_some_success() -> Result<()> {
        verify_that!(
            some(eq(1)).explain_match(&Some(1)),
            displays_as(eq("which has a value which is equal to 1"))
        )
    }

    #[test]
    fn some_explain_match_with_some_fail() -> Result<()> {
        verify_that!(
            some(eq(1)).explain_match(&Some(2)),
            displays_as(eq("which has a value which isn't equal to 1"))
        )
    }
}
