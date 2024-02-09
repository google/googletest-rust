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

/// Matches an iterable type whose elements contain a value matched by `inner`.
///
/// By default, this matches a container with any number of elements matched
/// by `inner`. Use the method [`ContainsMatcher::times`] to constrain the
/// matched containers to a specific number of matching elements.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(["Some value"], contains(eq("Some value")))?;  // Passes
/// verify_that!(vec!["Some value"], contains(eq("Some value")))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!([] as [String; 0], contains(eq("Some value")))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!(["Some value"], contains(eq("Some other value")))?;   // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// ```
pub fn contains<T, InnerMatcherT>(
    inner: InnerMatcherT,
) -> ContainsMatcher<T, InnerMatcherT, NoCountMatcher> {
    ContainsMatcher { inner, count: NoCountMatcher, phantom: Default::default() }
}

/// A matcher which matches a container containing one or more elements a given
/// inner [`Matcher`] matches.
pub struct ContainsMatcher<T, InnerMatcherT, CountMatcher> {
    inner: InnerMatcherT,
    count: CountMatcher,
    phantom: PhantomData<T>,
}

// Sentinel type to tag a `ContainsMatcher` as without a `CountMatcher`.
#[doc(hidden)]

pub struct NoCountMatcher;

impl<T, InnerMatcherT> ContainsMatcher<T, InnerMatcherT, NoCountMatcher> {
    /// Configures this instance to match containers which contain a number of
    /// matching items matched by `count`.
    ///
    /// For example, to assert that exactly three matching items must be
    /// present, use:
    ///
    /// ```ignore
    /// contains(...).times(eq(3))
    /// ```
    ///
    /// One can also use `times(eq(0))` to test for the *absence* of an item
    /// matching the expected value.
    pub fn times<CountMatcher: Matcher<ActualT = usize>>(
        self,
        count: CountMatcher,
    ) -> ContainsMatcher<T, InnerMatcherT, CountMatcher> {
        ContainsMatcher { inner: self.inner, count, phantom: self.phantom }
    }
}

impl<T: Debug, InnerMatcherT: Matcher<ActualT = T>, ContainerT: Debug> Matcher
    for ContainsMatcher<ContainerT, InnerMatcherT, NoCountMatcher>
where
    for<'a> &'a ContainerT: IntoIterator<Item = &'a T>,
{
    type ActualT = ContainerT;

    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
        for v in actual.into_iter() {
            if self.inner.matches(v).into() {
                return MatcherResult::Match;
            }
        }
        MatcherResult::NoMatch
    }

    // TODO use the inner matcher to produce a better error message.
    fn explain_match(&self, actual: &Self::ActualT) -> Description {
        match self.count_matches(actual) {
            0 => "which does not contain a matching element".into(),
            _ => "which contains a matching element".into(),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!(
                "contains at least one element which {}",
                self.inner.describe(MatcherResult::Match)
            )
            .into(),
            MatcherResult::NoMatch => {
                format!("contains no element which {}", self.inner.describe(MatcherResult::Match))
                    .into()
            }
        }
    }
}

impl<
    T: Debug,
    InnerMatcherT: Matcher<ActualT = T>,
    ContainerT: Debug,
    CountMatcher: Matcher<ActualT = usize>,
> Matcher for ContainsMatcher<ContainerT, InnerMatcherT, CountMatcher>
where
    for<'a> &'a ContainerT: IntoIterator<Item = &'a T>,
{
    type ActualT = ContainerT;

    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
        self.count.matches(&self.count_matches(actual))
    }

    fn explain_match(&self, actual: &Self::ActualT) -> Description {
        format!("which contains {} matching elements", self.count_matches(actual)).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!(
                "contains n elements which {}\n  where n {}",
                self.inner.describe(MatcherResult::Match),
                self.count.describe(MatcherResult::Match)
            )
            .into(),
            MatcherResult::NoMatch => format!(
                "doesn't contain n elements which {}\n  where n {}",
                self.inner.describe(MatcherResult::Match),
                self.count.describe(MatcherResult::Match)
            )
            .into(),
        }
    }
}

impl<ActualT, InnerMatcherT, CountMatcher> ContainsMatcher<ActualT, InnerMatcherT, CountMatcher> {
    fn count_matches<T: Debug, ContainerT>(&self, actual: &ContainerT) -> usize
    where
        for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
        InnerMatcherT: Matcher<ActualT = T>,
    {
        let mut count = 0;
        for v in actual.into_iter() {
            if self.inner.matches(v).into() {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::{contains, ContainsMatcher};
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;

    #[test]
    fn contains_matches_singleton_slice_with_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&vec![1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_matches_singleton_vec_with_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&vec![1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_matches_two_element_slice_with_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&[0, 1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_does_not_match_singleton_slice_with_wrong_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&[0]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_does_not_match_empty_slice() -> Result<()> {
        let matcher = contains(eq::<i32, _>(1));

        let result = matcher.matches(&[]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_matches_slice_with_repeated_value() -> Result<()> {
        let matcher = contains(eq(1)).times(eq(2));

        let result = matcher.matches(&[1, 1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_does_not_match_slice_with_too_few_of_value() -> Result<()> {
        let matcher = contains(eq(1)).times(eq(2));

        let result = matcher.matches(&[0, 1]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_does_not_match_slice_with_too_many_of_value() -> Result<()> {
        let matcher = contains(eq(1)).times(eq(1));

        let result = matcher.matches(&[1, 1]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_formats_without_multiplicity_by_default() -> Result<()> {
        let matcher: ContainsMatcher<Vec<i32>, _, _> = contains(eq(1));

        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("contains at least one element which is equal to 1"))
        )
    }

    #[test]
    fn contains_formats_with_multiplicity_when_specified() -> Result<()> {
        let matcher: ContainsMatcher<Vec<i32>, _, _> = contains(eq(1)).times(eq(2));

        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            displays_as(eq("contains n elements which is equal to 1\n  where n is equal to 2"))
        )
    }

    #[test]
    fn contains_mismatch_shows_number_of_times_element_was_found() -> Result<()> {
        verify_that!(
            contains(eq(3)).times(eq(1)).explain_match(&vec![1, 2, 3, 3]),
            displays_as(eq("which contains 2 matching elements"))
        )
    }

    #[test]
    fn contains_mismatch_shows_when_matches() -> Result<()> {
        verify_that!(
            contains(eq(3)).explain_match(&vec![1, 2, 3, 3]),
            displays_as(eq("which contains a matching element"))
        )
    }

    #[test]
    fn contains_mismatch_shows_when_no_matches() -> Result<()> {
        verify_that!(
            contains(eq(3)).explain_match(&vec![1, 2]),
            displays_as(eq("which does not contain a matching element"))
        )
    }

    #[test]
    fn fix_todo() -> Result<()> {
        let matcher = contains(eq(&42));
        let val = 42;
        let result = matcher.matches(&vec![&val]);
        verify_that!(result, eq(MatcherResult::Match))
    }
}
