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
use googletest::Result;
use indoc::indoc;

#[test]
fn matches_struct_containing_single_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(&AStruct { a_field: eq(123) }))
}

#[test]
fn matches_struct_containing_two_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(actual, matches_pattern!(&AStruct { a_field: eq(123), another_field: eq(234) }))
}

#[test]
fn matches_struct_non_exhaustive() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(actual, matches_pattern!(&AStruct { a_field: eq(123), .. }))
}

#[test]
fn matches_braced_struct_with_no_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {}
    let actual = AStruct {};

    verify_that!(actual, matches_pattern!(AStruct {}))
}

#[test]
#[rustfmt::skip]// Otherwise fmt strips the trailing comma
fn supports_trailing_comma_with_one_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(&AStruct {
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
        matches_pattern!(&AStruct {
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
        matches_pattern!(&AStruct {
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
        matches_pattern!(&AStruct { a_nested_struct: ref pat!(&ANestedStruct { a_field: eq(123) }) })
    )
}

#[test]
fn matches_struct_containing_nested_struct_with_field_with_binding_mode() -> Result<()> {
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
        matches_pattern!(AStruct { a_nested_struct: pat!(ANestedStruct { a_field: eq(&123) }) })
    )
}

#[test]
fn matches_struct_containing_non_copy_field_binding_mode() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_string: String,
    }
    let actual = AStruct { a_string: "123".into() };

    verify_that!(actual, matches_pattern!(AStruct { a_string: eq("123") }))
}

#[test]
fn matches_generic_struct_exhaustively() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct AStruct<T> {
        a: T,
        b: u32,
    }
    let actual = AStruct { a: 1, b: 3 };

    verify_that!(actual, matches_pattern!(&AStruct { a: _, b: _ }))?;
    verify_that!(actual, matches_pattern!(AStruct { a: _, b: _ }))
}

#[test]
fn matches_struct_with_interleaved_underscore() -> Result<()> {
    #[derive(Debug)]
    struct NonCopy;
    #[allow(dead_code)]
    #[derive(Debug)]
    struct AStruct {
        a: u32,
        b: NonCopy,
        c: u32,
    }
    let actual = AStruct { a: 1, b: NonCopy, c: 3 };

    verify_that!(actual, matches_pattern!(&AStruct { a: eq(1), b: _, c: eq(3) }))?;
    verify_that!(actual, matches_pattern!(AStruct { a: eq(&1), b: _, c: eq(&3) }))
}

#[test]
fn has_correct_assertion_failure_message_for_single_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };

    let result = verify_that!(actual, matches_pattern!(&AStruct { a_field: eq(234) }));

    const EXPECTED: &str = indoc!(
        "
        Value of: actual
        Expected: is & AStruct which has field `a_field`, which is equal to 234
        Actual: AStruct { a_field: 123 },
          which has field `a_field`, which isn't equal to 234
        "
    );
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
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
        matches_pattern!(&AStruct { a_field: eq(234), another_field: eq(123) })
    );

    const EXPECTED: &str = indoc!(
        "
        Value of: actual
        Expected: is & AStruct which has all the following properties:
          * has field `a_field`, which is equal to 234
          * has field `another_field`, which is equal to 123
        Actual: AStruct { a_field: 123, another_field: 234 },
          * which has field `a_field`, which isn't equal to 234
          * which has field `another_field`, which isn't equal to 123"
    );
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
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
        matches_pattern!(&AStruct { get_field(): eq(234), another_field: eq(123), .. })
    );

    const EXPECTED: &str = indoc!(
        "
    Value of: actual
    Expected: is & AStruct which has all the following properties:
      * has property `get_field()`, which is equal to 234
      * has field `another_field`, which is equal to 123
    Actual: AStruct { a_field: 123, another_field: 234 },
      * whose property `get_field()` is `123`, which isn't equal to 234
      * which has field `another_field`, which isn't equal to 123"
    );
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
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

    verify_that!(actual, matches_pattern!(&a_module::AStruct { a_field: eq(123) }))
}

#[test]
fn matches_match_pattern_struct() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct AStruct {
        a: u32,
    }
    let actual = AStruct { a: 123 };

    verify_that!(actual, matches_pattern!(AStruct { .. }))
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

    let result = verify_that!(actual, matches_pattern!(&AStruct { get_field(): eq(234) }));

    const EXPECTED: &str = "Expected: is & AStruct which has property `get_field()`";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
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

    let result = verify_that!(actual, matches_pattern!(&AStruct { get_field(): eq(&234) }));

    const EXPECTED: &str = "Expected: is & AStruct which has property `get_field()`";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
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
        verify_that!(actual, matches_pattern!(&AStruct { field: eq(123), get_field(): eq(234) }));

    const EXPECTED: &str = "Expected: is & AStruct which has all the following properties";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
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
        verify_that!(actual, matches_pattern!(&AStruct { field: eq(123), get_field(): eq(&234) }));

    const EXPECTED: &str = "Expected: is & AStruct which has all the following properties";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
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

    verify_that!(actual, matches_pattern!(&AStruct { get_field(): eq(123) }))
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

    verify_that!(actual, matches_pattern!(&AStruct { get_field(): eq(123), }))
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

    verify_that!(actual, matches_pattern!(&AStruct { add_to_field(2): eq(3) }))
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

    verify_that!(actual, matches_pattern!(&AStruct { add_product_to_field(2, 3): eq(7) }))
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

    verify_that!(actual, matches_pattern!(&AStruct { add_product_to_field(2, 3,): eq(7) }))
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

    verify_that!(actual, matches_pattern!(&AStruct { get_field_ref(): eq(&123) }))
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

    verify_that!(actual, matches_pattern!(&AStruct { get_field_ref(): eq(&123), }))
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

    verify_that!(actual, matches_pattern!(&AStruct { get_field_ref(2, 3): eq(&1) }))
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

    verify_that!(actual, matches_pattern!(&AStruct { get_field_ref(2, 3,): eq(&1) }))
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

    verify_that!(
        actual,
        matches_pattern!(&AStruct { get_field(): eq(123), another_field: eq(234), .. })
    )
}

#[test]
fn matches_struct_with_a_method_followed_by_a_field_with_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_value(&self) -> u32 {
            123
        }
    }
    let actual = AStruct { a_field: 234 };

    verify_that!(actual, matches_pattern!(&AStruct { get_value(): eq(123), a_field: eq(234), }))
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_trailing_comma_ret_ref_followed_by_a_field(
) -> Result<()> {
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
        matches_pattern!(&AStruct {
            another_field: eq(234),
            get_field_ref(2, 3,): eq(&123),
            a_third_field: eq(345),
            ..
        })
    )
}

#[test]
fn matches_struct_field_copy() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(&AStruct { a_field: eq(123) }))
}

#[test]
fn matches_struct_field_non_copy() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: String,
    }
    let actual = AStruct { a_field: "123".into() };

    verify_that!(
        actual,
        matches_pattern!(&AStruct {
            a_field: ref eq("123"),
        })
    )
}

#[test]
fn matches_copy_struct_field_copy() -> Result<()> {
    #[derive(Debug, Clone, Copy)]
    struct AStruct {
        a_field: i32,
    }
    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { a_field: eq(123) }))
}

#[test]
fn matches_struct_property_copy() -> Result<()> {
    #[derive(Debug)]
    struct AStruct;

    impl AStruct {
        fn prop(&self) -> i32 {
            123
        }
    }
    let actual = AStruct;

    verify_that!(actual, matches_pattern!(&AStruct { prop(): eq(123) }))
}

#[test]
fn matches_struct_property_non_copy() -> Result<()> {
    #[derive(Debug)]
    struct AStruct;

    impl AStruct {
        fn prop(&self) -> String {
            "123".into()
        }
    }
    let actual = AStruct;

    verify_that!(actual, matches_pattern!(&AStruct { prop(): ref eq("123") }))
}

#[test]
fn matches_copy_struct_property_copy() -> Result<()> {
    #[derive(Debug, Clone, Copy)]
    struct AStruct;

    impl AStruct {
        fn prop(self) -> i32 {
            123
        }
    }
    let actual = AStruct;

    verify_that!(actual, matches_pattern!(AStruct { prop(): eq(123) }))
}

#[test]
fn matches_copy_struct_property_non_copy() -> Result<()> {
    #[derive(Debug, Clone, Copy)]
    struct AStruct;

    impl AStruct {
        fn prop(self) -> String {
            "123".into()
        }
    }
    let actual = AStruct;

    verify_that!(actual, matches_pattern!(AStruct { prop(): ref eq("123") }))
}

#[test]
fn matches_struct_auto_eq() -> Result<()> {
    #[derive(Debug, Clone)]
    struct AStruct {
        int: i32,
        string: String,
        option: Option<i32>,
    }

    verify_that!(
        AStruct { int: 123, string: "123".into(), option: Some(123) },
        matches_pattern!(&AStruct { int: 123, string: ref "123", option: Some(123) })
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
        matches_pattern!(&AStruct { add_product_to_field(2, 3): eq(7), another_field: eq(123), .. })
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
        matches_pattern!(&AStruct {
            another_field: eq(234),
            get_field_ref(): eq(&123),
            a_third_field: eq(345),
            ..
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_ret_ref_followed_by_a_field_with_trailing_comma(
) -> Result<()> {
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
        matches_pattern!(&AStruct {
            another_field: eq(234),
            get_field_ref(): eq(&123),
            a_third_field: eq(345),
            ..
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_ret_ref_followed_by_a_field(
) -> Result<()> {
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
        matches_pattern!(&AStruct {
            another_field: eq(234),
            get_field_ref(2, 3): eq(&123),
            a_third_field: eq(345),
            ..
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_and_trailing_comma_ret_ref(
) -> Result<()> {
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
        matches_pattern!(&AStruct { another_field: eq(234), get_field_ref(2, 3,): eq(&123), .. })
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
        matches_pattern!(&AStruct {
            another_field: eq(234),
            get_field(): eq(123),
            a_third_field: eq(345),
            ..
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_followed_by_a_field_with_trailing_comma(
) -> Result<()> {
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
        matches_pattern!(&AStruct {
            a_field: eq(123),
            get_field(): eq(123),
            another_field: eq(234),
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
        matches_pattern!(&AStruct {
            another_field: eq(234),
            add_product_to_field(2, 3): eq(7),
            a_third_field: eq(345),
            ..
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_and_trailing_comma_followed_by_a_field(
) -> Result<()> {
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
        matches_pattern!(&AStruct {
            another_field: eq(234),
            add_product_to_field(2, 3,): eq(7),
            a_third_field: eq(345),
            ..
        })
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
        matches_pattern!(&AStruct { another_field: eq(234), add_product_to_field(2, 3,): eq(7), .. })
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
        matches_pattern!(&AStruct { another_field: eq(234), get_field_ref(): eq(&123), .. })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_returning_ref_and_trailing_comma() -> Result<()>
{
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

    verify_that!(
        actual,
        matches_pattern!(&AStruct { a_field: eq(123), get_field_ref(): eq(&123), })
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
        matches_pattern!(&AStruct { another_field: eq(234), get_field_ref(2, 3): eq(&123), .. })
    )
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_with_trailing_comma_ret_ref_and_field(
) -> Result<()> {
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
        matches_pattern!(&AStruct { get_field_ref(2, 3,): eq(&1), another_field: eq(123), .. })
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

    verify_that!(
        actual,
        matches_pattern!(&AStruct { another_field: eq(234), get_field(): eq(123), .. })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_value(&self) -> u32 {
            123
        }
    }
    let actual = AStruct { a_field: 234 };

    verify_that!(actual, matches_pattern!(&AStruct { a_field: eq(234), get_value(): eq(123), }))
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
        matches_pattern!(&AStruct { another_field: eq(234), add_product_to_field(2, 3): eq(7), .. })
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
        matches_pattern!(&AStruct {
            add_product_to_field(2, 3,): eq(7),
            another_field: eq(123),
            ..
        })
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
        matches_pattern!(&AStruct { get_field_ref(): eq(&123), another_field: eq(234), .. })
    )
}

#[test]
fn matches_struct_with_a_method_returning_reference_followed_by_a_field_with_trailing_comma(
) -> Result<()> {
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

    verify_that!(
        actual,
        matches_pattern!(&AStruct { get_field_ref(): eq(&123), a_field: eq(123), })
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
        matches_pattern!(&AStruct { get_field_ref(2, 3): eq(&1), another_field: eq(123), .. })
    )
}
