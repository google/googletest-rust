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

use googletest::matcher::{Matcher, MatcherResult};
use googletest::prelude::*;

#[derive(Debug)]
struct SomeStruct {
    a_property: u32,
}

impl SomeStruct {
    fn get_property(&self) -> u32 {
        self.a_property
    }

    fn get_property_ref(&self) -> &u32 {
        &self.a_property
    }

    fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
        self.a_property + a * b
    }

    fn get_property_ref_with_params(&self, _a: u32, _b: u32) -> &u32 {
        &self.a_property
    }
}

#[test]
fn matches_struct_with_matching_property() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(SomeStruct.get_property(), eq(10)))
}

#[test]
fn matches_struct_with_matching_property_with_parameters() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(SomeStruct.add_product_to_field(2, 3), eq(16)))
}

#[test]
fn matches_struct_with_matching_property_with_captured_arguments() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    let arg1 = 2;
    let arg2 = 3;
    verify_that!(value, property!(SomeStruct.add_product_to_field(arg1, arg2), eq(16)))
}

#[test]
fn matches_struct_with_matching_property_with_parameters_with_trailing_comma() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(SomeStruct.add_product_to_field(2, 3,), eq(16)))
}

#[test]
fn matches_struct_with_matching_property_ref() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(*SomeStruct.get_property_ref(), eq(10)))
}

#[test]
fn matches_struct_with_matching_string_reference_property() -> Result<()> {
    #[derive(Debug)]
    struct StructWithString {
        property: String,
    }
    impl StructWithString {
        fn get_property_ref(&self) -> &String {
            &self.property
        }
    }
    let value = StructWithString { property: "Something".into() };
    verify_that!(value, property!(*StructWithString.get_property_ref(), eq("Something")))
}

#[test]
fn matches_struct_with_matching_slice_property() -> Result<()> {
    #[derive(Debug)]
    struct StructWithVec {
        property: Vec<u32>,
    }
    impl StructWithVec {
        fn get_property_ref(&self) -> &[u32] {
            &self.property
        }
    }
    let value = StructWithVec { property: vec![1, 2, 3] };
    verify_that!(value, property!(*StructWithVec.get_property_ref(), eq([1, 2, 3])))
}

#[test]
fn matches_struct_with_matching_property_ref_with_parameters() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(*SomeStruct.get_property_ref_with_params(2, 3), eq(10)))
}

#[test]
fn matches_struct_with_matching_property_ref_with_parameters_and_trailing_comma() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(*SomeStruct.get_property_ref_with_params(2, 3,), eq(10)))
}

#[test]
fn does_not_match_struct_with_non_matching_property() -> Result<()> {
    let value = SomeStruct { a_property: 2 };
    verify_that!(value, not(property!(SomeStruct.get_property(), eq(1))))
}

#[test]
fn describes_itself_in_matching_case() -> Result<()> {
    verify_that!(
        property!(SomeStruct.get_property(), eq(1)).describe(MatcherResult::Match),
        displays_as(eq("has property `get_property()`, which is equal to 1"))
    )
}

#[test]
fn describes_itself_in_not_matching_case() -> Result<()> {
    verify_that!(
        property!(SomeStruct.get_property(), eq(1)).describe(MatcherResult::NoMatch),
        displays_as(eq("has property `get_property()`, which isn't equal to 1"))
    )
}

#[test]
fn explains_mismatch_referencing_explanation_of_inner_matcher() -> Result<()> {
    impl SomeStruct {
        fn get_a_collection(&self) -> Vec<u32> {
            vec![]
        }
    }
    let value = SomeStruct { a_property: 2 };
    let result = verify_that!(value, property!(SomeStruct.get_a_collection(), container_eq([1])));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "whose property `get_a_collection()` is `[]`, which is missing the element 1"
        )))
    )
}

#[test]
fn describes_itself_in_matching_case_for_ref() -> Result<()> {
    verify_that!(
        property!(*SomeStruct.get_property_ref(), eq(1)).describe(MatcherResult::Match),
        displays_as(eq("has property `get_property_ref()`, which is equal to 1"))
    )
}

#[test]
fn describes_itself_in_not_matching_case_for_ref() -> Result<()> {
    verify_that!(
        property!(*SomeStruct.get_property_ref(), eq(1)).describe(MatcherResult::NoMatch),
        displays_as(eq("has property `get_property_ref()`, which isn't equal to 1"))
    )
}

#[test]
fn explains_mismatch_referencing_explanation_of_inner_matcher_for_ref() -> Result<()> {
    static EMPTY_COLLECTION: Vec<u32> = vec![];
    impl SomeStruct {
        fn get_a_collection_ref(&self) -> &[u32] {
            &EMPTY_COLLECTION
        }
    }
    let value = SomeStruct { a_property: 2 };
    let result =
        verify_that!(value, property!(*SomeStruct.get_a_collection_ref(), container_eq([1])));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "whose property `get_a_collection_ref()` is `[]`, which is missing the element 1"
        )))
    )
}
