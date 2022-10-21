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
use std::marker::PhantomData;

/// Matches an `Option` containing a value matched by `inner`.
///
/// ```rust
/// verify_that!(Some("Some value"), some(eq("Some value")))?;  // Passes
/// verify_that!(None, some(eq("Some value")))?;   // Fails
/// verify_that!(Some("Some value"), some(eq("Some other value")))?;   // Fails
/// ```
pub fn some<T: Debug>(inner: impl Matcher<T>) -> impl Matcher<Option<T>> {
    SomeMatcher { inner, phantom: Default::default() }
}

struct SomeMatcher<T, InnerMatcherT> {
    inner: InnerMatcherT,
    phantom: PhantomData<T>,
}

impl<T: Debug, InnerMatcherT: Matcher<T>> Matcher<Option<T>> for SomeMatcher<T, InnerMatcherT> {
    fn matches(&self, actual: &Option<T>) -> MatcherResult {
        actual.as_ref().map(|v| self.inner.matches(v)).unwrap_or(MatcherResult::DoesNotMatch)
    }
    // TODO(b/261174693) Describe if the value is none or some(x) where x does not
    // match
}

impl<T, InnerMatcherT: Describe> Describe for SomeMatcher<T, InnerMatcherT> {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!("is some(x), where x {}", self.inner.describe(MatcherResult::Matches))
            }
            MatcherResult::DoesNotMatch => {
                format!(
                    "is none or some(x), where x {}",
                    self.inner.describe(MatcherResult::DoesNotMatch)
                )
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
    fn some_matches_option_with_value() -> Result<()> {
        let matcher = some(eq(1));

        let result = matcher.matches(&Some(1));

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn some_does_not_match_option_with_wrong_value() -> Result<()> {
        let matcher = some(eq(1));

        let result = matcher.matches(&Some(0));

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn some_does_not_match_option_with_none() -> Result<()> {
        let matcher = some(eq(1));

        let result = matcher.matches(&None);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }
}
