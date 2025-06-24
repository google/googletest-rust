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

use crate::{
    description::Description,
    matcher::{Matcher, MatcherBase, MatcherResult},
};
use std::fmt::Debug;

/// Matches an empty container.
///
/// `T` can be any container that implements `IntoIterator`. For instance, `T`
/// can be the reference of a common container like `&Vec` and
/// [`&HashSet`][std::collections::HashSet].
///
/// ```
/// # use googletest::prelude::*;
/// # use std::collections::HashSet;
/// # fn should_pass() -> Result<()> {
/// let value: Vec<i32> = vec![];
/// verify_that!(value, is_empty())?;
/// let value: HashSet<i32> = HashSet::new();
/// verify_that!(value, is_empty())?;
/// let value: &[u32] = &[];
/// verify_that!(value, is_empty())?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn is_empty() -> EmptyMatcher {
    EmptyMatcher
}

/// This is deprecated. Use `is_empty()` instead.
#[deprecated(since = "0.14.1", note = "Use `is_empty()` instead.")]
pub fn empty() -> EmptyMatcher {
    EmptyMatcher
}

#[derive(MatcherBase)]
pub struct EmptyMatcher;

impl<T: Debug + Copy> Matcher<T> for EmptyMatcher
where
    T: IntoIterator,
{
    fn matches(&self, actual: T) -> MatcherResult {
        actual.into_iter().next().is_none().into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        if matcher_result.into() { "is empty" } else { "isn't empty" }.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::Result;
    use std::collections::HashSet;

    #[test]
    fn empty_matcher_match_empty_vec() -> Result<()> {
        let value: Vec<i32> = vec![];
        verify_that!(value, is_empty())
    }

    #[test]
    fn empty_matcher_does_not_match_empty_vec() -> Result<()> {
        let value = vec![1, 2, 3];
        verify_that!(value, not(is_empty()))
    }

    #[test]
    fn empty_matcher_matches_empty_slice() -> Result<()> {
        let value: &[i32] = &[];
        verify_that!(value, is_empty())
    }

    #[test]
    fn empty_matcher_matches_empty_hash_set() -> Result<()> {
        let value: HashSet<i32> = HashSet::new();
        verify_that!(value, is_empty())
    }
}
