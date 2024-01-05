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

/// Matches a value greater than or equal to (in the sense of `>=`) `expected`.
///
/// The types of `ActualT` of `actual` and `ExpectedT` of `expected` must be
/// comparable via the `PartialOrd` trait. Namely, `ActualT` must implement
/// `PartialOrd<ExpectedT>`.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(1, ge(0))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!(0, ge(1))?; // Fails
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
/// verify_that!(123u32, ge(0u64))?; // Does not compile
/// verify_that!(123u32 as u64, ge(0u64))?; // Passes
/// #     Ok(())
/// # }
/// ```
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// # fn should_not_compile() -> Result<()> {
/// let actual: &u32 = &2;
/// let expected: u32 = 0;
/// verify_that!(actual, ge(expected))?; // Does not compile
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let actual: &u32 = &2;
/// let expected: u32 = 0;
/// verify_that!(actual, ge(&expected))?; // Compiles and passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// You can find the standard library `PartialOrd` implementation in
/// <https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html#implementors>
pub fn ge<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug>(
    expected: ExpectedT,
) -> impl Matcher<ActualT = ActualT> {
    GeMatcher::<ActualT, _> { expected, phantom: Default::default() }
}

struct GeMatcher<ActualT, ExpectedT> {
    expected: ExpectedT,
    phantom: PhantomData<ActualT>,
}

impl<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug> Matcher
    for GeMatcher<ActualT, ExpectedT>
{
    type ActualT = ActualT;

    fn matches(&self, actual: &ActualT) -> MatcherResult {
        (*actual >= self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("is greater than or equal to {:?}", self.expected).into()
            }
            MatcherResult::NoMatch => format!("is less than {:?}", self.expected).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ge;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;
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
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn ge_matches_bigger_str() -> Result<()> {
        verify_that!("B", ge("A"))
    }

    #[test]
    fn ge_does_not_match_lesser_str() -> Result<()> {
        let matcher = ge("z");
        let result = matcher.matches(&"a");
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn ge_mismatch_contains_actual_and_expected() -> Result<()> {
        let result = verify_that!(591, ge(927));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: 591
                Expected: is greater than or equal to 927
                Actual: 591,
                  which is less than 927
                "
            ))))
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
