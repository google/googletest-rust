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

#[doc(hidden)]
pub mod internal {
    use crate::description::Description;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::matcher_support::count_elements::count_elements;
    use std::collections::HashSet;
    use std::fmt::{Debug, Display};

    /// The requirements of the mapping between matchers and actual values by
    /// which [`UnorderedElementsAre`] is deemed to match its input.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    #[derive(Clone, Copy)]
    pub enum Requirements {
        /// There must be a 1:1 correspondence between the actual values and the
        /// matchers.
        PerfectMatch,

        /// The mapping from matched actual values to their corresponding
        /// matchers must be surjective.
        Superset,

        /// The mapping from matchers to matched actual values must be
        /// surjective.
        Subset,
    }

    impl Requirements {
        pub fn explain_size_mismatch<ContainerT: IntoIterator + Copy>(
            &self,
            actual: ContainerT,
            expected_size: usize,
        ) -> Option<Description> {
            let actual_size = count_elements(actual);
            match self {
                Requirements::PerfectMatch if actual_size != expected_size => Some(
                    format!("which has size {} (expected {})", actual_size, expected_size).into(),
                ),

                Requirements::Superset if actual_size < expected_size => Some(
                    format!("which has size {} (expected at least {})", actual_size, expected_size)
                        .into(),
                ),

                Requirements::Subset if actual_size > expected_size => Some(
                    format!("which has size {} (expected at most {})", actual_size, expected_size)
                        .into(),
                ),

                _ => None,
            }
        }
    }

    impl Display for Requirements {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Requirements::PerfectMatch => {
                    write!(f, "perfect")
                }
                Requirements::Superset => {
                    write!(f, "superset")
                }
                Requirements::Subset => {
                    write!(f, "subset")
                }
            }
        }
    }

    /// The bipartite matching graph between actual and expected elements.
    pub(crate) struct MatchMatrix<const N: usize>(Vec<[MatcherResult; N]>);

    impl<const N: usize> MatchMatrix<N> {
        pub(crate) fn generate<
            'a,
            T: Debug + Copy + 'a,
            ContainerT: Debug + Copy + IntoIterator<Item = T>,
        >(
            actual: ContainerT,
            expected: &[Box<dyn Matcher<T> + 'a>; N],
        ) -> Self {
            let mut matrix = MatchMatrix(vec![[MatcherResult::NoMatch; N]; count_elements(actual)]);
            for (actual_idx, actual) in actual.into_iter().enumerate() {
                for (expected_idx, expected) in expected.iter().enumerate() {
                    matrix.0[actual_idx][expected_idx] = expected.matches(actual);
                }
            }
            matrix
        }

        pub(crate) fn is_match_for(&self, requirements: Requirements) -> bool {
            match requirements {
                Requirements::PerfectMatch => {
                    !self.find_unmatchable_elements().has_unmatchable_elements()
                        && self.find_best_match().is_full_match()
                }
                Requirements::Superset => {
                    !self.find_unmatched_expected().has_unmatchable_elements()
                        && self.find_best_match().is_superset_match()
                }
                Requirements::Subset => {
                    !self.find_unmatched_actual().has_unmatchable_elements()
                        && self.find_best_match().is_subset_match()
                }
            }
        }

        pub(crate) fn explain_unmatchable(
            &self,
            requirements: Requirements,
        ) -> Option<Description> {
            let unmatchable_elements = match requirements {
                Requirements::PerfectMatch => self.find_unmatchable_elements(),
                Requirements::Superset => self.find_unmatched_expected(),
                Requirements::Subset => self.find_unmatched_actual(),
            };
            unmatchable_elements.get_explanation()
        }

        // Verifies that each actual matches at least one expected and that
        // each expected matches at least one actual.
        // This is a necessary condition but not sufficient. But it is faster
        // than `find_best_match()`.
        fn find_unmatchable_elements(&self) -> UnmatchableElements<N> {
            let unmatchable_actual =
                self.0.iter().map(|row| row.iter().all(|&e| e.is_no_match())).collect();
            let mut unmatchable_expected = [false; N];
            for (col_idx, expected) in unmatchable_expected.iter_mut().enumerate() {
                *expected = self.0.iter().map(|row| row[col_idx]).all(|e| e.is_no_match());
            }
            UnmatchableElements { unmatchable_actual, unmatchable_expected }
        }

        fn find_unmatched_expected(&self) -> UnmatchableElements<N> {
            let mut unmatchable_expected = [false; N];
            for (col_idx, expected) in unmatchable_expected.iter_mut().enumerate() {
                *expected = self.0.iter().map(|row| row[col_idx]).all(|e| e.is_no_match());
            }
            UnmatchableElements { unmatchable_actual: vec![false; N], unmatchable_expected }
        }

        fn find_unmatched_actual(&self) -> UnmatchableElements<N> {
            let unmatchable_actual =
                self.0.iter().map(|row| row.iter().all(|e| e.is_no_match())).collect();
            UnmatchableElements { unmatchable_actual, unmatchable_expected: [false; N] }
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
        // | [ source ]                                                              |
        // |   |||                                                                   |
        // |   |||                                                                   |
        // |   ||\-> actual_match[0]=Some(1) -\   expected_match[0]=None    ---\     |
        // |   ||                             |                                |     |
        // |   |\--> actual_match[1]=None     \-> expected_match[1]=Some(0) --\|     |
        // |   |                                                              ||     |
        // |   \---> actual_match[2]=Some(2)  --> expected_match[2]=Some(2) -\||     |
        // |                                                                 |||     |
        // |         elements                     matchers                   vvv     |
        // |                                                               [ sink ]  |
        //
        // See Also:
        //   [1] Cormen, et al (2001). "Section 26.2: The Ford-Fulkerson method".
        //       "Introduction to Algorithms (Second ed.)", pp. 651-664.
        //   [2] "Ford-Fulkerson algorithm", Wikipedia,
        //       'http://en.wikipedia.org/wiki/Ford%E2%80%93Fulkerson_algorithm'
        pub(crate) fn find_best_match(&self) -> BestMatch<N> {
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
                if self.0[actual_idx][expected_idx].is_no_match() {
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
    pub(crate) struct UnmatchableElements<const N: usize> {
        unmatchable_actual: Vec<bool>,
        unmatchable_expected: [bool; N],
    }

    impl<const N: usize> UnmatchableElements<N> {
        fn has_unmatchable_elements(&self) -> bool {
            self.unmatchable_actual.iter().any(|b| *b)
                || self.unmatchable_expected.iter().any(|b| *b)
        }

        fn get_explanation(&self) -> Option<Description> {
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
                (1, 0) => {
                    Some(format!("whose element {actual_idx} does not match any expected elements").into())
                }
                (_, 0) => {
                    Some(format!("whose elements {actual_idx} do not match any expected elements",).into())
                }
                (0, 1) => Some(format!(
                    "which has no element matching the expected element {expected_idx}"
                ).into()),
                (0, _) => Some(format!(
                    "which has no elements matching the expected elements {expected_idx}"
                ).into()),
                (1, 1) => Some(format!(
                    "whose element {actual_idx} does not match any expected elements and no elements match the expected element {expected_idx}"
                ).into()),
                (_, 1) => Some(format!(
                    "whose elements {actual_idx} do not match any expected elements and no elements match the expected element {expected_idx}"
                ).into()),
                (1, _) => Some(format!(
                    "whose element {actual_idx} does not match any expected elements and no elements match the expected elements {expected_idx}"
                ).into()),
                (_, _) => Some(format!(
                    "whose elements {actual_idx} do not match any expected elements and no elements match the expected elements {expected_idx}"
                ).into()),
            }
        }

        fn unmatchable_actual(&self) -> Vec<usize> {
            self.unmatchable_actual
                .iter()
                .enumerate()
                .filter_map(|(idx, b)| if *b { Some(idx) } else { None })
                .collect()
        }

        fn unmatchable_expected(&self) -> Vec<usize> {
            self.unmatchable_expected
                .iter()
                .enumerate()
                .filter_map(|(idx, b)| if *b { Some(idx) } else { None })
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
    pub(crate) struct BestMatch<const N: usize>(Vec<Option<usize>>);

    impl<const N: usize> BestMatch<N> {
        pub(crate) fn is_full_match(&self) -> bool {
            self.0.iter().all(|o| o.is_some())
        }

        pub(crate) fn is_subset_match(&self) -> bool {
            self.is_full_match()
        }

        pub(crate) fn is_superset_match(&self) -> bool {
            self.get_unmatched_expected().is_empty()
        }

        fn get_matches(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
            self.0.iter().enumerate().filter_map(|(actual_idx, maybe_expected_idx)| {
                maybe_expected_idx.map(|expected_idx| (actual_idx, expected_idx))
            })
        }

        fn get_unmatched_actual(&self) -> impl Iterator<Item = usize> + '_ {
            self.0
                .iter()
                .enumerate()
                .filter(|&(_, o)| o.is_none())
                .map(|(actual_idx, _)| actual_idx)
        }

        fn get_unmatched_expected(&self) -> Vec<usize> {
            let matched_expected: HashSet<_> = self.0.iter().flatten().collect();
            (0..N).filter(|expected_idx| !matched_expected.contains(expected_idx)).collect()
        }

        pub(crate) fn get_explanation<
            'a,
            T: Debug + Copy,
            ContainerT: Debug + Copy + IntoIterator<Item = T>,
        >(
            &self,
            actual: ContainerT,
            expected: &[Box<dyn Matcher<T> + 'a>; N],
            requirements: Requirements,
        ) -> Option<Description> {
            let actual: Vec<_> = actual.into_iter().collect();
            if self.is_full_match() {
                return None;
            }
            let mut error_message =
                format!("which does not have a {requirements} match with the expected elements.");

            error_message.push_str("\n  The best match found was: ");

            let matches = self.get_matches().map(|(actual_idx, expected_idx)|{
                format!(
                    "Actual element {:?} at index {actual_idx} matched expected element `{}` at index {expected_idx}.",
                    actual[actual_idx],
                    expected[expected_idx].describe(MatcherResult::Match),
            )});

            let unmatched_actual = self.get_unmatched_actual().map(|actual_idx| {
                format!(
                    "Actual element {:#?} at index {actual_idx} did not match any remaining expected element.",
                    actual[actual_idx]
                )
            });

            let unmatched_expected = self.get_unmatched_expected().into_iter().map(|expected_idx|{format!(
                "Expected element `{}` at index {expected_idx} did not match any remaining actual element.",
                expected[expected_idx].describe(MatcherResult::Match)
            )});

            let best_match = matches
                .chain(unmatched_actual)
                .chain(unmatched_expected)
                .collect::<Description>()
                .indent();
            Some(format!(
                "which does not have a {requirements} match with the expected elements. The best match found was:\n{best_match}"
            ).into())
        }
    }
}
