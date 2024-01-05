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

//! The components required to implement matchers.

use crate::description::Description;
use crate::internal::source_location::SourceLocation;
use crate::internal::test_outcome::TestAssertionFailure;
use crate::matchers::__internal_unstable_do_not_depend_on_these::ConjunctionMatcher;
use crate::matchers::__internal_unstable_do_not_depend_on_these::DisjunctionMatcher;
use std::fmt::Debug;

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
    /// fn describe(&self, matcher_result: MatcherResult) -> Description {
    ///     match matcher_result {
    ///         MatcherResult::Matches => {
    ///             Description::new()
    ///                 .text("has a value which")
    ///                 .nested(self.inner.describe(MatcherResult::Matches))
    ///       // Inner matcher: ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
    fn describe(&self, matcher_result: MatcherResult) -> Description;

    /// Prepares a [`String`] describing how the expected value
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
    /// fn explain_match(&self, actual: &Self::ActualT) -> Description {
    ///     self.expected.explain_match(actual.deref())
    /// }
    /// ```
    ///
    /// The matcher can also provide some additional context before deferring to
    /// an inner matcher. In that case it should invoke `explain_match` on the
    /// inner matcher at a point where a relative clause would fit. For example:
    ///
    /// ```ignore
    /// fn explain_match(&self, actual: &Self::ActualT) -> Description {
    ///     Description::new()
    ///         .text("which points to a value")
    ///         .nested(self.expected.explain_match(actual.deref()))
    /// }
    /// ```
    fn explain_match(&self, actual: &Self::ActualT) -> Description {
        format!("which {}", self.describe(self.matches(actual))).into()
    }

    /// Constructs a matcher that matches both `self` and `right`.
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_pass() -> Result<()> {
    /// verify_that!("A string", starts_with("A").and(ends_with("string")))?; // Passes
    /// #     Ok(())
    /// # }
    /// # fn should_fail_1() -> Result<()> {
    /// verify_that!("A string", starts_with("Another").and(ends_with("string")))?; // Fails
    /// #     Ok(())
    /// # }
    /// # fn should_fail_2() -> Result<()> {
    /// verify_that!("A string", starts_with("A").and(ends_with("non-string")))?; // Fails
    /// #     Ok(())
    /// # }
    /// # should_pass().unwrap();
    /// # should_fail_1().unwrap_err();
    /// # should_fail_2().unwrap_err();
    /// ```
    // TODO(b/264518763): Replace the return type with impl Matcher and reduce
    // visibility of ConjunctionMatcher once impl in return position in trait
    // methods is stable.
    fn and<Right: Matcher<ActualT = Self::ActualT>>(
        self,
        right: Right,
    ) -> ConjunctionMatcher<Self, Right>
    where
        Self: Sized,
    {
        ConjunctionMatcher::new(self, right)
    }

    /// Constructs a matcher that matches when at least one of `self` or `right`
    /// matches the input.
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # fn should_pass() -> Result<()> {
    /// verify_that!(10, eq(2).or(ge(5)))?;  // Passes
    /// verify_that!(10, eq(2).or(eq(5)).or(ge(9)))?;  // Passes
    /// #     Ok(())
    /// # }
    /// # fn should_fail() -> Result<()> {
    /// verify_that!(10, eq(2).or(ge(15)))?; // Fails
    /// #     Ok(())
    /// # }
    /// # should_pass().unwrap();
    /// # should_fail().unwrap_err();
    /// ```
    // TODO(b/264518763): Replace the return type with impl Matcher and reduce
    // visibility of DisjunctionMatcher once impl in return position in trait
    // methods is stable.
    fn or<Right: Matcher<ActualT = Self::ActualT>>(
        self,
        right: Right,
    ) -> DisjunctionMatcher<Self, Right>
    where
        Self: Sized,
    {
        DisjunctionMatcher::new(self, right)
    }
}

/// Any actual value whose debug length is greater than this value will be
/// pretty-printed. Otherwise, it will have normal debug output formatting.
const PRETTY_PRINT_LENGTH_THRESHOLD: usize = 60;

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
    let actual_formatted = format!("{actual:?}");
    let actual_formatted = if actual_formatted.len() > PRETTY_PRINT_LENGTH_THRESHOLD {
        format!("{actual:#?}")
    } else {
        actual_formatted
    };
    TestAssertionFailure::create(format!(
        "\
Value of: {actual_expr}
Expected: {}
Actual: {actual_formatted},
{}
{source_location}",
        matcher.describe(MatcherResult::Match),
        matcher.explain_match(actual).indent(),
    ))
}

/// The result of applying a [`Matcher`] on an actual value.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MatcherResult {
    /// The actual value matches according to the [`Matcher`] definition.
    Match,
    /// The actual value does not match according to the [`Matcher`] definition.
    NoMatch,
}

impl From<bool> for MatcherResult {
    fn from(b: bool) -> Self {
        if b { MatcherResult::Match } else { MatcherResult::NoMatch }
    }
}

impl From<MatcherResult> for bool {
    fn from(matcher_result: MatcherResult) -> Self {
        matcher_result.is_match()
    }
}

impl MatcherResult {
    /// Returns `true` if `self` is [`MatcherResult::Match`], otherwise
    /// `false`.
    pub fn is_match(self) -> bool {
        matches!(self, MatcherResult::Match)
    }

    /// Returns `true` if `self` is [`MatcherResult::NoMatch`], otherwise
    /// `false`.
    pub fn is_no_match(self) -> bool {
        matches!(self, MatcherResult::NoMatch)
    }
}
