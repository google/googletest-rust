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
use googletest::matcher::{Describe, MatchExplanation, Matcher, MatcherResult};
use std::fmt::Debug;

/// Matches a container all of whose elements are matched by the matcher
/// `inner`.
///
/// `T` can be any container such that `&T` implements `IntoIterator`.
///
/// ```rust
/// let value: Vec<i32> = vec![1, 2, 3];
/// verify_that!(value, each(gt(0)))?;  // Passes
/// verify_that!(value, each(lt(2)))?;  // Fails: 2 and 3 are not less than 2
///
/// let value: HashSet<i32> = [1, 2, 3].into();
/// verify_that!(value, each(gt(0)))?;  // Passes
/// ```
///
/// One can also verify the contents of a slice by dereferencing it:
///
/// ```rust
/// let value = &[1, 2, 3];
/// verify_that!(*value, each(gt(0)))?;
/// ```
pub fn each<ElementT: Debug, ActualT: Debug + ?Sized, MatcherT>(
    inner: MatcherT,
) -> impl Matcher<ActualT>
where
    for<'a> &'a ActualT: IntoIterator<Item = &'a ElementT>,
    MatcherT: Matcher<ElementT>,
{
    EachMatcher { inner }
}

struct EachMatcher<MatcherT> {
    inner: MatcherT,
}

impl<ElementT: Debug, ActualT: Debug + ?Sized, MatcherT> Matcher<ActualT> for EachMatcher<MatcherT>
where
    for<'a> &'a ActualT: IntoIterator<Item = &'a ElementT>,
    MatcherT: Matcher<ElementT>,
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        for element in actual {
            if matches!(self.inner.matches(element), MatcherResult::DoesNotMatch) {
                return MatcherResult::DoesNotMatch;
            }
        }
        MatcherResult::Matches
    }

    fn explain_match(&self, actual: &ActualT) -> MatchExplanation {
        let mut non_matching_elements = Vec::new();
        for (index, element) in actual.into_iter().enumerate() {
            if matches!(self.inner.matches(element), MatcherResult::DoesNotMatch) {
                non_matching_elements.push((index, element, self.inner.explain_match(element)));
            }
        }
        if non_matching_elements.is_empty() {
            return MatchExplanation::create(format!(
                "whose each element {}",
                self.inner.describe(MatcherResult::Matches)
            ));
        }
        if non_matching_elements.len() == 1 {
            let (idx, element, explanation) = non_matching_elements.remove(0);
            return MatchExplanation::create(format!(
                "whose element #{} is {:?}, {}",
                idx, element, explanation
            ));
        }

        let failed_indexes = non_matching_elements
            .iter()
            .map(|&(idx, _, _)| format!("#{}", idx))
            .collect::<Vec<_>>()
            .join(", ");
        let element_explanations = non_matching_elements
            .iter()
            .map(|&(_, element, ref explanation)| format!("{:?}, {}", element, explanation))
            .collect::<Vec<_>>()
            .join("\n");
        MatchExplanation::create(format!(
            "whose elements {} don't match\n{}",
            failed_indexes, element_explanations
        ))
    }
}

impl<MatcherT: Describe> Describe for EachMatcher<MatcherT> {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!(
                    "only contains elements that {}",
                    self.inner.describe(MatcherResult::Matches)
                )
            }
            MatcherResult::DoesNotMatch => {
                format!("contains no element that {}", self.inner.describe(MatcherResult::Matches))
            }
        }
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
    use matchers::{contains_substring, displays_as, eq, err, gt, not};
    use std::collections::HashSet;

    #[google_test]
    fn each_matches_empty_vec() -> Result<()> {
        let value: Vec<i32> = vec![];
        verify_that!(value, each(gt(0)))
    }

    #[google_test]
    fn each_matches_vec_with_one_element() -> Result<()> {
        let value = vec![1];
        verify_that!(value, each(gt(0)))
    }

    #[google_test]
    fn each_matches_vec_with_two_elements() -> Result<()> {
        let value = vec![1, 2];
        verify_that!(value, each(gt(0)))
    }

    #[google_test]
    fn each_matches_slice_with_one_element() -> Result<()> {
        let value = &[1];
        verify_that!(*value, each(gt(0)))
    }

    #[google_test]
    fn each_matches_hash_set_with_one_element() -> Result<()> {
        let value: HashSet<i32> = [1].into();
        verify_that!(value, each(gt(0)))
    }

    #[google_test]
    fn each_does_not_match_when_first_element_does_not_match() -> Result<()> {
        let value = vec![0];
        verify_that!(value, not(each(gt(1))))
    }

    #[google_test]
    fn each_does_not_match_when_second_element_does_not_match() -> Result<()> {
        let value = vec![2, 0];
        verify_that!(value, not(each(gt(1))))
    }

    #[google_test]
    fn each_shows_correct_message_when_first_item_does_not_match() -> Result<()> {
        let result = verify_that!(vec![0, 2, 3], each(gt(0)));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: vec![0, 2, 3]\n\
                Expected: only contains elements that is greater than 0\n\
                Actual: [0, 2, 3], whose element #0 is 0, which is less than or equal to 0"
            )))
        )
    }

    #[google_test]
    fn each_shows_correct_message_when_second_item_does_not_match() -> Result<()> {
        let result = verify_that!(vec![1, 0, 3], each(gt(0)));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: vec![1, 0, 3]\n\
                Expected: only contains elements that is greater than 0\n\
                Actual: [1, 0, 3], whose element #1 is 0, which is less than or equal to 0"
            )))
        )
    }

    #[google_test]
    fn each_shows_correct_message_when_first_two_items_do_not_match() -> Result<()> {
        let result = verify_that!(vec![0, 1, 3], each(gt(1)));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: vec![0, 1, 3]\n\
                Expected: only contains elements that is greater than 1\n\
                Actual: [0, 1, 3], whose elements #0, #1 don't match\n\
                0, which is less than or equal to 1\n\
                1, which is less than or equal to 1"
            )))
        )
    }
    #[google_test]
    fn each_shows_inner_explanation() -> Result<()> {
        let result = verify_that!(vec![vec![1, 2], vec![1]], each(each(eq(1))));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: vec![vec! [1, 2], vec! [1]]\n\
                Expected: only contains elements that only contains elements that is equal to 1\n\
                Actual: [[1, 2], [1]], whose element #0 is [1, 2], whose element #1 is 2, which isn't equal to 1"
            )))
        )
    }
}
