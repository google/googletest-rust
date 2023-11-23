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
use crate::matcher::{Matcher, MatcherResult};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

/// Matches a (smart) pointer pointing to a value matched by the [`Matcher`]
/// `expected`.
///
/// This allows easily matching smart pointers such as `Box`, `Rc`, and `Arc`.
/// For example:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(Box::new(123), points_to(eq(123)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn points_to<ExpectedT, MatcherT, ActualT>(
    expected: MatcherT,
) -> impl Matcher<ActualT = ActualT>
where
    ExpectedT: Debug,
    MatcherT: Matcher<ActualT = ExpectedT>,
    ActualT: Deref<Target = ExpectedT> + Debug + ?Sized,
{
    PointsToMatcher { expected, phantom: Default::default() }
}

struct PointsToMatcher<ActualT: ?Sized, MatcherT> {
    expected: MatcherT,
    phantom: PhantomData<ActualT>,
}

impl<ExpectedT, MatcherT, ActualT> Matcher for PointsToMatcher<ActualT, MatcherT>
where
    ExpectedT: Debug,
    MatcherT: Matcher<ActualT = ExpectedT>,
    ActualT: Deref<Target = ExpectedT> + Debug + ?Sized,
{
    type ActualT = ActualT;

    fn matches(&self, actual: &ActualT) -> MatcherResult {
        self.expected.matches(actual.deref())
    }

    fn explain_match(&self, actual: &ActualT) -> Description {
        self.expected.explain_match(actual.deref())
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.expected.describe(matcher_result)
    }
}

#[cfg(test)]
mod tests {
    use super::points_to;
    use crate::prelude::*;
    use indoc::indoc;
    use std::rc::Rc;

    #[test]
    fn points_to_matches_box_of_int_with_int() -> Result<()> {
        verify_that!(Box::new(123), points_to(eq(123)))
    }

    #[test]
    fn points_to_matches_rc_of_int_with_int() -> Result<()> {
        verify_that!(Rc::new(123), points_to(eq(123)))
    }

    #[test]
    fn points_to_matches_box_of_owned_string_with_string_reference() -> Result<()> {
        verify_that!(Rc::new("A string".to_string()), points_to(eq("A string")))
    }

    #[test]
    fn match_explanation_references_actual_value() -> Result<()> {
        let result = verify_that!(&vec![1], points_to(container_eq([])));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Actual: [1],
                      which contains the unexpected element 1
                "
            ))))
        )
    }
}
