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

// There are no visible documentation elements in this module; the declarative
// macro is documented at the top level.
#![doc(hidden)]

/// Matches a value which all of the given matchers match.
///
/// For example:
///
/// ```rust
/// verify_that!("A string", all!(starts_with("A"), ends_width("string")))?; // Passes
/// verify_that!("A string", all!(starts_with("A"), ends_width("not a string")))?; // Fails
/// ```
///
/// Using this macro is equivalent to using the
/// [`and`][crate::matchers::conjunction_matcher::AndMethodExt::and] extension
/// method:
///
/// ```rust
/// verify_that!("A string", starts_with("A").and(ends_width("string")))?; // Also passes
/// ```
///
/// Assertion failure messages are not guaranteed to be identical, however.
#[macro_export]
macro_rules! all {
    ($($matcher:expr),* $(,)?) => {{
        #[cfg(google3)]
        use $crate::internal::AllMatcher;
        #[cfg(not(google3))]
        use $crate::matchers::all_matcher::internal::AllMatcher;
        AllMatcher::new([$(&$matcher),*])
    }}
}

/// Functionality needed by the [`all`] macro.
///
/// For internal use only. API stablility is not guaranteed!
#[doc(hidden)]
pub mod internal {
    #[cfg(not(google3))]
    use crate as googletest;
    #[cfg(google3)]
    use anything_matcher::anything;
    use googletest::matcher::{Describe, MatchExplanation, Matcher, MatcherResult};
    #[cfg(not(google3))]
    use googletest::matchers::anything;
    use std::fmt::Debug;

    /// A matcher which matches an input value matched by all matchers in the
    /// array `components`.
    ///
    /// For internal use only. API stablility is not guaranteed!
    #[doc(hidden)]
    pub struct AllMatcher<'a, T: Debug + ?Sized, const N: usize> {
        components: [&'a dyn Matcher<T>; N],
    }

    impl<'a, T: Debug + ?Sized, const N: usize> AllMatcher<'a, T, N> {
        /// Constructs an [`AllMatcher`] with the given component matchers.
        ///
        /// Intended for use only by the [`all`] macro.
        pub fn new(components: [&'a dyn Matcher<T>; N]) -> Self {
            Self { components }
        }
    }

    impl<'a, T: Debug + ?Sized, const N: usize> Matcher<T> for AllMatcher<'a, T, N> {
        fn matches(&self, actual: &T) -> MatcherResult {
            for component in self.components {
                match component.matches(actual) {
                    MatcherResult::DoesNotMatch => {
                        return MatcherResult::DoesNotMatch;
                    }
                    MatcherResult::Matches => {}
                }
            }
            MatcherResult::Matches
        }

        fn explain_match(&self, actual: &T) -> MatchExplanation {
            match N {
                0 => anything::<T>().explain_match(actual),
                1 => self.components[0].explain_match(actual),
                _ => {
                    let failures = self
                        .components
                        .iter()
                        .filter(|component| {
                            matches!(component.matches(actual), MatcherResult::DoesNotMatch)
                        })
                        .map(|component| format!("{}", component.explain_match(actual)))
                        .collect::<Vec<_>>();
                    MatchExplanation::create(failures.join(" AND\n"))
                }
            }
        }
    }

    impl<'a, T: Debug + ?Sized, const N: usize> Describe for AllMatcher<'a, T, N> {
        fn describe(&self, matcher_result: MatcherResult) -> String {
            match N {
                0 => anything::<T>().describe(matcher_result),
                1 => self.components[0].describe(matcher_result),
                _ => self
                    .components
                    .iter()
                    .map(|m| m.describe(matcher_result))
                    .collect::<Vec<_>>()
                    .join(matcher_result.pick(" and\n", " or\n")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(google3))]
    use crate as googletest;
    use googletest::matcher::{Describe, MatcherResult};
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{google_test, matcher::Matcher, verify_that, Result};
    use matchers::{contains_substring, displays_as, ends_with, eq, not, starts_with};

    #[google_test]
    fn matches_any_value_when_list_is_empty() -> Result<()> {
        verify_that!((), all!())
    }

    #[google_test]
    fn matches_value_with_single_matching_component() -> Result<()> {
        verify_that!(123, all!(eq(123)))
    }

    #[google_test]
    fn does_not_match_value_with_single_non_matching_component() -> Result<()> {
        verify_that!(123, not(all!(eq(456))))
    }

    #[google_test]
    fn matches_value_with_two_matching_components() -> Result<()> {
        verify_that!("A string", all!(starts_with("A"), ends_with("string")))
    }

    #[google_test]
    fn does_not_match_value_with_one_non_matching_component_among_two_components() -> Result<()> {
        verify_that!(123, not(all!(eq(123), eq(456))))
    }

    #[google_test]
    fn supports_trailing_comma() -> Result<()> {
        verify_that!(
            "An important string",
            all!(starts_with("An"), contains_substring("important"), ends_with("string"),)
        )
    }

    #[google_test]
    fn description_shows_more_than_one_matcher() -> Result<()> {
        let first_matcher = starts_with("A");
        let second_matcher = ends_with("string");
        let matcher: internal::AllMatcher<String, 2> = all!(first_matcher, second_matcher);

        verify_that!(
            matcher.describe(MatcherResult::Matches),
            eq("starts with prefix \"A\" and\nends with suffix \"string\"")
        )
    }

    #[google_test]
    fn description_shows_one_matcher_directly() -> Result<()> {
        let first_matcher = starts_with("A");
        let matcher: internal::AllMatcher<String, 1> = all!(first_matcher);

        verify_that!(matcher.describe(MatcherResult::Matches), eq("starts with prefix \"A\""))
    }

    #[google_test]
    fn mismatch_description_shows_which_matcher_failed_if_more_than_one_constituent() -> Result<()>
    {
        let first_matcher = starts_with("Another");
        let second_matcher = ends_with("string");
        let matcher: internal::AllMatcher<str, 2> = all!(first_matcher, second_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }

    #[google_test]
    fn mismatch_description_is_simple_when_only_one_consistuent() -> Result<()> {
        let first_matcher = starts_with("Another");
        let matcher: internal::AllMatcher<str, 1> = all!(first_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }

    #[google_test]
    fn mismatch_description_two_failed_matchers() -> Result<()> {
        verify_that!(
            all!(starts_with("One"), starts_with("Two")).explain_match("Three"),
            displays_as(eq(
                "which does not start with \"One\" AND\nwhich does not start with \"Two\""
            ))
        )
    }

    #[google_test]
    fn mismatch_description_empty_matcher() -> Result<()> {
        verify_that!(all!().explain_match("Three"), displays_as(eq("which is anything")))
    }
}
