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
use googletest::matcher::{Describe, Matcher, MatcherResult};
use std::fmt::Debug;

/// Matches a value less (in the sense of `<`) than `expected`.
///
/// The types of `ActualT` of `actual` and `ExpectedT` of `expected` must be
/// comparable via the `PartialOrd` trait. Namely, `ActualT` must implement
/// `PartialOrd<ExpectedT>`.
///
/// ```rust
/// verify_that!(1, lt(2))?; // Passes
/// verify_that!(2, lt(2))?; // Fails
/// ```
///
/// In most cases the params neeed to be the same type or they need to be cast
/// explicitly. This can be surprising when comparing integer types or
/// references:
///
/// ```rust
/// verify_that!(123u32, lt(0u64))?; // Does not compile
/// verify_that!(123u32 as u64, lt(100000000u64))?; // Passes
/// ```
///
/// ```rust
/// let actual: &u32 = &2;
/// let expected: u32 = 70;
/// verify_that(actual, lt(expected))?; // Does not compile
/// verify_that(actual, lt(&expected))?; // Compiles and passes
/// ```
///
/// You can find the standard library `PartialOrd` implementation in
/// <https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html#implementors>
pub fn lt<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug>(
    expected: ExpectedT,
) -> impl Matcher<ActualT> {
    LtMatcher { expected }
}

pub struct LtMatcher<ExpectedT> {
    expected: ExpectedT,
}

impl<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug> Matcher<ActualT>
    for LtMatcher<ExpectedT>
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        if *actual < self.expected { MatcherResult::Matches } else { MatcherResult::DoesNotMatch }
    }
}

impl<ExpectedT: Debug> Describe for LtMatcher<ExpectedT> {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => format!("is less than {:?}", self.expected),
            MatcherResult::DoesNotMatch => {
                format!("is greater than or equal to {:?}", self.expected)
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
    use matchers::{contains_substring, eq};
    use std::ffi::OsString;

    #[google_test]
    fn lt_matches_i32_with_i32() -> Result<()> {
        let actual: i32 = 10000;
        let expected: i32 = 20000;
        verify_that!(actual, lt(expected))
    }

    #[google_test]
    fn lt_does_not_match_equal_i32() -> Result<()> {
        let matcher = lt(10);
        let result = matcher.matches(&10);
        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn lt_does_not_match_lower_i32() -> Result<()> {
        let matcher = lt(-50);
        let result = matcher.matches(&50);
        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn lt_matches_lesser_str() -> Result<()> {
        verify_that!("A", lt("B"))
    }

    #[google_test]
    fn lt_does_not_match_bigger_str() -> Result<()> {
        let matcher = lt("ab");
        let result = matcher.matches(&"az");
        verify_that!(result, eq(MatcherResult::DoesNotMatch))
    }

    #[google_test]
    fn lt_mismatch_contains_actual_and_expected() -> Result<()> {
        let result = verify_that!(481, lt(45));
        let formatted_message = format!("{}", result.unwrap_err());

        verify_that!(
            formatted_message.as_str(),
            contains_substring(
                "Value of: 481\n\
                Expected: is less than 45\n\
                Actual: 481, which is greater than or equal to 45"
            )
        )
    }

    // Test `lt` matcher where actual is `&OsString` and expected is `&str`.
    // Note that stdlib is a little bit inconsistent: `PartialOrd` exists for
    // `OsString` and `str`, but only in one direction: it's only possible to
    // compare `OsString` with `str` if `OsString` is on the left side of the
    // "<" operator (`impl PartialOrd<str> for OsString`).
    //
    // The comparison in the other direction is not defined.
    //
    // This means that the test case bellow effectively ensures that
    // `verify_that(actual, lt(expected))` works if `actual < expected` works
    // (regardless whether the `expected < actual` works`).
    #[google_test]
    fn lt_matches_owned_osstring_reference_with_string_reference() -> Result<()> {
        let expected = "C";
        let actual: OsString = "B".to_string().into();
        verify_that!(&actual, lt(expected))
    }

    #[google_test]
    fn lt_matches_ipv6addr_with_ipaddr() -> Result<()> {
        use std::net::IpAddr;
        use std::net::Ipv6Addr;
        let actual: IpAddr = "127.0.0.1".parse().unwrap();
        let expected: Ipv6Addr = "2001:4860:4860::8844".parse().unwrap();
        verify_that!(actual, lt(expected))
    }

    #[google_test]
    fn lt_matches_with_custom_partial_ord() -> Result<()> {
        /// A custom "number" that is smaller than all other numbers. The only
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

        let actual = VeryLowNumber {};
        let expected: u32 = 42;

        verify_that!(actual, lt(expected))
    }
}
