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
// macro is documented in the matcher module.
#![doc(hidden)]

/// Matches a value which all of the given matchers match.
///
/// Each argument is a [`Matcher`][crate::matcher::Matcher] which matches
/// against the actual value.
///
/// For example:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!("A string", all!(starts_with("A"), ends_with("string")))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!("A string", all!(starts_with("A"), ends_with("not a string")))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// Using this macro is equivalent to using the
/// [`and`][crate::matcher::Matcher::and] method:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(10, gt(9).and(lt(11)))?; // Also passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Assertion failure messages are not guaranteed to be identical, however.
#[macro_export]
#[doc(hidden)]
macro_rules! __all {
    ($(,)?) => {{
        $crate::matchers::anything()
    }} ;
    ($matcher:expr $(,)?) => {{
        $matcher
    }};
    ($head:expr, $head2:expr $(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::ConjunctionMatcher::new($head, $head2)
    }};
    ($head:expr, $head2:expr, $($tail:expr),+ $(,)?) => {{
        $crate::__all![
            $crate::matchers::__internal_unstable_do_not_depend_on_these::ConjunctionMatcher::new($head, $head2),
            $($tail),+
        ]
    }}
}

#[cfg(test)]
mod tests {
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn description_shows_more_than_one_matcher() -> Result<()> {
        let first_matcher: StrMatcher<String, _, _> = starts_with("A");
        let second_matcher = ends_with("string");
        let matcher = all!(first_matcher, second_matcher);

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq(indoc!(
                "
                has all the following properties:
                  * starts with prefix \"A\"
                  * ends with suffix \"string\""
            )))
        )
    }

    #[test]
    fn description_shows_one_matcher_directly() -> Result<()> {
        let first_matcher: StrMatcher<String, _, _> = starts_with("A");
        let matcher = all!(first_matcher);

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq("starts with prefix \"A\""))
        )
    }

    #[test]
    fn mismatch_description_shows_which_matcher_failed_if_more_than_one_constituent() -> Result<()>
    {
        let first_matcher: StrMatcher<str, _, _> = starts_with("Another");
        let second_matcher = ends_with("string");
        let matcher = all!(first_matcher, second_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }

    #[test]
    fn mismatch_description_is_simple_when_only_one_consistuent() -> Result<()> {
        let first_matcher = starts_with("Another");
        let matcher = all!(first_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }
}
