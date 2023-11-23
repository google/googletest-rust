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
    matcher::{Matcher, MatcherResult},
};
use std::{fmt::Debug, marker::PhantomData};

/// Matches a value less than or equal to (in the sense of `<=`) `expected`.
///
/// The types of `ActualT` of `actual` and `ExpectedT` of `expected` must be
/// comparable via the `PartialOrd` trait. Namely, `ActualT` must implement
/// `PartialOrd<ExpectedT>`.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(0, le(0))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(1, le(0))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// In most cases the params neeed to be the same type or they need to be cast
/// explicitly. This can be surprising when comparing integer types or
/// references:
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// # fn should_not_compile() -> Result<()> {
/// verify_that!(1u32, le(2u64))?; // Does not compile
/// verify_that!(1u32 as u64, le(2u64))?; // Passes
/// #     Ok(())
/// # }
/// ```
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// # fn should_not_compile() -> Result<()> {
/// let actual: &u32 = &1;
/// let expected: u32 = 2;
/// verify_that!(actual, le(expected))?; // Does not compile
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let actual: &u32 = &1;
/// let expected: u32 = 2;
/// verify_that!(actual, le(&expected))?; // Compiles and passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// You can find the standard library `PartialOrd` implementation in
/// <https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html#implementors>
pub fn le<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug>(
    expected: ExpectedT,
) -> impl Matcher<ActualT = ActualT> {
    LeMatcher::<ActualT, _> { expected, phantom: Default::default() }
}

struct LeMatcher<ActualT, ExpectedT> {
    expected: ExpectedT,
    phantom: PhantomData<ActualT>,
}

impl<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug> Matcher
    for LeMatcher<ActualT, ExpectedT>
{
    type ActualT = ActualT;

    fn matches(&self, actual: &ActualT) -> MatcherResult {
        (*actual <= self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("is less than or equal to {:?}", self.expected).into(),
            MatcherResult::NoMatch => format!("is greater than {:?}", self.expected).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::le;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;
    use std::ffi::OsString;

    #[test]
    fn le_matches_i32_with_i32() -> Result<()> {
        let actual: i32 = 0;
        let expected: i32 = 0;
        verify_that!(actual, le(expected))
    }

    #[test]
    fn le_does_not_match_bigger_i32() -> Result<()> {
        let matcher = le(0);
        let result = matcher.matches(&1);
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn le_matches_smaller_str() -> Result<()> {
        verify_that!("A", le("B"))
    }

    #[test]
    fn le_does_not_match_bigger_str() -> Result<()> {
        let matcher = le("a");
        let result = matcher.matches(&"z");
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn le_mismatch_contains_actual_and_expected() -> Result<()> {
        let result = verify_that!(489, le(294));
        let formatted_message = format!("{}", result.unwrap_err());

        verify_that!(
            formatted_message.as_str(),
            contains_substring(indoc!(
                "
                Value of: 489
                Expected: is less than or equal to 294
                Actual: 489,
                  which is greater than 294
                "
            ))
        )
    }

    // Test `le` matcher where actual is `&OsString` and expected is `&str`.
    // Note that stdlib is a little bit inconsistent: `PartialOrd` exists for
    // `OsString` and `str`, but only in one direction: it's only possible to
    // compare `OsString` with `str` if `OsString` is on the left side of the
    // "<=" operator (`impl PartialOrd<str> for OsString`).
    //
    // The comparison in the other direction is not defined.
    //
    // This means that the test case bellow effectively ensures that
    // `verify_that(actual, le(expected))` works if `actual <= expected` works
    // (regardless whether the `expected <= actual` works`).
    #[test]
    fn le_matches_owned_osstring_reference_with_string_reference() -> Result<()> {
        let expected = "B";
        let actual: OsString = "A".into();
        verify_that!(&actual, le(expected))
    }

    #[test]
    fn le_matches_ipv6addr_with_ipaddr() -> Result<()> {
        use std::net::IpAddr;
        use std::net::Ipv6Addr;
        let actual: IpAddr = "127.0.0.1".parse().unwrap();
        let expected: Ipv6Addr = "2001:4860:4860::8844".parse().unwrap();
        verify_that!(actual, le(expected))
    }

    #[test]
    fn le_matches_with_custom_partial_ord() -> Result<()> {
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

        let actual = VeryLowNumber {};
        let expected: u32 = 42;

        verify_that!(actual, le(expected))
    }
}
