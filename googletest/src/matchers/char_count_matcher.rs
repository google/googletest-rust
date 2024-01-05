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

use crate::{
    description::Description,
    matcher::{Matcher, MatcherResult},
};
use std::{fmt::Debug, marker::PhantomData};

/// Matches a string whose number of Unicode scalars matches `expected`.
///
/// In other words, the argument must match the output of
/// [`actual_string.chars().count()`][std::str::Chars].
///
/// This can have surprising effects when what appears to be a single character
/// is composed of multiple Unicode scalars. See [Rust documentation on
/// character
/// representation](https://doc.rust-lang.org/std/primitive.char.html#representation)
/// for more information.
///
/// This matches against owned strings and string slices.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let string_slice = "A string";
/// verify_that!(string_slice, char_count(eq(8)))?;
/// let non_ascii_string_slice = "Ä ſtřiɲğ";
/// verify_that!(non_ascii_string_slice, char_count(eq(8)))?;
/// let owned_string = String::from("A string");
/// verify_that!(owned_string, char_count(eq(8)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// The parameter `expected` can be any integer numeric matcher.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let string_slice = "A string";
/// verify_that!(string_slice, char_count(gt(4)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn char_count<T: Debug + ?Sized + AsRef<str>, E: Matcher<ActualT = usize>>(
    expected: E,
) -> impl Matcher<ActualT = T> {
    CharLenMatcher { expected, phantom: Default::default() }
}

struct CharLenMatcher<T: ?Sized, E> {
    expected: E,
    phantom: PhantomData<T>,
}

impl<T: Debug + ?Sized + AsRef<str>, E: Matcher<ActualT = usize>> Matcher for CharLenMatcher<T, E> {
    type ActualT = T;

    fn matches(&self, actual: &T) -> MatcherResult {
        self.expected.matches(&actual.as_ref().chars().count())
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!(
                "has character count, which {}",
                self.expected.describe(MatcherResult::Match)
            )
            .into(),
            MatcherResult::NoMatch => format!(
                "has character count, which {}",
                self.expected.describe(MatcherResult::NoMatch)
            )
            .into(),
        }
    }

    fn explain_match(&self, actual: &T) -> Description {
        let actual_size = actual.as_ref().chars().count();
        format!(
            "which has character count {}, {}",
            actual_size,
            self.expected.explain_match(&actual_size)
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::char_count;
    use crate::description::Description;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;
    use std::fmt::Debug;
    use std::marker::PhantomData;

    #[test]
    fn char_count_matches_string_slice() -> Result<()> {
        let value = "abcd";
        verify_that!(value, char_count(eq(4)))
    }

    #[test]
    fn char_count_matches_owned_string() -> Result<()> {
        let value = String::from("abcd");
        verify_that!(value, char_count(eq(4)))
    }

    #[test]
    fn char_count_counts_non_ascii_characters_correctly() -> Result<()> {
        let value = "äöüß";
        verify_that!(value, char_count(eq(4)))
    }

    #[test]
    fn char_count_explains_match() -> Result<()> {
        struct TestMatcher<T>(PhantomData<T>);
        impl<T: Debug> Matcher for TestMatcher<T> {
            type ActualT = T;

            fn matches(&self, _: &T) -> MatcherResult {
                false.into()
            }

            fn describe(&self, _: MatcherResult) -> Description {
                "called described".into()
            }

            fn explain_match(&self, _: &T) -> Description {
                "called explain_match".into()
            }
        }
        verify_that!(
            char_count(TestMatcher(Default::default())).explain_match(&"A string"),
            displays_as(eq("which has character count 8, called explain_match"))
        )
    }

    #[test]
    fn char_count_has_correct_failure_message() -> Result<()> {
        let result = verify_that!("äöüß", char_count(eq(3)));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                r#"
                Value of: "äöüß"
                Expected: has character count, which is equal to 3
                Actual: "äöüß",
                  which has character count 4, which isn't equal to 3"#
            ))))
        )
    }
}
