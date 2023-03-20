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

/// Generates a matcher which matches a container each of whose elements match
/// the given matcher name applied respectively to each element of the given
/// container.
///
/// For example, the following matches a container of integers each of which
/// does not exceed the given upper bounds:
///
/// ```
/// let value = vec![1, 2, 3];
/// verify_that!(value, pointwise!(le, [1, 3, 3]))?; // Passes
/// verify_that!(value, pointwise!(le, [1, 1, 3]))?; // Fails
/// ```
///
/// The actual value must be a container implementing [`IntoIterator`]. This
/// includes standard containers, slices (when dereferenced) and arrays.
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
/// [`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
/// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
/// [`Iterator::collect`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
/// [`Pointwise`]: https://google.github.io/googletest/reference/matchers.html#container-matchers
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[macro_export]
macro_rules! pointwise {
    ($matcher:expr, $container:expr) => {{
        #[cfg(google3)]
        use $crate::internal::PointwiseMatcher;
        #[cfg(not(google3))]
        use $crate::matchers::pointwise_matcher::internal::PointwiseMatcher;
        PointwiseMatcher::new($container.into_iter().map(|t| $matcher(t)).collect())
    }};
}

/// Module for use only by the procedural macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    #[cfg(not(google3))]
    use crate as googletest;
    #[cfg(not(google3))]
    use crate::matchers::zipped_iterator::zip;
    #[cfg(google3)]
    use description::Description;
    use googletest::matcher::{MatchExplanation, Matcher, MatcherResult};
    #[cfg(not(google3))]
    use googletest::matchers::description::Description;
    use std::fmt::Debug;
    #[cfg(google3)]
    use zipped_iterator::zip;

    /// This struct is meant to be used only through the `pointwise` macro.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub struct PointwiseMatcher<MatcherT> {
        matchers: Vec<MatcherT>,
    }

    impl<MatcherT> PointwiseMatcher<MatcherT> {
        pub fn new(matchers: Vec<MatcherT>) -> Self {
            Self { matchers }
        }
    }

    impl<T: Debug, MatcherT: Matcher<T>, ContainerT: ?Sized + Debug> Matcher<ContainerT>
        for PointwiseMatcher<MatcherT>
    where
        for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
    {
        fn matches(&self, actual: &ContainerT) -> MatcherResult {
            let mut zipped_iterator = zip(actual.into_iter(), self.matchers.iter());
            for (element, matcher) in zipped_iterator.by_ref() {
                if !matcher.matches(element).into_bool() {
                    return MatcherResult::DoesNotMatch;
                }
            }
            if zipped_iterator.has_size_mismatch() {
                MatcherResult::DoesNotMatch
            } else {
                MatcherResult::Matches
            }
        }

        fn explain_match(&self, actual: &ContainerT) -> MatchExplanation {
            // TODO(b/260819741) This code duplicates elements_are_matcher.rs. Consider
            // extract as a separate library. (or implement pointwise! with
            // elements_are)
            let actual_iterator = actual.into_iter();
            let mut zipped_iterator = zip(actual_iterator, self.matchers.iter());
            let mut mismatches = Vec::new();
            for (idx, (a, e)) in zipped_iterator.by_ref().enumerate() {
                if !e.matches(a).into_bool() {
                    mismatches.push(format!("element #{idx} is {a:?}, {}", e.explain_match(a)));
                }
            }
            if mismatches.is_empty() {
                if !zipped_iterator.has_size_mismatch() {
                    MatchExplanation::create("which matches all elements".to_string())
                } else {
                    MatchExplanation::create(format!(
                        "which has size {} (expected {})",
                        zipped_iterator.left_size(),
                        self.matchers.len()
                    ))
                }
            } else if mismatches.len() == 1 {
                MatchExplanation::create(format!("where {}", mismatches[0]))
            } else {
                let mismatches = mismatches.into_iter().collect::<Description>();
                MatchExplanation::create(format!("where:\n{}", mismatches.bullet_list().indent()))
            }
        }

        fn describe(&self, matcher_result: MatcherResult) -> String {
            format!(
                "{} elements satisfying respectively:\n{}",
                if matcher_result.into() { "has" } else { "doesn't have" },
                self.matchers
                    .iter()
                    .map(|m| m.describe(MatcherResult::Matches))
                    .collect::<Description>()
                    .enumerate()
                    .indent()
            )
        }
    }
}
