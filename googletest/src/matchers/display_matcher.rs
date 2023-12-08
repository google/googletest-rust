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

use crate::matcher::{Matcher, MatcherResult};
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

/// Matches the string representation of types that implement `Display`.
///
/// ```ignore
/// let result: impl Display = ...;
/// verify_that!(result, displays_as(eq(format!("{}", result))))?;
/// ```
pub fn displays_as<T: Debug + Display, InnerMatcher: Matcher<String>>(
    inner: InnerMatcher,
) -> impl Matcher<T> {
    DisplayMatcher::<T, _> { inner, phantom: Default::default() }
}

struct DisplayMatcher<T, InnerMatcher: Matcher<String>> {
    inner: InnerMatcher,
    phantom: PhantomData<T>,
}

impl<T: Debug + Display, InnerMatcher: Matcher<String>> Matcher<T>
    for DisplayMatcher<T, InnerMatcher>
{
    fn matches<'a>(&self, actual: &'a T) -> MatcherResult
    where
        T: 'a,
    {
        self.inner.matches(&format!("{actual}"))
    }

    fn explain_match<'a>(&self, actual: &'a T) -> String
    where
        T: 'a,
    {
        format!("which displays as a string {}", self.inner.explain_match(&format!("{actual}")))
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Match => {
                format!("displays as a string which {}", self.inner.describe(MatcherResult::Match))
            }
            MatcherResult::NoMatch => {
                format!(
                    "doesn't display as a string which {}",
                    self.inner.describe(MatcherResult::Match)
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::displays_as;
    use crate::prelude::*;
    use indoc::indoc;
    use std::fmt::{Debug, Display, Error, Formatter};

    #[test]
    fn display_matches_i32() -> Result<()> {
        let value = 32;
        verify_that!(value, displays_as(eq("32")))?;
        Ok(())
    }

    #[test]
    fn display_matches_str() -> Result<()> {
        let value = "32";
        verify_that!(value, displays_as(eq("32")))?;
        Ok(())
    }

    #[test]
    fn display_matches_struct() -> Result<()> {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct Struct {
            a: i32,
            b: i64,
        }
        impl Display for Struct {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
                write!(f, "{:?}", self)
            }
        }
        verify_that!(Struct { a: 123, b: 321 }, displays_as(eq("Struct { a: 123, b: 321 }")))?;
        Ok(())
    }

    #[test]
    fn display_displays_error_message_with_explanation_from_inner_matcher() -> Result<()> {
        let result = verify_that!("123\n234", displays_as(eq("123\n345")));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    which displays as a string which isn't equal to \"123\\n345\"
                    Difference(-actual / +expected):
                     123
                    -234
                    +345
                "
            ))))
        )
    }
}
