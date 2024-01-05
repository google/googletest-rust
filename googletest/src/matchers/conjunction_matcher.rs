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

// There are no visible documentation elements in this module.
#![doc(hidden)]

use crate::{
    description::Description,
    matcher::{Matcher, MatcherResult},
};
use std::fmt::Debug;

/// Matcher created by [`Matcher::and`].
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub struct ConjunctionMatcher<M1, M2> {
    m1: M1,
    m2: M2,
}

impl<M1, M2> ConjunctionMatcher<M1, M2> {
    pub(crate) fn new(m1: M1, m2: M2) -> Self {
        Self { m1, m2 }
    }
}

impl<M1: Matcher, M2: Matcher<ActualT = M1::ActualT>> Matcher for ConjunctionMatcher<M1, M2>
where
    M1::ActualT: Debug,
{
    type ActualT = M1::ActualT;

    fn matches(&self, actual: &M1::ActualT) -> MatcherResult {
        match (self.m1.matches(actual), self.m2.matches(actual)) {
            (MatcherResult::Match, MatcherResult::Match) => MatcherResult::Match,
            _ => MatcherResult::NoMatch,
        }
    }

    fn explain_match(&self, actual: &M1::ActualT) -> Description {
        match (self.m1.matches(actual), self.m2.matches(actual)) {
            (MatcherResult::Match, MatcherResult::Match) => Description::new()
                .nested(self.m1.explain_match(actual))
                .text("and")
                .nested(self.m2.explain_match(actual)),
            (MatcherResult::NoMatch, MatcherResult::Match) => self.m1.explain_match(actual),
            (MatcherResult::Match, MatcherResult::NoMatch) => self.m2.explain_match(actual),
            (MatcherResult::NoMatch, MatcherResult::NoMatch) => Description::new()
                .nested(self.m1.explain_match(actual))
                .text("and")
                .nested(self.m2.explain_match(actual)),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        format!("{}, and {}", self.m1.describe(matcher_result), self.m2.describe(matcher_result))
            .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn and_true_true_matches() -> Result<()> {
        verify_that!(1, anything().and(anything()))
    }

    #[test]
    fn and_true_false_does_not_match() -> Result<()> {
        let result = verify_that!(1, anything().and(not(anything())));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: 1
                Expected: is anything, and never matches
                Actual: 1,
                  which is anything
                "
            ))))
        )
    }

    #[test]
    fn and_false_true_does_not_match() -> Result<()> {
        let result = verify_that!(1, not(anything()).and(anything()));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: 1
                    Expected: never matches, and is anything
                    Actual: 1,
                      which is anything
                "
            ))))
        )
    }

    #[test]
    fn and_false_false_does_not_match() -> Result<()> {
        let result = verify_that!(1, not(anything()).and(not(anything())));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: 1
                Expected: never matches, and never matches
                Actual: 1,
                    which is anything
                  and
                    which is anything
                "
            ))))
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

    #[test]
    fn works_with_str_slices() -> Result<()> {
        verify_that!("A string", starts_with("A").and(ends_with("string")))
    }

    #[test]
    fn works_with_owned_strings() -> Result<()> {
        verify_that!("A string".to_string(), starts_with("A").and(ends_with("string")))
    }
}
