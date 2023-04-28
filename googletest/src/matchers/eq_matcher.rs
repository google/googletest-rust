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
#[cfg(google3)]
use googletest::*;
use std::{fmt::Debug, marker::PhantomData};

/// Matches a value equal (in the sense of `==`) to `expected`.
///
/// The type of `expected` must implement the [`PartialEq`] trait so that the
/// expected and actual values can be compared.
///
/// ```
/// # use googletest::{matchers::eq, verify_that, Result};
/// # fn should_pass() -> Result<()> {
/// verify_that!(123, eq(123))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(123, eq(234))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// `expected` to `actual` must be comparable with one another via the
/// [`PartialEq`] trait. In most cases, this means that they must be of the same
/// type. However, there are a few cases where different but closely related
/// types are comparable, for example `String` with `&str`.
///
/// ```
/// # use googletest::{matchers::eq, verify_that, Result};
/// # fn should_pass() -> Result<()> {
/// verify_that!(String::from("Some value"), eq("Some value"))?; // Passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// In most cases however, one must convert one of the arguments explicitly.
/// This can be surprising when comparing integer types or references.
///
/// ```compile_fail
/// verify_that!(123u32, eq(123u64))?; // Does not compile
/// verify_that!(123u32 as u64, eq(123u64))?; // Passes
/// ```
///
/// ```ignore
/// let actual: &T = ...;
/// let expected: T = T{...};
/// verify_that(actual, eq(expected))?; // Does not compile
/// verify_that(actual, eq(&expected))?; // Compiles
/// ```
///
/// When matching with string types (`&str` and `String`), one can set more
/// options on how equality is checked through the
/// [`StrMatcherConfigurator`][crate::matchers::str_matcher::StrMatcherConfigurator]
/// extension trait, which is implemented for this matcher.
pub fn eq<A: ?Sized, T>(expected: T) -> EqMatcher<A, T> {
    EqMatcher { expected, phantom: Default::default() }
}

/// A matcher which matches a value equal to `expected`.
///
/// See [`eq`].
pub struct EqMatcher<A: ?Sized, T> {
    pub(crate) expected: T,
    phantom: PhantomData<A>,
}

impl<A: Debug + ?Sized, T: PartialEq<A> + Debug> Matcher for EqMatcher<A, T> {
    type ActualT = A;

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
    use super::eq;
    use crate::{verify_that, Result};

    #[test]
    fn eq_matches_string_reference_with_string_reference() -> Result<()> {
        verify_that!("A string", eq("A string"))
    }

    #[test]
    fn eq_matches_owned_string_with_string_reference() -> Result<()> {
        let value = "A string".to_string();
        verify_that!(value, eq("A string"))
    }

    #[test]
    fn eq_matches_owned_string_reference_with_string_reference() -> Result<()> {
        let value = "A string".to_string();
        verify_that!(&value, eq("A string"))
    }

    #[test]
    fn eq_matches_i32_with_i32() -> Result<()> {
        verify_that!(123, eq(123))
    }
}
