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

/// Matches a container's elements to each matcher in order.
///
/// ```rust
/// verify_that!(vec![1,2,3], elements_are![eq(1), anything(), gt(0).and(lt(123))])
/// ```
///
/// Do not use with unordered containers. Prefer unordered_elements_are!(...).
#[macro_export]
macro_rules! elements_are {
    ($($matcher:expr),* $(,)?) => {{
        #[cfg(google3)]
        use $crate::internal::ElementsAre;
        #[cfg(not(google3))]
        use $crate::matchers::elements_are_matcher::internal::ElementsAre;
        ElementsAre::new(&[$(&$matcher),*])
    }}
}

/// Module for use only by the procedural macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    #[cfg(not(google3))]
    use crate as googletest;
    use googletest::matcher::{Describe, MatchExplanation, Matcher, MatcherResult};
    #[cfg(not(google3))]
    use googletest::matchers::has_size::HasSize;
    #[cfg(google3)]
    use has_size::HasSize;
    use std::fmt::Debug;

    /// This struct is meant to be used only by the macro `elements_are!`.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub struct ElementsAre<'a, T: Debug> {
        elements: &'a [&'a dyn Matcher<T>],
    }

    impl<'a, T: Debug> ElementsAre<'a, T> {
        /// Factory only intended for use in the macro `elements_are!`.
        ///
        /// **For internal use only. API stablility is not guaranteed!**
        #[doc(hidden)]
        pub fn new(elements: &'a [&'a dyn Matcher<T>]) -> Self {
            Self { elements }
        }
    }

    impl<'a, T: Debug, ContainerT: Debug + HasSize> Matcher<ContainerT> for ElementsAre<'a, T>
    where
        for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
    {
        fn matches(&self, actual: &ContainerT) -> MatcherResult {
            if actual.size() != self.elements.size() {
                return MatcherResult::DoesNotMatch;
            }
            for (a, e) in actual.into_iter().zip(self.elements) {
                if matches!(e.matches(a), MatcherResult::DoesNotMatch) {
                    return MatcherResult::DoesNotMatch;
                }
            }
            MatcherResult::Matches
        }

        fn explain_match(&self, actual: &ContainerT) -> MatchExplanation {
            if actual.size() != self.elements.size() {
                return MatchExplanation::create(format!("whose size is {}", actual.size(),));
            }
            let mismatches = actual
                .into_iter()
                .zip(self.elements)
                .enumerate()
                .filter(|&(_, (a, e))| matches!(e.matches(a), MatcherResult::DoesNotMatch))
                .map(|(idx, (a, e))| format!("element #{} is {:?}, {}", idx, a, e.explain_match(a)))
                .collect::<Vec<_>>();
            if mismatches.is_empty() {
                MatchExplanation::create("whose elements all match".to_string())
            } else {
                MatchExplanation::create(format!("whose {}", mismatches.join(" and\n")))
            }
        }
    }

    impl<'a, T: Debug> Describe for ElementsAre<'a, T> {
        fn describe(&self, matcher_result: MatcherResult) -> String {
            format!(
                "{} elements:\n{}",
                matcher_result.pick("has", "doesn't have"),
                self.elements
                    .iter()
                    .map(|m| m.describe(MatcherResult::Matches))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(google3))]
    use crate as googletest;
    use googletest::matcher::Matcher;
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{google_test, verify_that, Result};
    use matchers::{contains_substring, displays_as, eq, err, not};

    #[google_test]
    fn elements_are_matches_vector() -> Result<()> {
        let value = vec![1, 2, 3];
        verify_that!(value, elements_are![eq(1), eq(2), eq(3)])
    }

    #[google_test]
    fn elements_are_supports_trailing_comma() -> Result<()> {
        let value = vec![1, 2, 3];
        verify_that!(value, elements_are![eq(1), eq(2), eq(3),])
    }

    #[google_test]
    fn elements_are_returns_no_match_when_expected_and_actual_sizes_differ() -> Result<()> {
        let value = vec![1, 2];
        verify_that!(value, not(elements_are![eq(1), eq(2), eq(3)]))
    }

    #[google_test]
    fn elements_are_produces_correct_failure_message() -> Result<()> {
        let result = verify_that!(vec![1, 4, 3], elements_are![eq(1), eq(2), eq(3)]);
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: vec![1, 4, 3]\n\
                Expected: has elements:\n\
                is equal to 1\n\
                is equal to 2\n\
                is equal to 3\n\
                Actual: [1, 4, 3], whose element #1 is 4, which isn't equal to 2"
            )))
        )
    }

    #[google_test]
    fn elements_are_explain_match_wrong_size() -> Result<()> {
        verify_that!(
            elements_are![eq(1)].explain_match(&vec![1, 2]),
            displays_as(eq("whose size is 2"))
        )
    }
}
