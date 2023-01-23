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
use googletest::matcher::{Matcher, MatcherResult};
use std::fmt::Debug;
use std::ops::Deref;

/// Matches a (smart) pointer pointing to a value matched by the [`Matcher`]
/// `expected`.
///
/// This allows easily matching smart pointers such as `Box`, `Rc`, and `Arc`.
/// For example:
///
/// ```rust
/// verify_that!(Box::new(123), points_to(eq(123)))?;
/// ```
pub fn points_to<ExpectedT, MatcherT, ActualT>(expected: MatcherT) -> impl Matcher<ActualT>
where
    ExpectedT: Debug,
    MatcherT: Matcher<ExpectedT>,
    ActualT: Deref<Target = ExpectedT> + Debug + ?Sized,
{
    PointsToMatcher { expected }
}

struct PointsToMatcher<MatcherT> {
    expected: MatcherT,
}

impl<ExpectedT, MatcherT, ActualT> Matcher<ActualT> for PointsToMatcher<MatcherT>
where
    ExpectedT: Debug,
    MatcherT: Matcher<ExpectedT>,
    ActualT: Deref<Target = ExpectedT> + Debug + ?Sized,
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        self.expected.matches(actual.deref())
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        self.expected.describe(matcher_result)
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
    use std::rc::Rc;

    #[google_test]
    fn points_to_matches_box_of_int_with_int() -> Result<()> {
        verify_that!(Box::new(123), points_to(eq(123)))
    }

    #[google_test]
    fn points_to_matches_rc_of_int_with_int() -> Result<()> {
        verify_that!(Rc::new(123), points_to(eq(123)))
    }

    #[google_test]
    fn points_to_matches_box_of_owned_string_with_string_reference() -> Result<()> {
        verify_that!(Rc::new("A string".to_string()), points_to(eq("A string")))
    }
}
