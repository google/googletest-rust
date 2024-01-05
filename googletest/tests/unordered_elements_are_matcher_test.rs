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
use googletest::prelude::*;
use indoc::indoc;
use std::collections::HashMap;

#[test]
fn unordered_elements_are_matches_empty_vector() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, unordered_elements_are![])
}

#[test]
fn unordered_elements_are_matches_empty_vector_with_trailing_comma() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, unordered_elements_are![,])
}

#[test]
fn unordered_elements_are_matches_vector() -> Result<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, unordered_elements_are![eq(1), eq(2), eq(3)])
}

#[test]
fn unordered_elements_are_omitted() -> Result<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, {eq(3), eq(2), eq(1)})
}

#[test]
fn unordered_elements_are_matches_slice() -> Result<()> {
    let value = vec![1, 2, 3];
    let slice = value.as_slice();
    verify_that!(*slice, unordered_elements_are![eq(1), eq(2), eq(3)])
}

#[test]
fn unordered_elements_are_matches_hash_map() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]);
    verify_that!(
        value,
        unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))]
    )
}

#[test]
fn unordered_elements_are_matches_hash_map_with_trailing_comma() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]);
    verify_that!(
        value,
        unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three")),]
    )
}

#[test]
fn unordered_elements_are_does_not_match_hash_map_with_wrong_key() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Two"), (4, "Three")]);
    verify_that!(
        value,
        not(unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))])
    )
}

#[test]
fn unordered_elements_are_does_not_match_hash_map_with_wrong_value() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Two"), (3, "Four")]);
    verify_that!(
        value,
        not(unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))])
    )
}

#[test]
fn unordered_elements_are_does_not_match_hash_map_missing_element() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Two")]);
    verify_that!(
        value,
        not(unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))])
    )
}

#[test]
fn unordered_elements_are_does_not_match_hash_map_with_extra_element() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]);
    verify_that!(value, not(unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One"))]))
}

#[test]
fn unordered_elements_are_does_not_match_hash_map_with_mismatched_key_and_value() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Three"), (3, "Two")]);
    verify_that!(
        value,
        not(unordered_elements_are![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))])
    )
}

#[test]
fn unordered_elements_are_matches_vector_with_trailing_comma() -> Result<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, unordered_elements_are![eq(1), eq(2), eq(3),])
}

#[test]
fn unordered_elements_are_matches_size() -> Result<()> {
    let value = vec![1, 2];
    verify_that!(value, not(unordered_elements_are![eq(1), eq(2), eq(3)]))
}

#[test]
fn unordered_elements_are_admits_matchers_without_static_lifetime() -> Result<()> {
    #[derive(Debug, PartialEq)]
    struct AStruct(i32);
    let expected_value = AStruct(123);
    verify_that!(vec![AStruct(123)], unordered_elements_are![eq_deref_of(&expected_value)])
}

#[test]
fn unordered_elements_are_with_map_admits_matchers_without_static_lifetime() -> Result<()> {
    #[derive(Debug, PartialEq)]
    struct AStruct(i32);
    let expected_value = AStruct(123);
    verify_that!(
        HashMap::from([(1, AStruct(123))]),
        unordered_elements_are![(eq(1), eq_deref_of(&expected_value))]
    )
}

#[test]
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
            Actual: [1, 4, 3],
              whose element #1 does not match any expected elements and no elements match the expected element #1"
            ))))
    )
}

#[test]
fn unordered_elements_are_matches_unordered() -> Result<()> {
    let value = vec![1, 2];
    verify_that!(value, unordered_elements_are![eq(2), eq(1)])
}

#[test]
fn unordered_elements_are_matches_unordered_with_repetition() -> Result<()> {
    let value = vec![1, 2, 1, 2, 1];
    verify_that!(value, unordered_elements_are![eq(1), eq(1), eq(1), eq(2), eq(2)])
}

#[test]
fn unordered_elements_are_explains_mismatch_due_to_wrong_size() -> Result<()> {
    verify_that!(
        unordered_elements_are![eq(2), eq(3), eq(4)].explain_match(&vec![2, 3]),
        displays_as(eq("which has size 2 (expected 3)"))
    )
}

#[test]
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

#[test]
fn unordered_elements_are_unmatchable_expected_description_mismatch() -> Result<()> {
    verify_that!(
        unordered_elements_are![eq(1), eq(2), eq(3)].explain_match(&vec![1, 1, 3]),
        displays_as(eq("which has no element matching the expected element #1"))
    )
}

#[test]
fn unordered_elements_are_unmatchable_actual_description_mismatch() -> Result<()> {
    verify_that!(
        unordered_elements_are![eq(1), eq(1), eq(3)].explain_match(&vec![1, 2, 3]),
        displays_as(eq("whose element #1 does not match any expected elements"))
    )
}

fn create_matcher() -> impl Matcher<ActualT = Vec<i32>> {
    unordered_elements_are![eq(1)]
}

#[test]
fn unordered_elements_are_works_when_matcher_is_created_in_subroutine() -> Result<()> {
    verify_that!(vec![1], create_matcher())
}

fn create_matcher_for_map() -> impl Matcher<ActualT = HashMap<i32, i32>> {
    unordered_elements_are![(eq(1), eq(1))]
}

#[test]
fn unordered_elements_are_works_when_matcher_for_maps_is_created_in_subroutine() -> Result<()> {
    verify_that!(HashMap::from([(1, 1)]), create_matcher_for_map())
}

#[test]
fn contains_each_matches_when_one_to_one_correspondence_present() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!(eq(2), eq(3), eq(4)))
}

#[test]
fn contains_each_supports_trailing_comma() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!(eq(2), eq(3), eq(4),))
}

#[test]
fn contains_each_matches_hash_map() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]);
    verify_that!(value, contains_each![(eq(2), eq("Two")), (eq(1), eq("One"))])
}

#[test]
fn contains_each_matches_hash_map_with_trailing_comma() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]);
    verify_that!(value, contains_each![(eq(2), eq("Two")), (eq(1), eq("One")),])
}

#[test]
fn contains_each_matches_when_no_matchers_present() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!())
}

#[test]
fn contains_each_matches_when_no_matchers_present_and_trailing_comma() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!(,))
}

#[test]
fn contains_each_matches_when_list_is_empty_and_no_matchers_present() -> Result<()> {
    verify_that!(Vec::<u32>::new(), contains_each!())
}

#[test]
fn contains_each_matches_when_excess_elements_present() -> Result<()> {
    verify_that!(vec![1, 2, 3, 4], contains_each!(eq(2), eq(3), eq(4)))
}

#[test]
fn contains_each_does_not_match_when_matchers_are_unmatched() -> Result<()> {
    verify_that!(vec![1, 2, 3], not(contains_each!(eq(2), eq(3), eq(4))))
}

#[test]
fn contains_each_explains_mismatch_due_to_wrong_size() -> Result<()> {
    verify_that!(
        contains_each![eq(2), eq(3), eq(4)].explain_match(&vec![2, 3]),
        displays_as(eq("which has size 2 (expected at least 3)"))
    )
}

#[test]
fn contains_each_explains_missing_element_in_mismatch() -> Result<()> {
    verify_that!(
        contains_each![eq(2), eq(3), eq(4)].explain_match(&vec![1, 2, 3]),
        displays_as(eq("which has no element matching the expected element #2"))
    )
}

#[test]
fn contains_each_explains_missing_elements_in_mismatch() -> Result<()> {
    verify_that!(
        contains_each![eq(2), eq(3), eq(4), eq(5)].explain_match(&vec![0, 1, 2, 3]),
        displays_as(eq("which has no elements matching the expected elements #2, #3"))
    )
}

#[test]
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

#[test]
fn is_contained_in_matches_with_empty_vector() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, is_contained_in!())
}

#[test]
fn is_contained_in_matches_with_empty_vector_and_trailing_comma() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, is_contained_in!(,))
}

#[test]
fn is_contained_in_matches_when_one_to_one_correspondence_present() -> Result<()> {
    verify_that!(vec![2, 3, 4], is_contained_in!(eq(2), eq(3), eq(4)))
}

#[test]
fn is_contained_supports_trailing_comma() -> Result<()> {
    verify_that!(vec![2, 3, 4], is_contained_in!(eq(2), eq(3), eq(4),))
}

#[test]
fn is_contained_in_matches_hash_map() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Two")]);
    verify_that!(
        value,
        is_contained_in![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three"))]
    )
}

#[test]
fn is_contained_in_matches_hash_map_with_trailing_comma() -> Result<()> {
    let value: HashMap<u32, &'static str> = HashMap::from([(1, "One"), (2, "Two")]);
    verify_that!(
        value,
        is_contained_in![(eq(2), eq("Two")), (eq(1), eq("One")), (eq(3), eq("Three")),]
    )
}

#[test]
fn is_contained_in_matches_when_container_is_empty() -> Result<()> {
    verify_that!(vec![], is_contained_in!(eq::<i32, _>(2), eq(3), eq(4)))
}

#[test]
fn is_contained_in_matches_when_excess_matchers_present() -> Result<()> {
    verify_that!(vec![3, 4], is_contained_in!(eq(2), eq(3), eq(4)))
}

#[test]
fn is_contained_in_does_not_match_when_elements_are_unmatched() -> Result<()> {
    verify_that!(vec![1, 2, 3], not(is_contained_in!(eq(2), eq(3), eq(4))))
}

#[test]
fn is_contained_in_explains_mismatch_due_to_wrong_size() -> Result<()> {
    verify_that!(
        is_contained_in![eq(2), eq(3)].explain_match(&vec![2, 3, 4]),
        displays_as(eq("which has size 3 (expected at most 2)"))
    )
}

#[test]
fn is_contained_in_explains_missing_element_in_mismatch() -> Result<()> {
    verify_that!(
        is_contained_in![eq(2), eq(3), eq(4)].explain_match(&vec![1, 2, 3]),
        displays_as(eq("whose element #0 does not match any expected elements"))
    )
}

#[test]
fn is_contained_in_explains_missing_elements_in_mismatch() -> Result<()> {
    verify_that!(
        is_contained_in![eq(2), eq(3), eq(4), eq(5)].explain_match(&vec![0, 1, 2, 3]),
        displays_as(eq("whose elements #0, #1 do not match any expected elements"))
    )
}

#[test]
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
