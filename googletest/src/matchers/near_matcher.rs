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
use num_traits::{Float, FloatConst};
use std::fmt::Debug;

/// Matches a value equal within `max_abs_error` of `expected`.
///
/// The type `T` of the actual, `expected`, and `max_abs_error` values must
/// implement [`Float`].
///
/// The values `expected` and `max_abs_error` may not be NaN. The value
/// `max_abs_error` must be non-negative. The matcher panics on construction
/// otherwise.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass_1() -> Result<()> {
/// verify_that!(1.0, near(1.0, 0.1))?; // Passes
/// verify_that!(1.01, near(1.0, 0.1))?; // Passes
/// verify_that!(1.25, near(1.0, 0.25))?; // Passes
/// verify_that!(0.75, near(1.0, 0.25))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!(1.101, near(1.0, 0.1))?; // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!(0.899, near(1.0, 0.1))?; // Fails
/// #     Ok(())
/// # }
/// # fn should_pass_2() -> Result<()> {
/// verify_that!(100.25, near(100.0, 0.25))?; // Passes
/// #     Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_pass_2().unwrap();
/// ```
///
/// The default behaviour for special values is consistent with the IEEE
/// floating point standard. Thus infinity is infinitely far away from any
/// floating point value:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_fail_1() -> Result<()> {
/// verify_that!(f64::INFINITY, near(0.0, f64::MAX))?; // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!(0.0, near(f64::INFINITY, f64::MAX))?; // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_3() -> Result<()> {
/// verify_that!(f64::INFINITY, near(f64::INFINITY, f64::MAX))?; // Fails
/// #     Ok(())
/// # }
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_fail_3().unwrap_err();
/// ```
///
/// Similarly, by default, `NaN` is infinitely far away from any value:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_fail_1() -> Result<()> {
/// verify_that!(f64::NAN, near(0.0, f64::MAX))?; // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!(0.0, near(f64::NAN, f64::MAX))?; // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_3() -> Result<()> {
/// verify_that!(f64::NAN, near(f64::NAN, f64::MAX))?; // Fails
/// #     Ok(())
/// # }
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_fail_3().unwrap_err();
/// ```
///
/// To treat two `NaN` values as equal, use the method
/// [`NearMatcher::nans_are_equal`].
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(f64::NAN, near(f64::NAN, f64::MAX).nans_are_equal())?; // Passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn near<T: Debug + Float + Copy>(expected: T, max_abs_error: T) -> NearMatcher<T> {
    if max_abs_error.is_nan() {
        panic!("max_abs_error must not be NaN");
    }
    if max_abs_error < T::zero() {
        panic!("max_abs_error must be non-negative");
    }
    NearMatcher { expected, max_abs_error, nans_are_equal: false }
}

/// Matches a value approximately equal to `expected`.
///
/// This automatically computes a tolerance from the magnitude of `expected` and
/// matches any actual value within this tolerance of the expected value. The
/// tolerance is chosen to account for the inaccuracies in most ordinary
/// floating point calculations.
///
/// Otherwise this works analogously to [`near`]; see its documentation for
/// further notes.
pub fn approx_eq<T: Debug + Float + FloatConst + Copy>(expected: T) -> NearMatcher<T> {
    // The FloatConst trait doesn't offer 2 as a constant but does offer 1.
    let five_bits_of_mantissa = (T::one() + T::one()).powi(5);
    let abs_tolerance = five_bits_of_mantissa * T::epsilon();
    let max_abs_error = T::max(expected.abs() * abs_tolerance, abs_tolerance);
    NearMatcher { expected, max_abs_error, nans_are_equal: false }
}

/// A matcher which matches floating-point numbers approximately equal to its
/// expected value.
pub struct NearMatcher<T: Debug> {
    expected: T,
    max_abs_error: T,
    nans_are_equal: bool,
}

impl<T: Debug> NearMatcher<T> {
    /// Configures this instance to treat two NaNs as equal.
    ///
    /// This behaviour differs from the IEEE standad for floating point which
    /// treats two NaNs as infinitely far apart.
    pub fn nans_are_equal(mut self) -> Self {
        self.nans_are_equal = true;
        self
    }

    /// Configures this instance to treat two NaNs as not equal.
    ///
    /// This behaviour complies with the IEEE standad for floating point. It is
    /// the default behaviour for this matcher, so invoking this method is
    /// usually redunant.
    pub fn nans_are_not_equal(mut self) -> Self {
        self.nans_are_equal = false;
        self
    }
}

impl<T: Debug + Float> Matcher for NearMatcher<T> {
    type ActualT = T;

    fn matches(&self, actual: &T) -> MatcherResult {
        if self.nans_are_equal && self.expected.is_nan() && actual.is_nan() {
            return MatcherResult::Match;
        }

        let delta = *actual - self.expected;
        if delta >= -self.max_abs_error && delta <= self.max_abs_error {
            MatcherResult::Match
        } else {
            MatcherResult::NoMatch
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("is within {:?} of {:?}", self.max_abs_error, self.expected).into()
            }
            MatcherResult::NoMatch => {
                format!("isn't within {:?} of {:?}", self.max_abs_error, self.expected).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{approx_eq, near};
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;

    #[test]
    fn matches_value_inside_range() -> Result<()> {
        let matcher = near(1.0f64, 0.1f64);

        let result = matcher.matches(&1.0f64);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn matches_value_at_low_end_of_range() -> Result<()> {
        let matcher = near(1.0f64, 0.1f64);

        let result = matcher.matches(&0.9f64);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn matches_value_at_high_end_of_range() -> Result<()> {
        let matcher = near(1.0f64, 0.25f64);

        let result = matcher.matches(&1.25f64);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn does_not_match_value_below_low_end_of_range() -> Result<()> {
        let matcher = near(1.0f64, 0.1f64);

        let result = matcher.matches(&0.899999f64);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn does_not_match_value_above_high_end_of_range() -> Result<()> {
        let matcher = near(1.0f64, 0.1f64);

        let result = matcher.matches(&1.100001f64);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn nan_is_not_near_a_number() -> Result<()> {
        let matcher = near(0.0f64, f64::MAX);

        let result = matcher.matches(&f64::NAN);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn nan_is_not_near_nan_by_default() -> Result<()> {
        verify_that!(f64::NAN, not(near(f64::NAN, f64::MAX)))
    }

    #[test]
    fn nan_is_not_near_nan_when_explicitly_configured() -> Result<()> {
        verify_that!(f64::NAN, not(near(f64::NAN, f64::MAX).nans_are_not_equal()))
    }

    #[test]
    fn nan_is_near_nan_if_nans_are_equal() -> Result<()> {
        verify_that!(f64::NAN, near(f64::NAN, f64::MAX).nans_are_equal())
    }

    #[test]
    fn nan_is_not_near_number_when_nans_are_equal() -> Result<()> {
        verify_that!(f64::NAN, not(near(0.0, f64::MAX).nans_are_equal()))
    }

    #[test]
    fn number_is_not_near_nan_when_nans_are_equal() -> Result<()> {
        verify_that!(0.0, not(near(f64::NAN, f64::MAX).nans_are_equal()))
    }

    #[test]
    fn inf_is_not_near_inf() -> Result<()> {
        let matcher = near(f64::INFINITY, f64::MAX);

        let result = matcher.matches(&f64::INFINITY);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn inf_is_not_near_a_number() -> Result<()> {
        let matcher = near(f64::INFINITY, f64::MAX);

        let result = matcher.matches(&f64::MIN);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn any_two_numbers_are_within_inf_of_each_other() -> Result<()> {
        let matcher = near(f64::MIN, f64::INFINITY);

        let result = matcher.matches(&f64::MAX);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[::core::prelude::v1::test]
    #[should_panic]
    fn panics_if_max_abs_error_is_nan() {
        near(0.0, f64::NAN);
    }

    #[::core::prelude::v1::test]
    #[should_panic]
    fn panics_if_tolerance_is_negative() {
        near(0.0, -1.0);
    }

    #[test]
    fn approx_eq_matches_equal_number() -> Result<()> {
        verify_that!(1.0f64, approx_eq(1.0f64))
    }

    #[test]
    fn approx_eq_matches_really_close_f64_number() -> Result<()> {
        verify_that!(1.0f64, approx_eq(1.0 + 16.0 * f64::EPSILON))
    }

    #[test]
    fn approx_eq_matches_really_close_f64_number_to_large_number() -> Result<()> {
        verify_that!(1000f64, approx_eq(1000.0 + 16000.0 * f64::EPSILON))
    }

    #[test]
    fn approx_eq_matches_really_close_f64_number_to_zero() -> Result<()> {
        verify_that!(16.0 * f64::EPSILON, approx_eq(0.0))
    }

    #[test]
    fn approx_eq_matches_really_close_f32_number() -> Result<()> {
        verify_that!(1.0f32, approx_eq(1.0 + 16.0 * f32::EPSILON))
    }

    #[test]
    fn approx_eq_does_not_match_distant_number() -> Result<()> {
        verify_that!(0.0f64, not(approx_eq(1.0f64)))
    }
}
