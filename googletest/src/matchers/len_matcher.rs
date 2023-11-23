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

use crate::description::Description;
use crate::matcher::{Matcher, MatcherResult};
use crate::matcher_support::count_elements::count_elements;
use std::{fmt::Debug, marker::PhantomData};

/// Matches a container whose number of elements matches `expected`.
///
/// This matches against a container over which one can iterate. This includes
/// the standard Rust containers, arrays, and (when dereferenced) slices. More
/// precisely, a shared borrow of the actual type must implement
/// [`IntoIterator`].
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let array = [1,2,3];
/// verify_that!(array, len(eq(3)))?;
/// let vec = vec![1,2,3];
/// verify_that!(vec, len(eq(3)))?;
/// let slice = vec.as_slice();
/// verify_that!(*slice, len(eq(3)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// The parameter `expected` can be any integer numeric matcher.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let vec = vec![1,2,3];
/// verify_that!(vec, len(gt(1)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn len<T: Debug + ?Sized, E: Matcher<ActualT = usize>>(expected: E) -> impl Matcher<ActualT = T>
where
    for<'a> &'a T: IntoIterator,
{
    LenMatcher { expected, phantom: Default::default() }
}

struct LenMatcher<T: ?Sized, E> {
    expected: E,
    phantom: PhantomData<T>,
}

impl<T: Debug + ?Sized, E: Matcher<ActualT = usize>> Matcher for LenMatcher<T, E>
where
    for<'a> &'a T: IntoIterator,
{
    type ActualT = T;

    fn matches(&self, actual: &T) -> MatcherResult {
        self.expected.matches(&count_elements(actual))
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("has length, which {}", self.expected.describe(MatcherResult::Match)).into()
            }
            MatcherResult::NoMatch => {
                format!("has length, which {}", self.expected.describe(MatcherResult::NoMatch))
                    .into()
            }
        }
    }

    fn explain_match(&self, actual: &T) -> Description {
        let actual_size = count_elements(actual);
        format!("which has length {}, {}", actual_size, self.expected.explain_match(&actual_size))
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::len;
    use crate::description::Description;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;
    use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    };
    use std::fmt::Debug;
    use std::marker::PhantomData;

    #[test]
    fn len_matcher_match_vec() -> Result<()> {
        let value = vec![1, 2, 3];
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_match_array_reference() -> Result<()> {
        let value = &[1, 2, 3];
        verify_that!(*value, len(eq(3)))
    }

    #[test]
    fn len_matcher_match_slice_of_array() -> Result<()> {
        let value = &[1, 2, 3];
        verify_that!(value[0..1], len(eq(1)))
    }

    #[test]
    fn len_matcher_match_slice_of_vec() -> Result<()> {
        let value = vec![1, 2, 3];
        let slice = value.as_slice();
        verify_that!(*slice, len(eq(3)))
    }

    #[test]
    fn len_matcher_match_sized_slice() -> Result<()> {
        let value = [1, 2, 3];
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_match_btreemap() -> Result<()> {
        let value = BTreeMap::from([(1, 2), (2, 3), (3, 4)]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_match_btreeset() -> Result<()> {
        let value = BTreeSet::from([1, 2, 3]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_match_binaryheap() -> Result<()> {
        let value = BinaryHeap::from([1, 2, 3]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_match_hashmap() -> Result<()> {
        let value = HashMap::from([(1, 2), (2, 3), (3, 4)]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_match_hashset() -> Result<()> {
        let value = HashSet::from([1, 2, 3]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_match_linkedlist() -> Result<()> {
        let value = LinkedList::from([1, 2, 3]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_match_vecdeque() -> Result<()> {
        let value = VecDeque::from([1, 2, 3]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_explain_match() -> Result<()> {
        struct TestMatcher<T>(PhantomData<T>);
        impl<T: Debug> Matcher for TestMatcher<T> {
            type ActualT = T;

            fn matches(&self, _: &T) -> MatcherResult {
                false.into()
            }

            fn describe(&self, _: MatcherResult) -> Description {
                "called described".into()
            }

            fn explain_match(&self, _: &T) -> Description {
                "called explain_match".into()
            }
        }
        verify_that!(
            len(TestMatcher(Default::default())).explain_match(&[1, 2, 3]),
            displays_as(eq("which has length 3, called explain_match"))
        )
    }

    #[test]
    fn len_matcher_error_message() -> Result<()> {
        let result = verify_that!(vec![1, 2, 3, 4], len(eq(3)));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: vec![1, 2, 3, 4]
                Expected: has length, which is equal to 3
                Actual: [1, 2, 3, 4],
                  which has length 4, which isn't equal to 3"
            ))))
        )
    }
}
