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
const MAX_DISTANCE: i32 = 25;

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
    /// An extra `T` was added to the left sequence.
    ExtraLeft(T),

    /// An extra `T` was added to the right sequence.
    ExtraRight(T),

    /// An element was added to each sequence.
    Both(T),

    /// Additional (unlisted) elements are present in the left sequence.
    ///
    /// This is only output in the mode [`Mode::Prefix`]. Its presence precludes
    /// reconstructing the left sequence from the right sequence.
    AdditionalLeft,
}

/// Controls the termination condition of [`edit_list`].
pub(crate) enum Mode {
    /// Indicates that the two arguments are intended to be equal.
    ///
    /// The entire edit list to transform between `left` and `right` is
    /// returned.
    Exact,

    /// Indicates that `right` is inteded to be a prefix of `left`.
    ///
    /// Any additional parts of `left` after the prefix `right` are omitted from
    /// the output.
    Prefix,
}

/// Computes the edit list of `left` and `right`.
///
/// If `left` and `right` are equal, then this returns [`Difference::Equal`]. If
/// they are different but have an
/// [edit distance](https://en.wikipedia.org/wiki/Edit_distance)
/// of at most [`MAX_DISTANCE`], this returns [`Difference::Editable`] with the
/// sequence of [`Edit`] which can be applied to `left` to obtain `right`.
/// Otherwise this returns [`Difference::Unrelated`].
///
/// This uses [Myers Algorithm](https://neil.fraser.name/writing/diff/myers.pdf)
/// with a maximum edit distance of [`MAX_DISTANCE`]. Thus the worst-case
/// runtime is linear in both the input length and [`MAX_DISTANCE`].
pub(crate) fn edit_list<T: PartialEq + Copy>(
    left: impl IntoIterator<Item = T>,
    right: impl IntoIterator<Item = T>,
    mode: Mode,
) -> Difference<T> {
    let left: Vec<_> = left.into_iter().collect();
    let right: Vec<_> = right.into_iter().collect();

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
                    right.get(path_k_plus_1.right_endpoint).copied().map(Edit::ExtraRight),
                ),

                // k = distance. There is no next parent path yet.
                (Some(path_k_minus_1), None) => (
                    path_k_minus_1.extend_left_endpoint(),
                    left.get(path_k_minus_1.left_endpoint).copied().map(Edit::ExtraLeft),
                ),

                // k is strictly between -distance and distance. Both parent paths were set in the
                // last iteration.
                (Some(path_k_minus_1), Some(path_k_plus_1)) => {
                    // This decides whether the algorithm prefers to add an edit
                    // from the left or from the right when the rows differ. We
                    // alternate so that the elements of differing blocks
                    // interleave rather than all elements of each respective
                    // side being output in a single block.
                    if (distance % 2 == 0
                        && path_k_plus_1.left_endpoint > path_k_minus_1.left_endpoint)
                        || (distance % 2 == 1
                            && path_k_plus_1.right_endpoint > path_k_minus_1.right_endpoint)
                    {
                        (
                            path_k_plus_1.clone(),
                            right.get(path_k_plus_1.right_endpoint).copied().map(Edit::ExtraRight),
                        )
                    } else {
                        (
                            path_k_minus_1.extend_left_endpoint(),
                            left.get(path_k_minus_1.left_endpoint).copied().map(Edit::ExtraLeft),
                        )
                    }
                }
            };
            path.edits.extend(edit);

            // Advance through any common elements starting at the current path.
            let (mut left_endpoint, mut right_endpoint) =
                (path.left_endpoint, (path.left_endpoint as i32 - k) as usize);
            while left_endpoint < left.len()
                && right_endpoint < right.len()
                && left[left_endpoint] == right[right_endpoint]
            {
                path.edits.push(Edit::Both(left[left_endpoint]));
                (left_endpoint, right_endpoint) = (left_endpoint + 1, right_endpoint + 1);
            }

            // If we have exhausted both inputs, we are done.
            if left_endpoint == left.len() && right_endpoint == right.len() {
                return if path.edits.iter().any(|v| !matches!(v, Edit::Both(_))) {
                    Difference::Editable(path.edits)
                } else {
                    Difference::Equal
                };
            }

            path.left_endpoint = left_endpoint;
            path.right_endpoint = right_endpoint;
            paths_current.push(path);
        }

        if matches!(mode, Mode::Prefix) {
            if let Some(path) = paths_current
                .iter_mut()
                .filter(|p| p.right_endpoint == right.len())
                .max_by(|p1, p2| p1.edits.len().cmp(&p2.edits.len()))
            {
                if let Some(Edit::ExtraRight(_)) = path.edits.last() {
                    if path.left_endpoint < left.len() {
                        path.edits.push(Edit::ExtraLeft(left[path.left_endpoint]));
                    }
                }
                path.edits.push(Edit::AdditionalLeft);
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

#[derive(Clone)]
struct Path<T: Clone> {
    left_endpoint: usize,
    right_endpoint: usize,
    edits: Vec<Edit<T>>,
}

impl<T: Clone> Default for Path<T> {
    fn default() -> Self {
        Self { left_endpoint: 0, right_endpoint: 0, edits: vec![] }
    }
}

impl<T: Clone> Path<T> {
    fn extend_left_endpoint(&self) -> Self {
        Self {
            left_endpoint: self.left_endpoint + 1,
            right_endpoint: self.right_endpoint,
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
    fn returns_extra_left_when_only_left_has_content() -> Result<()> {
        let result = edit_list(["A string"], [], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![matches_pattern!(
                Edit::ExtraLeft(eq("A string"))
            )]))
        )
    }

    #[test]
    fn returns_extra_right_when_only_right_has_content() -> Result<()> {
        let result = edit_list([], ["A string"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![matches_pattern!(
                Edit::ExtraRight(eq("A string"))
            )]))
        )
    }

    #[test]
    fn returns_extra_left_followed_by_extra_right_with_two_unequal_strings() -> Result<()> {
        let result = edit_list(["A string"], ["Another string"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraLeft(eq("A string"))),
                matches_pattern!(Edit::ExtraRight(eq("Another string"))),
            ]))
        )
    }

    #[test]
    fn interleaves_extra_left_and_extra_right_when_multiple_lines_differ() -> Result<()> {
        let result =
            edit_list(["A string", "A string"], ["Another string", "Another string"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraLeft(eq("A string"))),
                matches_pattern!(Edit::ExtraRight(eq("Another string"))),
                matches_pattern!(Edit::ExtraLeft(eq("A string"))),
                matches_pattern!(Edit::ExtraRight(eq("Another string"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_difference_when_there_is_common_prefix() -> Result<()> {
        let result =
            edit_list(["Common part", "Left only"], ["Common part", "Right only"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_extra_left_when_left_has_extra_suffix() -> Result<()> {
        let result = edit_list(["Common part", "Left only"], ["Common part"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_extra_right_when_right_has_extra_suffix() -> Result<()> {
        let result = edit_list(["Common part"], ["Common part", "Right only"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
            ]))
        )
    }

    #[test]
    fn returns_difference_plus_common_part_when_there_is_common_suffix() -> Result<()> {
        let result =
            edit_list(["Left only", "Common part"], ["Right only", "Common part"], Mode::Exact);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
                matches_pattern!(Edit::Both(eq("Common part"))),
            ]))
        )
    }

    #[test]
    fn returns_difference_plus_common_part_plus_difference_when_there_is_common_infix() -> Result<()>
    {
        let result = edit_list(
            ["Left only (1)", "Common part", "Left only (2)"],
            ["Right only (1)", "Common part", "Right only (2)"],
            Mode::Exact,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraLeft(eq("Left only (1)"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only (1)"))),
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraLeft(eq("Left only (2)"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only (2)"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_difference_plus_common_part_when_there_is_common_prefix_and_suffix(
    ) -> Result<()> {
        let result = edit_list(
            ["Common part (1)", "Left only", "Common part (2)"],
            ["Common part (1)", "Right only", "Common part (2)"],
            Mode::Exact,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part (1)"))),
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
                matches_pattern!(Edit::Both(eq("Common part (2)"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_extra_left_plus_common_part_when_there_is_common_prefix_and_suffix(
    ) -> Result<()> {
        let result = edit_list(
            ["Common part (1)", "Left only", "Common part (2)"],
            ["Common part (1)", "Common part (2)"],
            Mode::Exact,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part (1)"))),
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
                matches_pattern!(Edit::Both(eq("Common part (2)"))),
            ]))
        )
    }

    #[test]
    fn returns_common_part_plus_extra_right_plus_common_part_when_there_is_common_prefix_and_suffix(
    ) -> Result<()> {
        let result = edit_list(
            ["Common part (1)", "Common part (2)"],
            ["Common part (1)", "Right only", "Common part (2)"],
            Mode::Exact,
        );
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::Both(eq("Common part (1)"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
                matches_pattern!(Edit::Both(eq("Common part (2)"))),
            ]))
        )
    }

    #[test]
    fn skips_extra_parts_on_left_at_end_in_prefix_mode() -> Result<()> {
        let result =
            edit_list(["Common part", "Left only"], ["Right only", "Common part"], Mode::Prefix);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(not(contains(matches_pattern!(
                Edit::ExtraLeft(eq("Left only"))
            )))))
        )
    }

    #[test]
    fn does_not_skip_left_line_corresponding_to_last_right_line_in_prefix_mode() -> Result<()> {
        let result = edit_list(["Left only"], ["Right only"], Mode::Prefix);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
            ]))
        )
    }

    #[test]
    fn does_not_skip_extra_parts_on_left_in_prefix_mode_at_end_when_they_are_in_common(
    ) -> Result<()> {
        let result =
            edit_list(["Left only", "Common part"], ["Right only", "Common part"], Mode::Prefix);
        verify_that!(
            result,
            matches_pattern!(Difference::Editable(elements_are![
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
                matches_pattern!(Edit::Both(eq("Common part"))),
            ]))
        )
    }

    #[test]
    fn returns_unrelated_when_maximum_distance_exceeded() -> Result<()> {
        let result = edit_list(0..=20, 20..40, Mode::Exact);
        verify_that!(result, matches_pattern!(Difference::Unrelated))
    }

    quickcheck! {
        fn edit_list_edits_left_to_right(
            left: Vec<Alphabet>,
            right: Vec<Alphabet>
        ) -> TestResult {
            match edit_list(left.clone(), right.clone(), Mode::Exact) {
                Difference::Equal => TestResult::from_bool(left == right),
                Difference::Editable(edit_list) => {
                    TestResult::from_bool(apply_edits_to_left(&edit_list, &left) == right)
                }
                Difference::Unrelated => {
                    if left == right {
                        TestResult::failed()
                    } else {
                        TestResult::discard()
                    }
                }
            }
        }
    }

    quickcheck! {
        fn edit_list_edits_right_to_left(
            left: Vec<Alphabet>,
            right: Vec<Alphabet>
        ) -> TestResult {
            match edit_list(left.clone(), right.clone(), Mode::Exact) {
                Difference::Equal => TestResult::from_bool(left == right),
                Difference::Editable(edit_list) => {
                    TestResult::from_bool(apply_edits_to_right(&edit_list, &right) == left)
                }
                Difference::Unrelated => {
                    if left == right {
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

    fn apply_edits_to_left<T: PartialEq + Debug + Copy>(
        edit_list: &[Edit<T>],
        left: &[T],
    ) -> Vec<T> {
        let mut result = Vec::new();
        let mut left_iter = left.iter();
        for edit in edit_list {
            match edit {
                Edit::ExtraLeft(value) => {
                    assert_that!(left_iter.next(), some(eq(value)));
                }
                Edit::ExtraRight(value) => {
                    result.push(*value);
                }
                Edit::Both(value) => {
                    assert_that!(left_iter.next(), some(eq(value)));
                    result.push(*value);
                }
                Edit::AdditionalLeft => {
                    fail!("Unexpected Edit::AdditionalLeft").unwrap();
                }
            }
        }
        assert_that!(left_iter.next(), none());
        result
    }

    fn apply_edits_to_right<T: PartialEq + Debug + Copy>(
        edit_list: &[Edit<T>],
        right: &[T],
    ) -> Vec<T> {
        let mut result = Vec::new();
        let mut right_iter = right.iter();
        for edit in edit_list {
            match edit {
                Edit::ExtraLeft(value) => {
                    result.push(*value);
                }
                Edit::ExtraRight(value) => {
                    assert_that!(right_iter.next(), some(eq(value)));
                }
                Edit::Both(value) => {
                    assert_that!(right_iter.next(), some(eq(value)));
                    result.push(*value);
                }
                Edit::AdditionalLeft => {
                    fail!("Unexpected Edit::AdditionalLeft").unwrap();
                }
            }
        }
        assert_that!(right_iter.next(), none());
        result
    }
}
