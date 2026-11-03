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
    matcher::{Matcher, MatcherBase, MatcherResult},
};
use std::fmt::Debug;

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
pub fn is_utf8_string<InnerMatcherT>(inner: InnerMatcherT) -> IsEncodedStringMatcher<InnerMatcherT>
where
    InnerMatcherT: for<'a> Matcher<&'a str>,
{
    IsEncodedStringMatcher { inner }
}

#[derive(MatcherBase)]
pub struct IsEncodedStringMatcher<InnerMatcherT> {
    inner: InnerMatcherT,
}

impl<ActualT: AsRef<[u8]> + Debug + Copy, InnerMatcherT> Matcher<ActualT>
    for IsEncodedStringMatcher<InnerMatcherT>
where
    InnerMatcherT: for<'a> Matcher<&'a str>,
{
    fn matches(&self, actual: ActualT) -> MatcherResult {
        std::str::from_utf8(actual.as_ref())
            .map(|s| self.inner.matches(s))
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

    fn explain_match(&self, actual: ActualT) -> Description {
        match std::str::from_utf8(actual.as_ref()) {
            Ok(s) => {
                format!("which is a UTF-8 encoded string {}", self.inner.explain_match(s)).into()
            }
            Err(e) => format!("which is not a UTF-8 encoded string: {e}").into(),
        }
    }

    fn format_actual(&self, actual: ActualT) -> String {
        match std::str::from_utf8(actual.as_ref()) {
            Ok(s) => s.to_owned(),
            Err(_) => format!("{actual:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::MatcherResult;
    use crate::prelude::*;
    use crate::Result;
    use indoc::indoc;
    use std::collections::HashMap;
    use std::rc::Rc;

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
        let matcher = is_utf8_string(eq("A string"));

        verify_that!(
            Matcher::<&[u8]>::describe(&matcher, MatcherResult::Match),
            displays_as(eq("is a UTF-8 encoded string which is equal to \"A string\""))
        )
    }

    #[test]
    fn has_correct_description_in_not_matched_case() -> Result<()> {
        let matcher = is_utf8_string(eq("A string"));

        verify_that!(
            Matcher::<&[u8]>::describe(&matcher, MatcherResult::NoMatch),
            displays_as(eq("is not a UTF-8 encoded string which is equal to \"A string\""))
        )
    }

    #[test]
    fn has_correct_explanation_in_matched_case() -> Result<()> {
        let explanation = is_utf8_string(eq("A string")).explain_match("A string".as_bytes());

        verify_that!(
            explanation,
            displays_as(eq("which is a UTF-8 encoded string which is equal to \"A string\""))
        )
    }

    #[test]
    fn has_correct_explanation_when_byte_array_is_not_utf8_encoded() -> Result<()> {
        let explanation = is_utf8_string(eq("A string")).explain_match([192, 128, 0, 64]);

        verify_that!(explanation, displays_as(starts_with("which is not a UTF-8 encoded string: ")))
    }

    #[test]
    fn has_correct_explanation_when_inner_matcher_does_not_match() -> Result<()> {
        let explanation = is_utf8_string(eq("A string")).explain_match("Another string".as_bytes());

        verify_that!(
            explanation,
            displays_as(eq("which is a UTF-8 encoded string which isn't equal to \"A string\""))
        )
    }

    #[test]
    fn formats_actual_as_utf8_in_direct_failure_message() -> Result<()> {
        let result = verify_that!("Hello".as_bytes(), is_utf8_string(eq("World")));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: \"Hello\".as_bytes()
                Expected: is a UTF-8 encoded string which is equal to \"World\"
                Actual: Hello,
                  which is a UTF-8 encoded string which isn't equal to \"World\"
                "
            ))))
        )
    }

    #[test]
    fn formats_actual_as_debug_when_not_utf8() -> Result<()> {
        let result = verify_that!(&[192u8, 128, 0, 64], is_utf8_string(eq("World")));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: &[192u8, 128, 0, 64]
                Expected: is a UTF-8 encoded string which is equal to \"World\"
                Actual: [192, 128, 0, 64],
                  which is not a UTF-8 encoded string: "
            ))))
        )
    }

    #[test]
    fn formats_actual_in_some_matcher_failure_message() -> Result<()> {
        let result = verify_that!(Some("Hello".as_bytes()), some(is_utf8_string(eq("World"))));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: Some(\"Hello\".as_bytes())
                Expected: has a value which is a UTF-8 encoded string which is equal to \"World\"
                Actual: Some(Hello),
                  which has a value
                    which is a UTF-8 encoded string which isn't equal to \"World\"
                "
            ))))
        )
    }

    #[test]
    fn formats_actual_in_ok_and_err_matcher_failure_messages() -> Result<()> {
        let ok_res: std::result::Result<&[u8], &[u8]> = Ok("Hello".as_bytes());
        let result = verify_that!(ok_res, ok(is_utf8_string(eq("World"))));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: ok_res
                Expected: is a success containing a value, which is a UTF-8 encoded string which is equal to \"World\"
                Actual: Ok(Hello),
                  which is a success
                    which is a UTF-8 encoded string which isn't equal to \"World\"
                "
            ))))
        )?;

        let err_res: std::result::Result<&[u8], &[u8]> = Err("Boom".as_bytes());
        let result = verify_that!(err_res, err(is_utf8_string(eq("ExpectedErr"))));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: err_res
                Expected: is an error which is a UTF-8 encoded string which is equal to \"ExpectedErr\"
                Actual: Err(Boom),
                  which is an error
                    which is a UTF-8 encoded string which isn't equal to \"ExpectedErr\"
                "
            ))))
        )
    }

    #[test]
    fn formats_actual_in_elements_are_failure_message() -> Result<()> {
        let result = verify_that!(
            vec!["foo".as_bytes(), "bar".as_bytes()],
            elements_are![is_utf8_string(eq("foo")), is_utf8_string(eq("baz"))]
        );

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: vec![\"foo\".as_bytes(), \"bar\".as_bytes()]
                Expected: has elements:
                  0. is a UTF-8 encoded string which is equal to \"foo\"
                  1. is a UTF-8 encoded string which is equal to \"baz\"
                Actual: [foo, bar],
                  where element #1 is bar, which is a UTF-8 encoded string which isn't equal to \"baz\"
                "
            ))))
        )
    }

    #[test]
    fn formats_actual_in_each_and_contains_failure_messages() -> Result<()> {
        let result =
            verify_that!(vec!["foo".as_bytes(), "bar".as_bytes()], each(is_utf8_string(eq("foo"))));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: vec![\"foo\".as_bytes(), \"bar\".as_bytes()]
                Expected: only contains elements that is a UTF-8 encoded string which is equal to \"foo\"
                Actual: [foo, bar],
                  whose element #1 is bar, which is a UTF-8 encoded string which isn't equal to \"foo\"
                "
            ))))
        )?;

        let result = verify_that!(
            vec!["foo".as_bytes(), "bar".as_bytes()],
            contains(is_utf8_string(eq("baz")))
        );

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: vec![\"foo\".as_bytes(), \"bar\".as_bytes()]
                Expected: contains at least one element which is a UTF-8 encoded string which is equal to \"baz\"
                Actual: [foo, bar],
                  which does not contain a matching element
                "
            ))))
        )
    }

    #[test]
    fn formats_actual_in_pointwise_failure_message() -> Result<()> {
        let result = verify_that!(
            vec!["foo".as_bytes(), "bar".as_bytes()],
            pointwise!(|s| is_utf8_string(eq(s)), vec!["foo", "baz"])
        );

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Actual: [foo, bar],
                  where element #1 is bar, which is a UTF-8 encoded string which isn't equal to \"baz\"
                "
            ))))
        )
    }

    #[test]
    fn formats_actual_in_tuple_matcher_failure_message() -> Result<()> {
        let result = verify_that!((42, "Hello".as_bytes()), (eq(42), is_utf8_string(eq("World"))));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: (42, \"Hello\".as_bytes())
                Expected: is a tuple whose values respectively match:
                  is equal to 42
                  is a UTF-8 encoded string which is equal to \"World\"
                Actual: (42, Hello),
                  which
                    is a tuple whose values do not respectively match:
                      is equal to 42
                      is a UTF-8 encoded string which is equal to \"World\"
                  Element #1 is Hello,
                    which is a UTF-8 encoded string which isn't equal to \"World\"
                "
            ))))
        )
    }

    #[test]
    fn formats_actual_in_not_and_combinator_failure_messages() -> Result<()> {
        let result = verify_that!("Secret".as_bytes(), not(is_utf8_string(eq("Secret"))));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: \"Secret\".as_bytes()
                Expected: is not a UTF-8 encoded string which is equal to \"Secret\"
                Actual: Secret,
                  which is a UTF-8 encoded string which is equal to \"Secret\"
                "
            ))))
        )?;

        let result = verify_that!(
            "Hello".as_bytes(),
            all!(is_utf8_string(starts_with("H")), is_utf8_string(ends_with("!")))
        );

        verify_that!(result, err(displays_as(contains_substring("Actual: Hello,"))))?;

        let result = verify_that!(
            "Hello".as_bytes(),
            any!(is_utf8_string(starts_with("X")), is_utf8_string(ends_with("!")))
        );

        verify_that!(result, err(displays_as(contains_substring("Actual: Hello,"))))
    }

    #[test]
    fn formats_actual_in_points_to_derefs_to_and_has_entry() -> Result<()> {
        let bytes = "Hello".as_bytes();
        let result = verify_that!(&bytes, points_to(is_utf8_string(eq("World"))));
        verify_that!(result, err(displays_as(contains_substring("Actual: Hello,"))))?;

        let rc_bytes = Rc::new("Hello".as_bytes());
        let result = verify_that!(rc_bytes, derefs_to(is_utf8_string(eq("World"))));
        verify_that!(result, err(displays_as(contains_substring("Actual: Hello,"))))?;

        let map = HashMap::from([("key", "Hello".as_bytes().to_vec())]);
        let result = verify_that!(map, has_entry("key", is_utf8_string(eq("World"))));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Actual: {\"key\": Hello},
                  which contains key \"key\", but is mapped to value Hello, which is a UTF-8 encoded string which isn't equal to \"World\"
                "
            ))))
        )
    }
}
