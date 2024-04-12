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

use crate::description::Description;
use crate::matcher::{Matcher, MatcherBase, MatcherResult};
use std::fmt::Debug;

/// Matches a container all of whose elements are matched by the matcher
/// `inner`.
///
/// `T` must implement [`IntoIterator`]. This
/// includes `&Vec`, arrays, and slices.
///
/// ```
/// # use googletest::prelude::*;
/// # use std::collections::HashSet;
/// # fn should_pass_1() -> Result<()> {
/// let value = vec![1, 2, 3];
/// verify_that!(value, each(gt(&0)))?;  // Passes
/// let array_value = [1, 2, 3];
/// verify_that!(array_value, each(gt(0)))?;  // Passes
/// let slice_value = &[1, 2, 3];
/// verify_that!(slice_value, each(gt(&0)))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// #     let value = vec![1, 2, 3];
/// verify_that!(value, each(lt(&2)))?;  // Fails: 2 and 3 are not less than 2
/// #     Ok(())
/// # }
///
/// # fn should_pass_2() -> Result<()> {
/// let value: HashSet<i32> = [1, 2, 3].into();
/// verify_that!(value, each(gt(&0)))?;  // Passes
/// #     Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_fail().unwrap_err();
/// # should_pass_2().unwrap();
/// ```
pub fn each<MatcherT>(inner: MatcherT) -> EachMatcher<MatcherT> {
    EachMatcher { inner }
}

#[derive(MatcherBase)]
pub struct EachMatcher<MatcherT> {
    inner: MatcherT,
}

impl<ElementT: Debug + Copy, ActualT: Debug + Copy, MatcherT> Matcher<ActualT>
    for EachMatcher<MatcherT>
where
    ActualT: IntoIterator<Item = ElementT>,
    MatcherT: Matcher<ElementT>,
{
    fn matches(&self, actual: ActualT) -> MatcherResult {
        for element in actual {
            if self.inner.matches(element).is_no_match() {
                return MatcherResult::NoMatch;
            }
        }
        MatcherResult::Match
    }

    fn explain_match(&self, actual: ActualT) -> Description {
        let mut non_matching_elements = Vec::new();
        for (index, element) in actual.into_iter().enumerate() {
            if self.inner.matches(element).is_no_match() {
                non_matching_elements.push((index, element, self.inner.explain_match(element)));
            }
        }
        if non_matching_elements.is_empty() {
            return format!("whose each element {}", self.inner.describe(MatcherResult::Match))
                .into();
        }
        if non_matching_elements.len() == 1 {
            let (idx, element, explanation) = non_matching_elements.remove(0);
            return format!("whose element #{idx} is {element:?}, {explanation}").into();
        }

        let failed_indexes = non_matching_elements
            .iter()
            .map(|&(idx, _, _)| format!("#{idx}"))
            .collect::<Vec<_>>()
            .join(", ");
        let element_explanations = non_matching_elements
            .iter()
            .map(|&(_, element, ref explanation)| format!("{element:?}, {explanation}"))
            .collect::<Description>()
            .indent();
        format!("whose elements {failed_indexes} don't match\n{element_explanations}").into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("only contains elements that {}", self.inner.describe(MatcherResult::Match))
                    .into()
            }
            MatcherResult::NoMatch => {
                format!("contains no element that {}", self.inner.describe(MatcherResult::Match))
                    .into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::each;
    use crate::prelude::*;
    use indoc::indoc;
    use std::collections::HashSet;

    #[test]
    fn each_matches_empty_vec() -> Result<()> {
        let value: Vec<i32> = vec![];
        verify_that!(value, each(gt(&0)))
    }

    #[test]
    fn each_matches_vec_with_one_element() -> Result<()> {
        let value = vec![1];
        verify_that!(value, each(gt(&0)))
    }

    #[test]
    fn each_matches_vec_with_two_elements() -> Result<()> {
        let value = vec![1, 2];
        verify_that!(value, each(gt(&0)))
    }

    #[test]
    fn each_matches_slice_with_one_element() -> Result<()> {
        let value = &[1];
        verify_that!(*value, each(gt(0)))
    }

    #[test]
    fn each_matches_hash_set_with_one_element() -> Result<()> {
        let value: HashSet<i32> = [1].into();
        verify_that!(value, each(gt(&0)))
    }

    #[test]
    fn each_does_not_match_when_first_element_does_not_match() -> Result<()> {
        let value = vec![0];
        verify_that!(value, not(each(gt(&1))))
    }

    #[test]
    fn each_does_not_match_when_second_element_does_not_match() -> Result<()> {
        let value = vec![2, 0];
        verify_that!(value, not(each(gt(&1))))
    }

    #[test]
    fn each_shows_correct_message_when_first_item_does_not_match() -> Result<()> {
        let result = verify_that!(vec![0, 2, 3], each(gt(&0)));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: vec![0, 2, 3]
                Expected: only contains elements that is greater than 0
                Actual: [0, 2, 3],
                  whose element #0 is 0, which is less than or equal to 0"
            ))))
        )
    }

    #[test]
    fn each_shows_correct_message_when_second_item_does_not_match() -> Result<()> {
        let result = verify_that!(vec![1, 0, 3], each(gt(&0)));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: vec![1, 0, 3]
                Expected: only contains elements that is greater than 0
                Actual: [1, 0, 3],
                  whose element #1 is 0, which is less than or equal to 0"
            ))))
        )
    }

    #[test]
    fn each_shows_correct_message_when_first_two_items_do_not_match() -> Result<()> {
        let result = verify_that!(vec![0, 1, 3], each(gt(&1)));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: vec![0, 1, 3]
                Expected: only contains elements that is greater than 1
                Actual: [0, 1, 3],
                  whose elements #0, #1 don't match
                    0, which is less than or equal to 1
                    1, which is less than or equal to 1"
            ))))
        )
    }
    #[test]
    fn each_shows_inner_explanation() -> Result<()> {
        let result = verify_that!(vec![vec![1, 2], vec![1]], each(each(eq(&1))));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Expected: only contains elements that only contains elements that is equal to 1
                Actual: [[1, 2], [1]],
                  whose element #0 is [1, 2], whose element #1 is 2, which isn't equal to 1"
            ))))
        )
    }
}
