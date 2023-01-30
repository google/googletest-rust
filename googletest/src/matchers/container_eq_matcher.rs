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
use googletest::matcher::{MatchExplanation, Matcher, MatcherResult};
use std::fmt::Debug;
use std::iter::zip;

/// Matches a container equal (in the sense of `==`) to `expected`.
///
/// This is similar to [`crate::matchers::eq`] except that an assertion failure
/// message generated from this matcher will include the missing and unexpected
/// items in the actual value, e.g.:
///
/// ```
/// Expected container to equal [1, 2, 3]
///   but was: [1, 2, 4]
///   Missing: [3]
///   Unexpected: [4]
/// ```
///
/// The type of `expected` must implement `IntoIterator` with an `Item` which
/// implements `PartialEq`. If the container type is a `Vec`, then the expected
/// type may be a slice of the same element type. For example:
///
/// ```rust
/// let vec = vec![1, 2, 3];
/// verify_that!(vec, container_eq([1, 2, 3]))?;
/// ```
///
/// As an exception, if the actual type is a `Vec<String>`, the expected type
/// may be a slice of `&str`:
///
/// ```rust
/// let vec: Vec<String> = vec!["A string".into(), "Another string".into()];
/// verify_that!(vec, container_eq(["A string", "Another string"]))?;
/// ```
///
/// These exceptions allow one to avoid unnecessary allocations in test
/// assertions.
///
/// One can also check container equality of a slice with an array. To do so,
/// dereference the slice:
///
/// ```rust
/// let value = &[1, 2, 3];
/// verify_that!(*value, container_eq([1, 2, 3]))?;
/// ```
///
/// Otherwise, the actual and expected types must be identical.
///
/// *Performance note*: In the event of a mismatch leading to an assertion
/// failure, the construction of the lists of missing and unexpected values
/// uses a naive algorithm requiring time proportional to the product of the
/// sizes of the expected and actual values. This should therefore only be used
/// when the containers are small enough that this is not a problem.
// This returns ContainerEqMatcher and not impl Matcher because
// ContainerEqMatcher has some specialisations for slice types (see
// documentation above). Returning impl Matcher would hide those from the
// compiler.
pub fn container_eq<ContainerT: PartialEq + Debug>(
    expected: ContainerT,
) -> ContainerEqMatcher<ContainerT> {
    ContainerEqMatcher { expected }
}

pub struct ContainerEqMatcher<T: Debug> {
    expected: T,
}

impl<T: PartialEq + Debug, ContainerT: PartialEq + Debug> Matcher<ContainerT>
    for ContainerEqMatcher<ContainerT>
where
    for<'a> &'a ContainerT: IntoIterator<Item = &'a T>,
{
    fn matches(&self, actual: &ContainerT) -> MatcherResult {
        if *actual == self.expected { MatcherResult::Matches } else { MatcherResult::DoesNotMatch }
    }

    fn explain_match(&self, actual: &ContainerT) -> MatchExplanation {
        self.explain_match_impl(actual)
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        self.describe_impl(matcher_result)
    }
}

impl<T: PartialEq + Debug, const N: usize> Matcher<Vec<T>> for ContainerEqMatcher<[T; N]> {
    fn matches(&self, actual: &Vec<T>) -> MatcherResult {
        if actual.as_slice() == self.expected {
            MatcherResult::Matches
        } else {
            MatcherResult::DoesNotMatch
        }
    }

    fn explain_match(&self, actual: &Vec<T>) -> MatchExplanation {
        self.explain_match_impl(actual)
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        self.describe_impl(matcher_result)
    }
}

impl<T: PartialEq + Debug, const N: usize> Matcher<[T]> for ContainerEqMatcher<[T; N]> {
    fn matches(&self, actual: &[T]) -> MatcherResult {
        if actual == self.expected { MatcherResult::Matches } else { MatcherResult::DoesNotMatch }
    }

    fn explain_match(&self, actual: &[T]) -> MatchExplanation {
        self.explain_match_impl(actual)
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        self.describe_impl(matcher_result)
    }
}

impl<const N: usize> Matcher<Vec<String>> for ContainerEqMatcher<[&str; N]> {
    fn matches(&self, actual: &Vec<String>) -> MatcherResult {
        if actual.len() != self.expected.len() {
            return MatcherResult::DoesNotMatch;
        }
        for (actual_element, expected_element) in zip(actual, self.expected) {
            if actual_element.as_str() != expected_element {
                return MatcherResult::DoesNotMatch;
            }
        }
        MatcherResult::Matches
    }

    fn explain_match(&self, actual: &Vec<String>) -> MatchExplanation {
        build_explanation(
            self.get_missing_str_items(actual),
            self.get_unexpected_string_items(actual),
        )
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        self.describe_impl(matcher_result)
    }
}

impl<T: PartialEq + Debug, ExpectedT: Debug> ContainerEqMatcher<ExpectedT>
where
    for<'a> &'a ExpectedT: IntoIterator<Item = &'a T>,
{
    fn explain_match_impl<ActualT: ?Sized>(&self, actual: &ActualT) -> MatchExplanation
    where
        for<'a> &'a ActualT: IntoIterator<Item = &'a T> + Debug,
    {
        build_explanation(self.get_missing_items(actual), self.get_unexpected_items(actual))
    }

    fn get_missing_items<ActualT: ?Sized>(&self, actual: &ActualT) -> Vec<&T>
    where
        for<'a> &'a ActualT: IntoIterator<Item = &'a T>,
    {
        self.expected.into_iter().filter(|&i| !actual.into_iter().any(|j| j == i)).collect()
    }

    fn get_unexpected_items<'a, ActualT: ?Sized>(&self, actual: &'a ActualT) -> Vec<&'a T>
    where
        for<'b> &'b ActualT: IntoIterator<Item = &'b T>,
    {
        actual.into_iter().filter(|&i| !self.expected.into_iter().any(|j| j == i)).collect()
    }
}

impl<ExpectedT: Debug> ContainerEqMatcher<ExpectedT> {
    fn describe_impl(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => format!("is equal to {:?}", self.expected),
            MatcherResult::DoesNotMatch => format!("isn't equal to {:?}", self.expected),
        }
    }
}

fn build_explanation<T: Debug, U: Debug>(missing: Vec<T>, unexpected: Vec<U>) -> MatchExplanation {
    match (missing.len(), unexpected.len()) {
        // TODO(b/261175849) add more data here (out of order elements, duplicated elements, etc...)
        (0, 0) => MatchExplanation::create("which contains all the elements".to_string()),
        (0, 1) => MatchExplanation::create(format!(
            "which contains the unexpected element {:?}",
            unexpected[0]
        )),
        (0, _) => MatchExplanation::create(format!(
            "which contains the unexpected elements {unexpected:?}",
        )),
        (1, 0) => {
            MatchExplanation::create(format!("which is missing the element {:?}", missing[0]))
        }
        (1, 1) => MatchExplanation::create(format!(
            "which is missing the element {:?} and contains the unexpected element {:?}",
            missing[0], unexpected[0]
        )),
        (1, _) => MatchExplanation::create(format!(
            "which is missing the element {:?} and contains the unexpected elements {unexpected:?}",
            missing[0]
        )),
        (_, 0) => MatchExplanation::create(format!("which is missing the elements {missing:?}")),
        (_, 1) => MatchExplanation::create(format!(
            "which is missing the elements {missing:?} and contains the unexpected element {:?}",
            unexpected[0]
        )),
        (_, _) => MatchExplanation::create(format!(
            "which is missing the elements {missing:?} and contains the unexpected elements {unexpected:?}",
        )),
    }
}

impl<const N: usize> ContainerEqMatcher<[&str; N]> {
    fn get_missing_str_items(&self, actual: &Vec<String>) -> Vec<&str> {
        self.expected
            .into_iter()
            .filter(|i| actual.into_iter().find(|j| &j.as_str() == i).is_none())
            .collect()
    }

    fn get_unexpected_string_items<'a>(&self, actual: &'a Vec<String>) -> Vec<&'a String> {
        actual
            .into_iter()
            .filter(|i| self.expected.into_iter().find(|j| j == &i.as_str()).is_none())
            .collect()
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
    use matchers::{contains_substring, displays_as, eq, err};
    use std::collections::HashSet;

    #[google_test]
    fn container_eq_returns_match_when_containers_match() -> Result<()> {
        verify_that!(vec![1, 2, 3], container_eq(vec![1, 2, 3]))
    }

    #[google_test]
    fn container_eq_matches_array_with_slice() -> Result<()> {
        let value = &[1, 2, 3];
        verify_that!(*value, container_eq([1, 2, 3]))
    }

    #[google_test]
    fn container_eq_matches_hash_set() -> Result<()> {
        let value: HashSet<i32> = [1, 2, 3].into();
        verify_that!(value, container_eq([1, 2, 3].into()))
    }

    #[google_test]
    fn container_eq_full_error_message() -> Result<()> {
        let result = verify_that!(vec![1, 3, 2], container_eq(vec![1, 2, 3]));
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "\
Value of: vec![1, 3, 2]
Expected: is equal to [1, 2, 3]
Actual: [
    1,
    3,
    2,
], which contains all the elements
"
            )))
        )
    }

    #[google_test]
    fn container_eq_returns_mismatch_when_elements_out_of_order() -> Result<()> {
        verify_that!(
            container_eq(vec![1, 2, 3]).explain_match(&vec![1, 3, 2]),
            displays_as(eq("which contains all the elements"))
        )
    }

    #[google_test]
    fn container_eq_mismatch_shows_missing_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq(vec![1, 2, 3]).explain_match(&vec![1, 2]),
            displays_as(eq("which is missing the element 3"))
        )
    }

    #[google_test]
    fn container_eq_mismatch_shows_surplus_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq(vec![1, 2]).explain_match(&vec![1, 2, 3]),
            displays_as(eq("which contains the unexpected element 3"))
        )
    }

    #[google_test]
    fn container_eq_mismatch_shows_missing_and_surplus_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq(vec![1, 2, 3]).explain_match(&vec![1, 2, 4]),
            displays_as(eq("which is missing the element 3 and contains the unexpected element 4"))
        )
    }

    #[google_test]
    fn container_eq_mismatch_does_not_show_duplicated_element() -> Result<()> {
        verify_that!(
            container_eq(vec![1, 2, 3]).explain_match(&vec![1, 2, 3, 3]),
            displays_as(eq("which contains all the elements"))
        )
    }

    #[google_test]
    fn container_eq_matches_owned_vec_with_slice() -> Result<()> {
        let vector = vec![123, 234];
        verify_that!(vector, container_eq([123, 234]))
    }

    #[google_test]
    fn container_eq_matches_owned_vec_of_owned_strings_with_slice_of_string_references()
    -> Result<()> {
        let vector = vec!["A string".to_string(), "Another string".to_string()];
        verify_that!(vector, container_eq(["A string", "Another string"]))
    }

    #[google_test]
    fn container_eq_matches_owned_vec_of_owned_strings_with_shorter_slice_of_string_references()
    -> Result<()> {
        let actual = vec!["A string".to_string(), "Another string".to_string()];
        let matcher = container_eq(["A string"]);

        let result = matcher.matches(&actual);

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn container_eq_mismatch_with_slice_shows_missing_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq([1, 2, 3]).explain_match(&vec![1, 2]),
            displays_as(eq("which is missing the element 3"))
        )
    }

    #[google_test]
    fn container_eq_mismatch_with_str_slice_shows_missing_elements_in_container() -> Result<()> {
        verify_that!(
            container_eq(["A", "B", "C"]).explain_match(&vec!["A".to_string(), "B".to_string()]),
            displays_as(eq("which is missing the element \"C\""))
        )
    }

    #[google_test]
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
}
