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

use crate::matcher::{MatchExplanation, Matcher, MatcherResult};
#[cfg(google3)]
use googletest::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// Matches a HashMap containing the given `key` whose value is matched by the
/// matcher `inner`.
///
/// ```
/// # use googletest::{matchers::{eq, gt, has_entry}, verify_that, Result};
/// # use std::collections::HashMap;
/// # fn should_pass() -> Result<()> {
/// let value = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(value, has_entry(0, eq(1)))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// # let value = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(value, has_entry(1, gt(0)))?;  // Fails: value not matched
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// # let value = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(value, has_entry(2, eq(0)))?;  // Fails: key not present
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// ```
///
/// Note: One could obtain the same effect by collecting entries into a `Vec`
/// and using `contains`:
///
/// ```
/// # use googletest::{matchers::{contains, eq}, verify_that, Result};
/// # use std::collections::HashMap;
/// # fn should_pass() -> Result<()> {
/// let value = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(value.into_iter().collect::<Vec<_>>(), contains(eq((0, 1))))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// However, `has_entry` will offer somewhat better diagnostic messages in the
/// case of assertion failure. And it avoid the extra allocation hidden in the
/// code above.
pub fn has_entry<KeyT: Debug + Eq + Hash, ValueT: Debug, MatcherT: Matcher>(
    key: KeyT,
    inner: MatcherT,
) -> impl Matcher {
    HasEntryMatcher { key, inner, phantom: Default::default() }
}

struct HasEntryMatcher<KeyT, ValueT, MatcherT> {
    key: KeyT,
    inner: MatcherT,
    phantom: PhantomData<ValueT>,
}

impl<KeyT: Debug + Eq + Hash, ValueT: Debug, MatcherT: Matcher> Matcher
    for HasEntryMatcher<KeyT, ValueT, MatcherT>
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
                "which contains key {:?}, but is mapped to value {:#?}, {}",
                self.key,
                value,
                self.inner.explain_match(value)
            ))
        } else {
            MatchExplanation::create(format!("which doesn't contain key {:?}", self.key))
        }
    }

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
    use super::has_entry;
    #[cfg(not(google3))]
    use crate::matchers;
    use crate::{verify_that, Result};
    use matchers::{contains_substring, displays_as, eq, err, not};
    use std::collections::HashMap;

    #[test]
    fn has_entry_does_not_match_empty_hash_map() -> Result<()> {
        let value: HashMap<i32, i32> = HashMap::new();
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[test]
    fn has_entry_matches_hash_map_with_value() -> Result<()> {
        let value: HashMap<i32, i32> = HashMap::from([(0, 0)]);
        verify_that!(value, has_entry(0, eq(0)))
    }

    #[test]
    fn has_entry_does_not_match_hash_map_with_wrong_value() -> Result<()> {
        let value: HashMap<i32, i32> = HashMap::from([(0, 1)]);
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[test]
    fn has_entry_does_not_match_hash_map_with_wrong_key() -> Result<()> {
        let value: HashMap<i32, i32> = HashMap::from([(1, 0)]);
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[test]
    fn has_entry_shows_correct_message_when_key_is_not_present() -> Result<()> {
        let result = verify_that!(HashMap::from([(0, 0)]), has_entry(1, eq(0)));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "\
Value of: HashMap::from([(0, 0)])
Expected: contains key 1, which value is equal to 0
Actual: {
    0: 0,
}, which doesn't contain key 1
"
            )))
        )
    }

    #[test]
    fn has_entry_shows_correct_message_when_key_has_non_matching_value() -> Result<()> {
        let result = verify_that!(HashMap::from([(0, 0)]), has_entry(0, eq(1)));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "\
Value of: HashMap::from([(0, 0)])
Expected: contains key 0, which value is equal to 1
Actual: {
    0: 0,
}, which contains key 0, but is mapped to value 0, which isn't equal to 1
"
            )))
        )
    }
}
