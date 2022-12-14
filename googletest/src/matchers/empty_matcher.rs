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
use googletest::matcher::{Describe, Matcher, MatcherResult};
use std::fmt::Debug;

/// Matches an empty container.
///
/// `T` can be any container such that `&T` implements `IntoIterator`.
///
/// ```rust
/// let value: Vec<i32> = vec![];
/// verify_that!(value, empty())?;
/// let value: HashSet<i32> = HashSet::new();
/// verify_that!(value, empty())?;
/// ```
///
/// One can also check whether a slice is empty by dereferencing it:
///
/// ```rust
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
        if actual.into_iter().next().is_none() {
            MatcherResult::Matches
        } else {
            MatcherResult::DoesNotMatch
        }
    }
}

impl Describe for EmptyMatcher {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        matcher_result.pick("is empty", "isn't empty").to_string()
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
    use matchers::not;
    use std::collections::HashSet;

    #[google_test]
    fn empty_matcher_match_empty_vec() -> Result<()> {
        let value: Vec<i32> = vec![];
        verify_that!(value, empty())
    }

    #[google_test]
    fn empty_matcher_does_not_match_empty_vec() -> Result<()> {
        let value = vec![1, 2, 3];
        verify_that!(value, not(empty()))
    }

    #[google_test]
    fn empty_matcher_matches_empty_slice() -> Result<()> {
        let value: &[i32] = &[];
        verify_that!(*value, empty())
    }

    #[google_test]
    fn empty_matcher_matches_empty_hash_set() -> Result<()> {
        let value: HashSet<i32> = HashSet::new();
        verify_that!(value, empty())
    }
}
