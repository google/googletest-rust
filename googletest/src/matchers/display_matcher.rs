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

use crate::description::Description;
use crate::matcher::{Matcher, MatcherExt, MatcherResult};
use std::fmt::{Debug, Display};

/// Matches the string representation of types that implement `Display`.
///
/// ```ignore
/// let result: impl Display = ...;
/// verify_that!(result, displays_as(eq(format!("{}", result))))?;
/// ```
pub fn displays_as<'a, InnerMatcher: Matcher<'a, String>>(
    inner: InnerMatcher,
) -> DisplayMatcher<InnerMatcher> {
    DisplayMatcher { inner }
}

#[derive(MatcherExt)]
pub struct DisplayMatcher<InnerMatcher> {
    inner: InnerMatcher,
}

impl<'a, 's, T: Debug + Display, InnerMatcher: Matcher<'s, String>> Matcher<'a, T>
    for DisplayMatcher<InnerMatcher>
{
    fn matches<'b>(&self, actual: &'b T) -> MatcherResult where 'a: 'b{
        self.inner.matches(&format!("{actual}"))
    }

    fn explain_match<'b>(&self, actual: &'b T) -> Description where 'a: 'b{
        format!(
            "which displays as {:?} {}",
            actual.to_string(),
            self.inner.explain_match(&format!("{actual}"))
        )
        .into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("displays as a string which {}", self.inner.describe(MatcherResult::Match))
                    .into()
            }
            MatcherResult::NoMatch => format!(
                "doesn't display as a string which {}",
                self.inner.describe(MatcherResult::Match)
            )
            .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::displays_as;
    use crate::prelude::*;
    use indoc::indoc;
    use serial_test::parallel;
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
    #[parallel]
    fn display_displays_error_message_with_explanation_from_inner_matcher() -> Result<()> {
        let result = verify_that!("123\n234", displays_as(eq("123\n345")));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                  Actual: \"123\\n234\",
                    which displays as \"123\\n234\" which isn't equal to \"123\\n345\"
                    Difference(-actual / +expected):
                     123
                    -234
                    +345
                "
            ))))
        )
    }
}
