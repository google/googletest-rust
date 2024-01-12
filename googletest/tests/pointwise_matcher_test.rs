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

use googletest::prelude::*;
use indoc::indoc;

#[test]
fn pointwise_matches_single_element() -> Result<()> {
    let value = vec![1];
    verify_that!(value, pointwise!(lt, vec![2]))
}

#[test]
fn pointwise_matches_two_elements() -> Result<()> {
    let value = vec![1, 2];
    verify_that!(value, pointwise!(lt, vec![2, 3]))
}

#[test]
fn pointwise_matches_two_elements_with_array() -> Result<()> {
    let value = vec![1, 2];
    verify_that!(value, pointwise!(lt, [2, 3]))
}

#[test]
fn pointwise_matches_two_element_slice() -> Result<()> {
    let value = vec![1, 2];
    let slice = value.as_slice();
    verify_that!(*slice, pointwise!(lt, [2, 3]))
}

#[test]
fn pointwise_does_not_match_value_of_wrong_length() -> Result<()> {
    let value = vec![1];
    verify_that!(value, not(pointwise!(lt, vec![2, 3])))
}

#[test]
fn pointwise_does_not_match_value_not_matching_in_first_position() -> Result<()> {
    let value = vec![1, 2];
    verify_that!(value, not(pointwise!(lt, vec![1, 3])))
}

#[test]
fn pointwise_does_not_match_value_not_matching_in_second_position() -> Result<()> {
    let value = vec![1, 2];
    verify_that!(value, not(pointwise!(lt, vec![2, 2])))
}

#[test]
fn pointwise_allows_qualified_matcher_name() -> Result<()> {
    mod submodule {
        pub(super) use super::lt;
    }
    let value = vec![1];
    verify_that!(value, pointwise!(submodule::lt, vec![2]))
}

#[test]
fn pointwise_returns_mismatch_when_actual_value_has_wrong_length() -> Result<()> {
    let result = verify_that!(vec![1, 2, 3], pointwise!(eq, vec![1, 2]));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: vec![1, 2, 3]
            Expected: has elements satisfying respectively:
              0. is equal to 1
              1. is equal to 2
            Actual: [1, 2, 3],
              which has size 3 (expected 2)
            "
        ))))
    )
}

#[test]
fn pointwise_returns_mismatch_when_actual_value_does_not_match_on_first_item() -> Result<()> {
    let result = verify_that!(vec![1, 2, 3], pointwise!(eq, vec![2, 2, 3]));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: vec![1, 2, 3]
            Expected: has elements satisfying respectively:
              0. is equal to 2
              1. is equal to 2
              2. is equal to 3
            Actual: [1, 2, 3],
              where element #0 is 1, which isn't equal to 2
            "
        ))))
    )
}

#[test]
fn pointwise_returns_mismatch_when_actual_value_does_not_match_on_second_item() -> Result<()> {
    let result = verify_that!(vec![1, 2, 3], pointwise!(eq, vec![1, 3, 3]));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: vec![1, 2, 3]
            Expected: has elements satisfying respectively:
              0. is equal to 1
              1. is equal to 3
              2. is equal to 3
            Actual: [1, 2, 3],
              where element #1 is 2, which isn't equal to 3
            "
        ))))
    )
}

#[test]
fn pointwise_returns_mismatch_when_actual_value_does_not_match_on_first_and_second_items()
-> Result<()> {
    let result = verify_that!(vec![1, 2, 3], pointwise!(eq, vec![2, 3, 3]));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: vec![1, 2, 3]
            Expected: has elements satisfying respectively:
              0. is equal to 2
              1. is equal to 3
              2. is equal to 3
            Actual: [1, 2, 3],
              where:
                * element #0 is 1, which isn't equal to 2
                * element #1 is 2, which isn't equal to 3"
        ))))
    )
}

#[test]
fn pointwise_matches_single_element_with_lambda_expression_with_extra_value() -> Result<()> {
    let value = vec![1.00001f32];
    verify_that!(value, pointwise!(|v| near(v, 0.0001), vec![1.0]))
}

#[test]
fn pointwise_matches_single_element_with_two_containers() -> Result<()> {
    let value = vec![1.00001f32];
    verify_that!(value, pointwise!(near, vec![1.0], vec![0.0001]))
}

#[test]
fn pointwise_matches_single_element_with_three_containers() -> Result<()> {
    let value = vec![1.00001f32];
    verify_that!(
        value,
        pointwise!(|v, t, u| near(v, t * u), vec![1.0f32], vec![0.0001f32], vec![0.5f32])
    )
}
