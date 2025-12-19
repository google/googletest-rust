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

use crate::{
    description::Description,
    matcher::{Matcher, MatcherBase, MatcherResult},
};
use std::fmt::Debug;

/// Matches a value is not equal (in the sense of `!=`) to `expected`.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(0, ne(1))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(0, ne(0))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
pub fn ne<T>(expected: T) -> NEMatcher<T> {
    NEMatcher { expected }
}

#[derive(MatcherBase)]
pub struct NEMatcher<T> {
    expected: T,
}

impl<T: Debug, A: Debug + Copy + PartialEq<T>> Matcher<A> for NEMatcher<T> {
    fn matches(&self, actual: A) -> MatcherResult {
        (actual != self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("is not equal to {:?}", self.expected).into(),
            MatcherResult::NoMatch => format!("is equal to {:?}", self.expected).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::Result;
    use indoc::indoc;

    #[test]
    fn ne_matches_i32_with_i32() -> Result<()> {
        verify_that!(123, ne(234))
    }

    #[test]
    fn ne_matches_string_reference_with_string_reference() -> Result<()> {
        verify_that!("A string", ne("B string"))
    }

    #[test]
    fn ne_matches_owned_string_with_string_reference() -> Result<()> {
        let value = "A string".to_string();
        verify_that!(value, ne("B string"))
    }

    #[test]
    fn ne_matches_owned_string_reference_with_string_reference() -> Result<()> {
        let value = "A string".to_string();
        verify_that!(&value, ne("B string"))
    }

    #[test]
    fn ne_struct_debug_diff() -> Result<()> {
        #[derive(Debug, PartialEq)]
        struct Strukt {
            int: i32,
            string: String,
        }

        let result = verify_that!(
            Strukt { int: 123, string: "something".into() },
            ne(&Strukt { int: 123, string: "something".into() })
        );
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {
            "
            Expected: is not equal to Strukt { int: 123, string: \"something\" }
            Actual: Strukt { int: 123, string: \"something\" },
              which is equal to Strukt { int: 123, string: \"something\" }
            "})))
        )
    }
}
