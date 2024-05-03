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
use crate::matcher::{Matcher, MatcherBase, MatcherResult};
use std::fmt::Debug;

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
pub fn none() -> NoneMatcher {
    NoneMatcher
}

#[derive(MatcherBase)]
pub struct NoneMatcher;

impl<T: Debug + Copy> Matcher<Option<T>> for NoneMatcher {
    fn matches(&self, actual: Option<T>) -> MatcherResult {
        actual.is_none().into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => "is none".into(),
            MatcherResult::NoMatch => "is some(_)".into(),
        }
    }
}

impl<'a, T: Debug> Matcher<&'a Option<T>> for NoneMatcher {
    fn matches(&self, actual: &'a Option<T>) -> MatcherResult {
        actual.is_none().into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => "is none".into(),
            MatcherResult::NoMatch => "is some(_)".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::MatcherResult;
    use crate::prelude::*;

    #[test]
    fn none_matches_option_with_none() -> Result<()> {
        let matcher = none();

        let result = matcher.matches(None::<i32>);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn none_does_not_match_option_with_value() -> Result<()> {
        let matcher = none();

        let result = matcher.matches(Some(0));

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn none_matches_option_by_ref() -> Result<()> {
        verify_that!(None::<String>, none())
    }
    #[test]
    fn none_does_not_match_option_with_value_by_ref() -> Result<()> {
        verify_that!(Some("123".to_string()), not(none()))
    }

    #[test]
    fn none_describe_match_option_by_ref() -> Result<()> {
        verify_that!(
            Matcher::<&Option<String>>::describe(&none(), MatcherResult::Match),
            displays_as(eq("is none"))
        )
    }
    #[test]
    fn none_describe_no_match_option_by_ref() -> Result<()> {
        verify_that!(
            Matcher::<&Option<String>>::describe(&none(), MatcherResult::NoMatch),
            displays_as(eq("is some(_)"))
        )
    }

    #[test]
    fn none_describe_match_option() -> Result<()> {
        verify_that!(
            Matcher::<Option<i32>>::describe(&none(), MatcherResult::Match),
            displays_as(eq("is none"))
        )
    }
    #[test]
    fn none_describe_no_match_option() -> Result<()> {
        verify_that!(
            Matcher::<Option<i32>>::describe(&none(), MatcherResult::NoMatch),
            displays_as(eq("is some(_)"))
        )
    }
}
