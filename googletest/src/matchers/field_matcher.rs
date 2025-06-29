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
// macro is documented in the matchers module.
#![doc(hidden)]

/// Matches a structure or enum with a given field which is matched by a given
/// matcher.
///
/// This takes two arguments:
///
///  * a specification of the field against which to match, and
///  * an inner [`Matcher`][crate::matcher::Matcher] to apply to that field.
///
/// For example:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// struct IntField {
///   int: i32
/// }
/// # fn should_pass() -> Result<()> {
/// verify_that!(IntField{int: 32}, field!(&IntField.int, eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Tuple structs are also supported via the index syntax:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// struct IntField(i32);
/// # fn should_pass() -> Result<()> {
/// verify_that!(IntField(32), field!(&IntField.0, eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Enums are also supported, in which case only the specified variant is
/// matched:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// enum MyEnum {
///     A(i32),
///     B,
/// }
/// # fn should_pass() -> Result<()> {
/// verify_that!(MyEnum::A(32), field!(&MyEnum::A.0, eq(32)))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(MyEnum::B, field!(&MyEnum::A.0, eq(32)))?; // Fails: wrong enum variant
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// The structure or enum may also be referenced from a separate module:
///
/// ```
/// # use googletest::prelude::*;
/// mod a_module {
///     #[derive(Debug)]
///     pub struct AStruct(pub i32);
/// }
/// # fn should_pass() -> Result<()> {
/// verify_that!(a_module::AStruct(32), field!(&a_module::AStruct.0, eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// If the inner matcher is `eq(...)`, it can be omitted:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// struct IntField {
///   int: i32
/// }
/// # fn should_pass() -> Result<()> {
/// verify_that!(IntField{int: 32}, field!(&IntField.int, 32))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Nested structures are *not supported*, however:
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// struct InnerStruct(i32);
/// #[derive(Debug)]
/// struct OuterStruct {
///     inner: InnerStruct,
/// }
/// # fn should_not_compile() -> Result<()> {
/// verify_that!(value, field!(OuterStruct.inner.0, eq(32)))?; // Does not compile
/// #     Ok(())
/// # }
/// ```
///
/// # Specification of the field pattern
///
/// The specification of the field follow the syntax: `(ref)? (&)?
/// $TYPE.$FIELD`.
/// The `&` allows to specify whether this matcher matches against an actual of
/// type `$TYPE` (`$TYPE`` must implement `Copy`) or a `&$TYPE`.
///
/// For instance:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// pub struct AStruct{a_field: i32};
/// # fn should_pass() -> Result<()> {
/// verify_that!(AStruct{a_field: 32}, field!(&AStruct.a_field, eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug, Clone, Copy)]
/// pub struct AStruct{a_field: i32};
/// # fn should_pass() -> Result<()> {
/// verify_that!(AStruct{a_field: 32}, field!(AStruct.a_field, eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// The `ref` allows to bind the field value by reference, which is required if
/// the field type does not implement `Copy`.
///
/// For instance:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// pub struct AStruct{a_field: i32};
/// # fn should_pass() -> Result<()> {
/// verify_that!(AStruct{a_field: 32}, field!(&AStruct.a_field, eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// If `field!` is qualified by both `&` and `ref`, they can both be omitted.
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// pub struct AStruct{a_field: String};
/// # fn should_pass() -> Result<()> {
/// verify_that!(AStruct{a_field: "32".into()}, field!(&AStruct.a_field, ref eq("32")))?;
/// verify_that!(AStruct{a_field: "32".into()}, field!(AStruct.a_field, eq("32")))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// See also the macro [`property`][crate::matchers::property] for an analogous
/// mechanism to extract a datum by invoking a method.
#[macro_export]
#[doc(hidden)]
macro_rules! __field {
    ($($t:tt)*) => { $crate::field_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
// We cannot use `path` or `ty` to capture the type as we are terminating the
// type with a . (dot).
#[doc(hidden)]
#[macro_export]
macro_rules! field_internal {
    // `ref` variant.
    (
        // The 3 cases of `$(& $($_amp:literal)?)?` -> `$(& $($_amp)*)*`
        // 1. outer group captures nothing => expansion produces nothing
        // 2. outer group captures just `&` => expansion produces `&`
        // 3. outer group captures `& <literal>` => disallowed by `@assert_empty` subrule invocation
        //
        // `$_amp:literal` works only because the following `$t:ident` or `::` can't be captured by
        // it.
        $(& $($_amp:literal)?)? // Optional `&`'s presence is implicitly captured by `_amp`.
        $(:: $($_cs:literal)?)? // Optional `::`'s presence is implicitly captured by `_cs`.
        $($t:ident)::+ $(::<$($t_ty_args:ty),* $(,)?>)?  .$field:tt,
        ref $m:expr) => {{
        $crate::field_internal!(@assert_empty $($($_amp)*)* $($($_cs)*)*);
        $crate::field_internal!(@internal
            struct_type:  [&_]
            field_prefix: [$(& $($_amp)*)*  $(:: $($_cs)*)*  $($t)::* $(::<$($t_ty_args),*>)*]
            [$field] [ref] [$m])
    }};

    // Non-`ref` variant.
    (
        // See comment on previous variant above.
        $(& $($_amp:literal)?)? // Optional `&`'s presence is implicitly captured by `_amp`.
        $(:: $($_cs:literal)?)? // Optional `::`'s presence is implicitly captured by `_cs`.
        $($t:ident)::+ $(::<$($t_ty_args:ty),* $(,)?>)? .$field:tt, $m:expr) => {{
        $crate::field_internal!(@assert_empty $($($_amp)*)* $($($_cs)*)*);
        $crate::field_internal!(@internal
            struct_type:  [$(& $($_amp)*)*  &_]
            field_prefix: [$(& $($_amp)*)*  $(:: $($_cs)*)*  $($t)::* $(::<$($t_ty_args),*>)*]
            [$field] [] [$m])
    }};

    (@assert_empty) => {};
    (@assert_empty $($l:literal)+) => {
        compile_error!("property! argument must start with an optional `&` followed by a path")
    };

    (@internal struct_type: [$struct_ty:ty]
               field_prefix: [$($field_prefix:tt)*]
               [$field:tt] [$($ref:tt)?] [$m:expr]) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::field_matcher(
            |o: $struct_ty| {
                match o {
                    $($field_prefix)* {$field: $($ref)* value, .. } => Some(value),
                    // The pattern below is unreachable if the type is a struct (as opposed to an
                    // enum). Since the macro can't know which it is, we always include it and just
                    // tell the compiler not to complain.
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            },
            &stringify!($field),
            $crate::matcher_support::__internal_unstable_do_not_depend_on_these::auto_eq!($m))
    }}
}

/// Functions for use only by the declarative macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::{
        description::Description,
        matcher::{Matcher, MatcherBase, MatcherResult},
    };
    use std::fmt::Debug;

    /// Creates a matcher to verify a specific field of the actual struct using
    /// the provided inner matcher.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn field_matcher<OuterT, InnerT, InnerMatcher>(
        field_accessor: fn(&OuterT) -> Option<&InnerT>,
        field_path: &'static str,
        inner: InnerMatcher,
    ) -> FieldMatcher<OuterT, InnerT, InnerMatcher> {
        FieldMatcher { field_accessor, field_path, inner }
    }

    #[derive(MatcherBase)]
    pub struct FieldMatcher<OuterT, InnerT, InnerMatcher> {
        field_accessor: fn(&OuterT) -> Option<&InnerT>,
        field_path: &'static str,
        inner: InnerMatcher,
    }

    impl<'a, OuterT: Debug + 'a, InnerT: Debug + 'a, InnerMatcher: Matcher<&'a InnerT>>
        Matcher<&'a OuterT> for FieldMatcher<OuterT, InnerT, InnerMatcher>
    {
        fn matches(&self, actual: &'a OuterT) -> MatcherResult {
            if let Some(value) = (self.field_accessor)(actual) {
                self.inner.matches(value)
            } else {
                MatcherResult::NoMatch
            }
        }

        fn explain_match(&self, actual: &'a OuterT) -> Description {
            if let Some(actual) = (self.field_accessor)(actual) {
                format!(
                    "which has field `{}`, {}",
                    self.field_path,
                    self.inner.explain_match(actual)
                )
                .into()
            } else {
                let formatted_actual_value = format!("{actual:?}");
                let without_fields = formatted_actual_value.split('(').next().unwrap_or("");
                let without_fields = without_fields.split('{').next().unwrap_or("").trim_end();
                format!("which has the wrong enum variant `{without_fields}`").into()
            }
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "has field `{}`, which {}",
                self.field_path,
                self.inner.describe(matcher_result)
            )
            .into()
        }
    }

    impl<OuterT: Debug + Copy, InnerT: Debug + Copy, InnerMatcher: Matcher<InnerT>> Matcher<OuterT>
        for FieldMatcher<OuterT, InnerT, InnerMatcher>
    {
        fn matches(&self, actual: OuterT) -> MatcherResult {
            if let Some(value) = (self.field_accessor)(&actual) {
                self.inner.matches(*value)
            } else {
                MatcherResult::NoMatch
            }
        }

        fn explain_match(&self, actual: OuterT) -> Description {
            if let Some(actual) = (self.field_accessor)(&actual) {
                format!(
                    "which has field `{}`, {}",
                    self.field_path,
                    self.inner.explain_match(*actual)
                )
                .into()
            } else {
                let formatted_actual_value = format!("{actual:?}");
                let without_fields = formatted_actual_value.split('(').next().unwrap_or("");
                let without_fields = without_fields.split('{').next().unwrap_or("").trim_end();
                format!("which has the wrong enum variant `{without_fields}`").into()
            }
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "has field `{}`, which {}",
                self.field_path,
                self.inner.describe(matcher_result)
            )
            .into()
        }
    }
}
