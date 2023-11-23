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
/// verify_that!(value, contains(property!(MyStruct.get_a_field(), eq(100))))
/// #    .unwrap();
/// ```
///
/// **Important**: The method should be pure function with a deterministic
/// output and no side effects. In particular, in the event of an assertion
/// failure, it will be invoked a second time, with the assertion failure output
/// reflecting the *second* invocation.
///
/// If the method returns a *reference*, then it must be preceded by a `*`:
///
/// ```
/// # use googletest::prelude::*;
/// # #[derive(Debug)]
/// # pub struct MyStruct {
/// #     a_field: u32,
/// # }
/// impl MyStruct {
///     pub fn get_a_field(&self) -> &u32 { &self.a_field }
/// }
///
/// # let value = vec![MyStruct { a_field: 100 }];
/// verify_that!(value, contains(property!(*MyStruct.get_a_field(), eq(100))))
/// #    .unwrap();
/// ```
///
/// The method may also take additional arguments:
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
/// verify_that!(value, contains(property!(MyStruct.add_to_a_field(50), eq(150))))
/// #    .unwrap();
/// ```
///
/// Unfortunately, this matcher does *not* work with methods returning string
/// slices:
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_string: String,
/// }
/// impl MyStruct {
///     pub fn get_a_string(&self) -> &str { &self.a_string }
/// }
///
/// let value = MyStruct { a_string: "A string".into() };
/// verify_that!(value, property!(*MyStruct.get_a_string(), eq("A string"))) // Does not compile
/// #    .unwrap();
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
    ($($t:ident)::+.$method:tt($($argument:tt),* $(,)?), $m:expr) => {{
         use $crate::matchers::__internal_unstable_do_not_depend_on_these::property_matcher;
        property_matcher(
            |o: &$($t)::+| o.$method($($argument),*),
            &stringify!($method($($argument),*)),
            $m)
    }};

    (* $($t:ident)::+.$method:tt($($argument:tt),* $(,)?), $m:expr) => {{
        use $crate::matchers::__internal_unstable_do_not_depend_on_these::property_ref_matcher;
        property_ref_matcher(
            |o: &$($t)::+| o.$method($($argument),*),
            &stringify!($method($($argument),*)),
            $m)
    }};
}

/// Items for use only by the declarative macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::{
        description::Description,
        matcher::{Matcher, MatcherResult},
    };
    use std::{fmt::Debug, marker::PhantomData};

    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn property_matcher<OuterT: Debug, InnerT: Debug, MatcherT: Matcher<ActualT = InnerT>>(
        extractor: impl Fn(&OuterT) -> InnerT,
        property_desc: &'static str,
        inner: MatcherT,
    ) -> impl Matcher<ActualT = OuterT> {
        PropertyMatcher { extractor, property_desc, inner, phantom: Default::default() }
    }

    struct PropertyMatcher<OuterT, ExtractorT, MatcherT> {
        extractor: ExtractorT,
        property_desc: &'static str,
        inner: MatcherT,
        phantom: PhantomData<OuterT>,
    }

    impl<InnerT, OuterT, ExtractorT, MatcherT> Matcher for PropertyMatcher<OuterT, ExtractorT, MatcherT>
    where
        InnerT: Debug,
        OuterT: Debug,
        ExtractorT: Fn(&OuterT) -> InnerT,
        MatcherT: Matcher<ActualT = InnerT>,
    {
        type ActualT = OuterT;

        fn matches(&self, actual: &OuterT) -> MatcherResult {
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

        fn explain_match(&self, actual: &OuterT) -> Description {
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
    pub fn property_ref_matcher<OuterT, InnerT, MatcherT>(
        extractor: fn(&OuterT) -> &InnerT,
        property_desc: &'static str,
        inner: MatcherT,
    ) -> impl Matcher<ActualT = OuterT>
    where
        OuterT: Debug,
        InnerT: Debug + ?Sized,
        MatcherT: Matcher<ActualT = InnerT>,
    {
        PropertyRefMatcher { extractor, property_desc, inner }
    }

    struct PropertyRefMatcher<InnerT: ?Sized, OuterT, MatcherT> {
        extractor: fn(&OuterT) -> &InnerT,
        property_desc: &'static str,
        inner: MatcherT,
    }

    impl<InnerT: Debug + ?Sized, OuterT: Debug, MatcherT: Matcher<ActualT = InnerT>> Matcher
        for PropertyRefMatcher<InnerT, OuterT, MatcherT>
    {
        type ActualT = OuterT;

        fn matches(&self, actual: &OuterT) -> MatcherResult {
            self.inner.matches((self.extractor)(actual))
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "has property `{}`, which {}",
                self.property_desc,
                self.inner.describe(matcher_result)
            )
            .into()
        }

        fn explain_match(&self, actual: &OuterT) -> Description {
            let actual_inner = (self.extractor)(actual);
            format!(
                "whose property `{}` is `{:#?}`, {}",
                self.property_desc,
                actual_inner,
                self.inner.explain_match(actual_inner)
            )
            .into()
        }
    }
}
