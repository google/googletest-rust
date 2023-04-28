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
pub trait Matcher {
    /// The type against which this matcher matches.
    type ActualT: Debug + ?Sized;

    /// Returns whether the condition matches the datum `actual`.
    ///
    /// The trait implementation defines what it means to "match". Often the
    /// matching condition is based on data stored in the matcher. For example,
    /// `eq` matches when its stored expected value is equal (in the sense of
    /// the `==` operator) to the value `actual`.
    fn matches(&self, actual: &Self::ActualT) -> MatcherResult;

    /// Returns a description of `self` or a negative description if
    /// `matcher_result` is `DoesNotMatch`.
    ///
    /// The function should print a verb phrase that describes the property a
    /// value matching, respectively not matching, this matcher should have.
    /// The subject of the verb phrase is the value being matched.
    ///
    /// The output appears next to `Expected` in an assertion failure message.
    /// For example:
    ///
    /// ```text
    /// Value of: ...
    /// Expected: is equal to 7
    ///           ^^^^^^^^^^^^^
    /// Actual: ...
    /// ```
    ///
    /// When the matcher contains one or more inner matchers, the implementation
    /// should invoke [`Self::describe`] on the inner matchers to complete the
    /// description. It should place the inner description at a point where a
    /// verb phrase would fit. For example, the matcher
    /// [`some`][crate::matchers::some] implements `describe` as follows:
    ///
    /// ```ignore
    /// fn describe(&self, matcher_result: MatcherResult) -> String {
    ///     match matcher_result {
    ///         MatcherResult::Matches => {
    ///             format!("has a value which {}", self.inner.describe(MatcherResult::Matches))
    ///               //  Inner matcher invocation: ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ///         }
    ///         MatcherResult::DoesNotMatch => {...} // Similar to the above
    ///     }
    /// }
    /// ```
    ///
    /// The output expectation differs from that of
    /// [`explain_match`][Self::explain_match] in that it is a verb phrase
    /// (beginning with a verb like "is") rather than a relative clause
    /// (beginning with "which" or "whose"). This difference is because the
    /// output of `explain_match` is always used adjectivally to describe the
    /// actual value, while `describe` is used in contexts where a relative
    /// clause would not make sense.
    fn describe(&self, matcher_result: MatcherResult) -> String;

    /// Prepares a [`MatchExplanation`] describing how the expected value
    /// encoded in this instance matches or does not match the given value
    /// `actual`.
    ///
    /// This should be in the form of a relative clause, i.e. something starting
    /// with a relative pronoun such as "which" or "whose". It will appear next
    /// to the actual value in an assertion failure. For example:
    ///
    /// ```text
    /// Value of: ...
    /// Expected: ...
    /// Actual: ["Something"], which does not contain "Something else"
    ///                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// ```
    ///
    /// The default implementation relies on [`describe`][Self::describe]. Thus
    /// it does not make any use of the actual value itself, but rather only
    /// whether the value is matched.
    ///
    /// Override the default implementation to provide additional context on why
    /// a particular value matched or did not match. For example, the
    /// [`container_eq`][crate::matchers::container_eq] matcher displays
    /// information on which elements of the actual value were not present in
    /// the expected value and vice versa.
    ///
    /// This implementation should be overridden in any matcher which contains
    /// one or more inner matchers. The implementation should invoke
    /// `explain_match` on the inner matchers, so that the generated match
    /// explanation also reflects their implementation. Without this, the match
    /// explanation of the inner matchers will not be able to make use of the
    /// actual value at all.
    ///
    /// For example, the `explain_match` implementation of the matcher
    /// [`points_to`][crate::matchers::points_to] defers immediately to the
    /// inner matcher and appears as follows:
    ///
    /// ```ignore
    /// fn explain_match(&self, actual: &Self::ActualT) -> MatchExplanation {
    ///     self.expected.explain_match(actual.deref())
    /// }
    /// ```
    ///
    /// The matcher can also provide some additional context before deferring to
    /// an inner matcher. In that case it should invoke `explain_match` on the
    /// inner matcher at a point where a relative clause would fit. For example:
    ///
    /// ```ignore
    /// fn explain_match(&self, actual: &Self::ActualT) -> MatchExplanation {
    ///     MatchExplanation::create(
    ///         format!("which points to a value {}", self.expected.explain_match(actual.deref()))
    ///             //   ^^^^^^^^^^^^^^^^^^^^ Expands to "points to a value which ..."
    ///     )
    /// }
    /// ```
    fn explain_match(&self, actual: &Self::ActualT) -> MatchExplanation {
        MatchExplanation::create(format!("which {}", self.describe(self.matches(actual))))
    }
}

/// Constructs a [`TestAssertionFailure`] reporting that the given `matcher`
/// does not match the value `actual`.
///
/// The parameter `actual_expr` contains the expression which was evaluated to
/// obtain `actual`.
pub(crate) fn create_assertion_failure<T: Debug + ?Sized>(
    matcher: &impl Matcher<ActualT = T>,
    actual: &T,
    actual_expr: &'static str,
    source_location: SourceLocation,
) -> TestAssertionFailure {
    TestAssertionFailure::create(format!(
        "Value of: {}\n\
             Expected: {}\n\
             Actual: {:#?}, {}\n\
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

impl From<bool> for MatcherResult {
    fn from(b: bool) -> Self {
        if b { MatcherResult::Matches } else { MatcherResult::DoesNotMatch }
    }
}

impl From<MatcherResult> for bool {
    fn from(matcher_result: MatcherResult) -> Self {
        match matcher_result {
            MatcherResult::Matches => true,
            MatcherResult::DoesNotMatch => false,
        }
    }
}

impl MatcherResult {
    /// Returns `true` if `self` is [`MatcherResult::Matches`], otherwise
    /// `false`.
    ///
    /// This delegates to `Into<bool>` but coerce the return type to `bool`
    /// instead only calling `into()`. This is useful in `if
    /// !matcher_result.into()`, which requires a constraint on the result
    /// type of `into()` to compile.
    pub fn into_bool(self) -> bool {
        self.into()
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
