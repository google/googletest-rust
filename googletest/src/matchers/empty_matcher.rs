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

use crate::matcher::{Matcher, MatcherResult};
#[cfg(google3)]
use googletest::*;
use std::fmt::Debug;

/// Matches an empty container.
///
/// `T` can be any container such that `&T` implements `IntoIterator`.
///
/// ```ignore
/// let value: Vec<i32> = vec![];
/// verify_that!(value, empty())?;
/// let value: HashSet<i32> = HashSet::new();
/// verify_that!(value, empty())?;
/// ```
///
/// One can also check whether a slice is empty by dereferencing it:
///
/// ```ignore
/// let value = &[];
/// verify_that!(*value, empty())?;
/// ```

pub fn empty<T: Debug + ?Sized>() -> impl Matcher<T>
where
    for<'a> &'a T: IntoIterator,
{
    EmptyMatcher {}
}

struct EmptyMatcher {}

impl<T: Debug + ?Sized> Matcher<T> for EmptyMatcher
where
    for<'a> &'a T: IntoIterator,
{
    fn matches(&self, actual: &T) -> MatcherResult {
        actual.into_iter().next().is_none().into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        if matcher_result.into() { "is empty" } else { "isn't empty" }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::empty;
    #[cfg(not(google3))]
    use crate::matchers;
    use crate::{verify_that, Result};
    use matchers::not;
    use std::collections::HashSet;

    #[test]
    fn empty_matcher_match_empty_vec() -> Result<()> {
        let value: Vec<i32> = vec![];
        verify_that!(value, empty())
    }

    #[test]
    fn empty_matcher_does_not_match_empty_vec() -> Result<()> {
        let value = vec![1, 2, 3];
        verify_that!(value, not(empty()))
    }

    #[test]
    fn empty_matcher_matches_empty_slice() -> Result<()> {
        let value: &[i32] = &[];
        verify_that!(*value, empty())
    }

    #[test]
    fn empty_matcher_matches_empty_hash_set() -> Result<()> {
        let value: HashSet<i32> = HashSet::new();
        verify_that!(value, empty())
    }
}
