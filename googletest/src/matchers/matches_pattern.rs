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
/// ```ignore
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
/// ```ignore
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
/// ```ignore
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
/// ```ignore
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_nested_struct: pat!(MyInnerStruct {
///         a_field: starts_with("Something"),
///     }),
/// }))
/// ```
///
/// In addition to fields, one can match on the outputs of methods
/// ("properties"):
///
/// ```ignore
/// impl MyStruct {
///     fn get_a_field(&self) -> String {...}
/// }
///
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     get_a_field(): starts_with("Something"),
/// }))
/// ```
///
/// These may also include extra parameters you pass in:
///
/// ```ignore
/// impl MyStruct {
///     fn append_to_a_field(&self, suffix: &str) -> String {...}
/// }
///
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     append_to_a_field("a suffix"): ends_with("a suffix"),
/// }))
/// ```
///
/// If the method returns a reference, precede it with the keyword `ref`:
///
/// ```ignore
/// impl MyStruct {
///     fn get_a_field_ref(&self) -> &String {...}
/// }
///
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     ref get_a_field_ref(): starts_with("Something"),
/// }))
/// ```
///
/// > Note: At the moment, this does not work properly with methods returning
/// > string references or slices.
///
/// One can also match tuple structs with up to 10 fields. In this case, all
/// fields must have matchers:
///
/// ```ignore
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
/// ```ignore
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
        { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }
    ) => {
        all!(property!($($struct_name)*.$property_name($($argument),*), $matcher))
    };

    (
        [$($struct_name:tt)*],
        { ref $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }
    ) => {
        all!(property!(ref $($struct_name)*.$property_name($($argument),*), $matcher))
    };

    (
        [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            all!(field!($($struct_name)*.$field_name, $matcher)),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            all!(property!($($struct_name)*.$property_name($($argument),*), $matcher)),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        [$($struct_name:tt)*],
        { ref $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            all!(property!(ref $($struct_name)*.$property_name($($argument),*), $matcher)),
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
        { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }
    ) => {
        all!(
            $($processed)*,
            property!($($struct_name)*.$property_name($($argument),*), $matcher)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        { ref $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }
    ) => {
        all!(
            $($processed)*,
            property!(ref $($struct_name)*.$property_name($($argument),*), $matcher)
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
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
        { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            all!(
                $($processed)*,
                property!($($struct_name)*.$property_name($($argument),*), $matcher)
            ),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        { ref $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            all!(
                $($processed)*,
                property!(ref $($struct_name)*.$property_name($($argument),*), $matcher)
            ),
            [$($struct_name)*],
            { $($rest)* }
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
        $crate::matches_pattern_internal!(
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
        $crate::matches_pattern_internal!(
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
        $crate::matches_pattern_internal!(
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
        $crate::matches_pattern_internal!(
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
        $crate::matches_pattern_internal!(
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
        $crate::matches_pattern_internal!(
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
        $crate::matches_pattern_internal!(
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
        $crate::matches_pattern_internal!(
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
        $crate::matches_pattern_internal!(
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
        $crate::matches_pattern_internal!([$($struct_name)* $first], $($rest)*)
    };

    ($first:tt $($rest:tt)*) => {{
        #[cfg(not(google3))]
        #[allow(unused)]
        use $crate::{all, field, property};
        #[cfg(google3)]
        #[allow(unused)]
        use all_matcher::all;
        #[cfg(google3)]
        #[allow(unused)]
        use field_matcher::field;
        #[cfg(google3)]
        #[allow(unused)]
        use property_matcher::property;
        $crate::matches_pattern_internal!([$first], $($rest)*)
    }};
}

/// An alias for [`matches_pattern`].
#[macro_export]
macro_rules! pat {
    ($($t:tt)*) => { $crate::matches_pattern_internal!($($t)*) }
}
