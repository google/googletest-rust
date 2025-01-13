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
use crate::matcher_support::match_matrix::internal::{MatchMatrix, Requirements};
use crate::matchers::eq_matcher::eq;
use std::fmt::Debug;

/// Matches a container equal (in the sense of `==`) to `expected`.
///
/// This is similar to [`crate::matchers::eq`] except that an assertion failure
/// message generated from this matcher will include the missing and unexpected
/// items in the actual value, e.g.:
///
/// ```text
/// Expected container to equal [1, 2, 3]
///   but was: [1, 2, 4]
///   Missing: [3]
///   Unexpected: [4]
/// ```
///
/// The actual value must be a container such as a `&Vec`, an array, or a
/// dereferenced slice. More precisely, the actual value must
/// implement [`IntoIterator`] whose `Item` type implements
/// [`PartialEq<ExpectedT>`], where `ExpectedT` is the element type of the
/// expected value.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let vec = vec![1, 2, 3];
/// verify_that!(vec, container_eq([1, 2, 3]))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// *Performance note*: In the event of a mismatch leading to an assertion
/// failure, the construction of the lists of missing and unexpected values
/// uses a naive algorithm requiring time proportional to the product of the
/// sizes of the expected and actual values. This should therefore only be used
/// when the containers are small enough that this is not a problem.
pub fn container_eq<ExpectedContainerT>(
    expected: ExpectedContainerT,
) -> ContainerEqMatcher<ExpectedContainerT>
where
    ExpectedContainerT: Debug,
{
    ContainerEqMatcher { expected }
}

#[derive(MatcherBase)]
pub struct ContainerEqMatcher<ExpectedContainerT> {
    expected: ExpectedContainerT,
}

impl<ActualElementT, ActualContainerT, ExpectedElementT, ExpectedContainerT>
    Matcher<ActualContainerT> for ContainerEqMatcher<ExpectedContainerT>
where
    ActualElementT: for<'a> PartialEq<&'a ExpectedElementT> + Debug + Copy,
    ActualContainerT: for<'a> PartialEq<&'a ExpectedContainerT> + Debug + Copy,
    ExpectedElementT: Debug,
    ExpectedContainerT: Debug,
    ActualContainerT: IntoIterator<Item = ActualElementT>,
    for<'a> &'a ExpectedContainerT: IntoIterator<Item = &'a ExpectedElementT>,
{
    fn matches(&self, actual: ActualContainerT) -> MatcherResult {
        (actual == &self.expected).into()
    }

    fn explain_match(&self, actual: ActualContainerT) -> Description {
        build_explanation(self.get_missing_items(actual), self.get_unexpected_items(actual)).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("is equal to {:?}", self.expected).into(),
            MatcherResult::NoMatch => format!("isn't equal to {:?}", self.expected).into(),
        }
    }
}

impl<ExpectedElementT, ExpectedContainerT> ContainerEqMatcher<ExpectedContainerT>
where
    for<'a> &'a ExpectedContainerT: IntoIterator<Item = &'a ExpectedElementT>,
{
    fn get_missing_items<ActualElementT, ActualContainerT>(
        &self,
        actual: ActualContainerT,
    ) -> Vec<&'_ ExpectedElementT>
    where
        ActualElementT: for<'a> PartialEq<&'a ExpectedElementT> + Copy,
        ActualContainerT: for<'a> PartialEq<&'a ExpectedContainerT> + Copy,
        ActualContainerT: IntoIterator<Item = ActualElementT>,
    {
        self.expected.into_iter().filter(|i| !actual.into_iter().any(|j| j == *i)).collect()
    }

    fn get_unexpected_items<ActualElementT, ActualContainerT>(
        &self,
        actual: ActualContainerT,
    ) -> Vec<ActualElementT>
    where
        ActualElementT: for<'a> PartialEq<&'a ExpectedElementT> + Copy,
        ActualContainerT: for<'a> PartialEq<&'a ExpectedContainerT> + Copy,
        ActualContainerT: IntoIterator<Item = ActualElementT>,
    {
        actual.into_iter().filter(|i| !self.expected.into_iter().any(|j| i == &j)).collect()
    }
}

fn build_explanation<T: Debug, U: Debug>(missing: Vec<T>, unexpected: Vec<U>) -> String {
    match (missing.len(), unexpected.len()) {
        // TODO(b/261175849) add more data here (out of order elements, duplicated elements, etc...)
        (0, 0) => "which contains all the elements".to_string(),
        (0, 1) => format!("which contains the unexpected element {:?}", unexpected[0]),
        (0, _) => format!("which contains the unexpected elements {unexpected:?}",),
        (1, 0) => format!("which is missing the element {:?}", missing[0]),
        (1, 1) => {
            format!(
                "which is missing the element {:?} and contains the unexpected element {:?}",
                missing[0], unexpected[0]
            )
        }
        (1, _) => {
            format!(
                "which is missing the element {:?} and contains the unexpected elements {unexpected:?}",
                missing[0]
            )
        }
        (_, 0) => format!("which is missing the elements {missing:?}"),
        (_, 1) => {
            format!(
                "which is missing the elements {missing:?} and contains the unexpected element {:?}",
                unexpected[0]
            )
        }
        (_, _) => {
            format!(
                "which is missing the elements {missing:?} and contains the unexpected elements {unexpected:?}",
            )
        }
    }
}

impl<ExpectedContainerT> ContainerEqMatcher<ExpectedContainerT> {
    /// Match container equality, but ignoring element order.
    pub fn ignore_order(self) -> IgnoringOrder<ExpectedContainerT> {
        IgnoringOrder { expected: self.expected }
    }
}

#[derive(MatcherBase)]
pub struct IgnoringOrder<ExpectedContainerT> {
    expected: ExpectedContainerT,
}

/// Implements a matcher that ignores the relative order of the elements.
impl<ActualElementT, ActualContainerT, ExpectedElementT, ExpectedContainerT>
    Matcher<ActualContainerT> for IgnoringOrder<ExpectedContainerT>
where
    ActualElementT: Debug + Copy + for<'a> PartialEq<&'a ExpectedElementT>,
    ActualContainerT: Debug + Copy + IntoIterator<Item = ActualElementT>,
    ExpectedElementT: Debug,
    for<'a> &'a ExpectedContainerT: IntoIterator<Item = &'a ExpectedElementT>,
{
    fn matches(&self, actual: ActualContainerT) -> MatcherResult {
        let expected: Vec<Box<dyn Matcher<ActualElementT>>> = self
            .expected
            .into_iter()
            .map(|x| Box::new(eq(x)) as Box<dyn Matcher<ActualElementT>>)
            .collect();
        let match_matrix = MatchMatrix::generate(actual, &expected);
        // TODO: investigate why Requirements::PerfectMatch and
        // match_matrix.is_full_match is not doing what we expect
        // here.
        (match_matrix.is_match_for(Requirements::Subset)
            && match_matrix.is_match_for(Requirements::Superset))
        .into()
    }

    fn explain_match(&self, actual: ActualContainerT) -> Description {
        // We need to materialize the collections in order to have reliable iteration
        // order when generating our reports.
        let expected_items: Vec<&ExpectedElementT> = self.expected.into_iter().collect();
        let actual_items: Vec<ActualElementT> = actual.into_iter().collect();

        let expected_matchers: Vec<_> = expected_items
            .iter()
            .map(|&x| Box::new(eq(x)) as Box<dyn Matcher<ActualElementT>>)
            .collect();
        let match_matrix = MatchMatrix::generate(actual_items.iter().copied(), &expected_matchers);

        let best_match = match_matrix.find_best_match();

        // Since we are doing equality checks, we can generate a slightly less verbose
        // message than BestMatch::get_explanation.
        let matches = best_match.get_matches().map(|(actual_idx, expected_idx)|{
            Description::new().text(
                format!(
                    "Actual element {:?} at index {actual_idx} is equal to expected element at index {expected_idx}.",
                    actual_items[actual_idx],
                ))});

        let unmatched_actual = best_match.get_unmatched_actual().map(|actual_idx| {
            Description::new().text(
                format!(
                    "Actual element {:?} at index {actual_idx} did not match any remaining expected element.",
                    actual_items[actual_idx],
                ))
        });

        let unmatched_expected =
            best_match.get_unmatched_expected().into_iter().map(|expected_idx| {
                Description::new().text(format!(
            "Expected element {:?} at index {expected_idx} did not match any remaining actual element.",
                    expected_items[expected_idx]
        ))
            });

        Description::new()
            .text("which does not have a perfect match.  The best match found was:")
            .collect(matches.chain(unmatched_actual).chain(unmatched_expected))
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        Description::new()
            .text(format!(
                "{} all elements matching in any order:",
                if matcher_result.into() { "contains" } else { "doesn't contain" },
            ))
            .nested(
                self.expected
                    .into_iter()
                    .map(|element| format!("{:?}", element))
                    .collect::<Description>()
                    .bullet_list(),
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::MatcherResult;
    use crate::prelude::*;
    use crate::Result;
    use indoc::indoc;
    use std::collections::HashSet;

    #[test]
    fn container_eq_returns_match_when_containers_match() -> Result<()> {
        verify_that!(vec![1, 2, 3], container_eq(vec![1, 2, 3]))
    }

    #[test]
    fn container_eq_matches_array_with_slice() -> Result<()> {
        let value = &[1, 2, 3];
        verify_that!(value, container_eq([1, 2, 3]))
    }

    #[test]
    fn container_eq_matches_hash_set() -> Result<()> {
        let value: HashSet<i32> = [1, 2, 3].into();
        verify_that!(value, container_eq([1, 2, 3].into()))
    }

    #[test]
    fn container_eq_full_error_message() -> Result<()> {
        let result = verify_that!(vec![1, 3, 2], container_eq(vec![1, 2, 3]));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: vec![1, 3, 2]
                    Expected: is equal to [1, 2, 3]
                    Actual: [1, 3, 2],
                      which contains all the elements
                "
            ))))
        )
    }

    #[test]
    fn container_eq_returns_mismatch_when_elements_out_of_order() -> Result<()> {
        verify_that!(
            container_eq(vec![1, 2, 3]).explain_match(&vec![1, 3, 2]),
            displays_as(eq("which contains all the elements"))
        )
    }

    #[test]
    fn container_eq_mismatch_shows_missing_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq(vec![1, 2, 3]).explain_match(&vec![1, 2]),
            displays_as(eq("which is missing the element 3"))
        )
    }

    #[test]
    fn container_eq_mismatch_shows_surplus_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq(vec![1, 2]).explain_match(&vec![1, 2, 3]),
            displays_as(eq("which contains the unexpected element 3"))
        )
    }

    #[test]
    fn container_eq_mismatch_shows_missing_and_surplus_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq(vec![1, 2, 3]).explain_match(&vec![1, 2, 4]),
            displays_as(eq("which is missing the element 3 and contains the unexpected element 4"))
        )
    }

    #[test]
    fn container_eq_mismatch_does_not_show_duplicated_element() -> Result<()> {
        verify_that!(
            container_eq(vec![1, 2, 3]).explain_match(&vec![1, 2, 3, 3]),
            displays_as(eq("which contains all the elements"))
        )
    }

    #[test]
    fn container_eq_matches_owned_vec_with_array() -> Result<()> {
        let vector = vec![123, 234];
        verify_that!(vector, container_eq([123, 234]))
    }

    #[test]
    fn container_eq_matches_owned_vec_of_owned_strings_with_slice_of_string_references(
    ) -> Result<()> {
        let vector = vec!["A string".to_string(), "Another string".to_string()];
        verify_that!(vector, container_eq(["A string", "Another string"]))
    }

    #[test]
    fn container_eq_matches_owned_vec_of_owned_strings_with_shorter_slice_of_string_references(
    ) -> Result<()> {
        let actual = vec!["A string".to_string(), "Another string".to_string()];
        let matcher = container_eq(["A string"]);

        let result = matcher.matches(&actual);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn container_eq_mismatch_with_slice_shows_missing_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq([1, 2, 3]).explain_match(&vec![1, 2]),
            displays_as(eq("which is missing the element 3"))
        )
    }

    #[test]
    fn container_eq_mismatch_with_str_slice_shows_missing_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq(["A", "B", "C"]).explain_match(&vec!["A".to_string(), "B".to_string()]),
            displays_as(eq("which is missing the element \"C\""))
        )
    }

    #[test]
    fn container_eq_mismatch_with_str_slice_shows_surplus_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq(["A", "B"]).explain_match(&vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string()
            ]),
            displays_as(eq("which contains the unexpected element \"C\""))
        )
    }

    #[test]
    fn ignoring_order_match() -> Result<()> {
        verify_that!(vec!["a", "b"], container_eq(["b", "a"]).ignore_order())
    }

    #[test]
    fn ignoring_order_mismatch() -> Result<()> {
        verify_that!(vec!["a", "b"], not(container_eq(["1", "2"]).ignore_order()))
    }

    #[test]
    fn ignoring_order_mismatch_explain() -> Result<()> {
        let expected_err = verify_that!(vec!["a", "b"], container_eq(["1", "2"]).ignore_order());
        verify_that!(
            expected_err,
            err(displays_as(contains_substring(indoc!(
                r#"
                Value of: vec!["a", "b"]
                Expected: contains all elements matching in any order:
                  * "1"
                  * "2"
                Actual: ["a", "b"],
                  which does not have a perfect match.  The best match found was:
                    Actual element "a" at index 0 did not match any remaining expected element.
                    Actual element "b" at index 1 did not match any remaining expected element.
                    Expected element "1" at index 0 did not match any remaining actual element.
                    Expected element "2" at index 1 did not match any remaining actual element.
                "#
            ))))
        )
    }

    #[test]
    fn ignoring_order_unaccounted_extra_expected() -> Result<()> {
        verify_that!(vec!["a", "b"], not(container_eq(["a", "b", "a"]).ignore_order()))
    }

    #[test]
    fn ignoring_order_unaccounted_extra_expected_explain() -> Result<()> {
        let expected_err =
            verify_that!(vec!["a", "b"], container_eq(["a", "b", "a"]).ignore_order());
        verify_that!(
            expected_err,
            err(displays_as(contains_substring(indoc!(
                r#"
                Value of: vec!["a", "b"]
                Expected: contains all elements matching in any order:
                  * "a"
                  * "b"
                  * "a"
                Actual: ["a", "b"],
                  which does not have a perfect match.  The best match found was:
                    Actual element "a" at index 0 is equal to expected element at index 0.
                    Actual element "b" at index 1 is equal to expected element at index 1.
                    Expected element "a" at index 2 did not match any remaining actual element.
                "#
            ))))
        )
    }

    #[test]
    fn ignoring_order_unaccounted_extra_actual() -> Result<()> {
        verify_that!(vec!["a", "b", "a"], not(container_eq(["b", "a"]).ignore_order()))
    }

    #[test]
    fn ignoring_order_unaccounted_extra_actual_explain() -> Result<()> {
        let expected_err =
            verify_that!(vec!["a", "b", "a"], container_eq(["b", "a"]).ignore_order());

        verify_that!(
            expected_err,
            err(displays_as(contains_substring(indoc!(
                r#"
                Value of: vec!["a", "b", "a"]
                Expected: contains all elements matching in any order:
                  * "b"
                  * "a"
                Actual: ["a", "b", "a"],
                  which does not have a perfect match.  The best match found was:
                    Actual element "a" at index 0 is equal to expected element at index 1.
                    Actual element "b" at index 1 is equal to expected element at index 0.
                    Actual element "a" at index 2 did not match any remaining expected element.
                "#
            ))))
        )
    }

    #[test]
    fn ignoring_order_on_sets() -> Result<()> {
        let mut actual = std::collections::HashSet::new();
        actual.insert("b");
        actual.insert("a");
        actual.insert("c");
        verify_that!(actual, container_eq(["c", "b", "a"]).ignore_order())
    }

    #[test]
    fn ignoring_order_on_sets_explain() -> Result<()> {
        let mut actual = std::collections::HashSet::new();
        actual.insert("b");
        actual.insert("a");
        actual.insert("c");
        let expected_err = verify_that!(actual, container_eq(["c", "a"]).ignore_order());
        verify_that!(
            expected_err,
            err(displays_as(contains_regex(indoc!(
                r#"
                Value of: actual
                Expected: contains all elements matching in any order:
                  \* "c"
                  \* "a"
                Actual: \{"\w", "\w", "\w"\},
                  which does not have a perfect match.  The best match found was:
                    Actual element "\w" at index \d is equal to expected element at index \d\.
                    Actual element "\w" at index \d is equal to expected element at index \d\.
                    Actual element "\w" at index \d did not match any remaining expected element\.
                "#
            ))))
        )
    }

    #[test]
    fn ignoring_order_on_number_sets() -> Result<()> {
        let mut actual = std::collections::HashSet::new();
        actual.insert(1);
        actual.insert(2);
        actual.insert(3);
        verify_that!(actual, container_eq([3, 2, 1]).ignore_order())
    }
}
