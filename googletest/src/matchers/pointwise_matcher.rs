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
// macro is documented in the matcher module.
#![doc(hidden)]

/// Generates a matcher which matches a container each of whose elements match
/// the given matcher name applied respectively to each element of the given
/// container.
///
/// For example, the following matches a container of integers each of which
/// does not exceed the given upper bounds:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let value = vec![1, 2, 3];
/// verify_that!(value, pointwise!(le, [1, 3, 3]))?; // Passes
/// verify_that!(value, pointwise!(le, vec![1, 3, 3]))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// #     let value = vec![1, 2, 3];
/// verify_that!(value, pointwise!(le, [1, 1, 3]))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// One can also use a closure which returns a matcher:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let value = vec![1.00001, 2.000001, 3.00001];
/// verify_that!(value, pointwise!(|v| near(v, 0.001), [1.0, 2.0, 3.0]))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// One can pass up to three containers to supply arguments to the function
/// creating the matcher:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let value = vec![1.00001, 2.000001, 3.00001];
/// verify_that!(value, pointwise!(|v, t| near(v, t), [1.0, 2.0, 3.0], [0.001, 0.0001, 0.01]))?;
/// verify_that!(value, pointwise!(near, [1.0, 2.0, 3.0], [0.001, 0.0001, 0.01]))?; // Same as above
/// verify_that!(
///     value,
///     pointwise!(
///         |v, t, u| near(v, t * u),
///         [1.0, 2.0, 3.0],
///         [0.001, 0.0001, 0.01],
///         [0.5, 0.5, 1.0]
///     )
/// )?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// When using `pointwise!` with multiple containers, the caller must ensure
/// that all of the containers have the same size. This matcher does not check
/// whether the sizes match.
///
/// The actual value must be a container such as a `Vec`, an array, or a
/// dereferenced slice. More precisely, a shared borrow of the actual value must
/// implement [`IntoIterator`].
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let value = vec![1, 2, 3];
/// verify_that!(*value.as_slice(), pointwise!(le, [1, 3, 3]))?; // Passes
/// verify_that!([1, 2, 3], pointwise!(le, [1, 3, 3]))?; // Passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// This matcher does not support matching directly against an [`Iterator`]. To
/// match against an iterator, use [`Iterator::collect`] to build a [`Vec`]
/// first.
///
/// The second argument can be any value implementing `IntoIterator`, such as a
/// `Vec` or an array. The container does not have to have the same type as the
/// actual value, but the value type must be the same.
///
/// **Note for users of the [`Pointwise`] matcher in C++ GoogleTest:**
///
/// This macro differs from `Pointwise` in that the first parameter is not a
/// matcher which matches a pair but rather the name of a function of one
/// argument whose output is a matcher. This means that one can use standard
/// matchers like `eq`, `le`, and so on with `pointwise!` but certain C++ tests
/// using `Pointwise` will require some extra work to port.
///
/// [`IntoIterator`]: std::iter::IntoIterator
/// [`Iterator`]: std::iter::Iterator
/// [`Iterator::collect`]: std::iter::Iterator::collect
/// [`Pointwise`]: https://google.github.io/googletest/reference/matchers.html#container-matchers
/// [`Vec`]: std::vec::Vec
#[macro_export]
#[doc(hidden)]
macro_rules! __pointwise {
    ($matcher:expr, $container:expr) => {{
        use $crate::matchers::__internal_unstable_do_not_depend_on_these::PointwiseMatcher;
        PointwiseMatcher::new($container.into_iter().map($matcher).collect())
    }};

    ($matcher:expr, $left_container:expr, $right_container:expr) => {{
        use $crate::matchers::__internal_unstable_do_not_depend_on_these::PointwiseMatcher;
        PointwiseMatcher::new(
            $left_container
                .into_iter()
                .zip($right_container.into_iter())
                .map(|(l, r)| $matcher(l, r))
                .collect(),
        )
    }};

    ($matcher:expr, $left_container:expr, $middle_container:expr, $right_container:expr) => {{
        use $crate::matchers::__internal_unstable_do_not_depend_on_these::PointwiseMatcher;
        PointwiseMatcher::new(
            $left_container
                .into_iter()
                .zip($right_container.into_iter().zip($middle_container.into_iter()))
                .map(|(l, (m, r))| $matcher(l, m, r))
                .collect(),
        )
    }};
}

/// Module for use only by the procedural macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::description::Description;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::matcher_support::zipped_iterator::zip;
    use std::{fmt::Debug, marker::PhantomData};

    /// This struct is meant to be used only through the `pointwise` macro.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub struct PointwiseMatcher<ContainerT: ?Sized, MatcherT> {
        matchers: Vec<MatcherT>,
        phantom: PhantomData<ContainerT>,
    }

    impl<ContainerT: ?Sized, MatcherT> PointwiseMatcher<ContainerT, MatcherT> {
        pub fn new(matchers: Vec<MatcherT>) -> Self {
            Self { matchers, phantom: Default::default() }
        }
    }

    impl<T: Debug, MatcherT: Matcher<ActualT = T>, ContainerT: ?Sized + Debug> Matcher
        for PointwiseMatcher<ContainerT, MatcherT>
    where
        for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
    {
        type ActualT = ContainerT;

        fn matches(&self, actual: &ContainerT) -> MatcherResult {
            let mut zipped_iterator = zip(actual.into_iter(), self.matchers.iter());
            for (element, matcher) in zipped_iterator.by_ref() {
                if matcher.matches(element).is_no_match() {
                    return MatcherResult::NoMatch;
                }
            }
            if zipped_iterator.has_size_mismatch() {
                MatcherResult::NoMatch
            } else {
                MatcherResult::Match
            }
        }

        fn explain_match(&self, actual: &ContainerT) -> Description {
            // TODO(b/260819741) This code duplicates elements_are_matcher.rs. Consider
            // extract as a separate library. (or implement pointwise! with
            // elements_are)
            let actual_iterator = actual.into_iter();
            let mut zipped_iterator = zip(actual_iterator, self.matchers.iter());
            let mut mismatches = Vec::new();
            for (idx, (a, e)) in zipped_iterator.by_ref().enumerate() {
                if e.matches(a).is_no_match() {
                    mismatches.push(format!("element #{idx} is {a:?}, {}", e.explain_match(a)));
                }
            }
            if mismatches.is_empty() {
                if !zipped_iterator.has_size_mismatch() {
                    "which matches all elements".into()
                } else {
                    format!(
                        "which has size {} (expected {})",
                        zipped_iterator.left_size(),
                        self.matchers.len()
                    )
                    .into()
                }
            } else if mismatches.len() == 1 {
                format!("where {}", mismatches[0]).into()
            } else {
                let mismatches = mismatches.into_iter().collect::<Description>();
                format!("where:\n{}", mismatches.bullet_list().indent()).into()
            }
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "{} elements satisfying respectively:\n{}",
                if matcher_result.into() { "has" } else { "doesn't have" },
                self.matchers
                    .iter()
                    .map(|m| m.describe(MatcherResult::Match))
                    .collect::<Description>()
                    .enumerate()
                    .indent()
            )
            .into()
        }
    }
}
