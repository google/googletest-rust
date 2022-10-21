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

/// Matches a `Result` containing `Ok` with a value matched by `inner`.
///
/// ```rust
/// verify_that!(Ok("Some value"), ok(eq("Some value")))?;  // Passes
/// verify_that!(Err("An error"), ok(eq("An error")))?;   // Fails
/// verify_that!(Ok("Some value"), ok(eq("Some other value")))?;   // Fails
/// ```
pub fn ok<T: Debug, E: Debug>(inner: impl Matcher<T>) -> impl Matcher<Result<T, E>> {
    OkMatcher { inner, phantom_t: Default::default(), phantom_e: Default::default() }
}

struct OkMatcher<T, E, InnerMatcherT> {
    inner: InnerMatcherT,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}

impl<T: Debug, E: Debug, InnerMatcherT: Matcher<T>> Matcher<Result<T, E>>
    for OkMatcher<T, E, InnerMatcherT>
{
    fn matches(&self, actual: &Result<T, E>) -> MatcherResult {
        actual.as_ref().map(|v| self.inner.matches(v)).unwrap_or(MatcherResult::DoesNotMatch)
    }
}

impl<T, E, InnerMatcherT: Describe> Describe for OkMatcher<T, E, InnerMatcherT> {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!("is ok(x) where x {}", self.inner.describe(MatcherResult::Matches))
            }
            MatcherResult::DoesNotMatch => {
                format!(
                    "is err(_) or is ok(x) where x {}",
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
    fn ok_matches_result_with_value() -> Result<()> {
        let matcher = ok(eq(1));
        let value: std::result::Result<i32, i32> = Ok(1);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn ok_does_not_match_result_with_wrong_value() -> Result<()> {
        let matcher = ok(eq(1));
        let value: std::result::Result<i32, i32> = Ok(0);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn ok_does_not_match_result_with_err() -> Result<()> {
        let matcher = ok(eq(1));
        let value: std::result::Result<i32, i32> = Err(1);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }
}
