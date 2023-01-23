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

use crate::internal::source_location::SourceLocation;
use crate::internal::test_outcome::TestAssertionFailure;
use std::fmt::{Debug, Display, Formatter, Result};

/// An interface for checking an arbitrary condition on a datum.
pub trait Matcher<T: Debug + ?Sized> {
    /// Returns whether the condition matches the datum `actual`.
    ///
    /// The trait implementation defines what it means to "match". Often the
    /// matching condition is based on data stored in the matcher. For example,
    /// `eq` matches when its stored expected value is equal (in the sense of
    /// the `==` operator) to the value `actual`.
    fn matches(&self, actual: &T) -> MatcherResult;

    /// Prepares a [`MatchExplanation`] describing how the expected value
    /// encoded in this instance matches or does not match the given value
    /// `actual`.
    ///
    /// The default implementation relies on [`Describe::describe`].
    fn explain_match(&self, actual: &T) -> MatchExplanation {
        MatchExplanation::create(format!("which {}", self.describe(self.matches(actual))))
    }

    /// Returns a description of `self` or a negative description if
    /// `matcher_result` is `DoesNotMatch`.
    ///
    /// The function should print a verb phrase that describes the property a
    /// value matching, respectively not matching, this matcher should have.
    /// The subject of the verb phrase is the value being matched.
    ///
    /// For example, eq(7).describe(MatcherResult::Matches) prints "is equal to
    /// 7".
    fn describe(&self, matcher_result: MatcherResult) -> String;
}

/// Constructs a [`TestAssertionFailure`] reporting that the given `matcher`
/// does not match the value `actual`.
///
/// The parameter `actual_expr` contains the expression which was evaluated to
/// obtain `actual`.
pub(crate) fn create_assertion_failure<T: Debug + ?Sized>(
    matcher: &impl Matcher<T>,
    actual: &T,
    actual_expr: &'static str,
    source_location: SourceLocation,
) -> TestAssertionFailure {
    TestAssertionFailure::create(format!(
        "Value of: {}\n\
             Expected: {}\n\
             Actual: {:?}, {}\n\
             {}",
        actual_expr,
        matcher.describe(MatcherResult::Matches),
        actual,
        matcher.explain_match(actual),
        source_location,
    ))
}

/// The result of applying a [`Matcher`] on an actual value.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MatcherResult {
    /// The actual value matches according to the [`Matcher`] definition.
    Matches,
    /// The actual value does not match according to the [`Matcher`] definition.
    DoesNotMatch,
}

impl MatcherResult {
    /// Returns `matches_value` if the `self` is `Matches`, otherwise
    /// `does_not_match_value`.
    ///
    /// This is a helper method to simplify `Describe` implementation.
    pub fn pick<T>(&self, matches_value: T, does_not_match_value: T) -> T {
        match self {
            MatcherResult::Matches => matches_value,
            MatcherResult::DoesNotMatch => does_not_match_value,
        }
    }
}

/// Human-readable explanation of why a value was matched or not matched by a
/// matcher.
///
/// This is formatted into an assertion failure message.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MatchExplanation(String);

impl MatchExplanation {
    pub fn create(explanation: String) -> Self {
        Self(explanation)
    }
}

impl Display for MatchExplanation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}
