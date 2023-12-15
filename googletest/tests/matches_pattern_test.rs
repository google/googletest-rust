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
fn matches_struct_containing_single_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { a_field: eq(123) }))
}

#[test]
fn matches_struct_containing_two_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(actual, matches_pattern!(AStruct { a_field: eq(123), another_field: eq(234) }))
}

#[test]
#[rustfmt::skip]// Otherwise fmt strips the trailing comma
fn supports_trailing_comma_with_one_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct {
        a_field: eq(123), // Block reformatting
    }))
}

#[test]
fn supports_trailing_comma_with_two_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_field: eq(123),
            another_field: eq(234), // Block reformatting
        })
    )
}

#[test]
fn supports_trailing_comma_with_three_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }
    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_field: eq(123),
            another_field: eq(234),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_containing_nested_struct_with_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_nested_struct: ANestedStruct,
    }
    #[derive(Debug)]
    struct ANestedStruct {
        a_field: u32,
    }
    let actual = AStruct { a_nested_struct: ANestedStruct { a_field: 123 } };

    verify_that!(
        actual,
        matches_pattern!(AStruct { a_nested_struct: pat!(ANestedStruct { a_field: eq(123) }) })
    )
}

#[test]
fn has_correct_assertion_failure_message_for_single_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };
    let result = verify_that!(actual, matches_pattern!(AStruct { a_field: eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc! {"
            Value of: actual
            Expected: is AStruct which has field `a_field`, which is equal to 234
            Actual: AStruct { a_field: 123 },
              which has field `a_field`, which isn't equal to 234
            "
        })))
    )
}

#[test]
fn has_correct_assertion_failure_message_for_two_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct { a_field: 123, another_field: 234 };
    let result = verify_that!(
        actual,
        matches_pattern!(AStruct { a_field: eq(234), another_field: eq(123) })
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: actual
            Expected: is AStruct which has all the following properties:
              * has field `a_field`, which is equal to 234
              * has field `another_field`, which is equal to 123
            Actual: AStruct { a_field: 123, another_field: 234 },
              * which has field `a_field`, which isn't equal to 234
              * which has field `another_field`, which isn't equal to 123"
        ))))
    )
}

#[test]
fn has_correct_assertion_failure_message_for_field_and_property() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }
    let actual = AStruct { a_field: 123, another_field: 234 };
    let result = verify_that!(
        actual,
        matches_pattern!(AStruct { get_field(): eq(234), another_field: eq(123) })
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: actual
            Expected: is AStruct which has all the following properties:
              * has property `get_field ()`, which is equal to 234
              * has field `another_field`, which is equal to 123
            Actual: AStruct { a_field: 123, another_field: 234 },
              * whose property `get_field ()` is `123`, which isn't equal to 234
              * which has field `another_field`, which isn't equal to 123"
        ))))
    )
}

#[test]
fn has_meaningful_assertion_failure_message_when_wrong_enum_variant_is_used() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
        #[allow(unused)]
        B(u32),
    }
    let actual = AnEnum::A(123);
    let result = verify_that!(actual, matches_pattern!(AnEnum::B(eq(123))));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc! {"
            Actual: A(123),
              which has the wrong enum variant `A`
            "
        })))
    )
}

#[test]
fn supports_qualified_struct_names() -> Result<()> {
    mod a_module {
        #[derive(Debug)]
        pub(super) struct AStruct {
            pub(super) a_field: u32,
        }
    }
    let actual = a_module::AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(a_module::AStruct { a_field: eq(123) }))
}

#[test]
fn matches_tuple_struct_containing_single_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32);
    let actual = AStruct(123);

    verify_that!(actual, matches_pattern!(AStruct(eq(123))))
}

#[test]
fn matches_tuple_struct_containing_two_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32);
    let actual = AStruct(123, 234);

    verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234))))
}

#[test]
fn matches_tuple_struct_containing_three_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32);
    let actual = AStruct(123, 234, 345);

    verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234), eq(345))))
}

#[test]
fn matches_tuple_struct_containing_four_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456);

    verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456))))
}

#[test]
fn matches_tuple_struct_containing_five_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567);

    verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456), eq(567))))
}

#[test]
fn matches_tuple_struct_containing_six_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678);

    verify_that!(
        actual,
        matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456), eq(567), eq(678)))
    )
}

#[test]
fn matches_tuple_struct_containing_seven_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789);

    verify_that!(
        actual,
        matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456), eq(567), eq(678), eq(789)))
    )
}

#[test]
fn matches_tuple_struct_containing_eight_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789, 890);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678),
            eq(789),
            eq(890)
        ))
    )
}

#[test]
fn matches_tuple_struct_containing_nine_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789, 890, 901);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678),
            eq(789),
            eq(890),
            eq(901)
        ))
    )
}

#[test]
fn matches_tuple_struct_containing_ten_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789, 890, 901, 12);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678),
            eq(789),
            eq(890),
            eq(901),
            eq(12)
        ))
    )
}

#[test]
fn matches_tuple_struct_with_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32);
    let actual = AStruct(123);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123), // Keep the trailing comma, block reformatting
        ))
    )
}

#[test]
fn matches_tuple_struct_with_two_fields_and_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32);
    let actual = AStruct(123, 234);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123),
            eq(234), // Keep the trailing comma, block reformatting
        ))
    )
}

#[test]
fn matches_enum_without_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A,
    }
    let actual = AnEnum::A;

    verify_that!(actual, matches_pattern!(AnEnum::A))
}

#[rustversion::before(1.76)]
const ANENUM_A_REPR: &str = "AnEnum :: A";

#[rustversion::since(1.76)]
const ANENUM_A_REPR: &str = "AnEnum::A";

#[test]
fn generates_correct_failure_output_when_enum_variant_without_field_is_not_matched() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(unused)]
        A,
        B,
    }
    let actual = AnEnum::B;

    let result = verify_that!(actual, matches_pattern!(AnEnum::A));

    verify_that!(result, err(displays_as(contains_substring(format!("is not {ANENUM_A_REPR}")))))
}

#[test]
fn generates_correct_failure_output_when_enum_variant_without_field_is_matched() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A,
    }
    let actual = AnEnum::A;

    let result = verify_that!(actual, not(matches_pattern!(AnEnum::A)));

    verify_that!(result, err(displays_as(contains_substring(format!("is {ANENUM_A_REPR}")))))
}

#[test]
fn matches_enum_with_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
    }
    let actual = AnEnum::A(123);

    verify_that!(actual, matches_pattern!(AnEnum::A(eq(123))))
}

#[test]
fn does_not_match_wrong_enum_value() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(unused)]
        A(u32),
        B,
    }
    let actual = AnEnum::B;

    verify_that!(actual, not(matches_pattern!(AnEnum::A(eq(123)))))
}

#[test]
fn includes_enum_variant_in_description_with_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
    }
    let actual = AnEnum::A(123);

    let result = verify_that!(actual, matches_pattern!(AnEnum::A(eq(234))));

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is {ANENUM_A_REPR} which has field `0`"
        ))))
    )
}

#[test]
fn includes_enum_variant_in_negative_description_with_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
    }
    let actual = AnEnum::A(123);

    let result = verify_that!(actual, not(matches_pattern!(AnEnum::A(eq(123)))));

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is not {ANENUM_A_REPR} which has field `0`, which is equal to"
        ))))
    )
}

#[test]
fn includes_enum_variant_in_description_with_two_fields() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32, u32),
    }
    let actual = AnEnum::A(123, 234);

    let result = verify_that!(actual, matches_pattern!(AnEnum::A(eq(234), eq(234))));

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is {ANENUM_A_REPR} which has all the following properties"
        ))))
    )
}

#[test]
fn includes_enum_variant_in_description_with_three_fields() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32, u32, u32),
    }
    let actual = AnEnum::A(123, 234, 345);

    let result = verify_that!(actual, matches_pattern!(AnEnum::A(eq(234), eq(234), eq(345))));

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is {ANENUM_A_REPR} which has all the following properties"
        ))))
    )
}

#[test]
fn includes_enum_variant_in_description_with_named_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A { field: u32 },
    }
    let actual = AnEnum::A { field: 123 };

    let result = verify_that!(actual, matches_pattern!(AnEnum::A { field: eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is {ANENUM_A_REPR} which has field `field`"
        ))))
    )
}

#[test]
fn includes_enum_variant_in_description_with_two_named_fields() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A { field: u32, another_field: u32 },
    }
    let actual = AnEnum::A { field: 123, another_field: 234 };

    let result = verify_that!(
        actual,
        matches_pattern!(AnEnum::A { field: eq(234), another_field: eq(234) })
    );

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is {ANENUM_A_REPR} which has all the following properties"
        ))))
    )
}

#[test]
fn includes_struct_name_in_description_with_property() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> u32 {
            self.field
        }
    }
    let actual = AStruct { field: 123 };

    let result = verify_that!(actual, matches_pattern!(AStruct { get_field(): eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "Expected: is AStruct which has property `get_field ()`"
        )))
    )
}

#[test]
fn includes_struct_name_in_description_with_ref_property() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> &u32 {
            &self.field
        }
    }
    let actual = AStruct { field: 123 };

    let result = verify_that!(actual, matches_pattern!(AStruct { *get_field(): eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "Expected: is AStruct which has property `get_field ()`"
        )))
    )
}

#[test]
fn includes_struct_name_in_description_with_property_after_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> u32 {
            self.field
        }
    }
    let actual = AStruct { field: 123 };

    let result =
        verify_that!(actual, matches_pattern!(AStruct { field: eq(123), get_field(): eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "Expected: is AStruct which has all the following properties"
        )))
    )
}

#[test]
fn includes_struct_name_in_description_with_ref_property_after_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> &u32 {
            &self.field
        }
    }
    let actual = AStruct { field: 123 };

    let result =
        verify_that!(actual, matches_pattern!(AStruct { field: eq(123), *get_field(): eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "Expected: is AStruct which has all the following properties"
        )))
    )
}

#[test]
fn matches_struct_with_a_method() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { get_field(): eq(123) }))
}

#[test]
fn matches_struct_with_a_method_and_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { get_field(): eq(123), }))
}

#[test]
fn matches_struct_with_a_method_taking_parameter() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn add_to_field(&self, a: u32) -> u32 {
            self.a_field + a
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { add_to_field(2): eq(3) }))
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { add_product_to_field(2, 3): eq(7) }))
}

#[test]
fn matches_struct_with_a_method_taking_enum_value_parameter() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_a_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { get_a_field(AnEnum::AVariant): eq(1) }))
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_with_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { add_product_to_field(2, 3,): eq(7) }))
}

#[test]
fn matches_struct_with_a_method_returning_a_reference() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { *get_field_ref(): eq(123) }))
}

#[test]
fn matches_struct_with_a_method_returning_a_reference_with_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { *get_field_ref(): eq(123), }))
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_ret_ref() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { *get_field_ref(2, 3): eq(1) }))
}

#[test]
fn matches_struct_with_a_method_returning_reference_taking_enum_value_parameter() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { *get_field_ref(AnEnum::AVariant): eq(1) }))
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_with_trailing_comma_ret_ref() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { *get_field_ref(2, 3,): eq(1) }))
}

#[test]
fn matches_struct_with_a_method_followed_by_a_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(actual, matches_pattern!(AStruct { get_field(): eq(123), another_field: eq(234) }))
}

#[test]
fn matches_struct_with_a_method_followed_by_a_field_with_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { get_field(): eq(123), another_field: eq(234), })
    )
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_and_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { add_product_to_field(2, 3): eq(7), another_field: eq(123) })
    )
}

#[test]
fn matches_struct_with_a_method_taking_enum_value_parameter_followed_by_field() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { get_field(AnEnum::AVariant): eq(1), another_field: eq(2) })
    )
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_with_trailing_comma_and_field() -> Result<()>
{
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { add_product_to_field(2, 3,): eq(7), another_field: eq(123) })
    )
}

#[test]
fn matches_struct_with_a_method_returning_reference_followed_by_a_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_field_ref(): eq(123), another_field: eq(234) })
    )
}

#[test]
fn matches_struct_with_a_method_returning_reference_followed_by_a_field_with_trailing_comma()
-> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_field_ref(): eq(123), another_field: eq(234), })
    )
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_ret_ref_and_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_field_ref(2, 3): eq(1), another_field: eq(123) })
    )
}

#[test]
fn matches_struct_with_a_method_taking_enum_value_param_ret_ref_followed_by_field() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_field_ref(AnEnum::AVariant): eq(1), another_field: eq(2) })
    )
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_with_trailing_comma_ret_ref_and_field()
-> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_field_ref(2, 3,): eq(1), another_field: eq(123) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(actual, matches_pattern!(AStruct { another_field: eq(234), get_field(): eq(123) }))
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), get_field(): eq(123), })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), add_product_to_field(2, 3): eq(7) })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(2), get_field(AnEnum::AVariant): eq(1) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_and_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), add_product_to_field(2, 3,): eq(7) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_returning_reference() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), *get_field_ref(): eq(123) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_returning_ref_and_trailing_comma() -> Result<()>
{
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), *get_field_ref(): eq(123), })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_ret_ref() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), *get_field_ref(2, 3): eq(123) })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param_ret_ref() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(2), *get_field_ref(AnEnum::AVariant): eq(1) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_and_trailing_comma_ret_ref()
-> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), *get_field_ref(2, 3,): eq(123) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_followed_by_a_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            get_field(): eq(123),
            a_third_field: eq(345)
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_followed_by_a_field_with_trailing_comma()
-> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            get_field(): eq(123),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_followed_by_a_field() -> Result<()>
{
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            add_product_to_field(2, 3): eq(7),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_and_trailing_comma_followed_by_a_field()
-> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            add_product_to_field(2, 3,): eq(7),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param_followed_by_field()
-> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2, a_third_field: 3 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(2),
            get_field(AnEnum::AVariant): eq(1),
            a_third_field: eq(3),
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_ret_ref_followed_by_a_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            *get_field_ref(): eq(123),
            a_third_field: eq(345)
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_ret_ref_followed_by_a_field_with_trailing_comma()
-> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            *get_field_ref(): eq(123),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_ret_ref_followed_by_a_field()
-> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            *get_field_ref(2, 3): eq(123),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param_ret_ref_followed_by_field()
-> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2, a_third_field: 3 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(2),
            *get_field_ref(AnEnum::AVariant): eq(1),
            a_third_field: eq(3),
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_trailing_comma_ret_ref_followed_by_a_field()
-> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            *get_field_ref(2, 3,): eq(123),
            a_third_field: eq(345),
        })
    )
}
