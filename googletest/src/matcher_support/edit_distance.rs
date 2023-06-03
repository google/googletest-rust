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

/// Compute the edit list of `left` and `right`.
///
/// This returns a vec of [`Edit`] which can be applied to `left` to obtain
/// `right`. See <https://en.wikipedia.org/wiki/Edit_distance> for more
/// information.
///
/// It uses [Myers Algorithm](https://neil.fraser.name/writing/diff/myers.pdf)
/// with a maximum edit distance of 25. If more than 25 insertions or deletions
/// are required to convert `left` to `right`, it returns a default fallback
/// edit list which deletes all items from `left` and inserts all items in
/// `right`.
pub(crate) fn edit_list<T: PartialEq + Copy>(
    left: impl IntoIterator<Item = T>,
    right: impl IntoIterator<Item = T>,
) -> Vec<Edit<T>> {
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
                // Always the case in the first (outer) iteration distance = 0.
                (None, None) => (Path::default(), None),

                // True when k = -distance. There is no previous parent path
                // yet.
                (None, Some(path_k_plus_1)) => (
                    path_k_plus_1.clone(),
                    right.get(path_k_plus_1.right_endpoint).copied().map(Edit::ExtraRight),
                ),

                // True when k = distance. There is no next parent path yet.
                (Some(path_k_minus_1), None) => (
                    path_k_minus_1.extend_left_endpoint(),
                    left.get(path_k_minus_1.left_endpoint).copied().map(Edit::ExtraLeft),
                ),

                // True when k is strictly between -distance and distance. Both
                // parent path were set in the last iteration.
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
                return path.edits;
            }

            path.left_endpoint = left_endpoint;
            path.right_endpoint = right_endpoint;
            paths_current.push(path);
        }
        paths_last = paths_current;
    }

    // Fallback when the distance is too large: assume the two are completely different.
    let mut result: Vec<_> = left.iter().map(|t| Edit::ExtraLeft(*t)).collect();
    result.extend(right.iter().map(|t| Edit::ExtraRight(*t)));
    result
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

/// An edit operation on two sequences of `T`.
#[derive(Debug, Clone)]
pub(crate) enum Edit<T> {
    /// An extra `T` was added to the left sequence.
    ExtraLeft(T),
    /// An extra `T` was added to the right sequence.
    ExtraRight(T),
    /// An element was added to each sequence.
    Both(T),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use quickcheck::{quickcheck, Arbitrary};

    #[test]
    fn returns_single_edit_when_strings_are_equal() -> Result<()> {
        let result = edit_list(["A string"], ["A string"]);
        verify_that!(result, elements_are![matches_pattern!(Edit::Both(eq("A string")))])
    }

    #[test]
    fn returns_sequence_of_two_common_parts() -> Result<()> {
        let result = edit_list(["A string (1)", "A string (2)"], ["A string (1)", "A string (2)"]);
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::Both(eq("A string (1)"))),
                matches_pattern!(Edit::Both(eq("A string (2)")))
            ]
        )
    }

    #[test]
    fn returns_extra_left_when_only_left_has_content() -> Result<()> {
        let result = edit_list(["A string"], []);
        verify_that!(result, elements_are![matches_pattern!(Edit::ExtraLeft(eq("A string"))),])
    }

    #[test]
    fn returns_extra_right_when_only_right_has_content() -> Result<()> {
        let result = edit_list([], ["A string"]);
        verify_that!(result, elements_are![matches_pattern!(Edit::ExtraRight(eq("A string"))),])
    }

    #[test]
    fn returns_extra_left_followed_by_extra_right_with_two_unequal_strings() -> Result<()> {
        let result = edit_list(["A string"], ["Another string"]);
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::ExtraLeft(eq("A string"))),
                matches_pattern!(Edit::ExtraRight(eq("Another string"))),
            ]
        )
    }

    #[test]
    fn interleaves_extra_left_and_extra_right_when_multiple_lines_differ() -> Result<()> {
        let result = edit_list(["A string", "A string"], ["Another string", "Another string"]);
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::ExtraLeft(eq("A string"))),
                matches_pattern!(Edit::ExtraRight(eq("Another string"))),
                matches_pattern!(Edit::ExtraLeft(eq("A string"))),
                matches_pattern!(Edit::ExtraRight(eq("Another string"))),
            ]
        )
    }

    #[test]
    fn returns_common_part_plus_difference_when_there_is_common_prefix() -> Result<()> {
        let result = edit_list(["Common part", "Left only"], ["Common part", "Right only"]);
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
            ]
        )
    }

    #[test]
    fn returns_common_part_plus_extra_left_when_left_has_extra_suffix() -> Result<()> {
        let result = edit_list(["Common part", "Left only"], ["Common part"]);
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
            ]
        )
    }

    #[test]
    fn returns_common_part_plus_extra_right_when_right_has_extra_suffix() -> Result<()> {
        let result = edit_list(["Common part"], ["Common part", "Right only"]);
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
            ]
        )
    }

    #[test]
    fn returns_difference_plus_common_part_when_there_is_common_suffix() -> Result<()> {
        let result = edit_list(["Left only", "Common part"], ["Right only", "Common part"]);
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
                matches_pattern!(Edit::Both(eq("Common part"))),
            ]
        )
    }

    #[test]
    fn returns_difference_plus_common_part_plus_difference_when_there_is_common_infix() -> Result<()>
    {
        let result = edit_list(
            ["Left only (1)", "Common part", "Left only (2)"],
            ["Right only (1)", "Common part", "Right only (2)"],
        );
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::ExtraLeft(eq("Left only (1)"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only (1)"))),
                matches_pattern!(Edit::Both(eq("Common part"))),
                matches_pattern!(Edit::ExtraLeft(eq("Left only (2)"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only (2)"))),
            ]
        )
    }

    #[test]
    fn returns_common_part_plus_difference_plus_common_part_when_there_is_common_prefix_and_suffix(
    ) -> Result<()> {
        let result = edit_list(
            ["Common part (1)", "Left only", "Common part (2)"],
            ["Common part (1)", "Right only", "Common part (2)"],
        );
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::Both(eq("Common part (1)"))),
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
                matches_pattern!(Edit::Both(eq("Common part (2)"))),
            ]
        )
    }

    #[test]
    fn returns_common_part_plus_extra_left_plus_common_part_when_there_is_common_prefix_and_suffix(
    ) -> Result<()> {
        let result = edit_list(
            ["Common part (1)", "Left only", "Common part (2)"],
            ["Common part (1)", "Common part (2)"],
        );
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::Both(eq("Common part (1)"))),
                matches_pattern!(Edit::ExtraLeft(eq("Left only"))),
                matches_pattern!(Edit::Both(eq("Common part (2)"))),
            ]
        )
    }

    #[test]
    fn returns_common_part_plus_extra_right_plus_common_part_when_there_is_common_prefix_and_suffix(
    ) -> Result<()> {
        let result = edit_list(
            ["Common part (1)", "Common part (2)"],
            ["Common part (1)", "Right only", "Common part (2)"],
        );
        verify_that!(
            result,
            elements_are![
                matches_pattern!(Edit::Both(eq("Common part (1)"))),
                matches_pattern!(Edit::ExtraRight(eq("Right only"))),
                matches_pattern!(Edit::Both(eq("Common part (2)"))),
            ]
        )
    }

    #[test]
    fn returns_rewrite_fallback_when_maximum_distance_exceeded() -> Result<()> {
        let result = edit_list(0..=20, 20..40);
        verify_that!(result, not(contains(matches_pattern!(Edit::Both(anything())))))
    }

    quickcheck! {
        #[test]
        fn edit_list_edits_left_to_right(
            left: Vec<Alphabet>,
            right: Vec<Alphabet>
        ) -> bool {
            let edit_list = edit_list(left.clone(), right.clone());
            apply_edits_to_left(&edit_list, &left) == right
        }
    }

    quickcheck! {
        fn edit_list_edits_right_to_left(
            left: Vec<Alphabet>,
            right: Vec<Alphabet>
        ) -> bool {
            let edit_list = edit_list(left.clone(), right.clone());
            apply_edits_to_right(&edit_list, &right) == left
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
            }
        }
        assert_that!(right_iter.next(), none());
        result
    }
}
