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
use std::fmt::Debug;
use std::marker::PhantomData;

/// Matches an `Option` containing `None`.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(None::<()>, none())?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(Some("Some value"), none())?;  // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
pub fn none<T: Debug>() -> impl Matcher<Option<T>> {
    NoneMatcher::<T> { phantom: Default::default() }
}

struct NoneMatcher<T> {
    phantom: PhantomData<T>,
}

impl<T: Debug> Matcher<Option<T>> for NoneMatcher<T> {
    fn matches<'a>(&self, actual: &'a Option<T>) -> MatcherResult
    where
        Option<T>: 'a,
    {
        (actual.is_none()).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Match => "is none".to_string(),
            MatcherResult::NoMatch => "is some(_)".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::none;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;

    #[test]
    fn none_matches_option_with_none() -> Result<()> {
        let matcher = none::<i32>();

        let result = matcher.matches(&None);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn none_does_not_match_option_with_value() -> Result<()> {
        let matcher = none();

        let result = matcher.matches(&Some(0));

        verify_that!(result, eq(MatcherResult::NoMatch))
    }
}
