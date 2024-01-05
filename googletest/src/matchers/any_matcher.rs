// Copyright 2023 Google LLC
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

// There are no visible documentation elements in this module; the declarative
// macro is documented in the matchers module.
#![doc(hidden)]

/// Matches a value which at least one of the given matchers match.
///
/// Each argument is a [`Matcher`][crate::matcher::Matcher] which matches
/// against the actual value.
///
/// For example:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!("A string", any!(starts_with("A"), ends_with("string")))?; // Passes
/// verify_that!("A string", any!(starts_with("A"), starts_with("string")))?; // Passes
/// verify_that!("A string", any!(ends_with("A"), ends_with("string")))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!("A string", any!(starts_with("An"), ends_with("not a string")))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// Using this macro is equivalent to using the
/// [`or`][crate::matcher::Matcher::or] method:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(10, gt(9).or(lt(8)))?; // Also passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Assertion failure messages are not guaranteed to be identical, however.
#[macro_export]
#[doc(hidden)]
macro_rules! __any {
    ($($matcher:expr),* $(,)?) => {{
        use $crate::matchers::__internal_unstable_do_not_depend_on_these::AnyMatcher;
        AnyMatcher::new([$(Box::new($matcher)),*])
    }}
}

/// Functionality needed by the [`any`] macro.
///
/// For internal use only. API stablility is not guaranteed!
#[doc(hidden)]
pub mod internal {
    use crate::description::Description;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::matchers::anything;
    use std::fmt::Debug;

    /// A matcher which matches an input value matched by all matchers in the
    /// array `components`.
    ///
    /// For internal use only. API stablility is not guaranteed!
    #[doc(hidden)]
    pub struct AnyMatcher<'a, T: Debug + ?Sized, const N: usize> {
        components: [Box<dyn Matcher<ActualT = T> + 'a>; N],
    }

    impl<'a, T: Debug + ?Sized, const N: usize> AnyMatcher<'a, T, N> {
        /// Constructs an [`AnyMatcher`] with the given component matchers.
        ///
        /// Intended for use only by the [`all`] macro.
        pub fn new(components: [Box<dyn Matcher<ActualT = T> + 'a>; N]) -> Self {
            Self { components }
        }
    }

    impl<'a, T: Debug + ?Sized, const N: usize> Matcher for AnyMatcher<'a, T, N> {
        type ActualT = T;

        fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
            MatcherResult::from(self.components.iter().any(|c| c.matches(actual).is_match()))
        }

        fn explain_match(&self, actual: &Self::ActualT) -> Description {
            match N {
                0 => format!("which {}", anything::<T>().describe(MatcherResult::NoMatch)).into(),
                1 => self.components[0].explain_match(actual),
                _ => {
                    let failures = self
                        .components
                        .iter()
                        .filter(|component| component.matches(actual).is_no_match())
                        .collect::<Vec<_>>();

                    if failures.len() == 1 {
                        failures[0].explain_match(actual)
                    } else {
                        Description::new()
                            .collect(
                                failures
                                    .into_iter()
                                    .map(|component| component.explain_match(actual)),
                            )
                            .bullet_list()
                    }
                }
            }
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            match N {
                0 => anything::<T>().describe(matcher_result),
                1 => self.components[0].describe(matcher_result),
                _ => {
                    let properties = self
                        .components
                        .iter()
                        .map(|m| m.describe(matcher_result))
                        .collect::<Description>()
                        .bullet_list()
                        .indent();
                    format!(
                        "{}:\n{properties}",
                        if matcher_result.into() {
                            "has at least one of the following properties"
                        } else {
                            "has none of the following properties"
                        }
                    )
                    .into()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::internal;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn description_shows_more_than_one_matcher() -> Result<()> {
        let first_matcher = starts_with("A");
        let second_matcher = ends_with("string");
        let matcher: internal::AnyMatcher<String, 2> = any!(first_matcher, second_matcher);

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq(indoc!(
                "
                has at least one of the following properties:
                  * starts with prefix \"A\"
                  * ends with suffix \"string\""
            )))
        )
    }

    #[test]
    fn description_shows_one_matcher_directly() -> Result<()> {
        let first_matcher = starts_with("A");
        let matcher: internal::AnyMatcher<String, 1> = any!(first_matcher);

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq("starts with prefix \"A\""))
        )
    }

    #[test]
    fn mismatch_description_shows_which_matcher_failed_if_more_than_one_constituent() -> Result<()>
    {
        let first_matcher = starts_with("Another");
        let second_matcher = ends_with("string");
        let matcher: internal::AnyMatcher<str, 2> = any!(first_matcher, second_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }

    #[test]
    fn mismatch_description_is_simple_when_only_one_constituent() -> Result<()> {
        let first_matcher = starts_with("Another");
        let matcher: internal::AnyMatcher<str, 1> = any!(first_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }
}
