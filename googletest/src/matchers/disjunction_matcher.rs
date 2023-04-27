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

use crate::matcher::{MatchExplanation, Matcher, MatcherResult};
#[cfg(google3)]
use googletest::*;
use std::{fmt::Debug, marker::PhantomData};

/// Extension trait providing the [`or`][OrMatcherExt::or] method.
pub trait OrMatcherExt<T: Debug>: Matcher {
    /// Constructs a matcher that matches when at least one of `self` or `right`
    /// matches the input.
    ///
    /// ```
    /// # use googletest::{matchers::{eq, ge, OrMatcherExt}, verify_that, Result};
    /// # fn should_pass() -> Result<()> {
    /// verify_that!(10, eq(2).or(ge(5)))?;  // Passes
    /// verify_that!(10, eq(2).or(eq(5)).or(ge(9)))?;  // Passes
    /// #     Ok(())
    /// # }
    /// # fn should_fail() -> Result<()> {
    /// verify_that!(10, eq(2).or(ge(15)))?; // Fails
    /// #     Ok(())
    /// # }
    /// # should_pass().unwrap();
    /// # should_fail().unwrap_err();
    /// ```
    // TODO(b/264518763): Replace the return type with impl Matcher and reduce
    // visibility of DisjunctionMatcher once impl in return position in trait
    // methods is stable.
    fn or<Right: Matcher>(self, right: Right) -> DisjunctionMatcher<T, Self, Right>
    where
        Self: Sized,
    {
        DisjunctionMatcher { m1: self, m2: right }
    }
}

impl<T: Debug, M> OrMatcherExt<T> for M where M: Matcher {}

/// Matcher created by [`OrMatcherExt::or`].
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub struct DisjunctionMatcher<T, M1, M2> {
    m1: M1,
    m2: M2,
    phantom: PhantomData<T>,
}

impl<T: Debug, M1: Matcher, M2: Matcher> Matcher for DisjunctionMatcher<T, M1, M2> {
    fn matches(&self, actual: &T) -> MatcherResult {
        match (self.m1.matches(actual), self.m2.matches(actual)) {
            (MatcherResult::DoesNotMatch, MatcherResult::DoesNotMatch) => {
                MatcherResult::DoesNotMatch
            }
            _ => MatcherResult::Matches,
        }
    }

    fn explain_match(&self, actual: &T) -> MatchExplanation {
        MatchExplanation::create(format!(
            "{} and\n{}",
            self.m1.explain_match(actual),
            self.m2.explain_match(actual)
        ))
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        format!("{}, or {}", self.m1.describe(matcher_result), self.m2.describe(matcher_result))
    }
}

#[cfg(test)]
mod tests {
    use super::OrMatcherExt;
    #[cfg(not(google3))]
    use crate::matchers;
    use crate::{verify_that, Result};
    use matchers::{anything, contains_substring, displays_as, eq, err, ge, not};

    #[test]
    fn or_true_true_matches() -> Result<()> {
        verify_that!(1, anything().or(anything()))
    }

    #[test]
    fn or_true_false_matches() -> Result<()> {
        verify_that!(1, anything().or(not(anything())))
    }

    #[test]
    fn or_false_true_matches() -> Result<()> {
        verify_that!(1, not(anything()).or(anything()))
    }

    #[test]
    fn or_false_false_does_not_match() -> Result<()> {
        let result = verify_that!(1, not(anything()).or(not(anything())));
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: 1\n\
                Expected: never matches, or never matches\n\
                Actual: 1, which is anything and\n\
                which is anything"
            )))
        )
    }

    #[test]
    fn chained_or_matches() -> Result<()> {
        verify_that!(10, eq(1).or(eq(5)).or(ge(9)))
    }
}
