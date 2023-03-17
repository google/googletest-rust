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

#[cfg(not(google3))]
use googletest::field;
use googletest::matcher::{Matcher, MatcherResult};
#[cfg(not(google3))]
use googletest::matchers;
use googletest::{google_test, verify_that, Result};
#[cfg(google3)]
use matchers::field;
use matchers::{container_eq, contains_substring, displays_as, eq, err, not};

#[derive(Debug)]
struct IntField {
    int: i32,
}

#[google_test]
fn field_matches_integer_field() -> Result<()> {
    verify_that!(IntField { int: 32 }, field!(IntField.int, eq(32)))
}

#[derive(Debug)]
struct StringField {
    strink: String,
}

#[google_test]
fn field_matches_string_field() -> Result<()> {
    verify_that!(StringField { strink: "yes".to_string() }, field!(StringField.strink, eq("yes")))
}

#[google_test]
fn field_error_message_shows_field_name_and_inner_matcher() -> Result<()> {
    let matcher = field!(IntField.int, eq(31));

    verify_that!(
        matcher.describe(MatcherResult::Matches),
        eq("has field `int`, which is equal to 31")
    )
}

mod sub {
    #[derive(Debug)]
    pub struct SubStruct {
        pub field: i32,
    }
}

#[google_test]
fn struct_in_other_module_matches() -> Result<()> {
    verify_that!(sub::SubStruct { field: 32 }, field!(sub::SubStruct.field, eq(32)))
}

#[derive(Debug)]
struct Tuple(i32, String);

#[google_test]
fn tuple_matches_with_index() -> Result<()> {
    verify_that!(Tuple(32, "yes".to_string()), field!(Tuple.0, eq(32)))
}

#[google_test]
fn matches_enum_value() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        AValue(u32),
    }
    let value = AnEnum::AValue(123);

    verify_that!(value, field!(AnEnum::AValue.0, eq(123)))
}

#[google_test]
fn shows_correct_failure_message_for_wrong_struct_entry() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a: Vec<u32>,
    }
    let value = AStruct { a: vec![1] };

    let result = verify_that!(value, field!(AStruct.a, container_eq([])));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "which has field `a`, which contains the unexpected element 1"
        )))
    )
}

#[google_test]
fn does_not_match_enum_value_with_wrong_enum_variant() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(dead_code)] // This variant is intentionally unused.
        AValue(u32),
        AnotherValue,
    }
    let value = AnEnum::AnotherValue;

    verify_that!(value, not(field!(AnEnum::AValue.0, eq(123))))
}

#[google_test]
fn shows_correct_failure_message_for_wrong_enum_value() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(dead_code)] // This variant is intentionally unused.
        AValue {
            a: u32,
        },
        AnotherValue,
    }
    let value = AnEnum::AnotherValue;

    let result = verify_that!(value, field!(AnEnum::AValue.a, eq(123)));

    verify_that!(result, err(displays_as(contains_substring("which has no field `a`"))))
}

#[google_test]
fn matches_struct_like_enum_value() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        AValue { a_field: u32 },
    }
    let value = AnEnum::AValue { a_field: 123 };

    verify_that!(value, field!(AnEnum::AValue.a_field, eq(123)))
}
