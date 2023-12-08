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
use std::fmt::Debug;

/// Extension trait providing the [`and`][AndMatcherExt::and] method.
pub trait AndMatcherExt<T: Debug>: Matcher<T> {
    /// Constructs a matcher that matches both `self` and `right`.
    ///
    /// ```
    /// # use googletest::{field, matchers::{eq, AndMatcherExt}, verify_that, Result};
    /// # #[derive(Debug)]
    /// # struct Struct { a: u32, b: u32 };
    /// # fn should_pass() -> Result<()> {
    /// verify_that!(Struct { a: 1, b: 2 }, field!(Struct.a, eq(1)).and(field!(Struct.b, eq(2))))?;
    /// #     Ok(())
    /// # }
    /// # should_pass().unwrap();
    /// ```
    // TODO(b/264518763): Replace the return type with impl Matcher and reduce
    // visibility of ConjunctionMatcher once impl in return position in trait
    // methods is stable.
    fn and<Right: Matcher<T>>(self, right: Right) -> ConjunctionMatcher<Self, Right>
    where
        Self: Sized,
    {
        ConjunctionMatcher { m1: self, m2: right }
    }
}

impl<T: Debug, M> AndMatcherExt<T> for M where M: Matcher<T> {}

/// Matcher created by [`AndMatcherExt::and`].
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub struct ConjunctionMatcher<M1, M2> {
    m1: M1,
    m2: M2,
}

impl<T: Debug, M1: Matcher<T>, M2: Matcher<T>> Matcher<T> for ConjunctionMatcher<M1, M2> {
    fn matches(&self, actual: &T) -> MatcherResult {
        match (self.m1.matches(actual), self.m2.matches(actual)) {
            (MatcherResult::Matches, MatcherResult::Matches) => MatcherResult::Matches,
            _ => MatcherResult::DoesNotMatch,
        }
    }

    fn explain_match(&self, actual: &T) -> MatchExplanation {
        match (self.m1.matches(actual), self.m2.matches(actual)) {
            (MatcherResult::Matches, MatcherResult::Matches) => MatchExplanation::create(format!(
                "{} and\n{}",
                self.m1.explain_match(actual),
                self.m2.explain_match(actual)
            )),
            (MatcherResult::DoesNotMatch, MatcherResult::Matches) => self.m1.explain_match(actual),
            (MatcherResult::Matches, MatcherResult::DoesNotMatch) => self.m2.explain_match(actual),
            (MatcherResult::DoesNotMatch, MatcherResult::DoesNotMatch) => MatchExplanation::create(
                format!("{} and\n{}", self.m1.explain_match(actual), self.m2.explain_match(actual)),
            ),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        format!("{}, and {}", self.m1.describe(matcher_result), self.m2.describe(matcher_result))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(google3))]
    use crate::{field, matchers};
    use crate::{verify_that, Result, matcher::Matcher, matchers::AndMatcherExt};
    #[cfg(google3)]
    use matchers::field;
    use matchers::{
        anything, contains_substring, displays_as, ends_with, eq, err, not, starts_with,
    };

    #[test]
    fn and_true_true_matches() -> Result<()> {
        verify_that!(1, anything().and(anything()))
    }

    #[test]
    fn and_true_false_does_not_match() -> Result<()> {
        let result = verify_that!(1, anything().and(not(anything())));
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: 1\n\
                Expected: is anything, and never matches\n\
                Actual: 1, which is anything"
            )))
        )
    }

    #[test]
    fn and_false_true_does_not_match() -> Result<()> {
        let result = verify_that!(1, not(anything()).and(anything()));
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: 1\n\
                Expected: never matches, and is anything\n\
                Actual: 1, which is anything"
            )))
        )
    }

    #[test]
    fn and_false_false_does_not_match() -> Result<()> {
        let result = verify_that!(1, not(anything()).and(not(anything())));
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: 1\n\
                Expected: never matches, and never matches\n\
                Actual: 1, which is anything and\n\
                which is anything"
            )))
        )
    }

    #[test]
    fn chained_and_matches() -> Result<()> {
        #[derive(Debug)]
        struct Struct {
            a: i32,
            b: i32,
            c: i32,
        }
        verify_that!(
            Struct { a: 1, b: 2, c: 3 },
            field!(Struct.a, eq(1)).and(field!(Struct.b, eq(2))).and(field!(Struct.c, eq(3)))
        )
    }

    fn chained_and_with_string_slice() -> Result<()> {
        let actual = "what goes up must come down";

        verify_that!(actual, starts_with("what goes up").and(ends_with("must come down")))
    }

    fn chained_and_with_owned_string() -> Result<()> {
        let actual = "what goes up must come down".to_string();

        verify_that!(actual, starts_with("what goes up").and(ends_with("must come down")))
    }
}
