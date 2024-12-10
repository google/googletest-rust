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
/// If any fields are provided in the pattern, then all fields must be
/// specified, or the pattern must end with `..`, just like regular match
/// patterns. Omitted fields have no effect on the output of the matcher.
/// The `..` is unnecessary when no fields are provided and only method
/// values are checked.
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
///     .. // another_field is missing, so it may be anything.
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
    ($($tt:tt)*) => {
        {
            use $crate::{self as googletest};
            #[allow(unused)]
            use $crate::matchers::{all, field, property};
            $crate::matchers::__internal_unstable_do_not_depend_on_these::__googletest_macro_matches_pattern!($($tt)*)
        }
    };
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

    pub use ::googletest_macro::__googletest_macro_matches_pattern;

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

    /// A matcher that ensures that the passed-in function compiles with the
    /// benefit of inference from the value being tested and the attached
    /// matcher invokes as well.
    ///
    /// It forwards all description responsibilities to the passed-in matcher.
    pub fn compile_assert_and_match<T, M>(
        must_compile_function: fn(&T),
        matcher: M,
    ) -> CompileAssertAndMatch<T, M> {
        CompileAssertAndMatch { must_compile_function, matcher }
    }

    #[derive(MatcherBase)]
    #[doc(hidden)]
    pub struct CompileAssertAndMatch<T, M> {
        #[allow(dead_code)]
        must_compile_function: fn(&T),
        matcher: M,
    }

    impl<'a, T: Debug, M> Matcher<&'a T> for CompileAssertAndMatch<T, M>
    where
        M: Matcher<&'a T>,
    {
        fn matches(&self, actual: &'a T) -> crate::matcher::MatcherResult {
            self.matcher.matches(actual)
        }

        fn describe(
            &self,
            matcher_result: crate::matcher::MatcherResult,
        ) -> crate::description::Description {
            self.matcher.describe(matcher_result)
        }

        fn explain_match(&self, actual: &'a T) -> crate::description::Description {
            self.matcher.explain_match(actual)
        }
    }

    impl<T: Debug + Copy, M> Matcher<T> for CompileAssertAndMatch<T, M>
    where
        M: Matcher<T>,
    {
        fn matches(&self, actual: T) -> crate::matcher::MatcherResult {
            self.matcher.matches(actual)
        }

        fn describe(
            &self,
            matcher_result: crate::matcher::MatcherResult,
        ) -> crate::description::Description {
            self.matcher.describe(matcher_result)
        }

        fn explain_match(&self, actual: T) -> crate::description::Description {
            self.matcher.explain_match(actual)
        }
    }
}

mod compile_fail_tests {
    /// ```compile_fail
    /// use ::googletest::prelude::*;
    /// #[derive(Debug)]
    /// struct Foo { a: u32 }
    /// impl Foo {
    ///   fn b() {}
    /// }
    /// let actual = Foo { a: 1 };
    /// verify_that!(actual, matches_pattern!(Foo { a: eq(&1), b(): _ }));
    /// ```
    fn _underscore_unsupported_for_methods() {}

    /// ```compile_fail
    /// use ::googletest::prelude::*;
    /// #[derive(Debug)]
    /// struct Foo { a: u32, b: u32 }
    /// let actual = Foo { a: 1, b: 2 };
    /// verify_that!(actual, matches_pattern!(Foo { a: eq(&1), .., }));
    /// ```
    fn _dot_dot_supported_only_at_end_of_struct_pattern() {}

    /// ```compile_fail
    /// use ::googletest::prelude::*;
    /// #[derive(Debug)]
    /// struct Foo(u32, u32);
    /// let actual = Foo(1, 2);
    /// verify_that!(actual, matches_pattern!(Foo(eq(&1), .., )));
    /// ```
    fn _dot_dot_supported_only_at_end_of_tuple_struct_pattern() {}

    /// ```compile_fail
    /// use ::googletest::prelude::*;
    /// #[derive(Debug)]
    /// struct Foo { a: u32, b: u32 }
    /// let actual = Foo { a: 1, b: 2 };
    /// verify_that!(actual, matches_pattern!(Foo { a: eq(&1) }));
    /// ```
    fn _unexhaustive_struct_field_check_requires_dot_dot() {}

    /// ```compile_fail
    /// use ::googletest::prelude::*;
    /// #[derive(Debug)]
    /// enum Foo {
    ///     Bar { a: u32, b: u32 }
    /// }
    /// let actual = Foo::Bar { a: 1, b: 2 };
    /// verify_that!(actual, matches_pattern!(Foo::Bar { a: eq(&1) }));
    /// ```
    fn _unexhaustive_enum_struct_field_check_requires_dot_dot() {}

    /// ```compile_fail
    /// use ::googletest::prelude::*;
    /// #[derive(Debug)]
    /// struct Foo(u32, u32, u32);
    /// let actual = Foo(1, 2, 3);
    /// verify_that!(actual, matches_pattern!(Foo(eq(&1), eq(&2) )));
    /// ```
    fn _unexhaustive_tuple_struct_field_check_requires_dot_dot() {}
}
