// Copyright 2024 Google LLC
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

/// Match a bool value or bool reference.
#[derive(MatcherBase)]
pub struct BoolMatcher {
    expected: bool,
}

impl BoolMatcher {
    fn matches(&self, actual: bool) -> MatcherResult {
        (actual == self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match (matcher_result, self.expected) {
            (MatcherResult::Match, true) | (MatcherResult::NoMatch, false) => "is true".into(),
            (MatcherResult::Match, false) | (MatcherResult::NoMatch, true) => "is false".into(),
        }
    }
}

impl Matcher<bool> for BoolMatcher {
    fn matches(&self, actual: bool) -> MatcherResult {
        self.matches(actual)
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.describe(matcher_result)
    }
}

impl<'a> Matcher<&'a bool> for BoolMatcher {
    fn matches(&self, actual: &'a bool) -> MatcherResult {
        self.matches(*actual)
    }
    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.describe(matcher_result)
    }
}

pub fn is_true() -> BoolMatcher {
    BoolMatcher { expected: true }
}

pub fn is_false() -> BoolMatcher {
    BoolMatcher { expected: false }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn match_value() -> Result<()> {
        verify_that!(true, is_true())?;
        verify_that!(true, not(is_false()))?;
        verify_that!(false, is_false())?;
        verify_that!(false, not(is_true()))
    }

    #[test]
    fn match_ref() -> Result<()> {
        let t = true;
        let f = false;

        verify_that!(&t, is_true())?;
        verify_that!(&t, not(is_false()))?;
        verify_that!(&f, is_false())?;
        verify_that!(&f, not(is_true()))
    }
}
