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
use std::fmt::Debug;
use std::marker::PhantomData;

/// Matches an `Option` containing `None`.
///
/// ```
/// verify_that!(None, none())?;   // Passes
/// verify_that!(Some("Some value"), none())?;  // Fails
/// ```
pub fn none<T: Debug>() -> impl Matcher<Option<T>> {
    NoneMatcher { phantom: Default::default() }
}

struct NoneMatcher<T> {
    phantom: PhantomData<T>,
}

impl<T: Debug> Matcher<Option<T>> for NoneMatcher<T> {
    fn matches(&self, actual: &Option<T>) -> MatcherResult {
        (actual.is_none()).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => "is none".to_string(),
            MatcherResult::DoesNotMatch => "is some(_)".to_string(),
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

    #[google_test]
    fn none_matches_option_with_none() -> Result<()> {
        let matcher = none::<i32>();

        let result = matcher.matches(&None);

        verify_that!(result, eq(MatcherResult::Matches))
    }

    #[google_test]
    fn none_does_not_match_option_with_value() -> Result<()> {
        let matcher = none();

        let result = matcher.matches(&Some(0));

        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }
}
