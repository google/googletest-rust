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

/// Matches a container whose elements in any order have a 1:1 correspondence
/// with the provided element matchers.
///
/// ```rust
/// verify_that!(vec![3, 2, 1], unordered_elements_are![eq(1), anything(), gt(0).and(lt(123))])
/// ```
#[macro_export]
macro_rules! unordered_elements_are {
    ($($matcher:expr),*) => {{
        #[cfg(google3)]
        use $crate::internal::{UnorderedElementsAre, Requirements};
        #[cfg(not(google3))]
        use $crate::matchers::unordered_elements_are_matcher::internal::{UnorderedElementsAre, Requirements};
        UnorderedElementsAre::new([$(&$matcher),*], Requirements::PerfectMatch)
    }}
}

/// Matches a container containing elements matched by the given matchers.
///
/// To match, each given matcher must have a corresponding element in the
/// container which it matches. There must be a mapping uniquely matching each
/// matcher to a container element. The container can, however, contain
/// additional elements that don't correspond to any matcher.
///
/// Put another way, `contains_each!` matches if there is a subset of the actual
/// container which [`unordered_elements_are`] would match.
///
/// ```rust
/// verify_that!(vec![3, 2, 1], contains_each![eq(2), ge(3)])?;   // Passes
/// verify_that!(vec![3, 2, 1], contains_each![ge(3), ge(3)])?;   // Passes
/// verify_that!(vec![1], contains_each![eq(1), ge(2)])?;         // Fails: container too small
/// verify_that!(vec![3, 2, 1], contains_each![eq(1), ge(4)])?;   // Fails: second matcher unmatched
/// verify_that!(vec![3, 2, 1], contains_each![ge(3), ge(3), ge(3)])?; // Fails: no matching
/// ```
///
/// The matcher proceeds in three stages:
///
/// 1. It first checks whether the actual value is large enough to
///    possibly be matched by each of the given matchers. If not, then it
///    immediately fails explaining that the size is too small.
///
/// 2. It then checks whether each matcher matches at least one corresponding
///    element in the actual container and fails if that is not the case. The
///    failure message indicates which matcher had no corresponding element.
///
/// 3. Finally, it checks whether the mapping of matchers to corresponding
///    actual elements is 1-1 and fails if that is not the case. The failure
///    message then shows the best matching it could find, including which
///    matchers did not have corresponding unique elements in the container.
#[macro_export]
macro_rules! contains_each {
    ($($matcher:expr),*) => {{
        #[cfg(google3)]
        use $crate::internal::{UnorderedElementsAre, Requirements};
        #[cfg(not(google3))]
        use $crate::matchers::unordered_elements_are_matcher::internal::{UnorderedElementsAre, Requirements};
        UnorderedElementsAre::new([$(&$matcher),*], Requirements::Superset)
    }}
}

/// Module for use only by the macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    #[cfg(not(google3))]
    use crate as googletest;
    use googletest::matcher::{Describe, MatchExplanation, Matcher, MatcherResult};
    #[cfg(not(google3))]
    use googletest::matchers::has_size::HasSize;
    #[cfg(google3)]
    use has_size::HasSize;
    use std::collections::HashSet;
    use std::fmt::{Debug, Display};

    /// The requirements of the mapping between matchers and actual values by
    /// which [`UnorderedElemetnsAre`] is deemed to match its input.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    #[derive(Clone, Copy)]
    pub enum Requirements {
        /// The mapping from matched actual values to their corresponding
        /// matchers must be surjective.
        Superset,

        /// There must be a 1:1 correspondence between the actual values and the
        /// matchers.
        PerfectMatch,
    }

    impl Display for Requirements {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Requirements::Superset => {
                    write!(f, "superset")
                }
                Requirements::PerfectMatch => {
                    write!(f, "perfect")
                }
            }
        }
    }

    /// This struct is meant to be used only through the
    /// `unordered_elements_are![...]` macro.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub struct UnorderedElementsAre<'a, T: Debug, const N: usize> {
        elements: [&'a dyn Matcher<T>; N],
        requirements: Requirements,
    }

    impl<'a, T: Debug, const N: usize> UnorderedElementsAre<'a, T, N> {
        pub fn new(elements: [&'a dyn Matcher<T>; N], requirements: Requirements) -> Self {
            Self { elements, requirements }
        }
    }

    // This matcher performs the checks in three different steps in both `matches`
    // and `explain_match`. This is useful for performance but also to produce
    // an actionable error message.
    // 1. `UnorderedElementsAre` verifies that both collections have the same
    // size
    // 2. `UnorderedElementsAre` verifies that each actual element matches at least
    // one expected element and vice versa.
    // 3. `UnorderedElementsAre` verifies that a perfect matching exists using
    // Ford-Fulkerson.
    impl<'a, T: Debug, ContainerT: Debug + HasSize, const N: usize> Matcher<ContainerT>
        for UnorderedElementsAre<'a, T, N>
    where
        for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
    {
        fn matches(&self, actual: &ContainerT) -> MatcherResult {
            match self.requirements {
                Requirements::PerfectMatch => {
                    if actual.size() != N {
                        return MatcherResult::DoesNotMatch;
                    }
                    let match_matrix = MatchMatrix::generate(actual, &self.elements);
                    if !match_matrix.find_unmatchable_elements().has_unmatchable_elements()
                        && match_matrix.find_best_match().is_full_match()
                    {
                        MatcherResult::Matches
                    } else {
                        MatcherResult::DoesNotMatch
                    }
                }
                Requirements::Superset => {
                    if actual.size() < N {
                        return MatcherResult::DoesNotMatch;
                    }
                    let match_matrix = MatchMatrix::generate(actual, &self.elements);
                    if !match_matrix.find_unmatched_expected().has_unmatchable_elements()
                        && match_matrix.find_best_match().is_superset_match()
                    {
                        MatcherResult::Matches
                    } else {
                        MatcherResult::DoesNotMatch
                    }
                }
            }
        }

        fn explain_match(&self, actual: &ContainerT) -> MatchExplanation {
            match self.requirements {
                Requirements::PerfectMatch => {
                    if actual.size() != N {
                        return MatchExplanation::create(format!(
                            "which has size {} (expected {})",
                            actual.size(),
                            N
                        ));
                    }
                }

                Requirements::Superset => {
                    if actual.size() < N {
                        return MatchExplanation::create(format!(
                            "which has size {} (expected at least {})",
                            actual.size(),
                            N
                        ));
                    }
                }
            }

            let match_matrix = MatchMatrix::generate(actual, &self.elements);
            let unmatchable_elements = match self.requirements {
                Requirements::PerfectMatch => match_matrix.find_unmatchable_elements(),
                Requirements::Superset => match_matrix.find_unmatched_expected(),
            };
            if let Some(unmatchable_explanation) = unmatchable_elements.get_explanation() {
                return MatchExplanation::create(unmatchable_explanation);
            }

            let best_match = match_matrix.find_best_match();
            if let Some(best_match_explanation) =
                best_match.get_explanation(actual, &self.elements, self.requirements)
            {
                MatchExplanation::create(best_match_explanation)
            } else {
                MatchExplanation::create("whose elements all match".to_string())
            }
        }
    }

    impl<'a, T: Debug, const N: usize> Describe for UnorderedElementsAre<'a, T, N> {
        fn describe(&self, matcher_result: MatcherResult) -> String {
            format!(
                "{} elements matching in any order:\n{}",
                matcher_result.pick("contains", "doesn't contain"),
                self.elements
                    .iter()
                    .map(|&m| m.describe(MatcherResult::Matches))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }

    /// The bipartite matching graph between actual and expected elements.
    struct MatchMatrix<const N: usize>(Vec<[MatcherResult; N]>);

    impl<const N: usize> MatchMatrix<N> {
        fn generate<'a, T: Debug, ContainerT: Debug + HasSize>(
            actual: &ContainerT,
            expected: &[&'a dyn Matcher<T>; N],
        ) -> Self
        where
            for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
        {
            let mut matrix = MatchMatrix(vec![[MatcherResult::DoesNotMatch; N]; actual.size()]);
            for (actual_idx, actual) in actual.into_iter().enumerate() {
                for (expected_idx, expected) in expected.iter().enumerate() {
                    matrix.0[actual_idx][expected_idx] = expected.matches(actual);
                }
            }
            matrix
        }

        // Verifies that each actual matches at least one expected and that
        // each expected matches at least one actual.
        // This is a necessary condition but not sufficient. But it is faster
        // than `find_best_match()`.
        fn find_unmatchable_elements(&self) -> UnmatchableElements<N> {
            let unmatchable_actual = self
                .0
                .iter()
                .map(|row| row.iter().all(|e| matches!(e, MatcherResult::DoesNotMatch)))
                .collect();
            let mut unmatchable_expected = [false; N];
            for (col_idx, expected) in unmatchable_expected.iter_mut().enumerate() {
                *expected = self
                    .0
                    .iter()
                    .map(|row| row[col_idx])
                    .all(|e| matches!(e, MatcherResult::DoesNotMatch));
            }
            UnmatchableElements { unmatchable_actual, unmatchable_expected }
        }

        fn find_unmatched_expected(&self) -> UnmatchableElements<N> {
            let mut unmatchable_expected = [false; N];
            for (col_idx, expected) in unmatchable_expected.iter_mut().enumerate() {
                *expected = self
                    .0
                    .iter()
                    .map(|row| row[col_idx])
                    .all(|e| matches!(e, MatcherResult::DoesNotMatch));
            }
            UnmatchableElements { unmatchable_actual: vec![false; N], unmatchable_expected }
        }

        // Verifies that a full match exists.
        //
        // Uses the well-known Ford-Fulkerson max flow method to find a maximum
        // bipartite matching. Flow is considered to be from actual to expected.
        // There is an implicit source node that is connected to all of the actual
        // nodes, and an implicit sink node that is connected to all of the
        // expected nodes. All edges have unit capacity.
        //
        // Neither the flow graph nor the residual flow graph are represented
        // explicitly. Instead, they are implied by the information in `self.0` and
        // the local `actual_match : [Option<usize>; N]` whose elements are initialized
        // to `None`. This represents the initial state of the algorithm,
        // where the flow graph is empty, and the residual flow graph has the
        // following edges:
        //   - An edge from source to each actual element node
        //   - An edge from each expected element node to sink
        //   - An edge from each actual element node to each expected element node, if
        //     the actual element matches the expected element, i.e.
        //     `matches!(self.0[actual_id][expected_id], Matches)`
        //
        // When the `try_augment(...)` method adds a flow, it sets `actual_match[l] =
        // Some(r)` for some nodes l and r. This induces the following changes:
        //   - The edges (source, l), (l, r), and (r, sink) are added to the flow graph.
        //   - The same three edges are removed from the residual flow graph.
        //   - The reverse edges (l, source), (r, l), and (sink, r) are added to the
        //     residual flow graph, which is a directional graph representing unused
        //     flow capacity.
        //
        // When the method augments a flow (changing `actual_match[l]` from `Some(r1)`
        // to `Some(r2)`), this can be thought of as "undoing" the above steps
        // with respect to r1 and "redoing" them with respect to r2.
        //
        // It bears repeating that the flow graph and residual flow graph are
        // never represented explicitly, but can be derived by looking at the
        // information in 'self.0' and in `actual_match`.
        //
        // As an optimization, there is a second local `expected_match: [Option<usize>;
        // N]` which does not provide any new information. Instead, it enables
        // more efficient queries about edges entering or leaving the expected elements
        // nodes of the flow or residual flow graphs. The following invariants
        // are maintained:
        //
        // actual_match[a] == None or expected_match[actual_match[a].unwrap()] ==
        // Some(a)
        // expected_match[r] == None or actual_match[expected_match[e].unwrap()] ==
        // Some(e)
        //
        // . [ source ]                                                              .
        // .   |||                                                                   .
        // .   |||                                                                   .
        // .   ||\-> actual_match[0]=Some(1) -\   expected_match[0]=None    ---\     .
        // .   ||                             |                                |     .
        // .   |\--> actual_match[1]=None     \-> expected_match[1]=Some(0) --\|     .
        // .   |                                                              ||     .
        // .   \---> actual_match[2]=Some(2)  --> expected_match[2]=Some(2) -\||     .
        // .                                                                 |||     .
        // .         elements                     matchers                   vvv     .
        // .                                                               [ sink ]  .
        //
        // See Also:
        //   [1] Cormen, et al (2001). "Section 26.2: The Ford-Fulkerson method".
        //       "Introduction to Algorithms (Second ed.)", pp. 651-664.
        //   [2] "Ford-Fulkerson algorithm", Wikipedia,
        //       'http://en.wikipedia.org/wiki/Ford%E2%80%93Fulkerson_algorithm'
        fn find_best_match(&self) -> BestMatch<N> {
            let mut actual_match = vec![None; self.0.len()];
            let mut expected_match: [Option<usize>; N] = [None; N];
            // Searches the residual flow graph for a path from each actual node to
            // the sink in the residual flow graph, and if one is found, add this path
            // to the graph.
            // It's okay to search through the actual nodes once. The
            // edge from the implicit source node to each previously-visited actual
            // node will have flow if that actual node has any path to the sink
            // whatsoever. Subsequent augmentations can only add flow to the
            // network, and cannot take away that previous flow unit from the source.
            // Since the source-to-actual edge can only carry one flow unit (or,
            // each actual element can be matched to only one expected element), there is no
            // need to visit the actual nodes more than once looking for
            // augmented paths. The flow is known to be possible or impossible
            // by looking at the node once.
            for actual_idx in 0..self.0.len() {
                assert!(actual_match[actual_idx].is_none());
                let mut seen = [false; N];
                self.try_augment(actual_idx, &mut seen, &mut actual_match, &mut expected_match);
            }
            BestMatch(actual_match)
        }

        // Perform a depth-first search from actual node `actual_idx` to the sink by
        // searching for an unassigned expected node. If a path is found, flow
        // is added to the network by linking the actual and expected vector elements
        // corresponding each segment of the path. Returns true if a path to
        // sink was found, which means that a unit of flow was added to the
        // network. The 'seen' array elements correspond to expected nodes and are
        // marked to eliminate cycles from the search.
        //
        // Actual nodes will only be explored at most once because they
        // are accessible from at most one expected node in the residual flow
        // graph.
        //
        // Note that `actual_match[actual_idx]` is the only element of `actual_match`
        // that `try_augment(...)` will potentially transition from `None` to
        // `Some(...)`. Any other `actual_match` element holding `None` before
        // `try_augment(...)` will be holding it when `try_augment(...)`
        // returns.
        //
        fn try_augment(
            &self,
            actual_idx: usize,
            seen: &mut [bool; N],
            actual_match: &mut [Option<usize>],
            expected_match: &mut [Option<usize>; N],
        ) -> bool {
            for expected_idx in 0..N {
                if seen[expected_idx] {
                    continue;
                }
                if matches!(self.0[actual_idx][expected_idx], MatcherResult::DoesNotMatch) {
                    continue;
                }
                // There is an edge between `actual_idx` and `expected_idx`.
                seen[expected_idx] = true;
                // Next a search is performed to determine whether
                // this edge is a dead end or leads to the sink.
                //
                // `expected_match[expected_idx].is_none()` means that there is residual flow
                // from expected node at index expected_idx to the sink, so we
                // can use that to finish this flow path and return success.
                //
                // Otherwise, we look for a residual flow starting from
                // `expected_match[expected_idx].unwrap()` by calling
                // ourselves recursively to see if this ultimately leads to
                // sink.
                if expected_match[expected_idx].is_none()
                    || self.try_augment(
                        expected_match[expected_idx].unwrap(),
                        seen,
                        actual_match,
                        expected_match,
                    )
                {
                    // We found a residual flow from source to sink. We thus need to add the new
                    // edge to the current flow.
                    // Note: this also remove the potential flow that existed by overwriting the
                    // value in the `expected_match` and `actual_match`.
                    expected_match[expected_idx] = Some(actual_idx);
                    actual_match[actual_idx] = Some(expected_idx);
                    return true;
                }
            }
            false
        }
    }

    /// The list of elements that do not match any element in the corresponding
    /// set.
    /// These lists are represented as fixed sized bit set to avoid
    /// allocation.
    /// TODO(bjacotg) Use BitArr!(for N) once generic_const_exprs is stable.
    struct UnmatchableElements<const N: usize> {
        unmatchable_actual: Vec<bool>,
        unmatchable_expected: [bool; N],
    }

    impl<const N: usize> UnmatchableElements<N> {
        fn has_unmatchable_elements(&self) -> bool {
            self.unmatchable_actual.iter().any(|b| *b)
                || self.unmatchable_expected.iter().any(|b| *b)
        }

        fn get_explanation(&self) -> Option<String> {
            let unmatchable_actual = self.unmatchable_actual();
            let actual_idx = unmatchable_actual
                .iter()
                .map(|idx| format!("#{}", idx))
                .collect::<Vec<_>>()
                .join(", ");
            let unmatchable_expected = self.unmatchable_expected();
            let expected_idx = unmatchable_expected
                .iter()
                .map(|idx| format!("#{}", idx))
                .collect::<Vec<_>>()
                .join(", ");
            match (unmatchable_actual.len(), unmatchable_expected.len()) {
                (0, 0) => None,
                (1, 0) => Some(format!(
                    "whose element {} does not match any expected elements",
                    actual_idx
                )),
                (_, 0) => Some(format!(
                    "whose elements {} do not match any expected elements",
                    actual_idx
                )),
                (0, 1) => {
                    Some(format!("which no element match the expected element {}", expected_idx))
                }
                (0, _) => {
                    Some(format!("which no element match the expected elements {}", expected_idx))
                }
                (1, 1) => Some(format!(
                    "whose element {} does not match any expected elements and no elements match the expected element {}",
                    actual_idx, expected_idx
                )),
                (_, 1) => Some(format!(
                    "whose elements {} do not match any expected elements and no elements match the expected element {}",
                    actual_idx, expected_idx
                )),
                (1, _) => Some(format!(
                    "whose element {} does not match any expected elements and no elements match the expected elements {}",
                    actual_idx, expected_idx
                )),
                (_, _) => Some(format!(
                    "whose elements {} do not match any expected elements and no elements match the expected elements {}",
                    actual_idx, expected_idx
                )),
            }
        }

        fn unmatchable_actual(&self) -> Vec<usize> {
            self.unmatchable_actual
                .iter()
                .enumerate()
                .filter_map(|(idx, b)| b.then_some(idx))
                .collect()
        }

        fn unmatchable_expected(&self) -> Vec<usize> {
            self.unmatchable_expected
                .iter()
                .enumerate()
                .filter_map(|(idx, b)| b.then_some(idx))
                .collect()
        }
    }

    /// The representation of a match between actual and expected.
    /// The value at idx represents to which expected the actual at idx is
    /// matched with. For example, `BestMatch([Some(0), None, Some(1)])`
    /// means:
    ///  * The 0th element in actual matches the 0th element in expected.
    ///  * The 1st element in actual does not match.
    ///  * The 2nd element in actual matches the 1st element in expected.
    struct BestMatch<const N: usize>(Vec<Option<usize>>);

    impl<const N: usize> BestMatch<N> {
        fn is_full_match(&self) -> bool {
            self.0.iter().all(|o| o.is_some())
        }

        fn is_superset_match(&self) -> bool {
            self.get_unmatched_expected().is_empty()
        }

        fn get_matches(&self) -> Vec<(usize, usize)> {
            self.0
                .iter()
                .enumerate()
                .filter_map(|(actual_idx, maybe_expected_idx)| {
                    maybe_expected_idx.map(|expected_idx| (actual_idx, expected_idx))
                })
                .collect()
        }

        fn get_unmatched_actual(&self) -> Vec<usize> {
            self.0
                .iter()
                .enumerate()
                .filter(|&(_, o)| o.is_none())
                .map(|(actual_idx, _)| actual_idx)
                .collect()
        }

        fn get_unmatched_expected(&self) -> Vec<usize> {
            let matched_expected: HashSet<_> = self.0.iter().flatten().collect();
            (0..N).filter(|expected_idx| !matched_expected.contains(expected_idx)).collect()
        }

        fn get_explanation<'a, T: Debug, ContainerT: Debug>(
            &self,
            actual: &ContainerT,
            expected: &[&'a dyn Matcher<T>; N],
            requirements: Requirements,
        ) -> Option<String>
        where
            for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
        {
            let actual: Vec<_> = actual.into_iter().collect();
            if self.is_full_match() {
                return None;
            }
            let mut error_message =
                format!("which does not have a {requirements} match with the expected elements.");

            error_message.push_str("\n  The best match found was: ");

            for (actual_idx, expected_idx) in self.get_matches() {
                error_message.push_str(
                    format!(
                        "\n    Actual element {:?} at index {} matched expected element `{}` at index {}.",
                        actual[actual_idx],
                        actual_idx,
                        expected[expected_idx].describe(MatcherResult::Matches),
                        expected_idx
                    )
                    .as_str(),
                );
            }

            for actual_idx in self.get_unmatched_actual() {
                error_message.push_str(format!(
                    "\n    Actual element {:?} at index {} did not match any remaining expected element.",
                    actual[actual_idx], actual_idx
                ).as_str());
            }
            for expected_idx in self.get_unmatched_expected() {
                error_message.push_str(format!(
                    "\n    Expected element `{}` at index {} did not match any remaining actual element.",
                    expected[expected_idx].describe(MatcherResult::Matches), expected_idx
                ).as_str());
            }

            Some(error_message)
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(google3))]
    use crate as googletest;
    use googletest::matcher::Matcher;
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{google_test, verify_that, Result};
    use matchers::{contains_substring, displays_as, eq, err, ge, not};

    #[google_test]
    fn unordered_elements_are_matches_vector() -> Result<()> {
        let value = vec![1, 2, 3];
        verify_that!(value, unordered_elements_are![eq(1), eq(2), eq(3)])
    }

    #[google_test]
    fn unordered_elements_are_matches_size() -> Result<()> {
        let value = vec![1, 2];
        verify_that!(value, not(unordered_elements_are![eq(1), eq(2), eq(3)]))
    }

    #[google_test]
    fn unordered_elements_are_description_mismatch() -> Result<()> {
        let result = verify_that!(vec![1, 4, 3], unordered_elements_are![eq(1), eq(2), eq(3)]);
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "\
Value of: vec![1, 4, 3]
Expected: contains elements matching in any order:
is equal to 1
is equal to 2
is equal to 3
Actual: [1, 4, 3], whose element #1 does not match any expected elements and no elements match the expected element #1"
            )))
        )
    }

    #[google_test]
    fn unordered_elements_are_matches_unordered() -> Result<()> {
        let value = vec![1, 2];
        verify_that!(value, unordered_elements_are![eq(2), eq(1)])
    }

    #[google_test]
    fn unordered_elements_are_matches_unordered_with_repetition() -> Result<()> {
        let value = vec![1, 2, 1, 2, 1];
        verify_that!(value, unordered_elements_are![eq(1), eq(1), eq(1), eq(2), eq(2)])
    }

    #[google_test]
    fn unordered_elements_are_explains_mismatch_due_to_wrong_size() -> Result<()> {
        verify_that!(
            unordered_elements_are![eq(2), eq(3), eq(4)].explain_match(&vec![2, 3]),
            displays_as(eq("which has size 2 (expected 3)"))
        )
    }

    #[google_test]
    fn unordered_elements_are_description_no_full_match() -> Result<()> {
        verify_that!(
            unordered_elements_are![eq(1), eq(2), eq(2)].explain_match(&vec![1, 1, 2]),
            displays_as(eq("which does not have a perfect match with the expected elements.
  The best match found was: 
    Actual element 1 at index 0 matched expected element `is equal to 1` at index 0.
    Actual element 2 at index 2 matched expected element `is equal to 2` at index 1.
    Actual element 1 at index 1 did not match any remaining expected element.
    Expected element `is equal to 2` at index 2 did not match any remaining actual element."))
        )
    }

    #[google_test]
    fn unordered_elements_are_unmatchable_expected_description_mismatch() -> Result<()> {
        verify_that!(
            unordered_elements_are![eq(1), eq(2), eq(3)].explain_match(&vec![1, 1, 3]),
            displays_as(eq("which no element match the expected element #1"))
        )
    }

    #[google_test]
    fn unordered_elements_are_unmatchable_actual_description_mismatch() -> Result<()> {
        verify_that!(
            unordered_elements_are![eq(1), eq(1), eq(3)].explain_match(&vec![1, 2, 3]),
            displays_as(eq("whose element #1 does not match any expected elements"))
        )
    }

    #[google_test]
    fn contains_each_matches_when_one_to_one_correspondence_present() -> Result<()> {
        verify_that!(vec![2, 3, 4], contains_each!(eq(2), eq(3), eq(4)))
    }

    #[google_test]
    fn contains_each_matches_when_no_matchers_present() -> Result<()> {
        verify_that!(vec![2, 3, 4], contains_each!())
    }

    #[google_test]
    fn contains_each_matches_when_excess_elements_present() -> Result<()> {
        verify_that!(vec![1, 2, 3, 4], contains_each!(eq(2), eq(3), eq(4)))
    }

    #[google_test]
    fn contains_each_does_not_match_when_matchers_are_unmatched() -> Result<()> {
        verify_that!(vec![1, 2, 3], not(contains_each!(eq(2), eq(3), eq(4))))
    }

    #[google_test]
    fn contains_each_explains_mismatch_due_to_wrong_size() -> Result<()> {
        verify_that!(
            contains_each![eq(2), eq(3), eq(4)].explain_match(&vec![2, 3]),
            displays_as(eq("which has size 2 (expected at least 3)"))
        )
    }

    #[google_test]
    fn contains_each_explains_missing_element_in_mismatch() -> Result<()> {
        verify_that!(
            contains_each![eq(2), eq(3), eq(4)].explain_match(&vec![1, 2, 3]),
            displays_as(eq("which no element match the expected element #2"))
        )
    }

    #[google_test]
    fn contains_each_explains_mismatch_due_to_no_graph_matching_found() -> Result<()> {
        verify_that!(
            contains_each![ge(2), ge(2)].explain_match(&vec![1, 2]),
            displays_as(eq("which does not have a superset match with the expected elements.
  The best match found was: 
    Actual element 2 at index 1 matched expected element `is greater than or equal to 2` at index 0.
    Actual element 1 at index 0 did not match any remaining expected element.
    Expected element `is greater than or equal to 2` at index 1 did not match any remaining actual element."))
        )
    }
}
