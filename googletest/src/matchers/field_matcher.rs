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

/// Matches a structure or enum with a given field which is matched by a given
/// matcher.
///
/// For example:
///
/// ```rust
/// struct IntField {
///   int: i32
/// }
/// verify_that!(IntField{int: 32}, field!(IntField.int, eq(32)))?;
/// ```
///
/// Tuple structs are also supported via the index syntax:
///
/// ```rust
/// struct IntField(i32)
/// verify_that!(IntField(32), field!(IntField.0, eq(32)))?;
/// ```
///
/// Enums are also supported, in which case only the specified variant is
/// matched:
///
/// ```rust
/// enum MyEnum {
///     A(i32),
///     B,
/// }
/// verify_that!(MyEnum::A(32), field!(MyEnum::A.0, eq(32)))?; // Passes
/// verify_that!(MyEnum::B, field!(MyEnum::A.0, eq(32)))?; // Fails: wrong enum variant
/// ```
///
/// The structure or enum may also be referenced from a separate module:
///
/// ```rust
/// mod a_module {
///     struct AStruct(i32);
/// }
/// verify_that(a_module::AStruct(32), field!(a_module::AStruct.0, eq(32)))?;
/// ```
///
/// Nested structures are *not supported*, however:
///
/// ```rust
/// struct InnerStruct(i32);
/// struct OuterStruct {
///     inner: InnerStruct,
/// }
/// verify_that(value, field!(OuterStruct.inner.0, eq(32)))?; // Does not compile
/// ```
#[macro_export]
macro_rules! field {
    ($($t:ident)::+.$field:tt, $m:expr) => {{
        #[cfg(google3)]
        use $crate::internal::field_matcher;
        #[cfg(not(google3))]
        use $crate::matchers::field_matcher::internal::field_matcher;
        field_matcher(
            |o| {
                match o {
                    $($t)::* { $field: value, .. } => Some(value),
                    // The pattern below is unreachable if the type is a struct (as opposed to an
                    // enum). Since the macro can't know which it is, we always include it and just
                    // tell the compiler not to complain.
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            },
            &stringify!($field),
            $m)
    }};
}

/// Functions for use only by the declarative macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    #[cfg(not(google3))]
    use crate as googletest;
    use googletest::matcher::{Describe, Matcher, MatcherResult};
    use std::fmt::Debug;

    /// Creates a matcher to verify a specific field of the actual struct using
    /// the provided inner matcher.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn field_matcher<O: Debug, I: Debug, InnerMatcher: Matcher<I>>(
        field_accessor: fn(&O) -> Option<&I>,
        field_path: &'static str,
        inner: InnerMatcher,
    ) -> impl Matcher<O> {
        FieldMatcher { field_accessor, field_path, inner }
    }

    struct FieldMatcher<O, I, InnerMatcher> {
        field_accessor: fn(&O) -> Option<&I>,
        field_path: &'static str,
        inner: InnerMatcher,
    }

    impl<O: Debug, I: Debug, InnerMatcher: Matcher<I>> Matcher<O> for FieldMatcher<O, I, InnerMatcher> {
        fn matches(&self, actual: &O) -> MatcherResult {
            if let Some(value) = (self.field_accessor)(actual) {
                self.inner.matches(value)
            } else {
                MatcherResult::DoesNotMatch
            }
        }
    }

    impl<O, I, M: Describe> Describe for FieldMatcher<O, I, M> {
        fn describe(&self, matcher_result: MatcherResult) -> String {
            format!(
                "has field `{}`, which {}",
                self.field_path,
                self.inner.describe(matcher_result)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(google3))]
    use crate as googletest;
    use googletest::matcher::{Describe, MatcherResult};
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{google_test, verify_that, Result};
    use matchers::{eq, not};

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
        verify_that!(
            StringField { strink: "yes".to_string() },
            field!(StringField.strink, eq("yes"))
        )
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
    fn does_not_match_enum_value_with_wrong_enum_value() -> Result<()> {
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
    fn matches_struct_like_enum_value() -> Result<()> {
        #[derive(Debug)]
        enum AnEnum {
            AValue { a_field: u32 },
        }
        let value = AnEnum::AValue { a_field: 123 };

        verify_that!(value, field!(AnEnum::AValue.a_field, eq(123)))
    }
}
