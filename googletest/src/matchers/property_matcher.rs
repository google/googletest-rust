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

// There are no visible documentation elements in this module; the declarative
// macro is documented in the matcher module.
#![doc(hidden)]

/// Matches an object which, upon calling the given method on it with the given
/// arguments, produces a value matched by the given inner matcher.
///
/// This is particularly useful as a nested matcher when the desired
/// property cannot be accessed through a field and must instead be
/// extracted through a method call. For example:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_field: u32,
/// }
///
/// impl MyStruct {
///     pub fn get_a_field(&self) -> u32 { self.a_field }
/// }
///
/// let value = vec![MyStruct { a_field: 100 }];
/// verify_that!(value, contains(property!(&MyStruct.get_a_field(), eq(100))))
/// #    .unwrap();
/// ```
///
///
/// If the inner matcher is `eq(...)`, it can be omitted:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_field: u32,
/// }
///
/// impl MyStruct {
///     pub fn get_a_field(&self) -> u32 { self.a_field }
/// }
///
/// let value = vec![MyStruct { a_field: 100 }];
/// verify_that!(value, contains(property!(&MyStruct.get_a_field(), 100)))
/// #    .unwrap();
/// ```
///
/// **Important**: The method should be pure function with a deterministic
/// output and no side effects. In particular, in the event of an assertion
/// failure, it will be invoked a second time, with the assertion failure output
/// reflecting the *second* invocation.
///
/// The method may also take additional litteral arguments:
///
/// ```
/// # use googletest::prelude::*;
/// # #[derive(Debug)]
/// # pub struct MyStruct {
/// #     a_field: u32,
/// # }
/// impl MyStruct {
///     pub fn add_to_a_field(&self, a: u32) -> u32 { self.a_field + a }
/// }
///
/// # let value = vec![MyStruct { a_field: 100 }];
/// verify_that!(value, contains(property!(&MyStruct.add_to_a_field(50), eq(150))))
/// #    .unwrap();
/// ```
///
/// The arguments must be litteral as `property!` is not able to capture them.
///
/// # Specification of the property pattern
///
/// The specification of the field follow the syntax: `(ref)? (&)?
/// $TYPE.$PROPERTY\($ARGUMENT\)`.
///
/// The `&` allows to specify whether this matcher matches against an actual of
/// type `$TYPE` (`$TYPE` must implement `Copy`) or a `&$TYPE`.
///
/// For instance:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// pub struct AStruct;
///
/// impl AStruct {
///   fn a_property(&self) -> i32 {32}
/// }
/// # fn should_pass() -> Result<()> {
/// verify_that!(AStruct, property!(&AStruct.a_property(), eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug, Clone, Copy)]
/// pub struct AStruct;
///
/// impl AStruct {
///   fn a_property(self) -> i32 {32}
/// }
/// # fn should_pass() -> Result<()> {
/// verify_that!(AStruct, property!(AStruct.a_property(), eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// The `ref` allows to bind the property returned value by reference, which is
/// required if the field type does not implement `Copy`.
///
/// For instance:
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// pub struct AStruct;
///
/// impl AStruct {
///   fn a_property(&self) -> i32 {32}
/// }
/// # fn should_pass() -> Result<()> {
/// verify_that!(AStruct, property!(&AStruct.a_property(), eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// If `property!` is qualified by both `&` and `ref`, they can both be omitted.
///
/// ```
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// pub struct AStruct;
///
/// impl AStruct {
///   fn a_property(&self) -> String {"32".into()}
/// }
/// # fn should_pass() -> Result<()> {
/// verify_that!(AStruct, property!(&AStruct.a_property(), ref eq("32")))?;
/// verify_that!(AStruct, property!(AStruct.a_property(), eq("32")))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// This macro is analogous to [`field`][crate::matchers::field], except that it
/// extracts the datum to be matched from the given object by invoking a method
/// rather than accessing a field.
///
/// The list of arguments may optionally have a trailing comma.
#[macro_export]
#[doc(hidden)]
macro_rules! __property {
    ($($t:tt)*) => { $crate::property_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! property_internal {

    (&$($t:ident)::+.$method:tt($($argument:expr),* $(,)?), ref $m:expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::property_ref_matcher(
            |o: &$($t)::+| $($t)::+::$method(o, $($argument),*),
            &stringify!($method($($argument),*)),
            $crate::matcher_support::__internal_unstable_do_not_depend_on_these::auto_eq!($m))
    }};
    ($($t:ident)::+.$method:tt($($argument:expr),* $(,)?), ref $m:expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::property_ref_matcher(
            |o: $($t)::+| $($t)::+::$method(o, $($argument),*),
            &stringify!($method($($argument),*)),
            $crate::matcher_support::__internal_unstable_do_not_depend_on_these::auto_eq!($m))
    }};
    (& $($t:ident)::+.$method:tt($($argument:expr),* $(,)?), $m:expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::property_matcher(
            |o: &&$($t)::+| o.$method($($argument),*),
            &stringify!($method($($argument),*)),
            $crate::matcher_support::__internal_unstable_do_not_depend_on_these::auto_eq!($m))
    }};
    ($($t:ident)::+.$method:tt($($argument:expr),* $(,)?), $m:expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::property_matcher(
            |o: &$($t)::+| o.$method($($argument),*),
            &stringify!($method($($argument),*)),
            $crate::matcher_support::__internal_unstable_do_not_depend_on_these::auto_eq!($m))
    }};
}

/// Items for use only by the declarative macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::{
        description::Description,
        matcher::{Matcher, MatcherBase, MatcherResult},
    };
    use std::fmt::Debug;

    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn property_matcher<OuterT: Debug, InnerT: Debug, MatcherT>(
        extractor: fn(&OuterT) -> InnerT,
        property_desc: &'static str,
        inner: MatcherT,
    ) -> PropertyMatcher<OuterT, InnerT, MatcherT> {
        PropertyMatcher { extractor, property_desc, inner }
    }

    #[derive(MatcherBase)]
    pub struct PropertyMatcher<OuterT, InnerT, MatcherT> {
        extractor: fn(&OuterT) -> InnerT,
        property_desc: &'static str,
        inner: MatcherT,
    }

    impl<InnerT, OuterT, MatcherT> Matcher<OuterT> for PropertyMatcher<OuterT, InnerT, MatcherT>
    where
        InnerT: Debug + Copy,
        OuterT: Debug + Copy,
        MatcherT: Matcher<InnerT>,
    {
        fn matches(&self, actual: OuterT) -> MatcherResult {
            self.inner.matches((self.extractor)(&actual))
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "has property `{}`, which {}",
                self.property_desc,
                self.inner.describe(matcher_result)
            )
            .into()
        }

        fn explain_match(&self, actual: OuterT) -> Description {
            let actual_inner = (self.extractor)(&actual);
            format!(
                "whose property `{}` is `{:#?}`, {}",
                self.property_desc,
                actual_inner,
                self.inner.explain_match(actual_inner)
            )
            .into()
        }
    }

    impl<'a, InnerT, OuterT, MatcherT> Matcher<&'a OuterT> for PropertyMatcher<OuterT, InnerT, MatcherT>
    where
        InnerT: Debug,
        OuterT: Debug,
        MatcherT: for<'b> Matcher<&'b InnerT>,
    {
        fn matches(&self, actual: &'a OuterT) -> MatcherResult {
            self.inner.matches(&(self.extractor)(actual))
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "has property `{}`, which {}",
                self.property_desc,
                self.inner.describe(matcher_result)
            )
            .into()
        }

        fn explain_match(&self, actual: &'a OuterT) -> Description {
            let actual_inner = (self.extractor)(actual);
            format!(
                "whose property `{}` is `{:#?}`, {}",
                self.property_desc,
                actual_inner,
                self.inner.explain_match(&actual_inner)
            )
            .into()
        }
    }

    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn property_ref_matcher<OuterT, InnerT, ExtractorT, MatcherT>(
        extractor: ExtractorT,
        property_desc: &'static str,
        inner: MatcherT,
    ) -> PropertyRefMatcher<ExtractorT, MatcherT>
    where
        OuterT: Debug,
        InnerT: Debug,
        MatcherT: for<'a> Matcher<&'a InnerT>,
        ExtractorT: Fn(OuterT) -> InnerT,
    {
        PropertyRefMatcher { extractor, property_desc, inner }
    }

    #[derive(MatcherBase)]
    pub struct PropertyRefMatcher<ExtractorT, MatcherT> {
        extractor: ExtractorT,
        property_desc: &'static str,
        inner: MatcherT,
    }

    impl<
            InnerT: Debug,
            OuterT: Debug + Copy,
            MatcherT: for<'a> Matcher<&'a InnerT>,
            ExtractorT: Fn(OuterT) -> InnerT,
        > Matcher<OuterT> for PropertyRefMatcher<ExtractorT, MatcherT>
    {
        fn matches(&self, actual: OuterT) -> MatcherResult {
            self.inner.matches(&(self.extractor)(actual))
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "has property `{}`, which {}",
                self.property_desc,
                self.inner.describe(matcher_result)
            )
            .into()
        }

        fn explain_match(&self, actual: OuterT) -> Description {
            let actual_inner = (self.extractor)(actual);
            format!(
                "whose property `{}` is `{:#?}`, {}",
                self.property_desc,
                actual_inner,
                self.inner.explain_match(&actual_inner)
            )
            .into()
        }
    }
}
