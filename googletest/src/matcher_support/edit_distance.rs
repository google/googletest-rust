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
use std::ops::Index;

/// Compute the edit list of `left` and `right`.
///
/// See <https://en.wikipedia.org/wiki/Edit_distance>
pub(crate) fn edit_list<T: Distance + Copy>(
    left: impl IntoIterator<Item = T>,
    right: impl IntoIterator<Item = T>,
    mode: Mode,
) -> Vec<Edit<T>> {
    let left: Vec<_> = left.into_iter().collect();
    let right: Vec<_> = right.into_iter().collect();

    struct TableElement<U> {
        cost: f64,
        last_edit: Edit<U>,
    }

    let mut table: Table<TableElement<T>> = Table::new(left.len() + 1, right.len() + 1);
    table.push(TableElement {
        cost: 0.0,
        // This is a placeholder value and should never be read.
        last_edit: Edit::ExtraLeft { left: left[0] },
    });

    // The mode changes how the beginning and the end of left is consumed.
    // EndsWith and Contains makes the consumption of elements of left before the
    // first element of right is consumed free. In the implementation, this leads
    // to table[(_, 0)] == 0.
    // StartsWith and Contains makes the consumption of character of left after the
    // last element of right is consumed free. In the implementation, this leads to
    // ExtraLeft being free when computing table[(_, right.len())].
    let (free_start, free_end) = match mode {
        Mode::FullMatch => (false, false),
        Mode::StartsWith => (false, true),
        Mode::EndsWith => (true, false),
        Mode::Contains => (true, true),
    };

    for idx in 1..(left.len() + 1) {
        table.push(TableElement {
            cost: if free_start { 0.0 } else { idx as _ },
            last_edit: Edit::ExtraLeft { left: left[idx - 1] },
        });
    }
    for idy in 1..(right.len() + 1) {
        table.push(TableElement {
            cost: idy as _,
            last_edit: Edit::ExtraRight { right: right[idy - 1] },
        });
        for idx in 1..(left.len() + 1) {
            let left_element = left[idx - 1];
            let right_element = right[idy - 1];
            let extra_left_cost = if free_end && idy == right.len() { 0.0 } else { 1.0 };
            let extra_left = TableElement {
                cost: extra_left_cost + table[(idx - 1, idy)].cost,
                last_edit: Edit::ExtraLeft { left: left_element },
            };
            let extra_right = TableElement {
                cost: 1.0 + table[(idx, idy - 1)].cost,
                last_edit: Edit::ExtraRight { right: right_element },
            };
            let distance = T::distance(left_element, right_element);
            let both = TableElement {
                cost: distance + table[(idx - 1, idy - 1)].cost,
                last_edit: Edit::Both { left: left_element, right: right_element, distance },
            };
            table.push(
                [extra_left, extra_right, both]
                    .into_iter()
                    .min_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap())
                    .unwrap(),
            );
        }
    }

    let mut path = Vec::with_capacity(left.len() + right.len());
    let mut current = (left.len(), right.len());
    while current != (0, 0) {
        let edit = table[current].last_edit.clone();
        current = match edit {
            Edit::ExtraLeft { .. } => (current.0 - 1, current.1),
            Edit::ExtraRight { .. } => (current.0, current.1 - 1),
            Edit::Both { .. } => (current.0 - 1, current.1 - 1),
        };
        path.push(edit);
    }
    path.reverse();
    path
}

/// Controls how `right` should match `left`.
pub(crate) enum Mode {
    /// `right` is fully matching `left`
    FullMatch,
    /// `right` should match the beginning of `left`
    #[allow(unused)]
    StartsWith,
    /// `right` should match the end of `left`
    #[allow(unused)]
    EndsWith,
    /// `right` should be contained in `left`
    #[allow(unused)]
    Contains,
}

/// An edit operation on two sequences of `T`.
#[derive(Debug, Clone)]
pub(crate) enum Edit<T> {
    /// An extra `T` was added to the left sequence.
    ExtraLeft { left: T },
    /// An extra `T` was added to the right sequence.
    ExtraRight { right: T },
    /// An element was added to each sequence.
    Both { left: T, right: T, distance: f64 },
}

/// Trait to implement the distance between two objects.
///
/// This allows to control the behavior of [`edit_list`] notably when two prefer
/// one [`Edit::Both`] or one [`Edit::ExtraRight`] and [`Edit::ExtraLeft`].
pub(crate) trait Distance {
    fn distance(left: Self, right: Self) -> f64;
}

/// Distance between two `char`s  which are not equal.
/// This value controls the behavior of [`edit_list`] on `char` to decide
/// between adding one [`Edit::ExtraLeft`] and one [`Edit::ExtraRight`] or
/// adding a single [`Edit::Both`]. This can have fairly surprising effect in
/// the more complicated cases.
const UNEQUAL_CHAR_DISTANCE: f64 = 1.5;

impl Distance for char {
    fn distance(left: Self, right: Self) -> f64 {
        if left == right { 0.0 } else { UNEQUAL_CHAR_DISTANCE }
    }
}

impl Distance for &str {
    /// &str::distance makes it slightly cheaper to consume both left and right
    /// at the same time than to consume left and then to consume right. The
    /// discount gets larger if the strings are very similar.
    fn distance(left: Self, right: Self) -> f64 {
        if left == right {
            return 0.0;
        }
        let edits: f64 = edit_list(left.chars(), right.chars(), Mode::FullMatch)
            .into_iter()
            .map(|edit| match edit {
                Edit::Both { distance, .. } => distance,
                _ => 1.0,
            })
            .sum();
        let left_len = left.chars().count() as f64;
        let right_len = right.chars().count() as f64;
        let maximum_edit_distance =
            UNEQUAL_CHAR_DISTANCE * right_len.min(left_len) + (right_len - left_len).abs();
        1. + edits / maximum_edit_distance
    }
}

/// 2D Table implemented with a Vec<_>.
struct Table<T> {
    size1: usize,
    table: Vec<T>,
}

impl<T> Table<T> {
    /// Create a new [`Table<T>`].
    ///
    /// The internal vector is allocated but not filled. Accessing a value
    /// before [`push`]ing it will result in a panic.
    fn new(size1: usize, size2: usize) -> Self {
        Self { size1, table: Vec::with_capacity(size1 * size2) }
    }

    /// Add [`new_element`] to [`self`].
    ///
    /// New values are added along the first dimension until it is filled. In
    /// other words, the first element is inserted at (0, 0), the second at
    /// (1, 0), and so on, until the ([`size1`] + 1)th is inserted at (0, 1).
    fn push(&mut self, new_element: T) {
        self.table.push(new_element);
    }
}

impl<T> Index<(usize, usize)> for Table<T> {
    type Output = T;

    fn index(&self, (idx1, idx2): (usize, usize)) -> &T {
        &self.table[idx1 + self.size1 * idx2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements_are;
    use crate::{matcher::Matcher, matchers::predicate, verify_that, Result};
    use indoc::indoc;
    use Mode::*;

    fn is_both<E: PartialEq + Debug>(
        l_expected: E,
        r_expected: E,
    ) -> impl Matcher<ActualT = Edit<E>> {
        predicate(move |edit: &Edit<E>| {
            matches!(edit,
                Edit::Both { left, right,.. } if left == &l_expected && right == &r_expected)
        })
    }

    fn is_extra_left<E: PartialEq + Debug>(l_expected: E) -> impl Matcher<ActualT = Edit<E>> {
        predicate(move |edit: &Edit<E>| {
            matches!(edit,
                Edit::ExtraLeft { left } if left == &l_expected)
        })
    }

    fn is_extra_right<E: PartialEq + Debug>(r_expected: E) -> impl Matcher<ActualT = Edit<E>> {
        predicate(move |edit: &Edit<E>| {
            matches!(edit,
                Edit::ExtraRight { right } if right == &r_expected)
        })
    }

    #[test]
    fn exact_match() -> Result<()> {
        let edits = edit_list("hello".chars(), "hello".chars(), FullMatch);
        verify_that!(
            edits,
            elements_are![
                is_both('h', 'h'),
                is_both('e', 'e'),
                is_both('l', 'l'),
                is_both('l', 'l'),
                is_both('o', 'o'),
            ]
        )
    }

    #[test]
    fn completely_different() -> Result<()> {
        let edits = edit_list("goodbye".chars(), "hello".chars(), FullMatch);
        verify_that!(
            edits,
            elements_are![
                is_both('g', 'h'),
                is_both('o', 'e'),
                is_extra_right('l'),
                is_extra_right('l'),
                is_both('o', 'o'),
                is_extra_left('d'),
                is_extra_left('b'),
                is_extra_left('y'),
                is_extra_left('e'),
            ]
        )
    }

    #[test]
    fn slightly_different() -> Result<()> {
        let edits = edit_list("floor".chars(), "flower".chars(), FullMatch);
        verify_that!(
            edits,
            elements_are![
                is_both('f', 'f'),
                is_both('l', 'l'),
                is_both('o', 'o'),
                is_both('o', 'w'),
                is_extra_right('e'),
                is_both('r', 'r'),
            ]
        )
    }

    #[test]
    fn lines_difference() -> Result<()> {
        let left = indoc!(
            r#"
            int: 123
            string: "something"
        "#
        );
        let right = indoc!(
            r#"
            int: 321
            string: "someone"
        "#
        );
        let edits = edit_list(left.lines(), right.lines(), FullMatch);
        verify_that!(
            edits,
            elements_are![
                is_both("int: 123", "int: 321"),
                is_both(r#"string: "something""#, r#"string: "someone""#),
            ]
        )
    }

    #[test]
    fn starts_with_imperfect_match() -> Result<()> {
        let edits = edit_list("123123".chars(), "1212".chars(), StartsWith);
        verify_that!(
            edits,
            elements_are![
                is_both('1', '1'),
                is_both('2', '2'),
                is_extra_left('3'),
                is_both('1', '1'),
                is_both('2', '2'),
                is_extra_left('3'),
            ]
        )
    }

    #[test]
    fn ends_with_imperfect_match() -> Result<()> {
        let edits = edit_list("123123".chars(), "124".chars(), EndsWith);
        verify_that!(
            edits,
            elements_are![
                is_extra_left('1'),
                is_extra_left('2'),
                is_extra_left('3'),
                is_both('1', '1'),
                is_both('2', '2'),
                is_both('3', '4'),
            ]
        )
    }

    #[test]
    fn contains_perfect_match() -> Result<()> {
        let edits = edit_list("123123".chars(), "312".chars(), Contains);
        verify_that!(
            edits,
            elements_are![
                is_extra_left('1'),
                is_extra_left('2'),
                is_both('3', '3'),
                is_both('1', '1'),
                is_both('2', '2'),
                is_extra_left('3'),
            ]
        )
    }

    #[test]
    fn contains_imperfect_match() -> Result<()> {
        let edits = edit_list("123123".chars(), "342".chars(), Contains);
        verify_that!(
            edits,
            elements_are![
                is_extra_left('1'),
                is_extra_left('2'),
                is_both('3', '3'),
                is_both('1', '4'),
                is_both('2', '2'),
                is_extra_left('3'),
            ]
        )
    }
}
