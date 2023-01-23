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
use googletest::matcher::{Matcher, MatcherResult};
#[cfg(not(google3))]
use googletest::matchers::has_size::HasSize;
#[cfg(google3)]
use has_size::HasSize;
use std::fmt::Debug;

/// Matches a container whose size matches `expected`.
///
/// `T` must be a container and implement the [`HasSize`] trait so that the size
/// can be extracted.
///
/// ```rust
/// let value = vec![1,2,3];
/// verify_that!(value, size(eq(3)))?;
/// ```
pub fn size<T: HasSize + Debug, E: Matcher<usize>>(expected: E) -> impl Matcher<T> {
    SizeMatcher { expected }
}

struct SizeMatcher<E> {
    expected: E,
}

impl<T: Debug + HasSize, E: Matcher<usize>> Matcher<T> for SizeMatcher<E> {
    fn matches(&self, actual: &T) -> MatcherResult {
        self.expected.matches(&actual.size())
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
    fn size_matcher_match_slice() -> Result<()> {
        let value = &[1, 2, 3];
        verify_that!(&value[0..1], size(eq(1)))
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
