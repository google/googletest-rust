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
    matcher::{Matcher, MatcherResult},
};
use std::{fmt::Debug, marker::PhantomData};

/// Matches an `Option` containing a value matched by `inner`.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(Some("Some value"), some(eq("Some value")))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!(None::<&str>, some(eq("Some value")))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!(Some("Some value"), some(eq("Some other value")))?;   // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// ```
pub fn some<T: Debug>(inner: impl Matcher<ActualT = T>) -> impl Matcher<ActualT = Option<T>> {
    SomeMatcher { inner, phantom: Default::default() }
}

struct SomeMatcher<T, InnerMatcherT> {
    inner: InnerMatcherT,
    phantom: PhantomData<T>,
}

impl<T: Debug, InnerMatcherT: Matcher<ActualT = T>> Matcher for SomeMatcher<T, InnerMatcherT> {
    type ActualT = Option<T>;

    fn matches(&self, actual: &Option<T>) -> MatcherResult {
        actual.as_ref().map(|v| self.inner.matches(v)).unwrap_or(MatcherResult::NoMatch)
    }

    fn explain_match(&self, actual: &Option<T>) -> Description {
        match (self.matches(actual), actual) {
            (_, Some(t)) => {
                Description::new().text("which has a value").nested(self.inner.explain_match(t))
            }
            (_, None) => "which is None".into(),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("has a value which {}", self.inner.describe(MatcherResult::Match)).into()
            }
            MatcherResult::NoMatch => format!(
                "is None or has a value which {}",
                self.inner.describe(MatcherResult::NoMatch)
            )
            .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::some;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn some_matches_option_with_value() -> Result<()> {
        let matcher = some(eq(1));

        let result = matcher.matches(&Some(1));

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn some_does_not_match_option_with_wrong_value() -> Result<()> {
        let matcher = some(eq(1));

        let result = matcher.matches(&Some(0));

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn some_does_not_match_option_with_none() -> Result<()> {
        let matcher = some(eq::<i32, _>(1));

        let result = matcher.matches(&None);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn some_full_error_message() -> Result<()> {
        let result = verify_that!(Some(2), some(eq(1)));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: Some(2)
                    Expected: has a value which is equal to 1
                    Actual: Some(2),
                      which has a value
                        which isn't equal to 1
                "
            ))))
        )
    }

    #[test]
    fn some_describe_matches() -> Result<()> {
        verify_that!(
            some(eq::<i32, _>(1)).describe(MatcherResult::Match),
            displays_as(eq("has a value which is equal to 1"))
        )
    }

    #[test]
    fn some_describe_does_not_match() -> Result<()> {
        verify_that!(
            some(eq::<i32, _>(1)).describe(MatcherResult::NoMatch),
            displays_as(eq("is None or has a value which isn't equal to 1"))
        )
    }

    #[test]
    fn some_explain_match_with_none() -> Result<()> {
        verify_that!(some(eq::<i32, _>(1)).explain_match(&None), displays_as(eq("which is None")))
    }

    #[test]
    fn some_explain_match_with_some_success() -> Result<()> {
        verify_that!(
            some(eq(1)).explain_match(&Some(1)),
            displays_as(eq("which has a value\n  which is equal to 1"))
        )
    }

    #[test]
    fn some_explain_match_with_some_fail() -> Result<()> {
        verify_that!(
            some(eq(1)).explain_match(&Some(2)),
            displays_as(eq("which has a value\n  which isn't equal to 1"))
        )
    }
}
