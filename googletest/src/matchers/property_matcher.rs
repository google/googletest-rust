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
// macro is documented at the top level.
#![doc(hidden)]

#[cfg(google3)]
use googletest::*;

/// Matches an object which, upon calling the given method on it with the given
/// arguments, produces a value matched by the given inner matcher.
///
/// This is particularly useful as a nested matcher when the desired
/// property cannot be accessed through a field and must instead be
/// extracted through a method call. For example:
///
/// ```
/// # use googletest::{matchers::{contains, eq}, property, verify_that};
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
/// If the method returns a *reference*, then it must be preceded by the keyword
/// `ref`:
///
/// ```
/// # use googletest::{matchers::{contains, eq}, property, verify_that};
/// # #[derive(Debug)]
/// # pub struct MyStruct {
/// #     a_field: u32,
/// # }
/// impl MyStruct {
///     pub fn get_a_field(&self) -> &u32 { &self.a_field }
/// }
///
/// # let value = vec![MyStruct { a_field: 100 }];
/// verify_that!(value, contains(property!(ref MyStruct.get_a_field(), eq(100))))
/// #    .unwrap();
/// ```
///
/// > Note: At the moment, this does not work properly with methods returning
/// > string references or slices.
///
/// The method may also take additional arguments:
///
/// ```
/// # use googletest::{matchers::{contains, eq}, property, verify_that};
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
/// > **Note**: The method should be pure function with a deterministic output
/// > and no side effects. In particular, in the event of an assertion failure,
/// > it will be invoked a second time, with the assertion failure output
/// > reflecting the *second* invocation.
///
/// This macro is analogous to [`field`][crate::field], except that it extracts
/// the datum to be matched from the given object by invoking a method rather
/// than accessing a field.
///
/// The list of arguments may optionally have a trailing comma.
#[macro_export]
macro_rules! property {
    ($($t:tt)*) => { $crate::property_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! property_internal {
    ($($t:ident)::+.$method:tt($($argument:tt),* $(,)?), $m:expr) => {{
        #[cfg(google3)]
        use $crate::internal::property_matcher;
        #[cfg(not(google3))]
        use $crate::matchers::property_matcher::internal::property_matcher;
        property_matcher(
            |o: &$($t)::+| o.$method($($argument),*),
            &stringify!($method($($argument),*)),
            $m)
    }};

    (ref $($t:ident)::+.$method:tt($($argument:tt),* $(,)?), $m:expr) => {{
        #[cfg(google3)]
        use $crate::internal::property_ref_matcher;
        #[cfg(not(google3))]
        use $crate::matchers::property_matcher::internal::property_ref_matcher;
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
    use crate::matcher::{MatchExplanation, Matcher, MatcherResult};
    use std::{fmt::Debug, marker::PhantomData};

    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn property_matcher<OuterT: Debug, InnerT: Debug, MatcherT: Matcher>(
        extractor: impl Fn(&OuterT) -> InnerT,
        property_desc: &'static str,
        inner: MatcherT,
    ) -> impl Matcher {
        PropertyMatcher { extractor, property_desc, inner, phantom: Default::default() }
    }

    struct PropertyMatcher<OuterT, ExtractorT, MatcherT> {
        extractor: ExtractorT,
        property_desc: &'static str,
        inner: MatcherT,
        phantom: PhantomData<OuterT>,
    }

    impl<InnerT: Debug, OuterT: Debug, ExtractorT: Fn(&OuterT) -> InnerT, MatcherT: Matcher> Matcher
        for PropertyMatcher<OuterT, ExtractorT, MatcherT>
    {
        fn matches(&self, actual: &OuterT) -> MatcherResult {
            self.inner.matches(&(self.extractor)(actual))
        }

        fn describe(&self, matcher_result: MatcherResult) -> String {
            format!(
                "has property `{}`, which {}",
                self.property_desc,
                self.inner.describe(matcher_result)
            )
        }

        fn explain_match(&self, actual: &OuterT) -> MatchExplanation {
            let actual_inner = (self.extractor)(actual);
            MatchExplanation::create(format!(
                "whose property `{}` is `{:#?}`, {}",
                self.property_desc,
                actual_inner,
                self.inner.explain_match(&actual_inner)
            ))
        }
    }

    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn property_ref_matcher<OuterT, InnerT, MatcherT>(
        extractor: fn(&OuterT) -> &InnerT,
        property_desc: &'static str,
        inner: MatcherT,
    ) -> impl Matcher
    where
        OuterT: Debug,
        InnerT: Debug + ?Sized,
        MatcherT: Matcher,
    {
        PropertyRefMatcher { extractor, property_desc, inner }
    }

    struct PropertyRefMatcher<InnerT: ?Sized, OuterT, MatcherT> {
        extractor: fn(&OuterT) -> &InnerT,
        property_desc: &'static str,
        inner: MatcherT,
    }

    impl<InnerT: Debug + ?Sized, OuterT: Debug, MatcherT: Matcher> Matcher
        for PropertyRefMatcher<InnerT, OuterT, MatcherT>
    {
        fn matches(&self, actual: &OuterT) -> MatcherResult {
            self.inner.matches((self.extractor)(actual))
        }

        fn describe(&self, matcher_result: MatcherResult) -> String {
            format!(
                "has property `{}`, which {}",
                self.property_desc,
                self.inner.describe(matcher_result)
            )
        }

        fn explain_match(&self, actual: &OuterT) -> MatchExplanation {
            let actual_inner = (self.extractor)(actual);
            MatchExplanation::create(format!(
                "whose property `{}` is `{:#?}`, {}",
                self.property_desc,
                actual_inner,
                self.inner.explain_match(actual_inner)
            ))
        }
    }
}
