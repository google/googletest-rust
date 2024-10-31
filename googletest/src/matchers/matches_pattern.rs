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
///      a_nested_struct: matches_pattern!(MyInnerStruct {
///         a_field: starts_with("Something"),
///     }),
/// }))
/// #     .unwrap();
/// ```
///
/// One can use the alias [`pat`][crate::matchers::pat] to make this less
/// verbose:
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
/// If an inner matcher is `eq(...)`, it can be omitted:
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
///     a_field: "this".into(),
///     another_field: "that".into()
/// };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_field: "this",
///     another_field: "that",
/// }))
/// #     .unwrap();
/// ```
///
/// **Important**: The method should be pure function with a deterministic
/// output and no side effects. In particular, in the event of an assertion
/// failure, it will be invoked a second time, with the assertion failure output
/// reflecting the *second* invocation.
///
/// These may also include extra litteral parameters you pass in:
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
/// verify_that!(my_struct, matches_pattern!(&MyStruct {
///     append_to_a_field("a suffix"): ref ends_with("a suffix"),
/// }))
/// #     .unwrap();
/// ```
///
/// You can precede both field and property matchers with a `ref` to match the
/// result by reference:
///
/// ```
/// # use googletest::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_field: String,
/// # }
/// #
/// impl MyStruct {
///     fn get_a_field_ref(&self) -> String { self.a_field.clone() }
/// }
///
/// # let my_struct = MyStruct { a_field: "Something to believe in".into() };
/// verify_that!(my_struct, matches_pattern!(&MyStruct {
///     get_a_field_ref(): ref starts_with("Something"),
/// }))
/// #    .unwrap();
/// ```
///
/// Note that if the `actual` is of type `&ActualT` and the pattern type is
/// `ActualT`, this is automatically performed. This behavior is similar to the
/// reference binding mode in pattern matching.
///
/// ```
/// # use googletest::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_field: String,
/// # }
/// #
/// impl MyStruct {
///     fn get_a_field_ref(&self) -> String { self.a_field.clone() }
/// }
///
/// # let my_struct = MyStruct { a_field: "Something to believe in".into() };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     get_a_field_ref(): starts_with("Something"),
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
///     matches_pattern!(&MyTupleStruct(ref eq("Something"), ref eq("Some other thing")))
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
/// verify_that!(MyEnum::A(123), matches_pattern!(&MyEnum::A(eq(123))))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(MyEnum::B, matches_pattern!(&MyEnum::A(eq(123))))?; // Fails - wrong enum variant
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// This macro does not support plain (non-struct) tuples. But it should not be
/// necessary as tuple of matchers are matchers of tuple. In other words, if
/// `MatcherU: Matcher<U>` and `MatcherT: Matcher<T>`, then `(MatcherU,
/// MatcherT): Matcher<(U, T)>`.
///
/// Trailing commas are allowed (but not required) in both ordinary and tuple
/// structs.
///
/// Note that the default format (rustfmt) can format macros if the macro
/// argument is parseable Rust code. This is mostly true for this macro with two
/// exceptions:
///  * property matching
///  * `ref` keyword with named fields
///
/// An option for formatting large is to avoid these exceptions (by removing the
/// parenthesis of properties and the `ref` keywords), run `rustfmt` and add
/// them back.
#[macro_export]
#[doc(hidden)]
macro_rules! __matches_pattern {
    ($($t:tt)*) => { $crate::matches_pattern_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! matches_pattern_internal {
    (
        @name [$($struct_name:tt)*],
        { $field_name:ident : ref $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(field!($($struct_name)*.$field_name, ref $matcher))
        )
    };

    (
        @name [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(field!($($struct_name)*.$field_name, $matcher))
        )
    };

    (
        @name [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : ref $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(property!($($struct_name)*.$property_name($($argument),*), ref $matcher))
        )
    };

    (
        @name [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(property!($($struct_name)*.$property_name($($argument),*), $matcher))
        )
    };

    (
        @name [$($struct_name:tt)*],
        { $field_name:ident : ref $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            @fields (field!($($struct_name)*.$field_name, ref $matcher)),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        @name [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            @fields (field!($($struct_name)*.$field_name, $matcher)),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        @name [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : ref $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            @fields (property!($($struct_name)*.$property_name($($argument),*), ref $matcher)),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        @name [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            @fields (property!($($struct_name)*.$property_name($($argument),*), $matcher)),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        { $field_name:ident : ref $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(
                $($processed)*,
                field!($($struct_name)*.$field_name, ref $matcher)
            ))
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(
                $($processed)*,
                field!($($struct_name)*.$field_name, $matcher)
            ))
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : ref $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(
                $($processed)*,
                property!($($struct_name)*.$property_name($($argument),*), ref $matcher)
            ))
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(
                $($processed)*,
                property!($($struct_name)*.$property_name($($argument),*), $matcher)
            ))
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        { $field_name:ident : ref $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.$field_name, ref $matcher)
            ),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        { $field_name:ident : $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.$field_name, $matcher)
            ),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : ref $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                property!(ref $($struct_name)*.$property_name($($argument),*), $matcher)
            ),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr, $($rest:tt)* }
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                property!($($struct_name)*.$property_name($($argument),*), $matcher)
            ),
            [$($struct_name)*],
            { $($rest)* }
        )
    };

    (
        @name [$($struct_name:tt)*],
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::pattern_only(
            |v| matches!(v, $($struct_name)*),
            concat!("is ", stringify!($($struct_name)*)),
            concat!("is not ", stringify!($($struct_name)*)))
    };

    (
        @name [$($struct_name:tt)*],
        (ref $matcher:expr $(,)?)
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(field!($($struct_name)*.0, ref $matcher))
        )
    };

    (
        @name [$($struct_name:tt)*],
        ($matcher:expr $(,)?)
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(field!($($struct_name)*.0, $matcher))
        )
    };

    (
        @name [$($struct_name:tt)*],
        (ref $matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                field!($($struct_name)*.0, ref $matcher)
            ),
            [$($struct_name)*],
            1,
            ($($rest)*)
        )
    };

    (
        @name [$($struct_name:tt)*],
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                field!($($struct_name)*.0, $matcher)
            ),
            [$($struct_name)*],
            1,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        $field:tt,
        (ref $matcher:expr $(,)?)
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(
                $($processed)*,
                field!($($struct_name)*.$field, ref $matcher)
            ))
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        $field:tt,
        ($matcher:expr $(,)?)
    ) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!($($struct_name)*),
            all!(
                $($processed)*,
                field!($($struct_name)*.$field, $matcher)
            ))
    };

    // We need to repeat this once for every supported field position, unfortunately. There appears
    // to be no way in declarative macros to compute $field + 1 and have the result evaluated to a
    // token which can be used as a tuple index.
    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        1,
        (ref $matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.1, ref $matcher)
            ),
            [$($struct_name)*],
            2,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        1,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.1, $matcher)
            ),
            [$($struct_name)*],
            2,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        2,
        (ref $matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.2, ref $matcher)
            ),
            [$($struct_name)*],
            3,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        2,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.2, $matcher)
            ),
            [$($struct_name)*],
            3,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        3,
        (ref $matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.3, ref $matcher)
            ),
            [$($struct_name)*],
            4,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        3,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.3, $matcher)
            ),
            [$($struct_name)*],
            4,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        4,
        (ref $matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.4, ref $matcher)
            ),
            [$($struct_name)*],
            5,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        4,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.4, $matcher)
            ),
            [$($struct_name)*],
            5,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        5,
        (ref $matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.5, ref $matcher)
            ),
            [$($struct_name)*],
            6,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        5,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.5, $matcher)
            ),
            [$($struct_name)*],
            6,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        6,
        (ref $matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.6, ref $matcher)
            ),
            [$($struct_name)*],
            7,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        6,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.6, $matcher)
            ),
            [$($struct_name)*],
            7,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        7,
        (ref $matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.7, ref $matcher)
            ),
            [$($struct_name)*],
            8,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        7,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.7, $matcher)
            ),
            [$($struct_name)*],
            8,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        8,
        (ref $matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.8, ref $matcher)
            ),
            [$($struct_name)*],
            9,
            ($($rest)*)
        )
    };

    (
        @fields ($($processed:tt)*),
        [$($struct_name:tt)*],
        8,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            @fields (
                $($processed)*,
                field!($($struct_name)*.8, $matcher)
            ),
            [$($struct_name)*],
            9,
            ($($rest)*)
        )
    };

    (@name [$($struct_name:tt)*], $first:tt $($rest:tt)*) => {
        $crate::matches_pattern_internal!(@name [$($struct_name)* $first], $($rest)*)
    };

    ($first:tt $($rest:tt)*) => {{
        #[allow(unused)]
        use $crate::matchers::{all, field, property};
        $crate::matches_pattern_internal!(@name [$first], $($rest)*)
    }};
}

/// An alias for [`matches_pattern`][crate::matchers::matches_pattern!].
#[macro_export]
#[doc(hidden)]
macro_rules! __pat {
    ($($t:tt)*) => { $crate::matches_pattern_internal!($($t)*) }
}

#[doc(hidden)]
pub mod internal {
    use crate::matcher::{Matcher, MatcherBase};
    use std::fmt::Debug;

    // Specialized implementation of the `predicate` matcher to support ref binding
    // mode for `matches_pattern`.
    pub fn pattern_only<T>(
        matcher_function: fn(&T) -> bool,
        match_description: &'static str,
        no_match_description: &'static str,
    ) -> PatternOnlyMatcher<T> {
        PatternOnlyMatcher { matcher_function, match_description, no_match_description }
    }

    #[derive(MatcherBase)]
    #[doc(hidden)]
    pub struct PatternOnlyMatcher<T> {
        matcher_function: fn(&T) -> bool,
        match_description: &'static str,
        no_match_description: &'static str,
    }

    impl<'a, T: Debug> Matcher<&'a T> for PatternOnlyMatcher<T> {
        fn matches(&self, actual: &'a T) -> crate::matcher::MatcherResult {
            (self.matcher_function)(actual).into()
        }

        fn describe(
            &self,
            matcher_result: crate::matcher::MatcherResult,
        ) -> crate::description::Description {
            match matcher_result {
                crate::matcher::MatcherResult::Match => self.match_description.into(),
                crate::matcher::MatcherResult::NoMatch => self.no_match_description.into(),
            }
        }
    }

    impl<T: Debug + Copy> Matcher<T> for PatternOnlyMatcher<T> {
        fn matches(&self, actual: T) -> crate::matcher::MatcherResult {
            (self.matcher_function)(&actual).into()
        }

        fn describe(
            &self,
            matcher_result: crate::matcher::MatcherResult,
        ) -> crate::description::Description {
            match matcher_result {
                crate::matcher::MatcherResult::Match => self.match_description.into(),
                crate::matcher::MatcherResult::NoMatch => self.no_match_description.into(),
            }
        }
    }
}
