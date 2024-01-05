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

/// Matches a value less (in the sense of `<`) than `expected`.
///
/// The types of `ActualT` of `actual` and `ExpectedT` of `expected` must be
/// comparable via the `PartialOrd` trait. Namely, `ActualT` must implement
/// `PartialOrd<ExpectedT>`.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(1, lt(2))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(2, lt(2))?; // Fails
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
/// verify_that!(123u32, lt(0u64))?; // Does not compile
/// verify_that!(123u32 as u64, lt(100000000u64))?; // Passes
/// #     Ok(())
/// # }
/// ```
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// # fn should_not_compile() -> Result<()> {
/// let actual: &u32 = &2;
/// let expected: u32 = 70;
/// verify_that!(actual, lt(expected))?; // Does not compile
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let actual: &u32 = &2;
/// let expected: u32 = 70;
/// verify_that!(actual, lt(&expected))?; // Compiles and passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// You can find the standard library `PartialOrd` implementation in
/// <https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html#implementors>
pub fn lt<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug>(
    expected: ExpectedT,
) -> impl Matcher<ActualT = ActualT> {
    LtMatcher::<ActualT, _> { expected, phantom: Default::default() }
}

struct LtMatcher<ActualT, ExpectedT> {
    expected: ExpectedT,
    phantom: PhantomData<ActualT>,
}

impl<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug> Matcher
    for LtMatcher<ActualT, ExpectedT>
{
    type ActualT = ActualT;

    fn matches(&self, actual: &ActualT) -> MatcherResult {
        (*actual < self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("is less than {:?}", self.expected).into(),
            MatcherResult::NoMatch => {
                format!("is greater than or equal to {:?}", self.expected).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lt;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;
    use std::ffi::OsString;

    #[test]
    fn lt_matches_i32_with_i32() -> Result<()> {
        let actual: i32 = 10000;
        let expected: i32 = 20000;
        verify_that!(actual, lt(expected))
    }

    #[test]
    fn lt_does_not_match_equal_i32() -> Result<()> {
        let matcher = lt(10);
        let result = matcher.matches(&10);
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn lt_does_not_match_lower_i32() -> Result<()> {
        let matcher = lt(-50);
        let result = matcher.matches(&50);
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn lt_matches_lesser_str() -> Result<()> {
        verify_that!("A", lt("B"))
    }

    #[test]
    fn lt_does_not_match_bigger_str() -> Result<()> {
        let matcher = lt("ab");
        let result = matcher.matches(&"az");
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn lt_mismatch_contains_actual_and_expected() -> Result<()> {
        let result = verify_that!(481, lt(45));
        let formatted_message = format!("{}", result.unwrap_err());

        verify_that!(
            formatted_message.as_str(),
            contains_substring(indoc!(
                "
                Value of: 481
                Expected: is less than 45
                Actual: 481,
                  which is greater than or equal to 45
                "
            ))
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
    #[test]
    fn lt_matches_owned_osstring_reference_with_string_reference() -> Result<()> {
        let expected = "C";
        let actual: OsString = "B".to_string().into();
        verify_that!(&actual, lt(expected))
    }

    #[test]
    fn lt_matches_ipv6addr_with_ipaddr() -> Result<()> {
        use std::net::IpAddr;
        use std::net::Ipv6Addr;
        let actual: IpAddr = "127.0.0.1".parse().unwrap();
        let expected: Ipv6Addr = "2001:4860:4860::8844".parse().unwrap();
        verify_that!(actual, lt(expected))
    }

    #[test]
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
