// Copyright 2023 Google LLC
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

/// Matches a byte sequence which is a UTF-8 encoded string matched by `inner`.
///
/// The matcher reports no match if either the string is not UTF-8 encoded or if
/// `inner` does not match on the decoded string.
///
/// The input may be a slice `&[u8]` or a `Vec` of bytes.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let bytes: &[u8] = "A string".as_bytes();
/// verify_that!(bytes, is_utf8_string(eq("A string")))?; // Passes
/// let bytes: Vec<u8> = "A string".as_bytes().to_vec();
/// verify_that!(bytes, is_utf8_string(eq("A string")))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// # let bytes: &[u8] = "A string".as_bytes();
/// verify_that!(bytes, is_utf8_string(eq("Another string")))?; // Fails (inner matcher does not match)
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// let bytes: Vec<u8> = vec![255, 64, 128, 32];
/// verify_that!(bytes, is_utf8_string(anything()))?; // Fails (not UTF-8 encoded)
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// ```
pub fn is_utf8_string<'a, ActualT: AsRef<[u8]> + Debug + 'a, InnerMatcherT>(
    inner: InnerMatcherT,
) -> impl Matcher<ActualT = ActualT>
where
    InnerMatcherT: Matcher<ActualT = String>,
{
    IsEncodedStringMatcher { inner, phantom: Default::default() }
}

struct IsEncodedStringMatcher<ActualT, InnerMatcherT> {
    inner: InnerMatcherT,
    phantom: PhantomData<ActualT>,
}

impl<'a, ActualT: AsRef<[u8]> + Debug + 'a, InnerMatcherT> Matcher
    for IsEncodedStringMatcher<ActualT, InnerMatcherT>
where
    InnerMatcherT: Matcher<ActualT = String>,
{
    type ActualT = ActualT;

    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
        String::from_utf8(actual.as_ref().to_vec())
            .map(|s| self.inner.matches(&s))
            .unwrap_or(MatcherResult::NoMatch)
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!(
                "is a UTF-8 encoded string which {}",
                self.inner.describe(MatcherResult::Match)
            )
            .into(),
            MatcherResult::NoMatch => format!(
                "is not a UTF-8 encoded string which {}",
                self.inner.describe(MatcherResult::Match)
            )
            .into(),
        }
    }

    fn explain_match(&self, actual: &Self::ActualT) -> Description {
        match String::from_utf8(actual.as_ref().to_vec()) {
            Ok(s) => {
                format!("which is a UTF-8 encoded string {}", self.inner.explain_match(&s)).into()
            }
            Err(e) => format!("which is not a UTF-8 encoded string: {e}").into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::MatcherResult;
    use crate::prelude::*;

    #[test]
    fn matches_string_as_byte_slice() -> Result<()> {
        verify_that!("A string".as_bytes(), is_utf8_string(eq("A string")))
    }

    #[test]
    fn matches_string_as_byte_vec() -> Result<()> {
        verify_that!("A string".as_bytes().to_vec(), is_utf8_string(eq("A string")))
    }

    #[test]
    fn matches_string_with_utf_8_encoded_sequences() -> Result<()> {
        verify_that!("äöüÄÖÜ".as_bytes().to_vec(), is_utf8_string(eq("äöüÄÖÜ")))
    }

    #[test]
    fn does_not_match_non_equal_string() -> Result<()> {
        verify_that!("äöüÄÖÜ".as_bytes().to_vec(), not(is_utf8_string(eq("A string"))))
    }

    #[test]
    fn does_not_match_non_utf_8_encoded_byte_sequence() -> Result<()> {
        verify_that!(&[192, 64, 255, 32], not(is_utf8_string(eq("A string"))))
    }

    #[test]
    fn has_correct_description_in_matched_case() -> Result<()> {
        let matcher = is_utf8_string::<&[u8], _>(eq("A string"));

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq("is a UTF-8 encoded string which is equal to \"A string\""))
        )
    }

    #[test]
    fn has_correct_description_in_not_matched_case() -> Result<()> {
        let matcher = is_utf8_string::<&[u8], _>(eq("A string"));

        verify_that!(
            matcher.describe(MatcherResult::NoMatch),
            displays_as(eq("is not a UTF-8 encoded string which is equal to \"A string\""))
        )
    }

    #[test]
    fn has_correct_explanation_in_matched_case() -> Result<()> {
        let explanation = is_utf8_string(eq("A string")).explain_match(&"A string".as_bytes());

        verify_that!(
            explanation,
            displays_as(eq("which is a UTF-8 encoded string which is equal to \"A string\""))
        )
    }

    #[test]
    fn has_correct_explanation_when_byte_array_is_not_utf8_encoded() -> Result<()> {
        let explanation = is_utf8_string(eq("A string")).explain_match(&&[192, 128, 0, 64]);

        verify_that!(explanation, displays_as(starts_with("which is not a UTF-8 encoded string: ")))
    }

    #[test]
    fn has_correct_explanation_when_inner_matcher_does_not_match() -> Result<()> {
        let explanation =
            is_utf8_string(eq("A string")).explain_match(&"Another string".as_bytes());

        verify_that!(
            explanation,
            displays_as(eq("which is a UTF-8 encoded string which isn't equal to \"A string\""))
        )
    }
}
