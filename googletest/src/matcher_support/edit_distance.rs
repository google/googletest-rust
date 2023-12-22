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

use std::fmt::Debug;

/// Maximum number of edits which can exist before [`edit_list`] falls back to a
/// complete rewrite to produce the edit list.
///
/// Increasing this limit increases the accuracy of [`edit_list`] while
/// quadratically increasing its worst-case runtime.
const MAX_DISTANCE: i32 = 50;

/// The difference between two inputs as produced by [`edit_list`].
#[derive(Debug)]
pub(crate) enum Difference<T> {
    /// No differences were detected at all.
    Equal,

    /// At most [`MAX_DISTANCE`] edits are required to convert one input to the
    /// other.
    ///
    /// Contains the list of [`Edit`] to perform the transformation.
    Editable(Vec<Edit<T>>),

    /// More than [`MAX_DISTANCE`] edits are required to convert one input to
    /// the other.
    ///
    /// The inputs are therefore considered unrelated and no edit list is
    /// provided.
    Unrelated,
}

/// An edit operation on two sequences of `T`.
#[derive(Debug, Clone)]
pub(crate) enum Edit<T> {
    /// An extra `T` was added to the actual sequence.
    ExtraActual(T),

    /// An extra `T` was added to the expected sequence.
    ExtraExpected(T),

    /// An element was added to each sequence.
    Both(T),

    /// Additional (unlisted) elements are present in the actual sequence.
    ///
    /// This is only output in the mode [`Mode::Prefix`]. Its presence precludes
    /// reconstructing the actual sequence from the expected sequence.
    AdditionalActual,
}

/// Controls the termination condition of [`edit_list`].
#[derive(Clone, Copy)]
pub(crate) enum Mode {
    /// Indicates that the two arguments are intended to be equal.
    ///
    /// The entire edit list to transform between `actual` and `expected` is
    /// returned.
    Exact,

    /// Indicates that `expected` is inteded to be a prefix of `actual`.
    ///
    /// Any additional parts of `actual` after the prefix `expected` are omitted
    /// from the output.
    Prefix,

    /// Similar to [`Mode::Prefix`], except it is also assumed that `actual` has
    /// some number of initial lines which should not be in the output.
    ///
    /// Any initial [`Edit::ExtraActual`] entries are replaced with
    /// [`Edit::AdditionalActual`] in the edit list. If the first entry which is
    /// not an [`Edit::ExtraActual`] is [`Edit::ExtraExpected`], then the last
    /// [`Edit::ExtraActual`] is actual in the output.
    Contains,
}

/// Computes the edit list of `actual` and `expected`.
///
/// If `actual` and `expected` are equal, then this returns
/// [`Difference::Equal`]. If they are different but have an
/// [edit distance](https://en.wikipedia.org/wiki/Edit_distance)
/// of at most [`MAX_DISTANCE`], this returns [`Difference::Editable`] with the
/// sequence of [`Edit`] which can be applied to `actual` to obtain `expected`.
/// Otherwise this returns [`Difference::Unrelated`].
///
/// This uses [Myers Algorithm](https://neil.fraser.name/writing/diff/myers.pdf)
/// with a maximum edit distance of [`MAX_DISTANCE`]. Thus the worst-case
/// runtime is linear in both the input length and [`MAX_DISTANCE`].
pub(crate) fn edit_list<T: PartialEq + Copy>(
    actual: impl IntoIterator<Item = T>,
    expected: impl IntoIterator<Item = T>,
    mode: Mode,
) -> Difference<T> {
    let actual: Vec<_> = actual.into_iter().collect();
    let expected: Vec<_> = expected.into_iter().collect();

    let mut paths_last: Vec<Path<T>> = Vec::new();

    for distance in 0..=MAX_DISTANCE {
        let mut paths_current = Vec::new();
        for k in (-distance..=distance).step_by(2) {
            // The following will be None when k is at the edges of the range,
            // since no paths have been created for k outside the range in the
            // previous iteration.
            let path_k_minus_1 = paths_last.get(index_of_k(k - 1, -distance + 1));
            let path_k_plus_1 = paths_last.get(index_of_k(k + 1, -distance + 1));

            let (mut path, edit) = match (path_k_minus_1, path_k_plus_1) {
                // This the first (outer) iteration.
                (None, None) => (Path::default(), None),

                // k = -distance. There is no previous parent path yet.
                (None, Some(path_k_plus_1)) => (
                    path_k_plus_1.clone(),
                    expected.get(path_k_plus_1.expected_endpoint).copied().map(Edit::ExtraExpected),
                ),

                // k = distance. There is no next parent path yet.
                (Some(path_k_minus_1), None) => (
                    path_k_minus_1.extend_actual_endpoint(),
                    actual.get(path_k_minus_1.actual_endpoint).copied().map(Edit::ExtraActual),
                ),

                // k is strictly between -distance and distance. Both parent paths were set in the
                // last iteration.
                (Some(path_k_minus_1), Some(path_k_plus_1)) => {
                    // This decides whether the algorithm prefers to add an edit
                    // from the actual or from the expected when the rows differ. We
                    // alternate so that the elements of differing blocks
                    // interleave rather than all elements of each respective
                    // side being output in a single block.
                    if (distance % 2 == 0
                        && path_k_plus_1.actual_endpoint > path_k_minus_1.actual_endpoint)
                        || (distance % 2 == 1
                            && path_k_plus_1.expected_endpoint > path_k_minus_1.expected_endpoint)
                    {
                        (
                            path_k_plus_1.clone(),
                            expected
                                .get(path_k_plus_1.expected_endpoint)
                                .copied()
                                .map(Edit::ExtraExpected),
                        )
                    } else {
                        (
                            path_k_minus_1.extend_actual_endpoint(),
                            actual
                                .get(path_k_minus_1.actual_endpoint)
                                .copied()
                                .map(Edit::ExtraActual),
                        )
                    }
                }
            };
            path.edits.extend(edit);

            // Advance through any common elements starting at the current path.
            let (mut actual_endpoint, mut expected_endpoint) =
                (path.actual_endpoint, (path.actual_endpoint as i32 - k) as usize);
            while actual_endpoint < actual.len()
                && expected_endpoint < expected.len()
                && actual[actual_endpoint] == expected[expected_endpoint]
            {
                path.edits.push(Edit::Both(actual[actual_endpoint]));
                (actual_endpoint, expected_endpoint) = (actual_endpoint + 1, expected_endpoint + 1);
            }

            // If we have exhausted both inputs, we are done.
            if actual_endpoint == actual.len() && expected_endpoint == expected.len() {
                return if path.edits.iter().any(|v| !matches!(v, Edit::Both(_))) {
                    if matches!(mode, Mode::Contains) {
                        compress_prefix_and_suffix(&mut path.edits);
                    }
                    Difference::Editable(path.edits)
                } else {
                    Difference::Equal
                };
            }

            path.actual_endpoint = actual_endpoint;
            path.expected_endpoint = expected_endpoint;
            paths_current.push(path);
        }

        if matches!(mode, Mode::Prefix) {
            if let Some(path) = paths_current
                .iter_mut()
                .filter(|p| p.expected_endpoint == expected.len())
                .max_by(|p1, p2| p1.edits.len().cmp(&p2.edits.len()))
            {
                // We've reached the end of the expected side but there could still be a
                // corresponding line on the actual which we haven't picked up into the edit
                // list. We'll just add it manually to the edit list. There's no
                // real harm doing so -- worst case is that there's an
                // additional line when there didn't have to be.
                if let Some(Edit::ExtraExpected(_)) = path.edits.last() {
                    if path.actual_endpoint < actual.len() {
                        // The edits from the actual should come before the corresponding one from
                        // the expected, so we insert rather than push.
                        path.edits.insert(
                            path.edits.len() - 1,
                            Edit::ExtraActual(actual[path.actual_endpoint]),
                        );
                    }
                }
                path.edits.push(Edit::AdditionalActual);
                return if path.edits.iter().any(|v| !matches!(v, Edit::Both(_))) {
                    Difference::Editable(std::mem::take(&mut path.edits))
                } else {
                    Difference::Equal
                };
            }
        }

        paths_last = paths_current;
    }

    Difference::Unrelated
}

fn index_of_k(k: i32, k_min: i32) -> usize {
    ((k - k_min) / 2) as usize
}

fn compress_prefix_and_suffix<T>(edits: &mut Vec<Edit<T>>) {
    if let Some(mut first_non_extra_actual_edit) =
        edits.iter().position(|e| !matches!(e, Edit::ExtraActual(_)))
    {
        if first_non_extra_actual_edit > 1
            && matches!(edits[first_non_extra_actual_edit], Edit::ExtraExpected(_))
        {
            first_non_extra_actual_edit -= 1;
        }
        edits.splice(..first_non_extra_actual_edit, [Edit::AdditionalActual]);
    }

    if let Some(mut last_non_extra_actual_edit) =
        edits.iter().rposition(|e| !matches!(e, Edit::ExtraActual(_)))
    {
        if last_non_extra_actual_edit < edits.len() - 1
            && matches!(edits[last_non_extra_actual_edit], Edit::ExtraExpected(_))
        {
            last_non_extra_actual_edit += 1;
        }
        edits.splice(last_non_extra_actual_edit + 1.., [Edit::AdditionalActual]);
    }
}

#[derive(Clone)]
struct Path<T: Clone> {
    actual_endpoint: usize,
    expected_endpoint: usize,
    edits: Vec<Edit<T>>,
}

impl<T: Clone> Default for Path<T> {
    fn default() -> Self {
        Self { actual_endpoint: 0, expected_endpoint: 0, edits: vec![] }
    }
}

impl<T: Clone> Path<T> {
    fn extend_actual_endpoint(&self) -> Self {
        Self {
            actual_endpoint: self.actual_endpoint + 1,
            expected_endpoint: self.expected_endpoint,
            edits: self.edits.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use quickcheck::{quickcheck, Arbitrary, TestResult};

    #[test]
    fn returns_equal_when_strings_are_equal() -> Result<()> {
        let result = edit_list(["A string"], ["A string"], Mode::Exact);
        verify_that!(result, matches_pattern!(Difference::Equal))
    }

    #[test]
    fn returns_sequence_of_two_common_parts() -> Result<()> {
        let result = edit_list(
            ["A string (1)", "A string (2)"],
            ["A string (1)", "A string (2)"],
            Mode::Exact,
        );
        verify_that!(result, matches_pattern!(Difference::Equal))
    }

    #[test]
    fn returns_extra_actual_when_only_actual_has_content() -> Result<()> {
        let result = edit_list(["A string"], [], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![matches_pattern!(
                Edit::ExtraActual(eq("A string"))
            )]))
        )
    }

    #[test]
    fn returns_extra_expected_when_only_expected_has_content() -> Result<()> {
        let result = edit_list([], ["A string"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![matches_pattern!(
                Edit::ExtraExpected(eq("A string"))
            )]))
        )
    }

    #[test]
    fn returns_extra_actual_followed_by_extra_expected_with_two_unequal_strings() -> Result<()> {
        let result = edit_list(["A string"], ["Another string"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraActual(eq("A string"))),
                matches_pattern!(Edit::ExtraExpected(eq("Another string"))),
            ]))
        )
    }

    #[test]
    fn interleaves_extra_actual_and_extra_expected_when_multiple_lines_differ() -> Result<()> {
        let result =
            edit_list(["A string", "A string"], ["Another string", "Another string"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraActual(eq("A string"))),
                matches_pattern!(Edit::ExtraExpected(eq("Another string"))),
                matches_pattern!(Edit::ExtraActual(eq("A string"))),
                matches_pattern!(Edit::ExtraExpected(eq("Another string"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_difference_when_there_is_common_prefix() -> Result<()> {
        let result = edit_list(
            ["Common part", "Actual only"],
            ["Common part", "Expected only"],
            Mode::Exact,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraActual(eq("Actual only"))),
                matches_pattern!(Edit::ExtraExpected(eq("Expected only"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_extra_actual_when_actual_has_extra_suffix() -> Result<()> {
        let result = edit_list(["Common part", "Actual only"], ["Common part"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraActual(eq("Actual only"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_extra_expected_when_expected_has_extra_suffix() -> Result<()> {
        let result = edit_list(["Common part"], ["Common part", "Expected only"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraExpected(eq("Expected only"))),
            ]))
        )
    }

    #[test]
    fn returns_difference_plus_common_part_when_there_is_common_suffix() -> Result<()> {
        let result = edit_list(
            ["Actual only", "Common part"],
            ["Expected only", "Common part"],
            Mode::Exact,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraActual(eq("Actual only"))),
                matches_pattern!(Edit::ExtraExpected(eq("Expected only"))),
                matches_pattern!(Edit::Both(eq("Common part"))),
            ]))
        )
    }

    #[test]
    fn returns_difference_plus_common_part_plus_difference_when_there_is_common_infix() -> Result<()>
    {
        let result = edit_list(
            ["Actual only (1)", "Common part", "Actual only (2)"],
            ["Expected only (1)", "Common part", "Expected only (2)"],
            Mode::Exact,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraActual(eq("Actual only (1)"))),
                matches_pattern!(Edit::ExtraExpected(eq("Expected only (1)"))),
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraActual(eq("Actual only (2)"))),
                matches_pattern!(Edit::ExtraExpected(eq("Expected only (2)"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_difference_plus_common_part_when_there_is_common_prefix_and_suffix()
    -> Result<()> {
        let result = edit_list(
            ["Common part (1)", "Actual only", "Common part (2)"],
            ["Common part (1)", "Expected only", "Common part (2)"],
            Mode::Exact,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part (1)"))),
                matches_pattern!(Edit::ExtraActual(eq("Actual only"))),
                matches_pattern!(Edit::ExtraExpected(eq("Expected only"))),
                matches_pattern!(Edit::Both(eq("Common part (2)"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_extra_actual_plus_common_part_when_there_is_common_prefix_and_suffix()
    -> Result<()> {
        let result = edit_list(
            ["Common part (1)", "Actual only", "Common part (2)"],
            ["Common part (1)", "Common part (2)"],
            Mode::Exact,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part (1)"))),
                matches_pattern!(Edit::ExtraActual(eq("Actual only"))),
                matches_pattern!(Edit::Both(eq("Common part (2)"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_extra_expected_plus_common_part_when_there_is_common_prefix_and_suffix()
    -> Result<()> {
        let result = edit_list(
            ["Common part (1)", "Common part (2)"],
            ["Common part (1)", "Expected only", "Common part (2)"],
            Mode::Exact,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part (1)"))),
                matches_pattern!(Edit::ExtraExpected(eq("Expected only"))),
                matches_pattern!(Edit::Both(eq("Common part (2)"))),
            ]))
        )
    }

    #[test]
    fn skips_extra_parts_on_actual_at_end_in_prefix_mode() -> Result<()> {
        let result = edit_list(
            ["Common part", "Actual only"],
            ["Expected only", "Common part"],
            Mode::Prefix,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(not(contains(matches_pattern!(
                Edit::ExtraActual(eq("Actual only"))
            )))))
        )
    }

    #[test]
    fn does_not_skip_extra_parts_on_actual_in_prefix_mode_at_end_when_they_are_in_common()
    -> Result<()> {
        let result = edit_list(
            ["Actual only", "Common part"],
            ["Expected only", "Common part"],
            Mode::Prefix,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraActual(eq("Actual only"))),
                matches_pattern!(Edit::ExtraExpected(eq("Expected only"))),
                matches_pattern!(Edit::Both(eq("Common part"))),
            ]))
        )
    }

    #[test]
    fn does_not_skip_corresponding_line_on_actual_when_actual_and_expected_differ_in_prefix_mode()
    -> Result<()> {
        let result = edit_list(["Actual only"], ["Expected only"], Mode::Prefix);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraActual(eq("Actual only"))),
                matches_pattern!(Edit::ExtraExpected(eq("Expected only"))),
                matches_pattern!(Edit::AdditionalActual),
            ]))
        )
    }

    #[test]
    fn returns_unrelated_when_maximum_distance_exceeded() -> Result<()> {
        let result = edit_list(0..=50, 60..110, Mode::Exact);
        verify_that!(result, matches_pattern!(Difference::Unrelated))
    }

    quickcheck! {
        fn edit_list_edits_actual_to_expected(
            actual: Vec<Alphabet>,
            expected: Vec<Alphabet>
        ) -> TestResult {
            match edit_list(actual.clone(), expected.clone(), Mode::Exact) {
                Difference::Equal => TestResult::from_bool(actual == expected),
                Difference::Editable(edit_list) => {
                    TestResult::from_bool(apply_edits_to_actual(&edit_list, &actual) == expected)
                }
                Difference::Unrelated => {
                    if actual == expected {
                        TestResult::failed()
                    } else {
                        TestResult::discard()
                    }
                }
            }
        }
    }

    quickcheck! {
        fn edit_list_edits_expected_to_actual(
            actual: Vec<Alphabet>,
            expected: Vec<Alphabet>
        ) -> TestResult {
            match edit_list(actual.clone(), expected.clone(), Mode::Exact) {
                Difference::Equal => TestResult::from_bool(actual == expected),
                Difference::Editable(edit_list) => {
                    TestResult::from_bool(apply_edits_to_expected(&edit_list, &expected) == actual)
                }
                Difference::Unrelated => {
                    if actual == expected {
                        TestResult::failed()
                    } else {
                        TestResult::discard()
                    }
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Alphabet {
        A,
        B,
        C,
    }

    impl Arbitrary for Alphabet {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            g.choose(&[Alphabet::A, Alphabet::B, Alphabet::C]).copied().unwrap()
        }
    }

    fn apply_edits_to_actual<T: PartialEq + Debug + Copy>(
        edit_list: &[Edit<T>],
        actual: &[T],
    ) -> Vec<T> {
        let mut result = Vec::new();
        let mut actual_iter = actual.iter();
        for edit in edit_list {
            match edit {
                Edit::ExtraActual(value) => {
                    assert_that!(actual_iter.next(), some(eq(value)));
                }
                Edit::ExtraExpected(value) => {
                    result.push(*value);
                }
                Edit::Both(value) => {
                    assert_that!(actual_iter.next(), some(eq(value)));
                    result.push(*value);
                }
                Edit::AdditionalActual => {
                    fail!("Unexpected Edit::AdditionalActual").unwrap();
                }
            }
        }
        assert_that!(actual_iter.next(), none());
        result
    }

    fn apply_edits_to_expected<T: PartialEq + Debug + Copy>(
        edit_list: &[Edit<T>],
        expected: &[T],
    ) -> Vec<T> {
        let mut result = Vec::new();
        let mut expected_iter = expected.iter();
        for edit in edit_list {
            match edit {
                Edit::ExtraActual(value) => {
                    result.push(*value);
                }
                Edit::ExtraExpected(value) => {
                    assert_that!(expected_iter.next(), some(eq(value)));
                }
                Edit::Both(value) => {
                    assert_that!(expected_iter.next(), some(eq(value)));
                    result.push(*value);
                }
                Edit::AdditionalActual => {
                    fail!("Unexpected Edit::AdditionalActual").unwrap();
                }
            }
        }
        assert_that!(expected_iter.next(), none());
        result
    }
}
