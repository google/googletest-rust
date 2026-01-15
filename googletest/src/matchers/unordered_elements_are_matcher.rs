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

/// Matches a container whose elements in any order have a 1:1 correspondence
/// with the provided element matchers.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(vec![3, 2, 1], unordered_elements_are![eq(&1), ge(&2), anything()])?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!(vec![1], unordered_elements_are![eq(&1), ge(&2)])?;              // Fails: container has wrong size
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!(vec![3, 2, 1], unordered_elements_are![eq(&1), ge(&4), eq(&2)])?; // Fails: second matcher not matched
/// #     Ok(())
/// # }
/// # fn should_fail_3() -> Result<()> {
/// verify_that!(vec![3, 2, 1], unordered_elements_are![ge(&3), ge(&3), ge(&3)])?; // Fails: no 1:1 correspondence
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_fail_3().unwrap_err();
/// ```
///
/// The actual value must be a container such as a `&Vec`, an array, or a
/// slice. More precisely, the actual value must implement [`IntoIterator`].
///
/// This can also be omitted in [`verify_that!`] macros and replaced with curly
/// brackets.
///
/// ```
/// # use googletest::prelude::*;
///  verify_that!(vec![1, 2], {eq(&2), eq(&1)})
/// #     .unwrap();
/// ```
///
/// Note: This behavior is only possible in [`verify_that!`] macros. In any
/// other cases, it is still necessary to use the
/// [`unordered_elements_are!`][crate::matchers::unordered_elements_are] macro.
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// verify_that!(vec![vec![1,2], vec![3]], {{eq(2), eq(1)}, {eq(3)}})
/// # .unwrap();
/// ```
///
/// Use this instead:
/// ```
/// # use googletest::prelude::*;
/// verify_that!(vec![vec![1,2], vec![3]],
///   {unordered_elements_are![eq(&2), eq(&1)], unordered_elements_are![eq(&3)]})
/// # .unwrap();
/// ```
///
///  If an inner matcher is `eq(...)`, it can be omitted:
///
/// ```
/// # use googletest::prelude::*;
///
/// verify_that!(vec![1,2,3], unordered_elements_are![lt(&2), gt(&1), &3])
/// #     .unwrap();
/// ```
///
/// The matcher proceeds in three stages:
///
/// 1. It first checks whether the actual value is of the right size to possibly
///    be matched by each of the given matchers. If not, then it immediately
///    fails explaining that the size is incorrect.
///
/// 2. It then checks whether each matcher matches at least one corresponding
///    element in the actual container and each element in the actual container
///    is matched by at least one matcher. If not, it fails with a message
///    indicating which matcher respectively container elements had no
///    counterparts.
///
/// 3. Finally, it checks whether the mapping of matchers to corresponding
///    actual elements is a 1-1 correspondence and fails if that is not the
///    case. The failure message then shows the best matching it could find,
///    including which matchers did not have corresponding unique elements in
///    the container and which container elements had no corresponding matchers.
///
/// [`IntoIterator`]: std::iter::IntoIterator
/// [`Iterator`]: std::iter::Iterator
/// [`Iterator::collect`]: std::iter::Iterator::collect
/// [`Vec`]: std::vec::Vec
#[macro_export]
#[doc(hidden)]
macro_rules! __unordered_elements_are {
    ($(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::
        UnorderedElementsAreMatcher::new(
            (),
            $crate::matchers::__internal_unstable_do_not_depend_on_these::
            Requirements::PerfectMatch)
    }};

    ($($matcher:expr),* $(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::
        UnorderedElementsAreMatcher::new(
            ($(
                $crate::matcher_support::__internal_unstable_do_not_depend_on_these::auto_eq!(
                    $matcher
                )
            ,)*),
            $crate::matchers::__internal_unstable_do_not_depend_on_these::
            Requirements::PerfectMatch)
    }};
}

/// Matches a container containing elements matched by the given matchers.
///
/// To match, each given matcher must have a corresponding element in the
/// container which it matches. There must be a mapping uniquely matching each
/// matcher to a container element. The container can, however, contain
/// additional elements that don't correspond to any matcher.
///
/// Put another way, `contains_each!` matches if there is a subset of the actual
/// container which
/// [`unordered_elements_are`][crate::matchers::unordered_elements_are] would
/// match.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(vec![3, 2, 1], contains_each![eq(&2), ge(&3)])?;   // Passes
/// verify_that!(vec![3, 2, 1], contains_each![ge(&2), ge(&2)])?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!(vec![1], contains_each![eq(&1), ge(&2)])?;         // Fails: container too small
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!(vec![3, 2, 1], contains_each![eq(&1), ge(&4)])?;   // Fails: second matcher unmatched
/// #     Ok(())
/// # }
/// # fn should_fail_3() -> Result<()> {
/// verify_that!(vec![3, 2, 1], contains_each![ge(&3), ge(&3), ge(&3)])?; // Fails: no matching
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_fail_3().unwrap_err();
/// ```
///
/// The actual value must be a container such as a `&Vec`, an array, or a
/// slice. More precisely, the actual value must implement [`IntoIterator`].
///
///  If an inner matcher is `eq(...)`, it can be omitted:
///
/// ```
/// # use googletest::prelude::*;
///
/// verify_that!(vec![1,2,3], contains_each![lt(&2), &3])
/// #     .unwrap();
/// ```
///
/// The matcher proceeds in three stages:
///
/// 1. It first checks whether the actual value is large enough to possibly be
///    matched by each of the given matchers. If not, then it immediately fails
///    explaining that the size is too small.
///
/// 2. It then checks whether each matcher matches at least one corresponding
///    element in the actual container and fails if that is not the case. The
///    failure message indicates which matcher had no corresponding element.
///
/// 3. Finally, it checks whether the mapping of matchers to corresponding
///    actual elements is 1-1 and fails if that is not the case. The failure
///    message then shows the best matching it could find, including which
///    matchers did not have corresponding unique elements in the container.
///
/// [`IntoIterator`]: std::iter::IntoIterator
/// [`Iterator`]: std::iter::Iterator
/// [`Iterator::collect`]: std::iter::Iterator::collect
/// [`Vec`]: std::vec::Vec
#[macro_export]
#[doc(hidden)]
macro_rules! __contains_each {
    ($(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::
        UnorderedElementsAreMatcher::new(
            (),
            $crate::matchers::__internal_unstable_do_not_depend_on_these::Requirements::Superset)
    }};

    ($($matcher:expr),* $(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::
        UnorderedElementsAreMatcher::new(
            ($(
                $crate::matcher_support::__internal_unstable_do_not_depend_on_these::auto_eq!(
                    $matcher
                )
            ,)*),
            $crate::matchers::__internal_unstable_do_not_depend_on_these::Requirements::Superset)
    }}
}

/// Matches a container all of whose elements are matched by the given matchers.
///
/// To match, each element in the container must have a corresponding matcher
/// which matches it. There must be a 1-1 mapping from container elements to
/// matchers, so that no matcher has more than one corresponding element.
///
/// There may, however, be matchers not corresponding to any elements in the
/// container.
///
/// Put another way, `is_contained_in!` matches if there is a subset of the
/// matchers which would match with
/// [`unordered_elements_are`][crate::matchers::unordered_elements_are].
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(vec![2, 1], is_contained_in![eq(&1), ge(&2)])?;   // Passes
/// verify_that!(vec![2, 1], is_contained_in![ge(&1), ge(&1)])?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!(vec![1, 2, 3], is_contained_in![eq(&1), ge(&2)])?; // Fails: container too large
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!(vec![2, 1], is_contained_in![eq(&1), ge(&4)])?;    // Fails: second matcher unmatched
/// #     Ok(())
/// # }
/// # fn should_fail_3() -> Result<()> {
/// verify_that!(vec![3, 1], is_contained_in![ge(&3), ge(&3), ge(&3)])?; // Fails: no matching
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_fail_3().unwrap_err();
/// ```
///
/// The actual value must be a container such as a `&Vec`, an array, or a slice.
/// More precisely, the actual value must implement [`IntoIterator`].
///
///  If an inner matcher is `eq(...)`, it can be omitted:
///
/// ```
/// # use googletest::prelude::*;
///
/// verify_that!(vec![1,2,3], is_contained_in![lt(&2), &3, &4, gt(&0)])
/// #     .unwrap();
/// ```
///
/// The matcher proceeds in three stages:
///
/// 1. It first checks whether the actual value is too large to possibly be
///    matched by each of the given matchers. If so, it immediately fails
///    explaining that the size is too large.
///
/// 2. It then checks whether each actual container element is matched by at
///    least one matcher and fails if that is not the case. The failure message
///    indicates which element had no corresponding matcher.
///
/// 3. Finally, it checks whether the mapping of elements to corresponding
///    matchers is 1-1 and fails if that is not the case. The failure message
///    then shows the best matching it could find, including which container
///    elements did not have corresponding matchers.
///
/// [`IntoIterator`]: std::iter::IntoIterator
/// [`Iterator`]: std::iter::Iterator
/// [`Iterator::collect`]: std::iter::Iterator::collect
/// [`Vec`]: std::vec::Vec
#[macro_export]
#[doc(hidden)]
macro_rules! __is_contained_in {
    ($(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::
        UnorderedElementsAreMatcher::new(
            (), $crate::matchers::__internal_unstable_do_not_depend_on_these::Requirements::Subset)
    }};

    ($($matcher:expr),* $(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::
        UnorderedElementsAreMatcher::new(
            ($(
                $crate::matcher_support::__internal_unstable_do_not_depend_on_these::auto_eq!(
                    $matcher
                )
            ,)*),
            $crate::matchers::__internal_unstable_do_not_depend_on_these::Requirements::Subset)
    }}
}

/// Module for use only by the macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::description::Description;
    use crate::matcher::{Matcher, MatcherBase, MatcherResult};
    use crate::matcher_support::match_matrix::internal::{MatchMatrix, Requirements};
    use std::fmt::Debug;

    /// A trait for a tuple of matchers.
    pub trait MatcherTuple<T: Debug + Copy> {
        /// Turn the tuple into a vector of matchers.
        fn to_matchers(&self) -> Vec<Box<dyn Matcher<T> + '_>>;
    }

    // Base case, no matchers.
    impl<T: Debug + Copy> MatcherTuple<T> for () {
        fn to_matchers(&self) -> Vec<Box<dyn Matcher<T> + '_>> {
            vec![]
        }
    }

    // A macro for generating a MatcherTuple implementation for a given number
    // of matchers.
    macro_rules! matcher_tuple_n {
        // Given list of (field_number, matcher) pairs, generate a MatcherTuple
        // implementation.
        ($([$field_number:tt, $matcher_type:ident]),*) => {
            impl<T: Debug + Copy, $($matcher_type: Matcher<T>),*> MatcherTuple<T>
            for ($($matcher_type,)*) {
                // Our implementation of to_matchers just converts our tuple into a vector of
                // matchers by expanding self.0, self.1, etc.
                fn to_matchers(&self) -> Vec<Box<dyn Matcher<T> + '_>> {
                    vec![$(Box::new(&self.$field_number)),*]
                }
            }
        };
    }

    matcher_tuple_n!([0, M0]);
    matcher_tuple_n!([0, M0], [1, M1]);
    matcher_tuple_n!([0, M0], [1, M1], [2, M2]);
    matcher_tuple_n!([0, M0], [1, M1], [2, M2], [3, M3]);
    matcher_tuple_n!([0, M0], [1, M1], [2, M2], [3, M3], [4, M4]);
    matcher_tuple_n!([0, M0], [1, M1], [2, M2], [3, M3], [4, M4], [5, M5]);
    matcher_tuple_n!([0, M0], [1, M1], [2, M2], [3, M3], [4, M4], [5, M5], [6, M6]);
    matcher_tuple_n!([0, M0], [1, M1], [2, M2], [3, M3], [4, M4], [5, M5], [6, M6], [7, M7]);
    matcher_tuple_n!(
        [0, M0],
        [1, M1],
        [2, M2],
        [3, M3],
        [4, M4],
        [5, M5],
        [6, M6],
        [7, M7],
        [8, M8]
    );
    matcher_tuple_n!(
        [0, M0],
        [1, M1],
        [2, M2],
        [3, M3],
        [4, M4],
        [5, M5],
        [6, M6],
        [7, M7],
        [8, M8],
        [9, M9]
    );
    matcher_tuple_n!(
        [0, M0],
        [1, M1],
        [2, M2],
        [3, M3],
        [4, M4],
        [5, M5],
        [6, M6],
        [7, M7],
        [8, M8],
        [9, M9],
        [10, M10]
    );
    matcher_tuple_n!(
        [0, M0],
        [1, M1],
        [2, M2],
        [3, M3],
        [4, M4],
        [5, M5],
        [6, M6],
        [7, M7],
        [8, M8],
        [9, M9],
        [10, M10],
        [11, M11]
    );

    /// This struct is meant to be used only through the
    /// `unordered_elements_are![...]` macro.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    #[derive(MatcherBase)]
    pub struct UnorderedElementsAreMatcher<M> {
        matchers: M,
        requirements: Requirements,
    }

    impl<M> UnorderedElementsAreMatcher<M> {
        pub fn new(matchers: M, requirements: Requirements) -> Self {
            Self { matchers, requirements }
        }
    }

    // This matcher performs the checks in three different steps in both `matches`
    // and `explain_match`. This is useful for performance but also to produce
    // an actionable error message.
    // 1. `UnorderedElementsAreMatcher` verifies that both collections have the same
    // size
    // 2. `UnorderedElementsAreMatcher` verifies that each actual element matches at
    // least one expected element and vice versa.
    // 3. `UnorderedElementsAreMatcher` verifies that a perfect matching exists
    // using Ford-Fulkerson.
    impl<T: Debug + Copy, ContainerT: Debug + Copy, M> Matcher<ContainerT>
        for UnorderedElementsAreMatcher<M>
    where
        ContainerT: IntoIterator<Item = T>,
        M: MatcherTuple<T>,
    {
        fn matches(&self, actual: ContainerT) -> MatcherResult {
            let matchers = self.matchers.to_matchers();
            let match_matrix = MatchMatrix::generate(actual, &matchers);
            match_matrix.is_match_for(self.requirements).into()
        }

        fn explain_match(&self, actual: ContainerT) -> Description {
            let matchers = self.matchers.to_matchers();
            if let Some(size_mismatch_explanation) =
                self.requirements.explain_size_mismatch(actual, matchers.len())
            {
                return size_mismatch_explanation;
            }

            let match_matrix = MatchMatrix::generate(actual, &matchers);
            if let Some(unmatchable_explanation) =
                match_matrix.explain_unmatchable(self.requirements)
            {
                return unmatchable_explanation;
            }

            let best_match = match_matrix.find_best_match();
            best_match
                .get_explanation(actual, &matchers, self.requirements)
                .unwrap_or("whose elements all match".into())
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            let matchers = self.matchers.to_matchers();
            format!(
                "{} elements matching in any order:\n{}",
                if matcher_result.into() { "contains" } else { "doesn't contain" },
                matchers
                    .iter()
                    .map(|matcher| matcher.describe(MatcherResult::Match))
                    .collect::<Description>()
                    .enumerate()
                    .indent()
            )
            .into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate as googletest;
    use crate::description::Description;
    use crate::matcher::MatcherResult;
    use crate::prelude::*;
    use indoc::indoc;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn has_correct_description_for_map() -> googletest::Result<()> {
        // UnorderedElementsAreMatcher maintains references to the matchers, so the
        // constituent matchers must live longer. Inside a verify_that! macro, the
        // compiler takes care of that, but when the matcher is created separately,
        // we must create the constitute matchers separately so that they
        // aren't dropped too early.
        let matchers = ((eq(&2), eq(&"Two")), (eq(&1), eq(&"One")), (eq(&3), eq(&"Three")));
        let matcher = unordered_elements_are![
            (matchers.0 .0, matchers.0 .1),
            (matchers.1 .0, matchers.1 .1),
            (matchers.2 .0, matchers.2 .1)
        ];
        verify_that!(
            Matcher::<&HashMap<i32, String>>::describe(&matcher, MatcherResult::Match),
            displays_as(eq(indoc!(
                "
                contains elements matching in any order:
                  0. is a tuple whose values respectively match:
                       is equal to 2
                       is equal to \"Two\"
                  1. is a tuple whose values respectively match:
                       is equal to 1
                       is equal to \"One\"
                  2. is a tuple whose values respectively match:
                       is equal to 3
                       is equal to \"Three\""
            )))
        )
    }

    #[test]
    fn unordered_elements_are_description_no_full_match_with_map() -> googletest::Result<()> {
        // UnorderedElementsAreMatcher maintains references to the matchers, so the
        // constituent matchers must live longer. Inside a verify_that! macro, the
        // compiler takes care of that, but when the matcher is created separately,
        // we must create the constitute matchers separately so that they
        // aren't dropped too early.
        let value: HashMap<u32, u32> = HashMap::from_iter([(0, 1), (1, 1), (2, 2)]);
        let matchers = ((anything(), eq(&1)), (anything(), eq(&2)), (anything(), eq(&2)));
        let matcher = unordered_elements_are![
            (matchers.0 .0, matchers.0 .1),
            (matchers.1 .0, matchers.1 .1),
            (matchers.2 .0, matchers.2 .1),
        ];
        verify_that!(
            matcher.explain_match(&value),
            all![
                displays_as(contains_regex(
                    "Actual element \\(2, 2\\) at index [0-2] matched expected element `is a tuple whose values respectively match:\n    is anything\n    is equal to 2` at index [0-2]."
                )),
                displays_as(contains_regex(
                    "Actual element \\(\n      [0-1],\n      [0-1],\n  \\) at index [0-2] did not match any remaining expected element."
                )),
                displays_as(contains_substring(
                    "Expected element `is a tuple whose values respectively match:\n    is anything\n    is equal to 2` at index 2 did not match any remaining actual element."
                ))
            ]
        )
    }

    // This test verifies that unordered_elements_are can be used as the inner
    // matcher of another matcher, where the inner matcher is given a reference
    // to a local variable. If unordered_elements_are stores its matchers in a
    // Vec with the same type and lifetimes then this fails to compile.
    #[test]
    fn works_with_inner_matcher_that_modifies_the_input() -> googletest::Result<()> {
        // A custom matcher that applies an operation to the input hashmap (filtering
        // out any keys present in `to_filter`) and then applies the inner
        // matcher to the modified input.
        fn matches_filtered<'a, M>(
            to_filter: &'a HashSet<i32>,
            inner: M,
        ) -> impl Matcher<&'a HashSet<i32>>
        where
            M: for<'b> Matcher<&'b HashSet<i32>>,
        {
            #[derive(MatcherBase)]
            struct FilteredMatcher<'a, M> {
                to_filter: &'a HashSet<i32>,
                inner: M,
            }

            impl<'a, M> Matcher<&'a HashSet<i32>> for FilteredMatcher<'a, M>
            where
                M: for<'b> Matcher<&'b HashSet<i32>>,
            {
                fn matches(&self, actual: &'a HashSet<i32>) -> MatcherResult {
                    let filtered: HashSet<i32> =
                        actual.iter().filter(|k| !self.to_filter.contains(k)).cloned().collect();
                    self.inner.matches(&filtered)
                }

                fn describe(&self, matcher_result: MatcherResult) -> Description {
                    self.inner.describe(matcher_result)
                }

                fn explain_match(&self, actual: &'a HashSet<i32>) -> Description {
                    let filtered: HashSet<i32> =
                        actual.iter().filter(|k| !self.to_filter.contains(k)).cloned().collect();
                    self.inner.explain_match(&filtered)
                }
            }

            FilteredMatcher { to_filter, inner }
        }

        let before: HashSet<i32> = HashSet::from([1]);
        let after: HashSet<i32> = HashSet::from([1, 2]);

        verify_that!(&after, matches_filtered(&before, unordered_elements_are![eq(&2)]))?;

        Ok(())
    }
}
