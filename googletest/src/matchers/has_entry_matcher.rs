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
use googletest::matcher::{Describe, MatchExplanation, Matcher, MatcherResult};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Matches a HashMap containing the given `key` whose value is matched by the
/// matcher `inner`.
///
/// ```rust
/// let value: HashMap<i32, i32> = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(value, has_entry(0, eq(1)))?;  // Passes
/// verify_that!(value, has_entry(1, gt(0)))?;  // Fails: value not matched
/// verify_that!(value, has_entry(2, eq(0)))?;  // Fails: key not present
/// ```
///
/// Note: One could obtain the same effect by collecting entries into a `Vec`
/// and using `contains`:
///
/// ```rust
/// let value: HashMap<i32, i32> = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(value.into_iter().collect::<Vec<_>>(), contains(eq((0, 1))))?;
/// ```
///
/// However, `has_entry` will offer somewhat better diagnostic messages in the
/// case of assertion failure. And it avoid the extra allocation hidden in the
/// code above.
pub fn has_entry<KeyT: Debug + Eq + Hash, ValueT: Debug, MatcherT: Matcher<ValueT>>(
    key: KeyT,
    inner: MatcherT,
) -> impl Matcher<HashMap<KeyT, ValueT>> {
    HasEntryMatcher { key, inner }
}

struct HasEntryMatcher<KeyT, MatcherT> {
    key: KeyT,
    inner: MatcherT,
}

impl<KeyT: Debug + Eq + Hash, ValueT: Debug, MatcherT: Matcher<ValueT>>
    Matcher<HashMap<KeyT, ValueT>> for HasEntryMatcher<KeyT, MatcherT>
{
    fn matches(&self, actual: &HashMap<KeyT, ValueT>) -> MatcherResult {
        if let Some(value) = actual.get(&self.key) {
            self.inner.matches(value)
        } else {
            MatcherResult::DoesNotMatch
        }
    }

    fn explain_match(&self, actual: &HashMap<KeyT, ValueT>) -> MatchExplanation {
        if let Some(value) = actual.get(&self.key) {
            MatchExplanation::create(format!(
                "which contains key {:?}, but is mapped to value {:?}, {}",
                self.key,
                value,
                self.inner.explain_match(value)
            ))
        } else {
            MatchExplanation::create(format!("which doesn't contain key {:?}", self.key))
        }
    }
}

impl<KeyT: Debug, MatcherT: Describe> Describe for HasEntryMatcher<KeyT, MatcherT> {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => format!(
                "contains key {:?}, which value {}",
                self.key,
                self.inner.describe(MatcherResult::Matches)
            ),
            MatcherResult::DoesNotMatch => format!(
                "doesn't contain key {:?} or contains key {:?}, which value {}",
                self.key,
                self.key,
                self.inner.describe(MatcherResult::DoesNotMatch)
            ),
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
    use matchers::{contains_substring, displays_as, eq, err, not};
    use std::collections::HashMap;

    #[google_test]
    fn has_entry_does_not_match_empty_hash_map() -> Result<()> {
        let value: HashMap<i32, i32> = HashMap::new();
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[google_test]
    fn has_entry_matches_hash_map_with_value() -> Result<()> {
        let value: HashMap<i32, i32> = HashMap::from([(0, 0)]);
        verify_that!(value, has_entry(0, eq(0)))
    }

    #[google_test]
    fn has_entry_does_not_match_hash_map_with_wrong_value() -> Result<()> {
        let value: HashMap<i32, i32> = HashMap::from([(0, 1)]);
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[google_test]
    fn has_entry_does_not_match_hash_map_with_wrong_key() -> Result<()> {
        let value: HashMap<i32, i32> = HashMap::from([(1, 0)]);
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[google_test]
    fn has_entry_shows_correct_message_when_key_is_not_present() -> Result<()> {
        let result = verify_that!(HashMap::from([(0, 0)]), has_entry(1, eq(0)));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: HashMap::from([(0, 0)])\n\
            Expected: contains key 1, which value is equal to 0\n\
            Actual: {0: 0}, which doesn't contain key 1"
            )))
        )
    }

    #[google_test]
    fn has_entry_shows_correct_message_when_key_has_non_matching_value() -> Result<()> {
        let result = verify_that!(HashMap::from([(0, 0)]), has_entry(0, eq(1)));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: HashMap::from([(0, 0)])\n\
            Expected: contains key 0, which value is equal to 1\n\
            Actual: {0: 0}, which contains key 0, but is mapped to value 0, which isn't equal to 1"
            )))
        )
    }
}
