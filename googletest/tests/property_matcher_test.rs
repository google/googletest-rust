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

use googletest::matcher::MatcherResult;
use googletest::prelude::*;
use googletest::Result;

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

    fn generic_method<U, V>(&self, _a: U, _b: V) -> u32 {
        self.a_property
    }

    fn generic_method_ref<U, V>(&self, _a: U, _b: V) -> &u32 {
        &self.a_property
    }
}

#[test]
fn matches_struct_with_matching_property() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(&SomeStruct.get_property(), eq(10)))
}

#[test]
fn matches_struct_with_matching_property_with_parameters() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(&SomeStruct.add_product_to_field(2, 3), eq(16)))
}

#[test]
fn matches_struct_with_matching_property_with_parameters_with_trailing_comma() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(&SomeStruct.add_product_to_field(2, 3,), eq(16)))
}

#[test]
fn matches_struct_with_matching_property_ref() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(&SomeStruct.get_property_ref(), eq(&10)))
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
    verify_that!(value, property!(&StructWithString.get_property_ref(), eq("Something")))
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
    verify_that!(value, property!(&StructWithVec.get_property_ref(), eq([1, 2, 3])))
}

#[test]
fn matches_struct_with_matching_property_ref_with_parameters() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(&SomeStruct.get_property_ref_with_params(2, 3), eq(&10)))
}

#[test]
fn matches_struct_with_matching_property_ref_with_parameters_and_trailing_comma() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(&SomeStruct.get_property_ref_with_params(2, 3,), eq(&10)))
}

#[test]
fn does_not_match_struct_with_non_matching_property() -> Result<()> {
    let value = SomeStruct { a_property: 2 };
    verify_that!(value, not(property!(&SomeStruct.get_property(), eq(1))))
}

#[test]
fn supports_fully_qualified_struct_path() -> Result<()> {
    // Ensure that the macro expands to the fully-qualified struct path.
    mod googletest {}

    let value = ::googletest::internal::test_data::TestStruct { value: 10 };
    verify_that!(
        value,
        property!(::googletest::internal::test_data::TestStruct.get_value(), eq(&10))
    )?;
    verify_that!(
        value,
        property!(&::googletest::internal::test_data::TestStruct.get_value(), eq(10))
    )?;
    verify_that!(
        value,
        property!(&::googletest::internal::test_data::TestStruct.get_value(), ref eq(&10))
    )?;
    Ok(())
}

#[test]
fn supports_generic_struct_matches() -> Result<()> {
    #[derive(Debug)]
    struct GenericStruct<S, T> {
        #[allow(dead_code)] // The property1 field is used only for adding the type parameter.
        property1: S,
        property2: T,
    }

    impl<S, T: Copy> GenericStruct<S, T> {
        fn get_property(&self) -> T {
            self.property2
        }

        fn get_property_ref(&self) -> &T {
            &self.property2
        }
    }

    let value = GenericStruct { property1: 1, property2: 10 };
    verify_that!(value, property!(GenericStruct::<i32, u32>.get_property(), eq(&10)))?;
    verify_that!(value, property!(&GenericStruct::<i32, u32>.get_property(), eq(10)))?;
    verify_that!(value, property!(&GenericStruct::<i32, u32>.get_property(), ref eq(&10)))?;
    verify_that!(value, property!(&GenericStruct::<i32, u32>.get_property_ref(), eq(&10)))?;
    verify_that!(value, property!(&GenericStruct::<i32, u32>.get_property_ref(), ref eq(&&10)))?;
    Ok(())
}

#[test]
fn supports_generic_method_matches() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, property!(SomeStruct.generic_method::<i32, i16>(1, 2), eq(&10)))?;
    verify_that!(value, property!(&SomeStruct.generic_method::<i32, i16>(1, 2), eq(10)))?;
    verify_that!(value, property!(&SomeStruct.generic_method::<i32, i16>(1, 2), ref eq(&10)))?;
    verify_that!(value, property!(&SomeStruct.generic_method_ref::<i32, i16>(1, 2), eq(&10)))?;
    verify_that!(value, property!(&SomeStruct.generic_method_ref::<i32, i16>(1, 2), ref eq(&&10)))?;
    Ok(())
}

#[test]
fn supports_fully_qualified_generic_struct_path() -> Result<()> {
    // Ensure that the macro expands to the fully-qualified struct path.
    mod googletest {}

    let value = ::googletest::internal::test_data::GenericTestStruct { value: 1 };
    verify_that!(
        value,
        property!(
            ::googletest::internal::test_data::GenericTestStruct::<i32>.get_value::<i16>(10),
            eq(&10)
        )
    )?;
    verify_that!(
        value,
        property!(
            &::googletest::internal::test_data::GenericTestStruct::<i32>.get_value::<i16>(10),
            eq(10)
        )
    )?;
    verify_that!(
        value,
        property!(
            &::googletest::internal::test_data::GenericTestStruct::<i32>.get_value::<i16>(10),
            ref eq(&10))
    )?;
    Ok(())
}

#[test]
fn describes_itself_in_matching_case() -> Result<()> {
    verify_that!(
        property!(&SomeStruct.get_property(), eq(1)).describe(MatcherResult::Match),
        displays_as(eq("has property `get_property()`, which is equal to 1"))
    )
}

#[test]
fn describes_itself_in_not_matching_case() -> Result<()> {
    verify_that!(
        property!(&SomeStruct.get_property(), eq(1)).describe(MatcherResult::NoMatch),
        displays_as(eq("has property `get_property()`, which isn't equal to 1"))
    )
}

#[test]
fn explains_mismatch_referencing_explanation_of_inner_matcher() -> Result<()> {
    #[derive(Debug)]
    struct SomeStruct;

    impl SomeStruct {
        fn get_a_collection(&self) -> Vec<u32> {
            vec![]
        }
    }
    let value = SomeStruct;
    let result =
        verify_that!(value, property!(&SomeStruct.get_a_collection(), ref container_eq([1])));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "whose property `get_a_collection()` is `[]`, which is missing the element 1"
        )))
    )
}

#[test]
fn explains_mismatch_referencing_explanation_of_inner_matcher_binding_mode() -> Result<()> {
    #[derive(Debug)]
    struct SomeStruct;
    impl SomeStruct {
        fn get_a_collection(&self) -> Vec<u32> {
            vec![]
        }
    }
    let result =
        verify_that!(SomeStruct, property!(SomeStruct.get_a_collection(), container_eq([1])));

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
        property!(&SomeStruct.get_property_ref(), eq(&1)).describe(MatcherResult::Match),
        displays_as(eq("has property `get_property_ref()`, which is equal to 1"))
    )
}

#[test]
fn describes_itself_in_not_matching_case_for_ref() -> Result<()> {
    verify_that!(
        property!(&SomeStruct.get_property_ref(), eq(&1)).describe(MatcherResult::NoMatch),
        displays_as(eq("has property `get_property_ref()`, which isn't equal to 1"))
    )
}

#[test]
fn explains_mismatch_referencing_explanation_of_inner_matcher_for_ref() -> Result<()> {
    static EMPTY_COLLECTION: Vec<u32> = vec![];
    #[derive(Debug)]
    struct SomeStruct;
    impl SomeStruct {
        fn get_a_collection_ref(&self) -> &[u32] {
            &EMPTY_COLLECTION
        }
    }
    let value = SomeStruct;
    let result =
        verify_that!(value, property!(&SomeStruct.get_a_collection_ref(), container_eq([1])));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "whose property `get_a_collection_ref()` is `[]`, which is missing the element 1"
        )))
    )
}

#[test]
fn matches_copy_to_copy() -> Result<()> {
    #[derive(Debug, Clone, Copy)]
    struct Struct;
    impl Struct {
        fn property(self) -> i32 {
            32
        }
    }

    verify_that!(Struct, property!(Struct.property(), eq(32)))
}

#[test]
fn matches_copy_to_ref() -> Result<()> {
    #[derive(Debug, Clone, Copy)]
    struct Struct;
    impl Struct {
        fn property(self) -> String {
            "something".into()
        }
    }

    verify_that!(Struct, property!(Struct.property(), ref eq("something")))
}

#[test]
fn matches_copy_but_by_ref() -> Result<()> {
    #[derive(Debug, Clone, Copy)]
    struct Struct;
    impl Struct {
        fn property(&self) -> String {
            "something".into()
        }
    }

    verify_that!(&Struct, property!(&Struct.property(), ref eq("something")))
}

#[test]
fn matches_ref_to_ref() -> Result<()> {
    #[derive(Debug)]
    struct Struct;
    impl Struct {
        fn property(&self) -> String {
            "something".into()
        }
    }

    verify_that!(Struct, property!(&Struct.property(), ref eq("something")))
}

#[test]
fn matches_ref_to_copy() -> Result<()> {
    #[derive(Debug)]
    struct Struct;
    impl Struct {
        fn property(&self) -> i32 {
            32
        }
    }

    verify_that!(Struct, property!(&Struct.property(), eq(32)))
}

#[test]
fn matches_ref_to_ref_with_binding_mode() -> Result<()> {
    #[derive(Debug)]
    struct Struct;
    impl Struct {
        fn property(&self) -> String {
            "something".into()
        }
    }

    verify_that!(Struct, property!(Struct.property(), eq("something")))
}

#[test]
fn matches_property_auto_eq() -> Result<()> {
    #[derive(Debug)]
    struct Struct;
    impl Struct {
        fn property(&self) -> String {
            "something".into()
        }
    }

    verify_that!(Struct, property!(Struct.property(), "something"))
}
