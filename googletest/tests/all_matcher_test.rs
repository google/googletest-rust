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
fn admits_matchers_without_static_lifetime() -> Result<()> {
    #[derive(Debug, PartialEq)]
    struct AStruct(i32);
    let expected_value = AStruct(123);
    verify_that!(AStruct(123), all![eq_deref_of(&expected_value)])
}

#[test]
fn mismatch_description_two_failed_matchers() -> Result<()> {
    verify_that!(
        all!(starts_with("One"), starts_with("Two")).explain_match("Three"),
        displays_as(eq("* which does not start with \"One\"\n* which does not start with \"Two\""))
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

#[test]
fn formats_error_message_correctly_when_all_is_inside_some() -> Result<()> {
    let value = Some(4);
    let result = verify_that!(value, some(all![eq(1), eq(2), eq(3)]));
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: value
            Expected: has a value which has all the following properties:
              * is equal to 1
              * is equal to 2
              * is equal to 3
            Actual: Some(4),
              which has a value
                * which isn't equal to 1
                * which isn't equal to 2
                * which isn't equal to 3"
        ))))
    )
}

#[test]
fn formats_error_message_correctly_when_all_is_inside_ok() -> Result<()> {
    let value: std::result::Result<i32, std::io::Error> = Ok(4);
    let result = verify_that!(value, ok(all![eq(1), eq(2), eq(3)]));
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: value
            Expected: is a success containing a value, which has all the following properties:
              * is equal to 1
              * is equal to 2
              * is equal to 3
            Actual: Ok(4),
              which is a success
                * which isn't equal to 1
                * which isn't equal to 2
                * which isn't equal to 3"
        ))))
    )
}

#[test]
fn formats_error_message_correctly_when_all_is_inside_err() -> Result<()> {
    let value: std::result::Result<(), &'static str> = Err("An error");
    let result = verify_that!(value, err(all![starts_with("Not"), ends_with("problem")]));
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: value
            Expected: is an error which has all the following properties:
              * starts with prefix "Not"
              * ends with suffix "problem"
            Actual: Err("An error"),
              which is an error
                * which does not start with "Not"
                * which does not end with "problem""#
        ))))
    )
}
