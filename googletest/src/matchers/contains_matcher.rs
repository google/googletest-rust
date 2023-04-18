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

/// Matches an iterable type whose elements contain a value matched by `inner`.
///
/// By default, this matches a container with any number of elements matched
/// by `inner`. Use the method [`ContainsMatcher::times`] to constrain the
/// matched containers to a specific number of matching elements.
///
/// ```
/// # use googletest::{matchers::{contains, eq}, verify_that, Result};
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
pub fn contains<T, InnerMatcherT>(inner: InnerMatcherT) -> ContainsMatcher<T, InnerMatcherT> {
    ContainsMatcher { inner, count: None, phantom: Default::default() }
}

pub struct ContainsMatcher<T, InnerMatcherT> {
    inner: InnerMatcherT,
    count: Option<Box<dyn Matcher<ActualT<'static> = usize>>>,
    phantom: PhantomData<T>,
}

impl<T, InnerMatcherT> ContainsMatcher<T, InnerMatcherT> {
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
    pub fn times(mut self, count: impl Matcher<ActualT<'static> = usize> + 'static) -> Self {
        self.count = Some(Box::new(count));
        self
    }
}

// TODO(hovinen): Revisit the trait bounds to see whether this can be made more
//  flexible. Namely, the following doesn't compile currently:
//
//      let matcher = contains(eq(&42));
//      let val = 42;
//      let _ = matcher.matches(&vec![&val]);
//
//  because val is dropped before matcher but the trait bound requires that
//  the argument to matches outlive the matcher. It works fine if one defines
//  val before matcher.
impl<T: Debug, InnerMatcherT: Matcher, ContainerT: Debug> Matcher
    for ContainsMatcher<ContainerT, InnerMatcherT>
where
    for<'a> &'a ContainerT: IntoIterator<Item = &'a T>,
{
    fn matches(&self, actual: &ContainerT) -> MatcherResult {
        if let Some(count) = &self.count {
            count.matches(&self.count_matches(actual))
        } else {
            for v in actual.into_iter() {
                if self.inner.matches(v).into() {
                    return MatcherResult::Matches;
                }
            }
            MatcherResult::DoesNotMatch
        }
    }

    fn explain_match(&self, actual: &ContainerT) -> MatchExplanation {
        let count = self.count_matches(actual);
        match (count, &self.count) {
            (_, Some(_)) => {
                MatchExplanation::create(format!("which contains {} matching elements", count))
            }
            (0, None) => {
                MatchExplanation::create("which does not contain a matching element".to_string())
            }
            (_, None) => MatchExplanation::create("which contains a matching element".to_string()),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match (matcher_result, &self.count) {
            (MatcherResult::Matches, Some(count)) => format!(
                "contains n elements which {}\n  where n {}",
                self.inner.describe(MatcherResult::Matches),
                count.describe(MatcherResult::Matches)
            ),
            (MatcherResult::DoesNotMatch, Some(count)) => format!(
                "doesn't contain n elements which {}\n  where n {}",
                self.inner.describe(MatcherResult::Matches),
                count.describe(MatcherResult::Matches)
            ),
            (MatcherResult::Matches, None) => format!(
                "contains at least one element which {}",
                self.inner.describe(MatcherResult::Matches)
            ),
            (MatcherResult::DoesNotMatch, None) => {
                format!("contains no element which {}", self.inner.describe(MatcherResult::Matches))
            }
        }
    }
}

impl<ActualT, InnerMatcherT> ContainsMatcher<ActualT, InnerMatcherT> {
    fn count_matches<T: Debug, ContainerT>(&self, actual: &ContainerT) -> usize
    where
        for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
        InnerMatcherT: Matcher,
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
    #[cfg(not(google3))]
    use crate::matchers;
    use crate::{
        matcher::{Matcher, MatcherResult},
        verify_that, Result,
    };
    use matchers::{displays_as, eq};

    #[test]
    fn contains_matches_singleton_slice_with_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&vec![1]);

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[test]
    fn contains_matches_singleton_vec_with_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&vec![1]);

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[test]
    fn contains_matches_two_element_slice_with_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&[0, 1]);

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[test]
    fn contains_does_not_match_singleton_slice_with_wrong_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&[0]);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[test]
    fn contains_does_not_match_empty_slice() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&[]);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[test]
    fn contains_matches_slice_with_repeated_value() -> Result<()> {
        let matcher = contains(eq(1)).times(eq(2));

        let result = matcher.matches(&[1, 1]);

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[test]
    fn contains_does_not_match_slice_with_too_few_of_value() -> Result<()> {
        let matcher = contains(eq(1)).times(eq(2));

        let result = matcher.matches(&[0, 1]);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[test]
    fn contains_does_not_match_slice_with_too_many_of_value() -> Result<()> {
        let matcher = contains(eq(1)).times(eq(1));

        let result = matcher.matches(&[1, 1]);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[test]
    fn contains_formats_without_multiplicity_by_default() -> Result<()> {
        let matcher = contains(eq(1));

        verify_that!(
            <ContainsMatcher<_> as Matcher<Vec<i32>>>::describe(&matcher, MatcherResult::Matches),
            eq("contains at least one element which is equal to 1")
        )
    }

    #[test]
    fn contains_formats_with_multiplicity_when_specified() -> Result<()> {
        let matcher = contains(eq(1)).times(eq(2));

        verify_that!(
            <ContainsMatcher<_> as Matcher<Vec<i32>>>::describe(&matcher, MatcherResult::Matches),
            eq("contains n elements which is equal to 1\n  where n is equal to 2")
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
}
