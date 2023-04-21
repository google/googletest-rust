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

/// Matches a value greater than or equal to (in the sense of `>=`) `expected`.
///
/// The types of `ActualT` of `actual` and `ExpectedT` of `expected` must be
/// comparable via the `PartialOrd` trait. Namely, `ActualT` must implement
/// `PartialOrd<ExpectedT>`.
///
/// ```
/// verify_that!(1, ge(0))?; // Passes
/// verify_that!(0, ge(1))?; // Fails
/// ```
///
/// In most cases the params neeed to be the same type or they need to be cast
/// explicitly. This can be surprising when comparing integer types or
/// references:
///
/// ```
/// verify_that!(123u32, ge(0u64))?; // Does not compile
/// verify_that!(123u32 as u64, ge(0u64))?; // Passes
/// ```
///
/// ```
/// let actual: &u32 = &2;
/// let expected: u32 = 0;
/// verify_that(actual, ge(expected))?; // Does not compile
/// verify_that(actual, ge(&expected))?; // Compiles and passes
/// ```
///
/// You can find the standard library `PartialOrd` implementation in
/// <https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html#implementors>
pub fn ge<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug>(
    expected: ExpectedT,
) -> impl Matcher<ActualT> {
    GeMatcher { expected }
}

pub struct GeMatcher<ExpectedT> {
    expected: ExpectedT,
}

impl<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug> Matcher<ActualT>
    for GeMatcher<ExpectedT>
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        (*actual >= self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => format!("is greater than or equal to {:?}", self.expected),
            MatcherResult::DoesNotMatch => format!("is less than {:?}", self.expected),
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
    use googletest::{verify_that, Result};
    use matchers::{contains_substring, displays_as, eq, err};
    use std::ffi::OsString;

    #[test]
    fn ge_matches_i32_with_i32() -> Result<()> {
        let actual: i32 = 0;
        let expected: i32 = 0;
        verify_that!(actual, ge(expected))
    }

    #[test]
    fn ge_does_not_match_smaller_i32() -> Result<()> {
        let matcher = ge(10);
        let result = matcher.matches(&9);
        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[test]
    fn ge_matches_bigger_str() -> Result<()> {
        verify_that!("B", ge("A"))
    }

    #[test]
    fn ge_does_not_match_lesser_str() -> Result<()> {
        let matcher = ge("z");
        let result = matcher.matches(&"a");
        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[test]
    fn ge_mismatch_contains_actual_and_expected() -> Result<()> {
        let result = verify_that!(591, ge(927));

        verify_that!(
            result,
            err(displays_as(contains_substring(
                "Value of: 591\n\
                Expected: is greater than or equal to 927\n\
                Actual: 591, which is less than 927"
            )))
        )
    }

    // Test `ge` matcher where actual is `&OsString` and expected is `&str`.
    // Note that stdlib is a little bit inconsistent: `PartialOrd` exists for
    // `OsString` and `str`, but only in one direction: it's only possible to
    // compare `OsString` with `str` if `OsString` is on the left side of the
    // ">=" operator (`impl PartialOrd<str> for OsString`).
    //
    // The comparison in the other direction is not defined.
    //
    // This means that the test case bellow effectively ensures that
    // `verify_that(actual, ge(expected))` works if `actual >= expected` works
    // (regardless whether the `expected >= actual` works`).
    #[test]
    fn ge_matches_owned_osstring_reference_with_string_reference() -> Result<()> {
        let expected = "A";
        let actual: OsString = "B".to_string().into();
        verify_that!(&actual, ge(expected))
    }

    #[test]
    fn ge_matches_ipv6addr_with_ipaddr() -> Result<()> {
        use std::net::IpAddr;
        use std::net::Ipv6Addr;
        let actual: Ipv6Addr = "2001:4860:4860::8844".parse().unwrap();
        let expected: IpAddr = "127.0.0.1".parse().unwrap();
        verify_that!(actual, ge(expected))
    }

    #[test]
    fn ge_matches_with_custom_partial_ord() -> Result<()> {
        /// A custom "number" that is lower than all other numbers. The only
        /// things we define about this "special" number is `PartialOrd` and
        /// `PartialEq` against `u32`.
        #[derive(Debug)]
        struct VeryLowNumber {}

        impl std::cmp::PartialEq<u32> for VeryLowNumber {
            fn eq(&self, _other: &u32) -> bool {
                false
            }
        }

        // PartialOrd (required for >) requires PartialEq.
        impl std::cmp::PartialOrd<u32> for VeryLowNumber {
            fn partial_cmp(&self, _other: &u32) -> Option<std::cmp::Ordering> {
                Some(std::cmp::Ordering::Less)
            }
        }

        impl std::cmp::PartialEq<VeryLowNumber> for u32 {
            fn eq(&self, _other: &VeryLowNumber) -> bool {
                false
            }
        }

        impl std::cmp::PartialOrd<VeryLowNumber> for u32 {
            fn partial_cmp(&self, _other: &VeryLowNumber) -> Option<std::cmp::Ordering> {
                Some(std::cmp::Ordering::Greater)
            }
        }

        let actual: u32 = 42;
        let expected = VeryLowNumber {};

        verify_that!(actual, ge(expected))
    }
}
