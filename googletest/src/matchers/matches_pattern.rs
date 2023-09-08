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
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_field: String,
///     another_field: String,
/// }
///
/// let my_struct = MyStruct {
///     a_field: "Something to believe in".into(),
///     another_field: "Something else".into()
/// };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_field: starts_with("Something"),
///     another_field: ends_with("else"),
/// }))
/// #     .unwrap();
/// ```
///
/// It is not required to include all named fields in the specification. Omitted
/// fields have no effect on the output of the matcher.
///
/// ```
/// # use googletest::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_field: String,
/// #     another_field: String,
/// # }
/// #
/// # let my_struct = MyStruct {
/// #     a_field: "Something to believe in".into(),
/// #     another_field: "Something else".into()
/// # };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_field: starts_with("Something"),
///     // another_field is missing, so it may be anything.
/// }))
/// #     .unwrap();
/// ```
///
/// One can use it recursively to match nested structures:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_nested_struct: MyInnerStruct,
/// }
///
/// #[derive(Debug)]
/// struct MyInnerStruct {
///     a_field: String,
/// }
///
/// let my_struct = MyStruct {
///     a_nested_struct: MyInnerStruct { a_field: "Something to believe in".into() },
/// };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_nested_struct: matches_pattern!(MyInnerStruct {
///         a_field: starts_with("Something"),
///     }),
/// }))
/// #     .unwrap();
/// ```
///
/// One can use the alias [`pat`][crate::pat] to make this less verbose:
///
/// ```
/// # use googletest::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_nested_struct: MyInnerStruct,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct MyInnerStruct {
/// #     a_field: String,
/// # }
/// #
/// # let my_struct = MyStruct {
/// #     a_nested_struct: MyInnerStruct { a_field: "Something to believe in".into() },
/// # };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_nested_struct: pat!(MyInnerStruct {
///         a_field: starts_with("Something"),
///     }),
/// }))
/// #     .unwrap();
/// ```
///
/// In addition to fields, one can match on the outputs of methods
/// ("properties"):
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_field: String,
/// }
///
/// impl MyStruct {
///     fn get_a_field(&self) -> String { self.a_field.clone() }
/// }
///
/// let my_struct = MyStruct { a_field: "Something to believe in".into() };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     get_a_field(): starts_with("Something"),
/// }))
/// #     .unwrap();
/// ```
///
/// **Important**: The method should be pure function with a deterministic
/// output and no side effects. In particular, in the event of an assertion
/// failure, it will be invoked a second time, with the assertion failure output
/// reflecting the *second* invocation.
///
/// These may also include extra parameters you pass in:
///
/// ```
/// # use googletest::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_field: String,
/// # }
/// #
/// impl MyStruct {
///     fn append_to_a_field(&self, suffix: &str) -> String { self.a_field.clone() + suffix }
/// }
///
/// # let my_struct = MyStruct { a_field: "Something to believe in".into() };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     append_to_a_field("a suffix"): ends_with("a suffix"),
/// }))
/// #     .unwrap();
/// ```
///
/// If the method returns a reference, precede it with the keyword `ref`:
///
/// ```
/// # use googletest::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_field: String,
/// # }
/// #
/// impl MyStruct {
///     fn get_a_field_ref(&self) -> &String { &self.a_field }
/// }
///
/// # let my_struct = MyStruct { a_field: "Something to believe in".into() };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     ref get_a_field_ref(): starts_with("Something"),
/// }))
/// #    .unwrap();
/// ```
///
/// One can also match tuple structs with up to 10 fields. In this case, all
/// fields must have matchers:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// struct MyTupleStruct(String, String);
///
/// let my_struct = MyTupleStruct("Something".into(), "Some other thing".into());
/// verify_that!(
///     my_struct,
///     matches_pattern!(MyTupleStruct(eq("Something"), eq("Some other thing")))
/// )
/// #    .unwrap();
/// ```
///
/// One can also match enum values:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// enum MyEnum {
///     A(u32),
///     B,
/// }
///
/// # fn should_pass() -> Result<()> {
/// verify_that!(MyEnum::A(123), matches_pattern!(MyEnum::A(eq(123))))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(MyEnum::B, matches_pattern!(MyEnum::A(eq(123))))?; // Fails - wrong enum variant
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// This macro does not support plain (non-struct) tuples. Use the macro
/// [`tuple`] for that purpose.
///
/// Trailing commas are allowed (but not required) in both ordinary and tuple
/// structs.
///
/// Unfortunately, this matcher does *not* work with methods returning string
/// slices:
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// # #[derive(Debug)]
/// pub struct MyStruct {
///     a_string: String,
/// }
/// impl MyStruct {
///     pub fn get_a_string(&self) -> &str { &self.a_string }
/// }
///
/// let value = MyStruct { a_string: "A string".into() };
/// verify_that!(value, matches_pattern!( MyStruct {
///     get_a_string(): eq("A string"),   // Does not compile
/// }))
/// #    .unwrap();
/// ```
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
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(field!($($struct_name)*.$field_name, $matcher))
        )
    };

    (
        [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(property!($($struct_name)*.$property_name($($argument),*), $matcher))
        )
    };

    (
        [$($struct_name:tt)*],
        { ref $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(property!(ref $($struct_name)*.$property_name($($argument),*), $matcher))
        )
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
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(stringify!($($struct_name)*), all!(
            $($processed)*,
            field!($($struct_name)*.$field_name, $matcher)
        ))
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(stringify!($($struct_name)*), all!(
            $($processed)*,
            property!($($struct_name)*.$property_name($($argument),*), $matcher)
        ))
    };

    (
        all!($($processed:tt)*),
        [$($struct_name:tt)*],
        { ref $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(stringify!($($struct_name)*), all!(
            $($processed)*,
            property!(ref $($struct_name)*.$property_name($($argument),*), $matcher)
        ))
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
    ) => {
        $crate::matchers::predicate(|v| matches!(v, $($struct_name)*))
            .with_description(
                concat!("is ", stringify!($($struct_name)*)),
                concat!("is not ", stringify!($($struct_name)*)),
            )
    };

    (
        [$($struct_name:tt)*],
        ($matcher:expr $(,)?)
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(field!($($struct_name)*.0, $matcher))
        )
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
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(stringify!($($struct_name)*), all!(
            $($processed)*,
            field!($($struct_name)*.$field, $matcher)
        ))
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
        #[allow(unused)]
        use $crate::{all, field, property};
        $crate::matches_pattern_internal!([$first], $($rest)*)
    }};
}

/// An alias for [`matches_pattern`].
#[macro_export]
macro_rules! pat {
    ($($t:tt)*) => { $crate::matches_pattern_internal!($($t)*) }
}
