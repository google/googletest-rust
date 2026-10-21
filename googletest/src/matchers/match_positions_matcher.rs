// Copyright 2026 Google LLC
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
use regex::Regex;
use std::fmt::Debug;
use std::ops::Deref;

/// A wrapper around a slice of match position byte indices.
#[derive(Debug, Clone, Copy)]
pub struct Positions<'a>(pub &'a [usize]);

impl<'a> IntoIterator for Positions<'a> {
    type Item = usize;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

/// Matches a string whose regular expression match positions (start byte
/// indices) match the given inner container matcher.
///
/// Panics if the given `pattern` is not a syntactically valid regular
/// expression.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let text = "token at 0, token at 12";
/// verify_that!(text, regex_positions("token", elements_are![eq(0), eq(12)]))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
#[track_caller]
pub fn regex_positions<PatternT: Deref<Target = str>, InnerMatcherT>(
    pattern: PatternT,
    inner: InnerMatcherT,
) -> RegexPositionsMatcher<PatternT, InnerMatcherT> {
    let regex = Regex::new(pattern.deref()).expect("pattern should be a valid regular expression");
    RegexPositionsMatcher { regex, pattern, inner }
}

/// Matches a string whose literal substring match positions (start byte
/// indices) match the given inner container matcher.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let text = "a.b a.b a.b";
/// verify_that!(text, substring_positions("a.b", elements_are![eq(0), eq(4), eq(8)]))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn substring_positions<SubstringT: Deref<Target = str>, InnerMatcherT>(
    substring: SubstringT,
    inner: InnerMatcherT,
) -> SubstringPositionsMatcher<SubstringT, InnerMatcherT> {
    SubstringPositionsMatcher { substring, inner }
}

/// A matcher matching string regular expression match positions.
#[derive(MatcherBase)]
pub struct RegexPositionsMatcher<PatternT: Deref<Target = str>, InnerMatcherT> {
    regex: Regex,
    pattern: PatternT,
    inner: InnerMatcherT,
}

impl<PatternT, ActualT, InnerMatcherT> Matcher<ActualT>
    for RegexPositionsMatcher<PatternT, InnerMatcherT>
where
    PatternT: Deref<Target = str>,
    ActualT: AsRef<str> + Debug + Copy,
    InnerMatcherT: for<'a> Matcher<Positions<'a>>,
{
    fn matches(&self, actual: ActualT) -> MatcherResult {
        let positions: Vec<usize> =
            self.regex.find_iter(actual.as_ref()).map(|m| m.start()).collect();
        self.inner.matches(Positions(&positions[..]))
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!(
                "has match positions for regular expression {:#?}, which {}",
                self.pattern.deref(),
                self.inner.describe(MatcherResult::Match)
            )
            .into(),
            MatcherResult::NoMatch => format!(
                "has match positions for regular expression {:#?}, which {}",
                self.pattern.deref(),
                self.inner.describe(MatcherResult::NoMatch)
            )
            .into(),
        }
    }

    fn explain_match(&self, actual: ActualT) -> Description {
        let positions: Vec<usize> =
            self.regex.find_iter(actual.as_ref()).map(|m| m.start()).collect();
        format!(
            "whose match positions are {:?}, {}",
            positions,
            self.inner.explain_match(Positions(&positions[..]))
        )
        .into()
    }
}

/// A matcher matching literal substring match positions.
#[derive(MatcherBase)]
pub struct SubstringPositionsMatcher<SubstringT: Deref<Target = str>, InnerMatcherT> {
    substring: SubstringT,
    inner: InnerMatcherT,
}

impl<SubstringT, ActualT, InnerMatcherT> Matcher<ActualT>
    for SubstringPositionsMatcher<SubstringT, InnerMatcherT>
where
    SubstringT: Deref<Target = str>,
    ActualT: AsRef<str> + Debug + Copy,
    InnerMatcherT: for<'a> Matcher<Positions<'a>>,
{
    fn matches(&self, actual: ActualT) -> MatcherResult {
        let needle = self.substring.deref();
        let positions: Vec<usize> =
            actual.as_ref().match_indices(needle).map(|(idx, _)| idx).collect();
        self.inner.matches(Positions(&positions[..]))
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!(
                "has match positions for substring {:#?}, which {}",
                self.substring.deref(),
                self.inner.describe(MatcherResult::Match)
            )
            .into(),
            MatcherResult::NoMatch => format!(
                "has match positions for substring {:#?}, which {}",
                self.substring.deref(),
                self.inner.describe(MatcherResult::NoMatch)
            )
            .into(),
        }
    }

    fn explain_match(&self, actual: ActualT) -> Description {
        let needle = self.substring.deref();
        let positions: Vec<usize> =
            actual.as_ref().match_indices(needle).map(|(idx, _)| idx).collect();
        format!(
            "whose match positions are {:?}, {}",
            positions,
            self.inner.explain_match(Positions(&positions[..]))
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::Result;

    #[test]
    fn regex_positions_matches_string_slice() -> Result<()> {
        let text = "token at 0, token at 12, token at 25";
        verify_that!(text, regex_positions("token", elements_are![eq(0), eq(12), eq(25)]))
    }

    #[test]
    fn regex_positions_matches_owned_string() -> Result<()> {
        let text = String::from("foo bar foo");
        verify_that!(text, regex_positions("foo", elements_are![eq(0), eq(8)]))
    }

    #[test]
    fn substring_positions_matches_literal_substring() -> Result<()> {
        let text = "a.b a.b a.b";
        verify_that!(text, substring_positions("a.b", elements_are![eq(0), eq(4), eq(8)]))
    }

    #[test]
    fn positions_matcher_failure_explanation() -> Result<()> {
        let text = "abc abc";
        let matcher = substring_positions("abc", elements_are![eq(0)]);
        verify_that!(
            Matcher::<&str>::explain_match(&matcher, text),
            displays_as(contains_substring("whose match positions are [0, 4]"))
        )
    }

    #[test]
    #[should_panic(expected = "pattern should be a valid regular expression")]
    fn regex_positions_panics_on_invalid_regex() {
        let _ = regex_positions("(", anything());
    }
}
