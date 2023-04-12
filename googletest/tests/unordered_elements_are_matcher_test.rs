// Copyright 2023 Google LLC
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

use googletest::matcher::Matcher;
#[cfg(not(google3))]
use googletest::matchers;
#[cfg(not(google3))]
use googletest::{contains_each, is_contained_in, unordered_elements_are};
use googletest::{google_test, matcher::MatcherResult, verify_that, Result};
use indoc::indoc;
use matchers::{
    anything, contains_regex, contains_substring, displays_as, eq, err, ge, not, AndMatcherExt,
};
#[cfg(google3)]
use matchers::{contains_each, is_contained_in, unordered_elements_are};
use std::collections::HashMap;

#[google_test]
fn unordered_elements_are_matches_empty_vector() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, unordered_elements_are![])
}

#[google_test]
fn unordered_elements_are_matches_empty_vector_with_trailing_comma() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, unordered_elements_are![,])
}

#[google_test]
fn unordered_elements_are_matches_vector() -> Result<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, unordered_elements_are![eq(1), eq(2), eq(3)])
}

#[google_test]
fn unordered_elements_are_matches_slice() -> Result<()> {
    let value = vec![1, 2, 3];
    let slice = value.as_slice();
    verify_that!(*slice, unordered_elements_are![eq(1), eq(2), eq(3)])
}

#[google_test]
fn unordered_elements_are_matches_hash_map() -> Result<()> {
    let value: HashMap<u32, &'static str> =
        HashMap::from_iter([(1, "One"), (2, "Two"), (3, "Three")]);
    verify_that!(
        value,
        unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))]
    )
}

#[google_test]
fn unordered_elements_are_matches_hash_map_with_trailing_comma() -> Result<()> {
    let value: HashMap<u32, &'static str> =
        HashMap::from_iter([(1, "One"), (2, "Two"), (3, "Three")]);
    verify_that!(
        value,
        unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three")),]
    )
}

#[google_test]
fn unordered_elements_are_does_not_match_hash_map_with_wrong_key() -> Result<()> {
    let value: HashMap<u32, &'static str> =
        HashMap::from_iter([(1, "One"), (2, "Two"), (4, "Three")]);
    verify_that!(
        value,
        not(unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))])
    )
}

#[google_test]
fn unordered_elements_are_does_not_match_hash_map_with_wrong_value() -> Result<()> {
    let value: HashMap<u32, &'static str> =
        HashMap::from_iter([(1, "One"), (2, "Two"), (3, "Four")]);
    verify_that!(
        value,
        not(unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))])
    )
}

#[google_test]
fn unordered_elements_are_does_not_match_hash_map_missing_element() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from_iter([(1, "One"), (2, "Two")]);
    verify_that!(
        value,
        not(unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))])
    )
}

#[google_test]
fn unordered_elements_are_does_not_match_hash_map_with_extra_element() -> Result<()> {
    let value: HashMap<u32, &'static str> =
        HashMap::from_iter([(1, "One"), (2, "Two"), (3, "Three")]);
    verify_that!(value, not(unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One"))]))
}

#[google_test]
fn unordered_elements_are_does_not_match_hash_map_with_mismatched_key_and_value() -> Result<()> {
    let value: HashMap<u32, &'static str> =
        HashMap::from_iter([(1, "One"), (2, "Three"), (3, "Two")]);
    verify_that!(
        value,
        not(unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))])
    )
}

#[google_test]
fn unordered_elements_are_matches_vector_with_trailing_comma() -> Result<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, unordered_elements_are![eq(1), eq(2), eq(3),])
}

#[google_test]
fn unordered_elements_are_matches_size() -> Result<()> {
    let value = vec![1, 2];
    verify_that!(value, not(unordered_elements_are![eq(1), eq(2), eq(3)]))
}

#[google_test]
fn unordered_elements_are_description_mismatch() -> Result<()> {
    let result = verify_that!(vec![1, 4, 3], unordered_elements_are![eq(1), eq(2), eq(3)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: vec![1, 4, 3]
            Expected: contains elements matching in any order:
              0. is equal to 1
              1. is equal to 2
              2. is equal to 3
            Actual: [
                1,
                4,
                3,
            ], whose element #1 does not match any expected elements and no elements match the expected element #1"
            ))))
    )
}

#[google_test]
fn has_correct_description_for_map() -> Result<()> {
    // UnorderedElementsAreMatcher maintains references to the matchers, so the
    // constituent matchers must live longer. Inside a verify_that! macro, the
    // compiler takes care of that, but when the matcher is created separately,
    // we must create the constitute matchers separately so that they
    // aren't dropped too early.
    let matchers = ((eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three")));
    let matcher = unordered_elements_are![
        (matchers.0.0, matchers.0.1),
        (matchers.1.0, matchers.1.1),
        (matchers.2.0, matchers.2.1)
    ];
    verify_that!(
        Matcher::<HashMap<u32, &'static str>>::describe(&matcher, MatcherResult::Matches),
        eq(indoc!(
            "
                contains elements matching in any order:
                  is equal to 2 => is equal to \"Two\"
                  is equal to 1 => is equal to \"One\"
                  is equal to 3 => is equal to \"Three\""
        ))
    )
}

#[google_test]
fn unordered_elements_are_matches_unordered() -> Result<()> {
    let value = vec![1, 2];
    verify_that!(value, unordered_elements_are![eq(2), eq(1)])
}

#[google_test]
fn unordered_elements_are_matches_unordered_with_repetition() -> Result<()> {
    let value = vec![1, 2, 1, 2, 1];
    verify_that!(value, unordered_elements_are![eq(1), eq(1), eq(1), eq(2), eq(2)])
}

#[google_test]
fn unordered_elements_are_explains_mismatch_due_to_wrong_size() -> Result<()> {
    verify_that!(
        unordered_elements_are![eq(2), eq(3), eq(4)].explain_match(&vec![2, 3]),
        displays_as(eq("which has size 2 (expected 3)"))
    )
}

#[google_test]
fn unordered_elements_are_description_no_full_match() -> Result<()> {
    verify_that!(
        unordered_elements_are![eq(1), eq(2), eq(2)].explain_match(&vec![1, 1, 2]),
        displays_as(eq(indoc!(
            "
            which does not have a perfect match with the expected elements. The best match found was:
              Actual element 1 at index 0 matched expected element `is equal to 1` at index 0.
              Actual element 2 at index 2 matched expected element `is equal to 2` at index 1.
              Actual element 1 at index 1 did not match any remaining expected element.
              Expected element `is equal to 2` at index 2 did not match any remaining actual element."
        )))
    )
}

#[google_test]
fn unordered_elements_are_description_no_full_match_with_map() -> Result<()> {
    // UnorderedElementsAreMatcher maintains references to the matchers, so the
    // constituent matchers must live longer. Inside a verify_that! macro, the
    // compiler takes care of that, but when the matcher is created separately,
    // we must create the constitute matchers separately so that they
    // aren't dropped too early.
    let matchers = ((anything(), eq(1)), (anything(), eq(2)), (anything(), eq(2)));
    let matcher = unordered_elements_are![
        (matchers.0.0, matchers.0.1),
        (matchers.1.0, matchers.1.1),
        (matchers.2.0, matchers.2.1),
    ];
    let value: HashMap<u32, u32> = HashMap::from_iter([(0, 1), (1, 1), (2, 2)]);
    verify_that!(
        matcher.explain_match(&value),
        displays_as(contains_regex(
            "Actual element 2 => 2 at index [0-2] matched expected element `is anything` => `is equal to 2` at index [0-2]."
        )).and(displays_as(contains_regex(
            "Actual element [0-1] => [0-1] at index [0-2] did not match any remaining expected element."
        ))).and(displays_as(contains_substring(
            "Expected element `is anything` => `is equal to 2` at index 2 did not match any remaining actual element."
        )))
    )
}

#[google_test]
fn unordered_elements_are_unmatchable_expected_description_mismatch() -> Result<()> {
    verify_that!(
        unordered_elements_are![eq(1), eq(2), eq(3)].explain_match(&vec![1, 1, 3]),
        displays_as(eq("which has no element matching the expected element #1"))
    )
}

#[google_test]
fn unordered_elements_are_unmatchable_actual_description_mismatch() -> Result<()> {
    verify_that!(
        unordered_elements_are![eq(1), eq(1), eq(3)].explain_match(&vec![1, 2, 3]),
        displays_as(eq("whose element #1 does not match any expected elements"))
    )
}

#[google_test]
fn contains_each_matches_when_one_to_one_correspondence_present() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!(eq(2), eq(3), eq(4)))
}

#[google_test]
fn contains_each_supports_trailing_comma() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!(eq(2), eq(3), eq(4),))
}

#[google_test]
fn contains_each_matches_hash_map() -> Result<()> {
    let value: HashMap<u32, &'static str> =
        HashMap::from_iter([(1, "One"), (2, "Two"), (3, "Three")]);
    verify_that!(value, contains_each![(eq(2), eq("Two")), (eq(1), eq("One"))])
}

#[google_test]
fn contains_each_matches_hash_map_with_trailing_comma() -> Result<()> {
    let value: HashMap<u32, &'static str> =
        HashMap::from_iter([(1, "One"), (2, "Two"), (3, "Three")]);
    verify_that!(value, contains_each![(eq(2), eq("Two")), (eq(1), eq("One")),])
}

#[google_test]
fn contains_each_matches_when_no_matchers_present() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!())
}

#[google_test]
fn contains_each_matches_when_no_matchers_present_and_trailing_comma() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!(,))
}

#[google_test]
fn contains_each_matches_when_list_is_empty_and_no_matchers_present() -> Result<()> {
    verify_that!(Vec::<u32>::new(), contains_each!())
}

#[google_test]
fn contains_each_matches_when_excess_elements_present() -> Result<()> {
    verify_that!(vec![1, 2, 3, 4], contains_each!(eq(2), eq(3), eq(4)))
}

#[google_test]
fn contains_each_does_not_match_when_matchers_are_unmatched() -> Result<()> {
    verify_that!(vec![1, 2, 3], not(contains_each!(eq(2), eq(3), eq(4))))
}

#[google_test]
fn contains_each_explains_mismatch_due_to_wrong_size() -> Result<()> {
    verify_that!(
        contains_each![eq(2), eq(3), eq(4)].explain_match(&vec![2, 3]),
        displays_as(eq("which has size 2 (expected at least 3)"))
    )
}

#[google_test]
fn contains_each_explains_missing_element_in_mismatch() -> Result<()> {
    verify_that!(
        contains_each![eq(2), eq(3), eq(4)].explain_match(&vec![1, 2, 3]),
        displays_as(eq("which has no element matching the expected element #2"))
    )
}

#[google_test]
fn contains_each_explains_missing_elements_in_mismatch() -> Result<()> {
    verify_that!(
        contains_each![eq(2), eq(3), eq(4), eq(5)].explain_match(&vec![0, 1, 2, 3]),
        displays_as(eq("which has no elements matching the expected elements #2, #3"))
    )
}

#[google_test]
fn contains_each_explains_mismatch_due_to_no_graph_matching_found() -> Result<()> {
    verify_that!(
        contains_each![ge(2), ge(2)].explain_match(&vec![1, 2]),
        displays_as(eq(indoc!(
            "
            which does not have a superset match with the expected elements. The best match found was:
              Actual element 2 at index 1 matched expected element `is greater than or equal to 2` at index 0.
              Actual element 1 at index 0 did not match any remaining expected element.
              Expected element `is greater than or equal to 2` at index 1 did not match any remaining actual element."))
    ))
}

#[google_test]
fn is_contained_in_matches_with_empty_vector() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, is_contained_in!())
}

#[google_test]
fn is_contained_in_matches_with_empty_vector_and_trailing_comma() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, is_contained_in!(,))
}

#[google_test]
fn is_contained_in_matches_when_one_to_one_correspondence_present() -> Result<()> {
    verify_that!(vec![2, 3, 4], is_contained_in!(eq(2), eq(3), eq(4)))
}

#[google_test]
fn is_contained_supports_trailing_comma() -> Result<()> {
    verify_that!(vec![2, 3, 4], is_contained_in!(eq(2), eq(3), eq(4),))
}

#[google_test]
fn is_contained_in_matches_hash_map() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from_iter([(1, "One"), (2, "Two")]);
    verify_that!(
        value,
        is_contained_in![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))]
    )
}

#[google_test]
fn is_contained_in_matches_hash_map_with_trailing_comma() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from_iter([(1, "One"), (2, "Two")]);
    verify_that!(
        value,
        is_contained_in![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three")),]
    )
}

#[google_test]
fn is_contained_in_matches_when_container_is_empty() -> Result<()> {
    verify_that!(vec![], is_contained_in!(eq(2), eq(3), eq(4)))
}

#[google_test]
fn is_contained_in_matches_when_excess_matchers_present() -> Result<()> {
    verify_that!(vec![3, 4], is_contained_in!(eq(2), eq(3), eq(4)))
}

#[google_test]
fn is_contained_in_does_not_match_when_elements_are_unmatched() -> Result<()> {
    verify_that!(vec![1, 2, 3], not(is_contained_in!(eq(2), eq(3), eq(4))))
}

#[google_test]
fn is_contained_in_explains_mismatch_due_to_wrong_size() -> Result<()> {
    verify_that!(
        is_contained_in![eq(2), eq(3)].explain_match(&vec![2, 3, 4]),
        displays_as(eq("which has size 3 (expected at most 2)"))
    )
}

#[google_test]
fn is_contained_in_explains_missing_element_in_mismatch() -> Result<()> {
    verify_that!(
        is_contained_in![eq(2), eq(3), eq(4)].explain_match(&vec![1, 2, 3]),
        displays_as(eq("whose element #0 does not match any expected elements"))
    )
}

#[google_test]
fn is_contained_in_explains_missing_elements_in_mismatch() -> Result<()> {
    verify_that!(
        is_contained_in![eq(2), eq(3), eq(4), eq(5)].explain_match(&vec![0, 1, 2, 3]),
        displays_as(eq("whose elements #0, #1 do not match any expected elements"))
    )
}

#[google_test]
fn is_contained_in_explains_mismatch_due_to_no_graph_matching_found() -> Result<()> {
    verify_that!(
        is_contained_in![ge(1), ge(3)].explain_match(&vec![1, 2]),
        displays_as(eq(indoc!(
            "
            which does not have a subset match with the expected elements. The best match found was:
              Actual element 1 at index 0 matched expected element `is greater than or equal to 1` at index 0.
              Actual element 2 at index 1 did not match any remaining expected element.
              Expected element `is greater than or equal to 3` at index 1 did not match any remaining actual element."))
    ))
}
