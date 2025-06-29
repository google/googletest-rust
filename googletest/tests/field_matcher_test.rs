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
struct IntField {
    int: i32,
}

#[test]
fn field_matches_integer_field() -> Result<()> {
    verify_that!(IntField { int: 32 }, field!(&IntField.int, eq(32)))
}

#[derive(Debug)]
struct StringField {
    strink: String,
}

#[test]
fn field_matches_string_field() -> Result<()> {
    verify_that!(
        StringField { strink: "yes".to_string() },
        field!(&StringField.strink, ref eq("yes"))
    )
}

#[test]
fn field_error_message_shows_field_name_and_inner_matcher() -> Result<()> {
    let matcher = field!(&IntField.int, eq(31));

    verify_that!(
        matcher.describe(MatcherResult::Match),
        displays_as(eq("has field `int`, which is equal to 31"))
    )
}

mod sub {
    #[derive(Debug)]
    pub struct SubStruct {
        pub field: i32,
    }
}

#[test]
fn struct_in_other_module_matches() -> Result<()> {
    verify_that!(sub::SubStruct { field: 32 }, field!(&sub::SubStruct.field, eq(32)))
}

#[derive(Debug)]
struct Tuple(i32, #[allow(unused)] String);

#[test]
fn tuple_matches_with_index() -> Result<()> {
    verify_that!(Tuple(32, "yes".to_string()), field!(&Tuple.0, eq(32)))
}

#[test]
fn matches_enum_value() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        AValue(u32),
    }
    let value = AnEnum::AValue(123);

    verify_that!(value, field!(&AnEnum::AValue.0, eq(123)))
}

#[test]
fn shows_correct_failure_message_for_wrong_struct_entry() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a: Vec<u32>,
    }
    let value = AStruct { a: vec![1] };

    let result = verify_that!(value, field!(&AStruct.a, ref container_eq([])));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "which has field `a`, which contains the unexpected element 1"
        )))
    )
}

#[test]
fn does_not_match_enum_value_with_wrong_enum_variant() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(dead_code)] // This variant is intentionally unused.
        AValue(u32),
        AnotherValue,
    }
    let value = AnEnum::AnotherValue;

    verify_that!(&value, not(field!(&AnEnum::AValue.0, eq(123))))
}

#[test]
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

    let result = verify_that!(value, field!(&AnEnum::AValue.a, eq(123)));

    verify_that!(
        result,
        err(displays_as(contains_substring("which has the wrong enum variant `AnotherValue`")))
    )
}

#[test]
fn shows_correct_failure_message_for_wrong_enum_value_with_tuple_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(dead_code)] // This variant is intentionally unused.
        AValue(u32),
        #[allow(unused)]
        AnotherValue(u32),
    }
    let value = AnEnum::AnotherValue(123);

    let result = verify_that!(value, field!(&AnEnum::AValue.0, eq(123)));

    verify_that!(
        result,
        err(displays_as(contains_substring("which has the wrong enum variant `AnotherValue`")))
    )
}

#[test]
fn shows_correct_failure_message_for_wrong_enum_value_with_named_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(dead_code)] // This variant is intentionally unused.
        AValue(u32),
        AnotherValue {
            #[allow(unused)]
            a: u32,
        },
    }
    let value = AnEnum::AnotherValue { a: 123 };

    let result = verify_that!(value, field!(&AnEnum::AValue.0, eq(123)));

    verify_that!(
        result,
        err(displays_as(contains_substring("which has the wrong enum variant `AnotherValue`")))
    )
}

#[test]
fn matches_struct_like_enum_value() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        AValue { a_field: u32 },
    }
    let value = AnEnum::AValue { a_field: 123 };

    verify_that!(value, field!(&AnEnum::AValue.a_field, eq(123)))
}

#[test]
fn matches_struct_copy_to_copy() -> Result<()> {
    #[derive(Debug, Clone, Copy)]
    struct Strukt {
        a_field: i32,
    }

    verify_that!(Strukt { a_field: 32 }, field!(Strukt.a_field, eq(32)))
}

#[test]
fn matches_struct_ref_to_copy() -> Result<()> {
    #[derive(Debug)]
    struct Strukt {
        a_field: i32,
    }

    verify_that!(Strukt { a_field: 32 }, field!(&Strukt.a_field, eq(32)))
}

#[test]
fn matches_struct_ref_to_ref() -> Result<()> {
    #[derive(Debug)]
    struct Strukt {
        a_field: String,
    }

    verify_that!(Strukt { a_field: "32".into() }, field!(&Strukt.a_field, ref eq("32")))
}

#[test]
fn matches_struct_copy_to_ref() -> Result<()> {
    // It is not possible to have a copy struct with non-copy field. Hence, this
    // test case is not necessary.
    Ok(())
}

#[test]
fn matches_struct_ref_to_ref_binding_mode() -> Result<()> {
    #[derive(Debug)]
    struct Strukt {
        a_field: String,
    }

    verify_that!(Strukt { a_field: "32".into() }, field!(Strukt.a_field, eq("32")))
}

#[test]
fn matches_struct_with_auto_eq() -> Result<()> {
    #[derive(Debug)]
    struct Strukt {
        a_field: String,
    }

    verify_that!(Strukt { a_field: "32".into() }, field!(Strukt.a_field, "32"))
}

#[test]
fn matches_enum_with_auto_eq() -> Result<()> {
    #[derive(Debug)]
    enum Enum {
        Str(String),
        #[allow(unused)]
        Int(i32),
    }

    verify_that!(Enum::Str("32".into()), field!(Enum::Str.0, "32"))
}

#[test]
fn matches_enum_with_auto_eq_with_wrapper() -> Result<()> {
    #[derive(Debug)]
    struct Wrapper<I> {
        wrapped: I,
    }

    verify_that!(
        Wrapper { wrapped: Wrapper { wrapped: 23 } },
        field!(Wrapper.wrapped, field!(Wrapper.wrapped, &23))
    )
}

#[test]
fn supports_fully_qualified_struct_path() -> Result<()> {
    // Ensure that the macro expands to the fully-qualified struct path.
    mod googletest {}

    let value = ::googletest::internal::test_data::TestStruct { value: 10 };
    verify_that!(value, field!(&::googletest::internal::test_data::TestStruct.value, ref eq(&10)))?;
    verify_that!(value, field!(&::googletest::internal::test_data::TestStruct.value, eq(10)))?;
    verify_that!(value, field!(::googletest::internal::test_data::TestStruct.value, eq(&10)))?;
    Ok(())
}

#[test]
fn supports_generic_struct() -> Result<()> {
    #[derive(Debug)]
    struct Struct<S, T> {
        #[allow(dead_code)] // The property1 field is used only for adding the type parameter.
        property1: S,
        property2: T,
    }

    let value = Struct { property1: 1, property2: 10 };
    verify_that!(&value, field!(&Struct::<i32, u32>.property2, ref eq(&10)))?;
    verify_that!(value, field!(&Struct::<i32, u32>.property2, eq(10)))?;
    verify_that!(value, field!(Struct::<i32, u32>.property2, eq(&10)))?;
    Ok(())
}

#[test]
fn supports_fully_qualified_generic_struct() -> Result<()> {
    // Ensure that the macro expands to the fully-qualified struct path.
    mod googletest {}

    let value = ::googletest::internal::test_data::GenericTestStruct { value: 10 };
    verify_that!(
        &value,
        field!(&::googletest::internal::test_data::GenericTestStruct::<i32>.value, ref eq(&10))
    )?;
    verify_that!(
        value,
        field!(&::googletest::internal::test_data::GenericTestStruct::<i32>.value, eq(10))
    )?;
    verify_that!(
        value,
        field!(::googletest::internal::test_data::GenericTestStruct::<i32>.value, eq(&10))
    )?;
    Ok(())
}
