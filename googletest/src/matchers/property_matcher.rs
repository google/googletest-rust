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
        $($t:ident)::+ $(::<$($t_ty_args:ty),* $(,)?>)?
        .$method:tt $(::<$($m_ty_args:ty),* $(,)?>)? ($($argument:expr),* $(,)?),
        ref $m:expr
    ) => {{
        $crate::property_internal!(@assert_empty $($($_amp)*)* $($($_cs)*)*);
        $crate::property_internal!(
            @self_arg
            struct_type:   [$(& $($_amp)*)*  $(:: $($_cs)*)*  $($t)::+ $(  <$($t_ty_args),*>)*]
            method_prefix: [                 $(:: $($_cs)*)*  $($t)::+ $(::<$($t_ty_args),*>)*]
            [$method $(::<$($m_ty_args),*>)*] [$($argument),*] [$m])
    }};

    // Non-`ref` variant.
    (
        // See comment on previous variant above.
        $(& $($_amp:literal)?)? // Optional `&`'s presence is implicitly captured by `_amp`.
        $(:: $($_cs:literal)?)? // Optional `::`'s presence is implicitly captured by `_cs`.
        $($t:ident)::+ $(::<$($t_ty_args:ty),* $(,)?>)?
        .$method:tt $(::<$($m_ty_args:ty),* $(,)?>)? ($($argument:expr),* $(,)?),
        $m:expr) => {{
        $crate::property_internal!(@assert_empty $($($_amp)*)* $($($_cs)*)*);
        $crate::property_internal!(
            @self_dot
            struct_type: [$(& $($_amp)*)*  $(:: $($_cs)*)*  $($t)::+ $(<$($t_ty_args),*>)*]
            [$method $(::<$($m_ty_args),*>)*] [$($argument),*] [$m])
    }};

    (@assert_empty) => {};
    (@assert_empty $($l:literal)+) => {
        compile_error!("property! argument must start with an optional `&` followed by a path")
    };

    (@self_arg struct_type: [$struct_ty:ty]
               method_prefix: [$($method_prefix:tt)+]
               [$($method:tt)*] [$($argument:expr),*] [$m:expr]) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::property_ref_matcher(
            |o: $struct_ty| $($method_prefix)*::$($method)* (o, $($argument),*),
            &stringify!($($method)* ($($argument),*)),
            $crate::matcher_support::__internal_unstable_do_not_depend_on_these::auto_eq!($m))
    }};

    (@self_dot struct_type: [$struct_ty:ty]
               [$($method:tt)*] [$($argument:expr),*] [$m:expr]) => {{
        match $crate::matcher_support::__internal_unstable_do_not_depend_on_these::auto_eq!($m) {
            inner => {
                #[allow(unused_imports)]
                use $crate::matchers::__internal_unstable_do_not_depend_on_these::{
                    PropertyByRef as _, PropertyByValue as _, PropertyFallback as _,
                    PropertyModeSelector,
                };
                (&&PropertyModeSelector::new(
                    |o: &$struct_ty| o.$($method)* ($($argument),*),
                    &stringify!($($method)* ($($argument),*)),
                    &inner,
                ))
                    .select_mode()
                    .build(inner)
            }
        }
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
    use std::marker::PhantomData;

    #[doc(hidden)]
    pub struct ByValueMode;

    #[doc(hidden)]
    pub struct ByRefMode;

    #[doc(hidden)]
    pub struct PropertyBuilder<OuterT, InnerT, Mode> {
        extractor: fn(&OuterT) -> InnerT,
        property_desc: &'static str,
        mode: PhantomData<Mode>,
    }

    impl<OuterT, InnerT, Mode> PropertyBuilder<OuterT, InnerT, Mode> {
        #[inline]
        pub fn build<MatcherT>(
            self,
            inner: MatcherT,
        ) -> PropertyMatcher<OuterT, InnerT, MatcherT, Mode> {
            PropertyMatcher {
                extractor: self.extractor,
                property_desc: self.property_desc,
                inner,
                mode: PhantomData,
            }
        }
    }

    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn property_matcher<OuterT: Debug, InnerT: Debug, MatcherT>(
        extractor: fn(&OuterT) -> InnerT,
        property_desc: &'static str,
        inner: MatcherT,
    ) -> PropertyMatcher<OuterT, InnerT, MatcherT, ByValueMode> {
        PropertyMatcher { extractor, property_desc, inner, mode: PhantomData }
    }

    #[doc(hidden)]
    pub struct PropertyModeSelector<OuterT, InnerT, MatcherT> {
        extractor: fn(&OuterT) -> InnerT,
        property_desc: &'static str,
        _marker: PhantomData<fn() -> MatcherT>,
    }

    impl<OuterT, InnerT, MatcherT> PropertyModeSelector<OuterT, InnerT, MatcherT> {
        #[inline]
        pub fn new(
            extractor: fn(&OuterT) -> InnerT,
            property_desc: &'static str,
            _inner: &MatcherT,
        ) -> Self {
            Self { extractor, property_desc, _marker: PhantomData }
        }
    }

    #[doc(hidden)]
    pub trait PropertyByValue<OuterT, InnerT> {
        fn select_mode(self) -> PropertyBuilder<OuterT, InnerT, ByValueMode>;
    }

    impl<OuterT, InnerT: Debug, MatcherT: Matcher<InnerT>> PropertyByValue<OuterT, InnerT>
        for &&PropertyModeSelector<OuterT, InnerT, MatcherT>
    {
        #[inline]
        fn select_mode(self) -> PropertyBuilder<OuterT, InnerT, ByValueMode> {
            PropertyBuilder {
                extractor: self.extractor,
                property_desc: self.property_desc,
                mode: PhantomData,
            }
        }
    }

    #[doc(hidden)]
    pub trait PropertyByRef<OuterT, InnerT> {
        fn select_mode(self) -> PropertyBuilder<OuterT, InnerT, ByRefMode>;
    }

    impl<OuterT, InnerT: Debug, MatcherT: for<'b> Matcher<&'b InnerT>> PropertyByRef<OuterT, InnerT>
        for &PropertyModeSelector<OuterT, InnerT, MatcherT>
    {
        #[inline]
        fn select_mode(self) -> PropertyBuilder<OuterT, InnerT, ByRefMode> {
            PropertyBuilder {
                extractor: self.extractor,
                property_desc: self.property_desc,
                mode: PhantomData,
            }
        }
    }

    #[doc(hidden)]
    pub trait PropertyFallback<OuterT, InnerT> {
        fn select_mode(self) -> PropertyBuilder<OuterT, InnerT, ByValueMode>;
    }

    impl<OuterT, InnerT, MatcherT> PropertyFallback<OuterT, InnerT>
        for PropertyModeSelector<OuterT, InnerT, MatcherT>
    {
        #[inline]
        fn select_mode(self) -> PropertyBuilder<OuterT, InnerT, ByValueMode> {
            PropertyBuilder {
                extractor: self.extractor,
                property_desc: self.property_desc,
                mode: PhantomData,
            }
        }
    }

    #[derive(MatcherBase)]
    pub struct PropertyMatcher<OuterT, InnerT, MatcherT, Mode = ByValueMode> {
        extractor: fn(&OuterT) -> InnerT,
        property_desc: &'static str,
        inner: MatcherT,
        mode: PhantomData<Mode>,
    }

    impl<InnerT, OuterT, MatcherT> Matcher<OuterT>
        for PropertyMatcher<OuterT, InnerT, MatcherT, ByValueMode>
    where
        InnerT: Debug,
        OuterT: Debug,
        MatcherT: Matcher<InnerT>,
    {
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

    impl<'a, InnerT, OuterT, MatcherT> Matcher<&'a OuterT>
        for PropertyMatcher<OuterT, InnerT, MatcherT, ByValueMode>
    where
        InnerT: Debug,
        OuterT: Debug,
        MatcherT: for<'b> Matcher<&'b InnerT>,
    {
        fn matches(&self, actual: &&'a OuterT) -> MatcherResult {
            self.inner.matches(&&(self.extractor)(*actual))
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "has property `{}`, which {}",
                self.property_desc,
                self.inner.describe(matcher_result)
            )
            .into()
        }

        fn explain_match(&self, actual: &&'a OuterT) -> Description {
            let actual_inner = (self.extractor)(*actual);
            format!(
                "whose property `{}` is `{:#?}`, {}",
                self.property_desc,
                actual_inner,
                self.inner.explain_match(&&actual_inner)
            )
            .into()
        }
    }

    impl<InnerT, OuterT, MatcherT> Matcher<OuterT>
        for PropertyMatcher<OuterT, InnerT, MatcherT, ByRefMode>
    where
        InnerT: Debug,
        OuterT: Debug,
        MatcherT: for<'b> Matcher<&'b InnerT>,
    {
        fn matches(&self, actual: &OuterT) -> MatcherResult {
            self.inner.matches(&&(self.extractor)(actual))
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
                self.inner.explain_match(&&actual_inner)
            )
            .into()
        }
    }

    // `Matcher<OuterT>` and `Matcher<&OuterT>` are distinct traits, so the
    // by-reference mode needs its own impl for `&OuterT`. Without it, the
    // matcher cannot be nested under a matcher which hands it a reference, such
    // as `matches_pattern!` on a type which is not `Copy`.
    impl<'a, InnerT, OuterT, MatcherT> Matcher<&'a OuterT>
        for PropertyMatcher<OuterT, InnerT, MatcherT, ByRefMode>
    where
        InnerT: Debug,
        OuterT: Debug,
        MatcherT: for<'b> Matcher<&'b InnerT>,
    {
        fn matches(&self, actual: &&'a OuterT) -> MatcherResult {
            self.inner.matches(&&(self.extractor)(*actual))
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "has property `{}`, which {}",
                self.property_desc,
                self.inner.describe(matcher_result)
            )
            .into()
        }

        fn explain_match(&self, actual: &&'a OuterT) -> Description {
            let actual_inner = (self.extractor)(*actual);
            format!(
                "whose property `{}` is `{:#?}`, {}",
                self.property_desc,
                actual_inner,
                self.inner.explain_match(&&actual_inner)
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
        fn matches(&self, actual: &OuterT) -> MatcherResult {
            self.inner.matches(&&(self.extractor)(*actual))
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
            let actual_inner = (self.extractor)(*actual);
            format!(
                "whose property `{}` is `{:#?}`, {}",
                self.property_desc,
                actual_inner,
                self.inner.explain_match(&&actual_inner)
            )
            .into()
        }
    }
}
