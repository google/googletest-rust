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

/// Matches the actual value exactly when the inner matcher does _not_ match.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(0, not(eq(1)))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(0, not(eq(0)))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
pub fn not<T: Debug, InnerMatcherT: Matcher<ActualT = T>>(
    inner: InnerMatcherT,
) -> impl Matcher<ActualT = T> {
    NotMatcher::<T, _> { inner, phantom: Default::default() }
}

struct NotMatcher<T, InnerMatcherT> {
    inner: InnerMatcherT,
    phantom: PhantomData<T>,
}

impl<T: Debug, InnerMatcherT: Matcher<ActualT = T>> Matcher for NotMatcher<T, InnerMatcherT> {
    type ActualT = T;

    fn matches(&self, actual: &T) -> MatcherResult {
        match self.inner.matches(actual) {
            MatcherResult::Match => MatcherResult::NoMatch,
            MatcherResult::NoMatch => MatcherResult::Match,
        }
    }

    fn explain_match(&self, actual: &T) -> Description {
        self.inner.explain_match(actual)
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.inner.describe(if matcher_result.into() {
            MatcherResult::NoMatch
        } else {
            MatcherResult::Match
        })
    }
}

#[cfg(test)]
mod tests {
    use super::not;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn matches_when_inner_matcher_does_not_match() -> Result<()> {
        let matcher = not(eq(1));

        let result = matcher.matches(&0);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn does_not_match_when_inner_matcher_matches() -> Result<()> {
        let matcher = not(eq(1));

        let result = matcher.matches(&1);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn match_explanation_references_actual_value() -> Result<()> {
        let result = verify_that!([1], not(container_eq([1])));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Actual: [1],
                  which contains all the elements
                "
            ))))
        )
    }
}
