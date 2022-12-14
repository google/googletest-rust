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

#[cfg(not(google3))]
use crate as googletest;
use googletest::matcher::{Describe, MatchExplanation, Matcher, MatcherResult};
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

/// Matches the string representation of types that implement `Display`.
///
///
/// ```rust
/// let result: impl Display = ...;
/// verify_that!(result, displays_as(eq(format!("{}", result))))?;
/// ```
pub fn displays_as<T: Debug + Display, InnerMatcher: Matcher<String>>(
    inner: InnerMatcher,
) -> impl Matcher<T> {
    DisplayMatcher { inner, phantom: Default::default() }
}

struct DisplayMatcher<T, InnerMatcher: Matcher<String>> {
    inner: InnerMatcher,
    phantom: PhantomData<T>,
}

impl<T: Debug + Display, InnerMatcher: Matcher<String>> Matcher<T>
    for DisplayMatcher<T, InnerMatcher>
{
    fn matches(&self, actual: &T) -> MatcherResult {
        self.inner.matches(&format!("{actual}"))
    }

    fn explain_match(&self, actual: &T) -> MatchExplanation {
        MatchExplanation::create(format!("which displays as \"{}\"", actual))
    }
}

impl<T, InnerMatcherT: Matcher<String>> Describe for DisplayMatcher<T, InnerMatcherT> {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!(
                    "displays as a string which {}",
                    self.inner.describe(MatcherResult::Matches)
                )
            }
            MatcherResult::DoesNotMatch => {
                format!(
                    "doesn't display as a string which {}",
                    self.inner.describe(MatcherResult::Matches)
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(google3))]
    use crate as googletest;
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{google_test, verify_that, Result};
    use matchers::eq;
    use std::fmt::{Debug, Display, Error, Formatter};

    #[google_test]
    fn display_matches_i32() -> Result<()> {
        let value = 32;
        verify_that!(value, displays_as(eq("32")))?;
        Ok(())
    }

    #[google_test]
    fn display_matches_str() -> Result<()> {
        let value = "32";
        verify_that!(value, displays_as(eq("32")))?;
        Ok(())
    }

    #[google_test]
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
    #[google_test]
    fn display_displays_error_message() -> Result<()> {
        verify_that!(
            displays_as(eq("31")).explain_match(&43),
            displays_as(eq("which displays as \"43\""))
        )
    }
}
