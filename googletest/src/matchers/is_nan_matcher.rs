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
use num_traits::float::Float;
use std::fmt::Debug;

/// Matches a floating point value which is NaN.
pub fn is_nan<T: Float + Debug>() -> impl Matcher<T> {
    IsNanMatcher
}

struct IsNanMatcher;

impl<T: Float + Debug> Matcher<T> for IsNanMatcher {
    fn matches(&self, actual: &T) -> MatcherResult {
        if actual.is_nan() { MatcherResult::Matches } else { MatcherResult::DoesNotMatch }
    }
}

impl Describe for IsNanMatcher {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        matcher_result.pick("is NaN", "isn't NaN").to_string()
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

    #[google_test]
    fn matches_f32_nan() -> Result<()> {
        verify_that!(f32::NAN, is_nan())
    }

    #[google_test]
    fn does_not_match_f32_number() -> Result<()> {
        verify_that!(0.0f32, not(is_nan()))
    }

    #[google_test]
    fn matches_f64_nan() -> Result<()> {
        verify_that!(f64::NAN, is_nan())
    }

    #[google_test]
    fn does_not_match_f64_number() -> Result<()> {
        verify_that!(0.0f64, not(is_nan()))
    }
}
