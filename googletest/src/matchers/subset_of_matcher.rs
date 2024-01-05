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

/// Matches a container all of whose items are in the given container
/// `superset`.
///
/// The element type `ElementT` must implement `PartialEq` to allow element
/// comparison.
///
/// `ActualT` and `ExpectedT` can each be any container a reference to which
/// implements `IntoIterator`. For instance, `T` can be a common container like
/// `Vec` or arrays. They need not be the same container type.
///
/// ```
/// # use googletest::prelude::*;
/// # use std::collections::HashSet;
/// # fn should_pass_1() -> Result<()> {
/// let value = vec![1, 2, 3];
/// verify_that!(value, subset_of([1, 2, 3, 4]))?;  // Passes
/// let array_value = [1, 2, 3];
/// verify_that!(array_value, subset_of([1, 2, 3, 4]))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// # let value = vec![1, 2, 3];
/// verify_that!(value, subset_of([1, 2]))?;  // Fails: 3 is not in the superset
/// #     Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_fail().unwrap_err();
///
/// # fn should_pass_2() -> Result<()> {
/// let value: HashSet<i32> = [1, 2, 3].into();
/// verify_that!(value, subset_of([1, 2, 3]))?;  // Passes
/// #     Ok(())
/// # }
/// # should_pass_2().unwrap();
/// ```
///
/// Item multiplicity in both the actual and expected containers is ignored:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let value: Vec<i32> = vec![0, 0, 1];
/// verify_that!(value, subset_of([0, 1]))?;  // Passes
/// verify_that!(value, subset_of([0, 1, 1]))?;  // Passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// One can also verify the contents of a slice by dereferencing it:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let value = &[1, 2, 3];
/// verify_that!(*value, subset_of([1, 2, 3]))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// A note on performance: This matcher uses a naive algorithm with a worst-case
/// runtime proportional to the *product* of the sizes of the actual and
/// expected containers as well as the time to check equality of each pair of
/// items. It should not be used on especially large containers.
pub fn subset_of<ElementT: Debug + PartialEq, ActualT: Debug + ?Sized, ExpectedT: Debug>(
    superset: ExpectedT,
) -> impl Matcher<ActualT = ActualT>
where
    for<'a> &'a ActualT: IntoIterator<Item = &'a ElementT>,
    for<'a> &'a ExpectedT: IntoIterator<Item = &'a ElementT>,
{
    SubsetOfMatcher::<ActualT, _> { superset, phantom: Default::default() }
}

struct SubsetOfMatcher<ActualT: ?Sized, ExpectedT> {
    superset: ExpectedT,
    phantom: PhantomData<ActualT>,
}

impl<ElementT: Debug + PartialEq, ActualT: Debug + ?Sized, ExpectedT: Debug> Matcher
    for SubsetOfMatcher<ActualT, ExpectedT>
where
    for<'a> &'a ActualT: IntoIterator<Item = &'a ElementT>,
    for<'a> &'a ExpectedT: IntoIterator<Item = &'a ElementT>,
{
    type ActualT = ActualT;

    fn matches(&self, actual: &ActualT) -> MatcherResult {
        for actual_item in actual {
            if self.expected_is_missing(actual_item) {
                return MatcherResult::NoMatch;
            }
        }
        MatcherResult::Match
    }

    fn explain_match(&self, actual: &ActualT) -> Description {
        let unexpected_elements = actual
            .into_iter()
            .enumerate()
            .filter(|&(_, actual_item)| self.expected_is_missing(actual_item))
            .map(|(idx, actual_item)| format!("{actual_item:#?} at #{idx}"))
            .collect::<Vec<_>>();

        match unexpected_elements.len() {
            0 => "which no element is unexpected".into(),
            1 => format!("whose element {} is unexpected", &unexpected_elements[0]).into(),
            _ => format!("whose elements {} are unexpected", unexpected_elements.join(", ")).into(),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("is a subset of {:#?}", self.superset).into(),
            MatcherResult::NoMatch => format!("isn't a subset of {:#?}", self.superset).into(),
        }
    }
}

impl<ActualT: ?Sized, ElementT: PartialEq, ExpectedT> SubsetOfMatcher<ActualT, ExpectedT>
where
    for<'a> &'a ExpectedT: IntoIterator<Item = &'a ElementT>,
{
    fn expected_is_missing(&self, needle: &ElementT) -> bool {
        !self.superset.into_iter().any(|item| *item == *needle)
    }
}

#[cfg(test)]
mod tests {
    use super::subset_of;
    use crate::prelude::*;
    use indoc::indoc;
    use std::collections::HashSet;

    #[test]
    fn subset_of_matches_empty_vec() -> Result<()> {
        let value: Vec<i32> = vec![];
        verify_that!(value, subset_of([]))
    }

    #[test]
    fn subset_of_matches_vec_with_one_element() -> Result<()> {
        let value = vec![1];
        verify_that!(value, subset_of([1]))
    }

    #[test]
    fn subset_of_matches_vec_with_two_elements() -> Result<()> {
        let value = vec![1, 2];
        verify_that!(value, subset_of([1, 2]))
    }

    #[test]
    fn subset_of_matches_vec_when_expected_has_excess_element() -> Result<()> {
        let value = vec![1, 2];
        verify_that!(value, subset_of([1, 2, 3]))
    }

    #[test]
    fn subset_of_matches_vec_when_expected_has_excess_element_first() -> Result<()> {
        let value = vec![1, 2];
        verify_that!(value, subset_of([3, 1, 2]))
    }

    #[test]
    fn subset_of_matches_slice_with_one_element() -> Result<()> {
        let value = &[1];
        verify_that!(*value, subset_of([1]))
    }

    #[test]
    fn subset_of_matches_hash_set_with_one_element() -> Result<()> {
        let value: HashSet<i32> = [1].into();
        verify_that!(value, subset_of([1]))
    }

    #[test]
    fn subset_of_does_not_match_when_first_element_does_not_match() -> Result<()> {
        let value = vec![0];
        verify_that!(value, not(subset_of([1])))
    }

    #[test]
    fn subset_of_does_not_match_when_second_element_does_not_match() -> Result<()> {
        let value = vec![2, 0];
        verify_that!(value, not(subset_of([2])))
    }

    #[test]
    fn subset_of_shows_correct_message_when_first_item_does_not_match() -> Result<()> {
        let result = verify_that!(vec![0, 2, 3], subset_of([1, 2, 3]));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: vec![0, 2, 3]
                    Expected: is a subset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [0, 2, 3],
                      whose element 0 at #0 is unexpected
                "
            ))))
        )
    }

    #[test]
    fn subset_of_shows_correct_message_when_second_item_does_not_match() -> Result<()> {
        let result = verify_that!(vec![1, 0, 3], subset_of([1, 2, 3]));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: vec![1, 0, 3]
                    Expected: is a subset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [1, 0, 3],
                      whose element 0 at #1 is unexpected
                "
            ))))
        )
    }

    #[test]
    fn subset_of_shows_correct_message_when_first_two_items_do_not_match() -> Result<()> {
        let result = verify_that!(vec![0, 0, 3], subset_of([1, 2, 3]));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: vec![0, 0, 3]
                    Expected: is a subset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [0, 0, 3],
                      whose elements 0 at #0, 0 at #1 are unexpected
                "
            ))))
        )
    }
}
