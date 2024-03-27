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

use crate::{
    description::Description,
    matcher::{Matcher, MatcherExt, MatcherResult},
};
use std::fmt::Debug;

/// Matches a `Result` containing `Err` with a value matched by `inner`.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> googletest::Result<()> {
/// verify_that!(Err::<(), _>("Some error"), err(eq("Some error")))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> googletest::Result<()> {
/// verify_that!(Ok::<_, &str>("A value"), err(eq("A value")))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> googletest::Result<()> {
/// verify_that!(Err::<(), _>("Some error"), err(eq("Some other error")))?;   // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// ```
pub fn err<Inner>(inner: Inner) -> ErrMatcher<Inner> {
    ErrMatcher { inner }
}

#[derive(MatcherExt)]
pub struct ErrMatcher<InnerMatcherT> {
    inner: InnerMatcherT,
}

impl<'a, T: Debug, E: Debug, InnerMatcherT: Matcher<'a, E>> Matcher<'a, std::result::Result<T, E>>
    for ErrMatcher<InnerMatcherT>
{
    fn matches<'b>(&self, actual: &'b std::result::Result<T, E>) -> MatcherResult where 'a: 'b{
        actual.as_ref().err().map(|v| self.inner.matches(v)).unwrap_or(MatcherResult::NoMatch)
    }

    fn explain_match<'b>(&self, actual: &'b std::result::Result<T, E>) -> Description where 'a: 'b{
        match actual {
            Err(e) => {
                Description::new().text("which is an error").nested(self.inner.explain_match(e))
            }
            Ok(_) => "which is a success".into(),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("is an error which {}", self.inner.describe(MatcherResult::Match)).into()
            }
            MatcherResult::NoMatch => format!(
                "is a success or is an error containing a value which {}",
                self.inner.describe(MatcherResult::NoMatch)
            )
            .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::err;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn err_matches_result_with_err_value() -> Result<()> {
        let matcher = err(eq(1));
        let value: std::result::Result<i32, i32> = Err(1);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn err_does_not_match_result_with_wrong_err_value() -> Result<()> {
        let matcher = err(eq(1));
        let value: std::result::Result<i32, i32> = Err(0);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn err_does_not_match_result_with_ok() -> Result<()> {
        let matcher = err(eq(1));
        let value: std::result::Result<i32, i32> = Ok(1);

        let result = matcher.matches(&value);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn err_full_error_message() -> Result<()> {
        let result = verify_that!(Err::<i32, i32>(1), err(eq(2)));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: Err::<i32, i32>(1)
                    Expected: is an error which is equal to 2
                    Actual: Err(1),
                      which is an error
                        which isn't equal to 2
                "
            ))))
        )
    }

    #[test]
    fn err_describe_matches() -> Result<()> {
        let matcher = err(eq(1));
        verify_that!(
            Matcher::<std::result::Result<i32, i32>>::describe(&matcher, MatcherResult::Match),
            displays_as(eq("is an error which is equal to 1"))
        )
    }
}
