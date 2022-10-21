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

/// Matches anything. This matcher always succeeds.
///
/// This is useful to check if `actual` matches the specific structure (like
/// `Some(...)`)  but without caring about the internal value.
///
/// ```rust
/// let option = Some("Some value");
/// verify_that!(option, some(anything()))?;
/// ```
pub fn anything<T: Debug + ?Sized>() -> impl Matcher<T> {
    Anything {}
}

struct Anything {}

impl<T: Debug + ?Sized> Matcher<T> for Anything {
    fn matches(&self, _: &T) -> MatcherResult {
        MatcherResult::Matches
    }
}

impl Describe for Anything {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => "is anything".to_string(),
            MatcherResult::DoesNotMatch => "never matches".to_string(),
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
    use matchers::some;

    #[google_test]
    fn anything_matches_i32() -> Result<()> {
        let value = 32;
        verify_that!(value, anything())?;
        Ok(())
    }

    #[google_test]
    fn anything_matches_str() -> Result<()> {
        let value = "32";
        verify_that!(value, anything())?;
        Ok(())
    }

    #[google_test]
    fn anything_matches_option() -> Result<()> {
        let value = Some(32);
        verify_that!(value, some(anything()))?;
        Ok(())
    }
}
