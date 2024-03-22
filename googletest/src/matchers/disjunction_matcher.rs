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
    matcher::{Matcher, MatcherExt, MatcherResult},
};
use std::fmt::Debug;

/// Matcher created by [`Matcher::or`] and [`any!`].
///
/// Both [`Matcher::or`] and [`any!`] nest on m1. In other words,
/// both `x.or(y).or(z)` and `any![x, y, z]` produce:
/// ```ignore
/// DisjunctionMatcher {
///     m1: DisjunctionMatcher {
///         m1: x, m2: y
///     },
///     m2: z
/// }
/// ```
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
#[derive(MatcherExt)]
pub struct DisjunctionMatcher<M1, M2> {
    m1: M1,
    m2: M2,
}

impl<M1, M2> DisjunctionMatcher<M1, M2> {
    pub fn new(m1: M1, m2: M2) -> Self {
        Self { m1, m2 }
    }
}

impl<T: Debug + Copy, M1: Matcher<T>, M2: Matcher<T>> Matcher<T> for DisjunctionMatcher<M1, M2> {
    fn matches(&self, actual: T) -> MatcherResult {
        match (self.m1.matches(actual), self.m2.matches(actual)) {
            (MatcherResult::NoMatch, MatcherResult::NoMatch) => MatcherResult::NoMatch,
            _ => MatcherResult::Match,
        }
    }

    fn explain_match(&self, actual: T) -> Description {
        match (self.m1.matches(actual), self.m2.matches(actual)) {
            (MatcherResult::NoMatch, MatcherResult::Match) => self.m1.explain_match(actual),
            (MatcherResult::Match, MatcherResult::NoMatch) => self.m2.explain_match(actual),
            (_, _) => {
                let m1_description = self.m1.explain_match(actual);
                if m1_description.is_disjunction_description() {
                    m1_description.nested(self.m2.explain_match(actual))
                } else {
                    Description::new()
                        .bullet_list()
                        .collect([m1_description, self.m2.explain_match(actual)])
                        .disjunction_description()
                }
            }
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        let m1_description = self.m1.describe(matcher_result);
        if m1_description.is_disjunction_description() {
            m1_description.push_in_last_nested(self.m2.describe(matcher_result))
        } else {
            let header = if matcher_result.into() {
                "has at least one of the following properties:"
            } else {
                "has all of the following properties:"
            };
            Description::new()
                .text(header)
                .nested(
                    Description::new()
                        .bullet_list()
                        .collect([m1_description, self.m2.describe(matcher_result)]),
                )
                .disjunction_description()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use indoc::indoc;

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
            err(displays_as(contains_substring(indoc!(
                "
                Value of: 1
                Expected: has at least one of the following properties:
                  * never matches
                  * never matches
                Actual: 1,
                  * which is anything
                  * which is anything
                "
            ))))
        )
    }

    #[test]
    fn chained_or_matches() -> Result<()> {
        verify_that!(10, eq(1).or(eq(5)).or(ge(9)))
    }

    #[test]
    fn works_with_str_slices() -> Result<()> {
        verify_that!("A string", ends_with("A").or(ends_with("string")))
    }

    #[test]
    fn works_with_owned_strings() -> Result<()> {
        verify_that!("A string".to_string(), ends_with("A").or(ends_with("string")))
    }
}
