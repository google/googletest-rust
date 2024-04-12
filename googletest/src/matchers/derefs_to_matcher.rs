// Copyright 2024 Google LLC
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
    matcher::{Matcher, MatcherBase, MatcherResult},
};
use std::{fmt::Debug, ops::Deref};

/// Dereferences the `actual` value and verifies that the returned reference
/// matches the `inner` matcher.
///
/// ```
/// # use googletest::{matchers::{derefs_to, eq}, verify_that};
/// verify_that!(Box::new(123), derefs_to(eq(&123)))
/// #    .unwrap()
/// ```
pub fn derefs_to<Inner>(inner: Inner) -> DerefsTo<Inner> {
    DerefsTo { inner }
}

/// A matcher which derefs a value and verifies that the result matches the
/// `inner` matcher.
///
/// See [`deref_to`].
#[derive(MatcherBase)]
pub struct DerefsTo<InnerT> {
    pub(crate) inner: InnerT,
}

impl<'a, ActualT, ExpectedT, Inner> Matcher<&'a ActualT> for DerefsTo<Inner>
where
    ActualT: Deref<Target = ExpectedT> + Debug,
    ExpectedT: Copy + Debug + 'a,
    Inner: Matcher<&'a ExpectedT>,
{
    fn matches(&self, actual: &'a ActualT) -> MatcherResult {
        self.inner.matches(actual.deref())
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.inner.describe(matcher_result)
    }

    fn explain_match(&self, actual: &'a ActualT) -> Description {
        self.inner.explain_match(actual.deref())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn deref_to_matches_box_of_int_with_int() -> Result<()> {
        let actual = Box::new(123);
        verify_that!(actual, derefs_to(eq(&123)))
    }

    #[test]
    fn deref_to_matches_rc_of_int_with_int() -> Result<()> {
        verify_that!(Rc::new(123), derefs_to(eq(&123)))
    }

    #[test]
    fn deref_to_combines_with_points_to_for_copy() -> Result<()> {
        verify_that!(Rc::new(123), derefs_to(points_to(eq(123))))
    }

    #[test]
    fn match_explanation_references_actual_value() -> Result<()> {
        let actual = Box::new(1);
        let result = verify_that!(actual, derefs_to(eq(&0)));

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
