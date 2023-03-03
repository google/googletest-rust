// Copyright 2022 Google LLC
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

// There are no visible documentation elements in this module; the declarative
// macro is documented at the top level.
#![doc(hidden)]

/// Matches a value according to a pattern of matchers.
///
/// This takes as an argument a specification similar to a struct or enum
/// initialiser, where each value is a [`Matcher`][crate::matcher::Matcher]
/// which is applied to the corresponding field.
///
/// This can be used to match arbitrary combinations of fields on structures
/// using arbitrary matchers:
///
/// ```
/// struct MyStruct {
///     a_field: String,
///     another_field: String,
/// }
///
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_field: starts_with("Something"),
///     another_field: ends_with("else"),
/// }))
/// ```
///
/// It is not required to include all named fields in the specification:
///
/// ```
/// struct MyStruct {
///     a_field: String,
///     another_field: String,
/// }
///
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_field: starts_with("Something"),
///     // another_field is missing, so it may be anything.
/// }))
/// ```
///
/// One can use it recursively to match nested structures:
///
/// ```
/// struct MyStruct {
///     a_nested_struct: MyInnerStruct,
/// }
///
/// struct MyInnerStruct {
///     a_field: String,
/// }
///
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_nested_struct: matches_pattern!(MyInnerStruct {
///         a_field: starts_with("Something"),
///     }),
/// }))
/// ```
///
/// One can use the alias [`pat`][crate::pat] to make this less verbose:
///
/// ```
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_nested_struct: pat!(MyInnerStruct {
///         a_field: starts_with("Something"),
///     }),
/// }))
/// ```
///
/// One can also match tuple structs with up to 10 fields. In this case, all
/// fields must have matchers:
///
/// ```
/// struct MyTupleStruct(String, String);
///
/// verify_that!(
///     my_struct,
///     matches_pattern!(MyTupleStruct(eq("Something"), eq("Some other thing")))
/// )
/// ```
///
/// One can also match enum values:
///
/// ```
/// enum MyEnum {
///     A(u32),
///     B,
/// }
///
/// verify_that(MyEnum::A(123), matches_pattern!(MyEnum::A(eq(123))))?; // Passes
/// verify_that(MyEnum::B, matches_pattern!(MyEnum::A(eq(123))))?; // Fails - wrong enum variant
/// ```
///
/// It is perfectly okay to omit fields from a pattern with named fields. The
/// values of omitted fields then have no effect on the output of the matcher.
///
/// This macro does not support plain (non-struct) tuples. Use the macro
/// [`tuple`] for that purpose.
///
/// Trailing commas are allowed (but not required) in both ordinary and tuple
/// structs.
#[macro_export]
macro_rules! matches_pattern {
    ($($t:tt)*) => { $crate::matches_pattern_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! matches_pattern_internal {
    (
        [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr $(,)? }
    ) => {
        all!(field!($($struct_name)*.$field_name, $matcher))
    };

    (
        [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr, $($rest:tt)* }
    ) => {
        matches_pattern_internal!(
            all!(field!($($struct_name)*.$field_name, $matcher)),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr $(,)? }
    ) => {
        all!(
            $($processed)*,
            field!($($struct_name)*.$field_name, $matcher)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr, $($rest:tt)* }
    ) => {
        matches_pattern_internal!(
            all!(
                $($processed)*,
                field!($($struct_name)*.$field_name, $matcher)
            ),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr }
    ) => {
        all!(
            $($processed)*,
            field!($($struct_name)*.$field_name, $matcher)
        )
    };

    (
        [$($struct_name:tt)*],
        ($matcher:expr $(,)?)
    ) => {
        all!(field!($($struct_name)*.0, $matcher))
    };

    (
        [$($struct_name:tt)*],
        ($matcher:expr, $($rest:tt)*)
    ) => {
        matches_pattern_internal!(
            all!(
                field!($($struct_name)*.0, $matcher)
            ),
            [$($struct_name)*],
            1,
            ($($rest)*)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        $field:tt,
        ($matcher:expr $(,)?)
    ) => {
        all!(
            $($processed)*,
            field!($($struct_name)*.$field, $matcher)
        )
    };

    // We need to repeat this once for every supported field position, unfortunately. There appears
    // to be no way in declarative macros to compute $field + 1 and have the result evaluated to a
    // token which can be used as a tuple index.
    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        1,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        matches_pattern_internal!(
            all!(
                $($processed)*,
                field!($($struct_name)*.1, $matcher)
            ),
            [$($struct_name)*],
            2,
            ($($rest)*)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        2,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        matches_pattern_internal!(
            all!(
                $($processed)*,
                field!($($struct_name)*.2, $matcher)
            ),
            [$($struct_name)*],
            3,
            ($($rest)*)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        3,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        matches_pattern_internal!(
            all!(
                $($processed)*,
                field!($($struct_name)*.3, $matcher)
            ),
            [$($struct_name)*],
            4,
            ($($rest)*)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        4,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        matches_pattern_internal!(
            all!(
                $($processed)*,
                field!($($struct_name)*.4, $matcher)
            ),
            [$($struct_name)*],
            5,
            ($($rest)*)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        5,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        matches_pattern_internal!(
            all!(
                $($processed)*,
                field!($($struct_name)*.5, $matcher)
            ),
            [$($struct_name)*],
            6,
            ($($rest)*)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        6,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        matches_pattern_internal!(
            all!(
                $($processed)*,
                field!($($struct_name)*.6, $matcher)
            ),
            [$($struct_name)*],
            7,
            ($($rest)*)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        7,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        matches_pattern_internal!(
            all!(
                $($processed)*,
                field!($($struct_name)*.7, $matcher)
            ),
            [$($struct_name)*],
            8,
            ($($rest)*)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        8,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        matches_pattern_internal!(
            all!(
                $($processed)*,
                field!($($struct_name)*.8, $matcher)
            ),
            [$($struct_name)*],
            9,
            ($($rest)*)
        )
    };

    ([$($struct_name:tt)*], $first:tt $($rest:tt)*) => {
        matches_pattern_internal!([$($struct_name)* $first], $($rest)*)
    };

    ($first:tt $($rest:tt)*) => {{
        #[cfg(not(google3))]
        #[allow(unused)]
        use $crate::{all, field};
        #[cfg(google3)]
        #[allow(unused)]
        use all_matcher::all;
        #[cfg(google3)]
        #[allow(unused)]
        use field_matcher::field;
        matches_pattern_internal!([$first], $($rest)*)
    }};
}

/// An alias for [`matches_pattern`].
#[macro_export]
macro_rules! pat {
    ($($t:tt)*) => { matches_pattern_internal!($($t)*) }
}

#[cfg(test)]
mod tests {
    #[cfg(not(google3))]
    use crate as googletest;
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{google_test, verify_that, Result};
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
        let actual = AStruct { a_field: 123, another_field: 234 };

        verify_that!(actual, matches_pattern!(AStruct { a_field: eq(123), another_field: eq(234) }))
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
        let actual = AStruct { a_field: 123, another_field: 234 };

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
        let actual = AStruct { a_nested_struct: ANestedStruct { a_field: 123 } };

        verify_that!(
            actual,
            matches_pattern!(AStruct { a_nested_struct: pat!(ANestedStruct { a_field: eq(123) }) })
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
            err(displays_as(contains_substring(
                "\
Value of: actual
Expected: has field `a_field`, which is equal to 234
Actual: AStruct {
    a_field: 123,
}, which has field `a_field`, which isn't equal to 234
"
            )))
        )
    }

    #[google_test]
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
        // TODO(bjacotg) Improve this error message. The format is not obvious.
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "\
Value of: actual
Expected: has field `a_field`, which is equal to 234 and
has field `another_field`, which is equal to 123
Actual: AStruct {
    a_field: 123,
    another_field: 234,
}, which has field `a_field`, which isn't equal to 234 AND
which has field `another_field`, which isn't equal to 123
"
            )))
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

        verify_that!(actual, matches_pattern!(a_module::AStruct { a_field: eq(123) }))
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

        verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456))))
    }

    #[google_test]
    fn matches_tuple_struct_containing_five_fields() -> Result<()> {
        #[derive(Debug)]
        struct AStruct(u32, u32, u32, u32, u32);
        let actual = AStruct(123, 234, 345, 456, 567);

        verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456), eq(567))))
    }

    #[google_test]
    fn matches_tuple_struct_containing_six_fields() -> Result<()> {
        #[derive(Debug)]
        struct AStruct(u32, u32, u32, u32, u32, u32);
        let actual = AStruct(123, 234, 345, 456, 567, 678);

        verify_that!(
            actual,
            matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456), eq(567), eq(678)))
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
}
