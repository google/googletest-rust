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

#[cfg(not(google3))]
use crate as googletest;
#[cfg(not(google3))]
use crate::matchers::count_elements::count_elements;
#[cfg(google3)]
use count_elements::count_elements;
use googletest::matcher::{Matcher, MatcherResult};
use std::fmt::Debug;

/// Matches a container whose size matches `expected`.
///
/// This matches against a container over which one can iterate. This includes
/// the standard Rust containers, arrays, and (when dereferenced) slices.
///
/// ```
/// let array = [1,2,3];
/// verify_that!(array, size(eq(3)))?;
/// let vec = vec![1,2,3];
/// verify_that!(vec, size(eq(3)))?;
/// let slice = value.as_slice();
/// verify_that!(*slice, size(eq(3)))?;
/// ```
///
/// The parameter `expected` can be any integer numeric matcher.
///
/// ```
/// let vec = vec![1,2,3];
/// verify_that!(vec, size(gt(1)))?;
/// ```
pub fn size<T: Debug + ?Sized, E: Matcher<usize>>(expected: E) -> impl Matcher<T>
where
    for<'b> &'b T: IntoIterator,
{
    SizeMatcher { expected }
}

struct SizeMatcher<E> {
    expected: E,
}

impl<T: Debug + ?Sized, E: Matcher<usize>> Matcher<T> for SizeMatcher<E>
where
    for<'b> &'b T: IntoIterator,
{
    fn matches(&self, actual: &T) -> MatcherResult {
        self.expected.matches(&count_elements(actual))
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!("has size, which {}", self.expected.describe(MatcherResult::Matches))
            }
            MatcherResult::DoesNotMatch => {
                format!("has size, which {}", self.expected.describe(MatcherResult::DoesNotMatch))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(google3))]
    use crate as googletest;
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{google_test, verify_that, Result};
    use matchers::eq;
    use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    };

    #[google_test]
    fn size_matcher_match_vec() -> Result<()> {
        let value = vec![1, 2, 3];
        verify_that!(value, size(eq(3)))
    }

    #[google_test]
    fn size_matcher_match_array_reference() -> Result<()> {
        let value = &[1, 2, 3];
        verify_that!(*value, size(eq(3)))
    }

    #[google_test]
    fn size_matcher_match_slice_of_array() -> Result<()> {
        let value = &[1, 2, 3];
        verify_that!(value[0..1], size(eq(1)))
    }

    #[google_test]
    fn size_matcher_match_slice_of_vec() -> Result<()> {
        let value = vec![1, 2, 3];
        let slice = value.as_slice();
        verify_that!(*slice, size(eq(3)))
    }

    #[google_test]
    fn size_matcher_match_sized_slice() -> Result<()> {
        let value = [1, 2, 3];
        verify_that!(value, size(eq(3)))
    }

    #[google_test]
    fn size_matcher_match_btreemap() -> Result<()> {
        let value = BTreeMap::from([(1, 2), (2, 3), (3, 4)]);
        verify_that!(value, size(eq(3)))
    }

    #[google_test]
    fn size_matcher_match_btreeset() -> Result<()> {
        let value = BTreeSet::from([1, 2, 3]);
        verify_that!(value, size(eq(3)))
    }

    #[google_test]
    fn size_matcher_match_binaryheap() -> Result<()> {
        let value = BinaryHeap::from([1, 2, 3]);
        verify_that!(value, size(eq(3)))
    }

    #[google_test]
    fn size_matcher_match_hashmap() -> Result<()> {
        let value = HashMap::from([(1, 2), (2, 3), (3, 4)]);
        verify_that!(value, size(eq(3)))
    }

    #[google_test]
    fn size_matcher_match_hashset() -> Result<()> {
        let value = HashSet::from([1, 2, 3]);
        verify_that!(value, size(eq(3)))
    }

    #[google_test]
    fn size_matcher_match_linkedlist() -> Result<()> {
        let value = LinkedList::from([1, 2, 3]);
        verify_that!(value, size(eq(3)))
    }

    #[google_test]
    fn size_matcher_match_vecdeque() -> Result<()> {
        let value = VecDeque::from([1, 2, 3]);
        verify_that!(value, size(eq(3)))
    }
}
