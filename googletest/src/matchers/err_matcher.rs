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

/// Matches a `Result` containing `Err` with a value matched by `inner`.
///
/// ```rust
/// verify_that!(Err("Some error"), err(eq("Some error")))?;  // Passes
/// verify_that!(Ok("A value"), err(eq("A value")))?;   // Fails
/// verify_that!(Err("Some error"), err(eq("Some error value")))?;   // Fails
/// ```
pub fn err<T: Debug, E: Debug>(inner: impl Matcher<E>) -> impl Matcher<Result<T, E>> {
    ErrMatcher { inner, phantom_t: Default::default(), phantom_e: Default::default() }
}

struct ErrMatcher<T, E, InnerMatcherT> {
    inner: InnerMatcherT,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}

impl<T: Debug, E: Debug, InnerMatcherT: Matcher<E>> Matcher<Result<T, E>>
    for ErrMatcher<T, E, InnerMatcherT>
{
    fn matches(&self, actual: &Result<T, E>) -> MatcherResult {
        actual.as_ref().err().map(|v| self.inner.matches(v)).unwrap_or(MatcherResult::DoesNotMatch)
    }

    // TODO(b/261174693) implement explain_match to differentiate when the
    // match is non matching err or ok.
}

impl<T: Debug, E: Debug, InnerMatcherT: Describe> Describe for ErrMatcher<T, E, InnerMatcherT> {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!("is an error, which {}", self.inner.describe(MatcherResult::Matches))
            }
            MatcherResult::DoesNotMatch => {
                format!(
                    "isn't an error or is an error, which {}",
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
    fn ok_matches_result_with_err_value() -> Result<()> {
        let matcher = err(eq(1));
        let value: std::result::Result<i32, i32> = Err(1);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn ok_does_not_match_result_with_wrong_err_value() -> Result<()> {
        let matcher = err(eq(1));
        let value: std::result::Result<i32, i32> = Err(0);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn ok_does_not_match_result_with_ok() -> Result<()> {
        let matcher = err(eq(1));
        let value: std::result::Result<i32, i32> = Ok(1);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }
}
