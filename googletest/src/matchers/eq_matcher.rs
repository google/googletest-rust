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
use googletest::matcher::{Matcher, MatcherResult};
use std::fmt::Debug;

/// Matches a value equal (in the sense of `==`) to `expected`.
///
/// The type of `expected` must implement the [`PartialEq`] trait so that the
/// expected and actual values can be compared.
///
/// ```
/// verify_that!(123, eq(123))?; // Passes
/// verify_that!(123, eq(234))?; // Fails
/// ```
///
/// `expected` to `actual` must be comparable with one another via the
/// `PartialEq` trait. In most cases, this means that they must be of the same
/// type. However, there are a few cases where different but closely related
/// types are comparable, for example `String` with `&str`.
///
/// ```
/// verify_that!(String::new("Some value"), eq("Some value"))?; // Passes
/// ```
///
/// In most cases however, one must convert one of the arguments explicitly.
/// This can be surprising when comparing integer types or references.
///
/// ```
/// verify_that!(123u32, eq(123u64))?; // Does not compile
/// verify_that!(123u32 as u64, eq(123u64))?; // Passes
/// ```
///
/// ```
/// let actual: &T = ...;
/// let expected: T = T{...};
/// verify_that(actual, eq(expected))?; // Does not compile
/// verify_that(actual, eq(&expected))?; // Compiles
/// ```
///
/// You can find the standard library PartialEq implementation in
/// <https://doc.rust-lang.org/core/cmp/trait.PartialEq.html#implementors>
pub fn eq<T>(expected: T) -> EqMatcher<T> {
    EqMatcher { expected }
}

/// A matcher which matches a value equal to `expected`.
///
/// See [`eq`].
pub struct EqMatcher<T> {
    pub(crate) expected: T,
}

impl<A: Debug, T: PartialEq<A> + Debug> Matcher<A> for EqMatcher<T> {
    fn matches(&self, actual: &A) -> MatcherResult {
        (self.expected == *actual).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => format!("is equal to {:?}", self.expected),
            MatcherResult::DoesNotMatch => format!("isn't equal to {:?}", self.expected),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(google3))]
    use crate as googletest;
    use googletest::{google_test, verify_that, Result};

    #[google_test]
    fn eq_matches_string_reference_with_string_reference() -> Result<()> {
        verify_that!("A string", eq("A string"))
    }

    #[google_test]
    fn eq_matches_owned_string_with_string_reference() -> Result<()> {
        let value = "A string".to_string();
        verify_that!(value, eq("A string"))
    }

    #[google_test]
    fn eq_matches_owned_string_reference_with_string_reference() -> Result<()> {
        let value = "A string".to_string();
        verify_that!(&value, eq("A string"))
    }

    #[google_test]
    fn eq_matches_i32_with_i32() -> Result<()> {
        verify_that!(123, eq(123))
    }
}
