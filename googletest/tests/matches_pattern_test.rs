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
use googletest::matchers;
use googletest::{google_test, matches_pattern, pat, verify_that, Result};
use indoc::indoc;
use matchers::{contains_substring, displays_as, eq, err, not};

#[google_test]
fn matches_struct_containing_single_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { a_field: eq(123) }))
}

#[google_test]
fn matches_struct_containing_two_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct {
        a_field: 123,
        another_field: 234,
    };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_field: eq(123),
            another_field: eq(234)
        })
    )
}

#[google_test]
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

#[google_test]
fn supports_trailing_comma_with_two_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct {
        a_field: 123,
        another_field: 234,
    };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_field: eq(123),
            another_field: eq(234), // Block reformatting
        })
    )
}

#[google_test]
fn supports_trailing_comma_with_three_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }
    let actual = AStruct {
        a_field: 123,
        another_field: 234,
        a_third_field: 345,
    };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_field: eq(123),
            another_field: eq(234),
            a_third_field: eq(345),
        })
    )
}

#[google_test]
fn matches_struct_containing_nested_struct_with_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_nested_struct: ANestedStruct,
    }
    #[derive(Debug)]
    struct ANestedStruct {
        a_field: u32,
    }
    let actual = AStruct {
        a_nested_struct: ANestedStruct { a_field: 123 },
    };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_nested_struct: pat!(ANestedStruct { a_field: eq(123) })
        })
    )
}

#[google_test]
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
            Expected: has field `a_field`, which is equal to 234
            Actual: AStruct {
                a_field: 123,
            }, which has field `a_field`, which isn't equal to 234
            "
        })))
    )
}

#[google_test]
fn has_correct_assertion_failure_message_for_two_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct {
        a_field: 123,
        another_field: 234,
    };
    let result = verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_field: eq(234),
            another_field: eq(123)
        })
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: actual
            Expected: has all the following properties:
              * has field `a_field`, which is equal to 234
              * has field `another_field`, which is equal to 123
            Actual: AStruct {
                a_field: 123,
                another_field: 234,
            }, 
              * which has field `a_field`, which isn't equal to 234
              * which has field `another_field`, which isn't equal to 123"
        ))))
    )
}

#[google_test]
fn supports_qualified_struct_names() -> Result<()> {
    mod a_module {
        #[derive(Debug)]
        pub(super) struct AStruct {
            pub(super) a_field: u32,
        }
    }
    let actual = a_module::AStruct { a_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(a_module::AStruct { a_field: eq(123) })
    )
}

#[google_test]
fn matches_tuple_struct_containing_single_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32);
    let actual = AStruct(123);

    verify_that!(actual, matches_pattern!(AStruct(eq(123))))
}

#[google_test]
fn matches_tuple_struct_containing_two_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32);
    let actual = AStruct(123, 234);

    verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234))))
}

#[google_test]
fn matches_tuple_struct_containing_three_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32);
    let actual = AStruct(123, 234, 345);

    verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234), eq(345))))
}

#[google_test]
fn matches_tuple_struct_containing_four_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456);

    verify_that!(
        actual,
        matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456)))
    )
}

#[google_test]
fn matches_tuple_struct_containing_five_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567);

    verify_that!(
        actual,
        matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456), eq(567)))
    )
}

#[google_test]
fn matches_tuple_struct_containing_six_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678)
        ))
    )
}

#[google_test]
fn matches_tuple_struct_containing_seven_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678),
            eq(789)
        ))
    )
}

#[google_test]
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

#[google_test]
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

#[google_test]
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

#[google_test]
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

#[google_test]
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

#[google_test]
fn matches_enum_value() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
    }
    let actual = AnEnum::A(123);

    verify_that!(actual, matches_pattern!(AnEnum::A(eq(123))))
}

#[google_test]
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
