// Copyright 2026 Google LLC
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

use crate::description::Description;
use crate::matcher::{Matcher, MatcherBase, MatcherResult};
use std::fmt::Debug;

/// Matches a reference or raw pointer that points to the same memory location
/// as `expected`.
///
/// This compares the memory addresses of the actual and expected references
/// or raw pointers using `std::ptr::eq`.
///
/// Note: For fat pointers (e.g., slices or trait objects), both the data
/// address and the metadata (length/vtable) must match.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let value = 123;
/// verify_that!(&value, ptr_eq(&value))?; // Passes
///
/// let const_ptr: *const i32 = &value;
/// verify_that!(const_ptr, ptr_eq(&value))?; // Passes
///
/// let mut mut_value = 123;
/// let mut_ptr: *mut i32 = &mut mut_value;
/// verify_that!(mut_ptr, ptr_eq(&mut_value))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// let value1 = 123;
/// let value2 = 123;
/// verify_that!(&value1, ptr_eq(&value2))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
pub fn ptr_eq<T: Debug + ?Sized>(expected: &T) -> PtrEqMatcher<'_, T> {
    PtrEqMatcher { expected }
}

/// A matcher which matches a reference pointing to the same memory location as
/// `expected`.
///
/// See [`ptr_eq`].
#[derive(MatcherBase)]
pub struct PtrEqMatcher<'a, T: ?Sized> {
    expected: &'a T,
}

impl<'a, T: Debug + ?Sized> PtrEqMatcher<'a, T> {
    fn describe_impl(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!(
                "points to the same memory location as {:?} ({:p})",
                self.expected, self.expected
            )
            .into(),
            MatcherResult::NoMatch => format!(
                "does not point to the same memory location as {:?} ({:p})",
                self.expected, self.expected
            )
            .into(),
        }
    }

    fn explain_match_impl(&self, actual_addr: *const T) -> Description {
        format!("which points to {:p} (expected points to {:p})", actual_addr, self.expected).into()
    }
}

impl<'a, 'b, T: Debug + ?Sized> Matcher<&'b T> for PtrEqMatcher<'a, T> {
    fn matches(&self, actual: &'b T) -> MatcherResult {
        std::ptr::eq(actual, self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.describe_impl(matcher_result)
    }

    fn explain_match(&self, actual: &'b T) -> Description {
        self.explain_match_impl(actual as *const T)
    }
}

impl<'a, 'b, 'c, T: Debug + ?Sized> Matcher<&'b &'c T> for PtrEqMatcher<'a, T> {
    fn matches(&self, actual: &'b &'c T) -> MatcherResult {
        std::ptr::eq(*actual, self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.describe_impl(matcher_result)
    }

    fn explain_match(&self, actual: &'b &'c T) -> Description {
        self.explain_match_impl(*actual as *const T)
    }
}

impl<'a, 'b, 'c, T: Debug + ?Sized> Matcher<&'b &'c mut T> for PtrEqMatcher<'a, T> {
    fn matches(&self, actual: &'b &'c mut T) -> MatcherResult {
        std::ptr::eq(*actual, self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.describe_impl(matcher_result)
    }

    fn explain_match(&self, actual: &'b &'c mut T) -> Description {
        self.explain_match_impl(*actual as *const T)
    }
}

impl<'a, T: Debug + ?Sized> Matcher<*const T> for PtrEqMatcher<'a, T> {
    fn matches(&self, actual: *const T) -> MatcherResult {
        std::ptr::eq(actual, self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.describe_impl(matcher_result)
    }

    fn explain_match(&self, actual: *const T) -> Description {
        self.explain_match_impl(actual)
    }
}

impl<'a, T: Debug + ?Sized> Matcher<*mut T> for PtrEqMatcher<'a, T> {
    fn matches(&self, actual: *mut T) -> MatcherResult {
        std::ptr::eq(actual as *const T, self.expected as *const T).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        self.describe_impl(matcher_result)
    }

    fn explain_match(&self, actual: *mut T) -> Description {
        self.explain_match_impl(actual as *const T)
    }
}

#[cfg(test)]
mod tests {
    use super::ptr_eq;
    use crate::prelude::*;
    use crate::Result;

    #[test]
    fn ptr_eq_matches_same_reference() -> Result<()> {
        let value = 123;
        verify_that!(&value, ptr_eq(&value))
    }

    #[test]
    fn ptr_eq_does_not_match_different_reference_with_same_value() -> Result<()> {
        let value1 = 123;
        let value2 = 123;
        verify_that!(&value1, not(ptr_eq(&value2)))
    }

    #[test]
    fn ptr_eq_matches_slice_references() -> Result<()> {
        let arr = [1, 2, 3];
        let slice: &[i32] = &arr;
        verify_that!(slice, ptr_eq(slice))
    }

    #[test]
    fn ptr_eq_does_not_match_different_slices_of_same_array() -> Result<()> {
        let arr = [1, 2, 3, 4];
        let slice1: &[i32] = &arr[0..2];
        let slice2: &[i32] = &arr[1..3];
        verify_that!(slice1, not(ptr_eq(slice2)))
    }

    #[test]
    fn match_explanation_shows_memory_addresses() -> Result<()> {
        let value1 = 123;
        let value2 = 123;
        let result = verify_that!(&value1, ptr_eq(&value2));

        verify_that!(
            result,
            err(displays_as(contains_regex(
                r"which points to 0x[0-9a-fA-F]+ \(expected points to 0x[0-9a-fA-F]+\)"
            )))
        )
    }

    #[test]
    fn ptr_eq_works_with_elements_are() -> Result<()> {
        let n0 = 17;
        let n1 = 19;

        fn foo<'a>(a: &'a i32, b: &'a i32) -> Vec<&'a i32> {
            vec![a, b]
        }
        let results = foo(&n0, &n1);

        verify_that!(results, elements_are![ptr_eq(&n0), ptr_eq(&n1)])
    }

    #[test]
    fn ptr_eq_matches_same_string_reference() -> Result<()> {
        let s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        verify_that!(r1, ptr_eq(r2))
    }

    #[test]
    fn ptr_eq_matches_same_substring_reference() -> Result<()> {
        let s = "hello world";
        let sub1 = &s[0..5];
        let sub2 = &s[0..5];
        verify_that!(sub1, ptr_eq(sub2))
    }

    #[test]
    fn ptr_eq_matches_same_const_ptr() -> Result<()> {
        let value = 123;
        let ptr: *const i32 = &value;
        verify_that!(ptr, ptr_eq(&value))
    }

    #[test]
    fn ptr_eq_matches_same_mut_ptr() -> Result<()> {
        let mut value = 123;
        let ptr: *mut i32 = &mut value;
        verify_that!(ptr, ptr_eq(&value))
    }

    #[test]
    fn ptr_eq_matches_same_const_ptr_array() -> Result<()> {
        let arr = [1, 2, 3];
        let ptr: *const [i32; 3] = &arr;
        verify_that!(ptr, ptr_eq(&arr))
    }

    #[test]
    fn ptr_eq_matches_same_const_ptr_slice() -> Result<()> {
        let arr = [1, 2, 3];
        let slice: &[i32] = &arr;
        let ptr: *const [i32] = slice;
        verify_that!(ptr, ptr_eq(slice))
    }

    #[test]
    fn ptr_eq_matches_same_const_ptr_string() -> Result<()> {
        let s = String::from("hello");
        let ptr: *const String = &s;
        verify_that!(ptr, ptr_eq(&s))
    }

    #[test]
    fn ptr_eq_matches_same_const_ptr_substring() -> Result<()> {
        let s = "hello world";
        let sub: &str = &s[0..5];
        let ptr: *const str = sub;
        verify_that!(ptr, ptr_eq(sub))
    }

    #[test]
    fn ptr_eq_failure_message_shows_expected_address() -> Result<()> {
        let value1 = 123;
        let value2 = 123;
        let result = verify_that!(&value1, ptr_eq(&value2));

        verify_that!(
            result,
            err(displays_as(contains_regex(
                r"Expected: points to the same memory location as 123 \(0x[0-9a-fA-F]+\)"
            )))
        )
    }

    #[test]
    fn ptr_eq_not_failure_message_shows_expected_address() -> Result<()> {
        let value = 123;
        let result = verify_that!(&value, not(ptr_eq(&value)));

        verify_that!(
            result,
            err(displays_as(contains_regex(
                r"Expected: does not point to the same memory location as 123 \(0x[0-9a-fA-F]+\)"
            )))
        )
    }

    #[test]
    fn ptr_eq_matches_mut_reference() -> Result<()> {
        let mut value = 123;
        let mut_ref = &mut value;
        let expected = &*mut_ref;
        verify_that!(mut_ref, ptr_eq(expected))
    }
}
