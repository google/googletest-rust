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

use crate::description::Description;
use crate::matcher::{Matcher, MatcherBase, MatcherResult};
use std::fmt::Debug;

/// Matches a reference pointing to a value matched by the [`Matcher`]
/// `expected`.
///
/// This is useful for combining matchers, especially when working with
/// iterators.
///
/// For example:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(&123, points_to(eq(123)))?;
/// verify_that!(vec![1,2,3], each(points_to(gt(0))))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn points_to<MatcherT>(expected: MatcherT) -> PointsToMatcher<MatcherT> {
    PointsToMatcher { expected }
}

#[derive(MatcherBase)]
pub struct PointsToMatcher<MatcherT> {
    expected: MatcherT,
}

impl<'a, ExpectedT, MatcherT> Matcher<&'a ExpectedT> for PointsToMatcher<MatcherT>
where
    ExpectedT: Debug + Copy,
    MatcherT: Matcher<ExpectedT>,
{
    fn matches(&self, actual: &'a ExpectedT) -> MatcherResult {
        self.expected.matches(*actual)
    }

    fn explain_match(&self, actual: &'a ExpectedT) -> Description {
        self.expected.explain_match(*actual)
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.expected.describe(matcher_result)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn points_to_matches_ref() -> Result<()> {
        verify_that!(&123, points_to(eq(123)))
    }

    #[test]
    fn match_explanation_references_actual_value() -> Result<()> {
        let result = verify_that!(&1, points_to(eq(0)));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Actual: 1,
                      which isn't equal to 0
                "
            ))))
        )
    }
}
