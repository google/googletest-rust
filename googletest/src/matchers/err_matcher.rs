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

/// Matches a `Result` containing `Err` with a value matched by `inner`.
///
/// ```
/// verify_that!(Err("Some error"), err(eq("Some error")))?;  // Passes
/// verify_that!(Ok("A value"), err(eq("A value")))?;   // Fails
/// verify_that!(Err("Some error"), err(eq("Some error value")))?;   // Fails
/// ```
pub fn err<T: Debug, E: Debug>(inner: impl Matcher<E>) -> impl Matcher<Result<T, E>> {
    ErrMatcher { inner }
}

struct ErrMatcher<InnerMatcherT> {
    inner: InnerMatcherT,
}

impl<T: Debug, E: Debug, InnerMatcherT: Matcher<E>> Matcher<Result<T, E>>
    for ErrMatcher<InnerMatcherT>
{
    fn matches(&self, actual: &Result<T, E>) -> MatcherResult {
        actual.as_ref().err().map(|v| self.inner.matches(v)).unwrap_or(MatcherResult::DoesNotMatch)
    }

    fn explain_match(&self, actual: &Result<T, E>) -> MatchExplanation {
        match actual {
            Err(e) => MatchExplanation::create(format!(
                "which is an error {}",
                self.inner.explain_match(e)
            )),
            Ok(_) => MatchExplanation::create("which is a success".to_string()),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!("is an error which {}", self.inner.describe(MatcherResult::Matches))
            }
            MatcherResult::DoesNotMatch => {
                format!(
                    "is a success or is an error containing a value which {}",
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
    use matchers::{contains_substring, displays_as, eq};

    #[google_test]
    fn err_matches_result_with_err_value() -> Result<()> {
        let matcher = err(eq(1));
        let value: std::result::Result<i32, i32> = Err(1);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn err_does_not_match_result_with_wrong_err_value() -> Result<()> {
        let matcher = err(eq(1));
        let value: std::result::Result<i32, i32> = Err(0);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn err_does_not_match_result_with_ok() -> Result<()> {
        let matcher = err(eq(1));
        let value: std::result::Result<i32, i32> = Ok(1);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn err_full_error_message() -> Result<()> {
        let result = verify_that!(Err::<i32, i32>(1), err(eq(2)));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "\
Value of: Err::<i32, i32>(1)
Expected: is an error which is equal to 2
Actual: Err(
    1,
), which is an error which isn't equal to 2
"
            )))
        )
    }

    #[google_test]
    fn err_describe_matches() -> Result<()> {
        verify_that!(
            err::<i32, i32>(eq(1)).describe(MatcherResult::Matches),
            eq("is an error which is equal to 1")
        )
    }
}
