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
fn matches_enum_without_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A,
    }
    let actual = AnEnum::A;

    verify_that!(actual, matches_pattern!(&AnEnum::A))?;
    verify_that!(actual, matches_pattern!(&AnEnum::A,))
}

#[test]
fn matches_enum_without_field_ref_binding_mode() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A,
    }
    let actual = AnEnum::A;

    verify_that!(actual, matches_pattern!(AnEnum::A))?;
    verify_that!(actual, matches_pattern!(AnEnum::A,))
}

#[test]
fn matches_enum_without_field_copy() -> Result<()> {
    #[derive(Debug, Clone, Copy)]
    enum AnEnum {
        A,
    }
    let actual = AnEnum::A;

    verify_that!(actual, matches_pattern!(AnEnum::A))?;
    verify_that!(actual, matches_pattern!(AnEnum::A,))
}

#[test]
fn generates_correct_failure_output_when_enum_variant_without_field_is_not_matched() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(unused)]
        A,
        B,
    }
    let actual = AnEnum::B;

    let result = verify_that!(actual, matches_pattern!(&AnEnum::A));

    const EXPECTED: &str = "is not & AnEnum :: A";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn generates_correct_failure_output_when_enum_variant_without_field_is_matched() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A,
    }
    let actual = AnEnum::A;

    let result = verify_that!(actual, not(matches_pattern!(&AnEnum::A)));

    const EXPECTED: &str = "is & AnEnum :: A";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn has_meaningful_assertion_failure_message_when_wrong_enum_variant_is_used() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(unused)]
        A(u32),
        #[allow(unused)]
        B(u32),
    }
    let actual = AnEnum::A(123);

    let result = verify_that!(actual, matches_pattern!(&AnEnum::B(eq(123))));

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
fn matches_enum_struct_non_exhaustive() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1 { a_field: u32, another_field: u32 },
    }
    let actual: AnEnum = AnEnum::Variant1 { a_field: 123, another_field: 234 };

    verify_that!(actual, matches_pattern!(&AnEnum::Variant1 { a_field: eq(123), .. }))
}

#[test]
fn matches_enum_struct_with_all_non_exhaustive_fields() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1 { a: u32, b: u32 },
        Variant2 { c: u32, d: u32 },
    }
    let actual: AnEnum = AnEnum::Variant1 { a: 123, b: 234 };

    verify_that!(actual, matches_pattern!(&AnEnum::Variant1 { .. }))
}

#[test]
fn has_failure_when_wrong_enum_struct_variant_is_matched_with_all_non_exhaustive_fields(
) -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1 { a: u32, b: u32 },
        Variant2 { c: u32, d: u32 },
    }
    let actual: AnEnum = AnEnum::Variant1 { a: 123, b: 234 };

    let result = verify_that!(actual, matches_pattern!(&AnEnum::Variant2 { .. }));

    const EXPECTED: &str = indoc!(
        "
        Expected: is & AnEnum :: Variant2 { .. }
        Actual: Variant1 { a: 123, b: 234 },
          which is not & AnEnum :: Variant2 { .. }
        "
    );
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn matches_enum_struct_with_all_wildcard_fields() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1 { a: u32, b: u32 },
        Variant2 { c: u32, d: u32 },
    }
    let actual: AnEnum = AnEnum::Variant1 { a: 123, b: 234 };

    verify_that!(actual, matches_pattern!(&AnEnum::Variant1 { a: _, b: _ }))
}

#[test]
fn has_failure_when_wrong_enum_struct_variant_is_matched_with_all_wildcard_fields() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1 { a: u32, b: u32 },
        Variant2 { c: u32, d: u32 },
    }
    let actual: AnEnum = AnEnum::Variant1 { a: 123, b: 234 };

    let result = verify_that!(actual, matches_pattern!(&AnEnum::Variant2 { c: _, d: _ }));

    const EXPECTED: &str = indoc!(
        "
        Expected: is & AnEnum :: Variant2 { c : _, d : _, }
        Actual: Variant1 { a: 123, b: 234 },
          which is not & AnEnum :: Variant2 { c : _, d : _, }
        "
    );
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn matches_enum_struct_non_exhaustive_with_wildcard_fields() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1 { a: u32, b: u32 },
        Variant2 { c: u32, d: u32 },
    }
    let actual: AnEnum = AnEnum::Variant1 { a: 123, b: 234 };

    verify_that!(actual, matches_pattern!(&AnEnum::Variant1 { a: _, .. }))
}

#[test]
fn has_failure_when_wrong_enum_struct_variant_is_matched_non_exhaustive_with_wildcard_fields(
) -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1 { a: u32, b: u32 },
        Variant2 { c: u32, d: u32 },
    }
    let actual: AnEnum = AnEnum::Variant1 { a: 123, b: 234 };

    let result = verify_that!(actual, matches_pattern!(&AnEnum::Variant2 { c: _, .. }));

    const EXPECTED: &str = indoc!(
        "
        Expected: is & AnEnum :: Variant2 { c : _, .. }
        Actual: Variant1 { a: 123, b: 234 },
          which is not & AnEnum :: Variant2 { c : _, .. }
        "
    );
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn matches_enum_struct_exhaustive_with_multiple_variants() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1 { a_field: u32 },
        Variant2,
    }
    let actual: AnEnum = AnEnum::Variant1 { a_field: 123 };

    verify_that!(actual, matches_pattern!(&AnEnum::Variant1 { a_field: eq(123) }))
}

#[test]
fn matches_match_pattern_literal() -> Result<()> {
    let actual = false;
    #[allow(clippy::redundant_pattern_matching)]
    verify_that!(actual, matches_pattern!(false))?;
    #[allow(clippy::redundant_pattern_matching)]
    verify_that!(actual, matches_pattern!(false,))?;
    let actual = 1;
    verify_that!(actual, matches_pattern!(1))?;
    verify_that!(actual, matches_pattern!(1,))?;
    let actual = "test";
    verify_that!(actual, matches_pattern!(&"test"))?;
    verify_that!(actual, matches_pattern!(&"test",))
}

#[test]
fn has_failure_when_wrong_enum_variant_is_matched_non_exhaustively() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1(i8),
        Variant2,
    }
    let actual: AnEnum = AnEnum::Variant2;

    let result = verify_that!(actual, matches_pattern!(&AnEnum::Variant1(..)));

    const EXPECTED: &str = indoc!(
        "
        Expected: is & AnEnum :: Variant1(..)
        Actual: Variant2,
          which is not & AnEnum :: Variant1(..)
        "
    );
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn has_failure_when_wrong_enum_variant_is_matched_with_underscore() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1(i8),
        Variant2,
    }
    let actual: AnEnum = AnEnum::Variant2;

    let result = verify_that!(actual, matches_pattern!(&AnEnum::Variant1(_)));

    const EXPECTED: &str = indoc!(
        "
        Expected: is & AnEnum :: Variant1(_)
        Actual: Variant2,
          which is not & AnEnum :: Variant1(_)
        "
    );
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn has_failure_when_wrong_enum_variant_is_matched_with_value() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1(i8),
        Variant2,
    }
    let actual: AnEnum = AnEnum::Variant2;

    let result = verify_that!(actual, matches_pattern!(&AnEnum::Variant1(123)));

    const EXPECTED: &str = indoc!(
        "
        Expected: is & AnEnum :: Variant1 which has field `0`, which is equal to 123
        Actual: Variant2,
          which has the wrong enum variant `Variant2`
        "
    );
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn matches_enum_struct_field_with_mutliple_variants() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1(i8),
        Variant2,
    }
    let actual: AnEnum = AnEnum::Variant2;

    verify_that!(actual, matches_pattern!(&AnEnum::Variant2))
}

#[test]
fn matches_enum_struct_field_with_multiple_variants_non_exhaustive() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1(i8),
        Variant2,
    }
    let actual: AnEnum = AnEnum::Variant1(123);

    verify_that!(actual, matches_pattern!(&AnEnum::Variant1(..)))
}

#[test]
fn matches_enum_struct_field_with_multiple_variants_with_wildcard() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum AnEnum {
        Variant1(i8),
        Variant2,
    }
    let actual: AnEnum = AnEnum::Variant1(123);

    verify_that!(actual, matches_pattern!(&AnEnum::Variant1(_)))
}

#[test]
fn matches_enum_with_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
    }
    let actual = AnEnum::A(123);

    verify_that!(actual, matches_pattern!(&AnEnum::A(eq(123))))
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

    verify_that!(actual, not(matches_pattern!(&AnEnum::A(eq(123)))))
}

#[test]
fn includes_enum_variant_in_description_with_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
    }
    let actual = AnEnum::A(123);

    let result = verify_that!(actual, matches_pattern!(&AnEnum::A(eq(234))));

    const EXPECTED: &str = "Expected: is & AnEnum :: A which has field `0`";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn includes_enum_variant_in_negative_description_with_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
    }
    let actual = AnEnum::A(123);

    let result = verify_that!(actual, not(matches_pattern!(&AnEnum::A(eq(123)))));

    const EXPECTED: &str = "Expected: is not & AnEnum :: A which has field `0`, which is equal to";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn includes_enum_variant_in_description_with_two_fields() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32, u32),
    }
    let actual = AnEnum::A(123, 234);

    let result = verify_that!(actual, matches_pattern!(&AnEnum::A(eq(234), eq(234))));

    const EXPECTED: &str = "Expected: is & AnEnum :: A which has all the following properties";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn includes_enum_variant_in_description_with_three_fields() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32, u32, u32),
    }
    let actual = AnEnum::A(123, 234, 345);

    let result = verify_that!(actual, matches_pattern!(&AnEnum::A(eq(234), eq(234), eq(345))));

    const EXPECTED: &str = "Expected: is & AnEnum :: A which has all the following properties";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
}

#[test]
fn includes_enum_variant_in_description_with_named_field() -> Result<()> {
    #[derive(Debug)]
    enum AnEnum {
        A { field: u32 },
    }
    let actual = AnEnum::A { field: 123 };

    let result = verify_that!(actual, matches_pattern!(&AnEnum::A { field: eq(234) }));

    const EXPECTED: &str = "Expected: is & AnEnum :: A which has field `field`";
    verify_that!(result, err(displays_as(contains_substring(EXPECTED))))
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
        matches_pattern!(&AnEnum::A { field: eq(234), another_field: eq(234) })
    );

    const EXPECTED: &str = "Expected: is & AnEnum :: A which has all the following properties";
    verify_that!(&result, err(displays_as(contains_substring(EXPECTED))))
}
