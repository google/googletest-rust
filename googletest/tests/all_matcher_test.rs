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
fn matches_any_value_when_list_is_empty() -> Result<()> {
    verify_that!((), all!())
}

#[test]
fn matches_value_with_single_matching_component() -> Result<()> {
    verify_that!(123, all!(eq(123)))
}

#[test]
fn does_not_match_value_with_single_non_matching_component() -> Result<()> {
    verify_that!(123, not(all!(eq(456))))
}

#[test]
fn matches_value_with_two_matching_components() -> Result<()> {
    verify_that!("A string", all!(starts_with("A"), ends_with("string")))
}

#[test]
fn does_not_match_value_with_one_non_matching_component_among_two_components() -> Result<()> {
    verify_that!(123, not(all!(eq(123), eq(456))))
}

#[test]
fn supports_trailing_comma() -> Result<()> {
    verify_that!(
        "An important string",
        all!(starts_with("An"), contains_substring("important"), ends_with("string"),)
    )
}

#[test]
fn mismatch_description_two_failed_matchers() -> Result<()> {
    verify_that!(
        all!(starts_with("One"), starts_with("Two")).explain_match("Three"),
        displays_as(eq(
            "\n  * which does not start with \"One\"\n  * which does not start with \"Two\""
        ))
    )
}

#[test]
fn mismatch_description_empty_matcher() -> Result<()> {
    verify_that!(all!().explain_match("Three"), displays_as(eq("which is anything")))
}

#[test]
fn all_multiple_failed_assertions() -> Result<()> {
    let result = verify_that!(4, all![eq(1), eq(2), eq(3)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: 4
            Expected: has all the following properties:
              * is equal to 1
              * is equal to 2
              * is equal to 3
            Actual: 4, 
              * which isn't equal to 1
              * which isn't equal to 2
              * which isn't equal to 3"
        ))))
    )
}
