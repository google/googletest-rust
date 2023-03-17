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

#[cfg(not(google3))]
use googletest::matchers;
#[cfg(not(google3))]
use googletest::tuple;
use googletest::{
    google_test,
    matcher::{Matcher, MatcherResult},
    verify_that, Result,
};
use indoc::indoc;
#[cfg(google3)]
use matchers::tuple;
use matchers::{displays_as, eq, not};

#[google_test]
fn empty_matcher_matches_empty_tuple() -> Result<()> {
    verify_that!((), tuple!())
}

#[google_test]
fn singleton_matcher_matches_matching_singleton_tuple() -> Result<()> {
    verify_that!((123,), tuple!(eq(123)))
}

#[google_test]
fn singleton_matcher_with_trailing_comma_matches_matching_singleton_tuple() -> Result<()> {
    verify_that!((123,), tuple!(eq(123),))
}

#[google_test]
fn singleton_matcher_does_not_match_non_matching_singleton_tuple() -> Result<()> {
    verify_that!((123,), not(tuple!(eq(456))))
}

#[google_test]
fn pair_matcher_matches_matching_pair_tuple() -> Result<()> {
    verify_that!((123, 456), tuple!(eq(123), eq(456)))
}

#[google_test]
fn pair_matcher_matches_matching_pair_tuple_with_different_types() -> Result<()> {
    verify_that!((123, "A string"), tuple!(eq(123), eq("A string")))
}

#[google_test]
fn pair_matcher_with_trailing_comma_matches_matching_pair_tuple() -> Result<()> {
    verify_that!((123, 456), tuple!(eq(123), eq(456),))
}

#[google_test]
fn tuple_matcher_matches_matching_3_tuple() -> Result<()> {
    verify_that!((1, 2, 3), tuple!(eq(1), eq(2), eq(3)))
}

#[google_test]
fn tuple_matcher_matches_matching_4_tuple() -> Result<()> {
    verify_that!((1, 2, 3, 4), tuple!(eq(1), eq(2), eq(3), eq(4)))
}

#[google_test]
fn tuple_matcher_matches_matching_5_tuple() -> Result<()> {
    verify_that!((1, 2, 3, 4, 5), tuple!(eq(1), eq(2), eq(3), eq(4), eq(5)))
}

#[google_test]
fn tuple_matcher_matches_matching_6_tuple() -> Result<()> {
    verify_that!((1, 2, 3, 4, 5, 6), tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6)))
}

#[google_test]
fn tuple_matcher_matches_matching_7_tuple() -> Result<()> {
    verify_that!((1, 2, 3, 4, 5, 6, 7), tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7)))
}

#[google_test]
fn tuple_matcher_matches_matching_8_tuple() -> Result<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8),
        tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8))
    )
}

#[google_test]
fn tuple_matcher_matches_matching_9_tuple() -> Result<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9),
        tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9))
    )
}

#[google_test]
fn tuple_matcher_matches_matching_10_tuple() -> Result<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10),
        tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9), eq(10))
    )
}

#[google_test]
fn tuple_matcher_matches_matching_11_tuple() -> Result<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),
        tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9), eq(10), eq(11))
    )
}

#[google_test]
fn tuple_matcher_matches_matching_12_tuple() -> Result<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
        tuple!(
            eq(1),
            eq(2),
            eq(3),
            eq(4),
            eq(5),
            eq(6),
            eq(7),
            eq(8),
            eq(9),
            eq(10),
            eq(11),
            eq(12)
        )
    )
}

#[google_test]
fn tuple_matcher_with_trailing_comma_matches_matching_3_tuple() -> Result<()> {
    verify_that!((1, 2, 3), tuple!(eq(1), eq(2), eq(3),))
}

#[google_test]
fn tuple_matcher_with_trailing_comma_matches_matching_4_tuple() -> Result<()> {
    verify_that!((1, 2, 3, 4), tuple!(eq(1), eq(2), eq(3), eq(4),))
}

#[google_test]
fn tuple_matcher_with_trailing_comma_matches_matching_5_tuple() -> Result<()> {
    verify_that!((1, 2, 3, 4, 5), tuple!(eq(1), eq(2), eq(3), eq(4), eq(5),))
}

#[google_test]
fn tuple_matcher_with_trailing_comma_matches_matching_6_tuple() -> Result<()> {
    verify_that!((1, 2, 3, 4, 5, 6), tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6),))
}

#[google_test]
fn tuple_matcher_with_trailing_comma_matches_matching_7_tuple() -> Result<()> {
    verify_that!((1, 2, 3, 4, 5, 6, 7), tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7),))
}

#[google_test]
fn tuple_matcher_with_trailing_comma_matches_matching_8_tuple() -> Result<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8),
        tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8),)
    )
}

#[google_test]
fn tuple_matcher_with_trailing_comma_matches_matching_9_tuple() -> Result<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9),
        tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9),)
    )
}

#[google_test]
fn tuple_matcher_with_trailing_comma_matches_matching_10_tuple() -> Result<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10),
        tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9), eq(10),)
    )
}

#[google_test]
fn tuple_matcher_with_trailing_comma_matches_matching_11_tuple() -> Result<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),
        tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9), eq(10), eq(11),)
    )
}

#[google_test]
fn tuple_matcher_with_trailing_comma_matches_matching_12_tuple() -> Result<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
        tuple!(
            eq(1),
            eq(2),
            eq(3),
            eq(4),
            eq(5),
            eq(6),
            eq(7),
            eq(8),
            eq(9),
            eq(10),
            eq(11),
            eq(12),
        )
    )
}

#[google_test]
fn tuple_matcher_1_has_correct_description_for_match() -> Result<()> {
    verify_that!(
        tuple!(eq(1)).describe(MatcherResult::Matches),
        eq(indoc!(
            "
            is a tuple whose values respectively match:
              is equal to 1,
            "
        ))
    )
}

#[google_test]
fn tuple_matcher_1_has_correct_description_for_mismatch() -> Result<()> {
    verify_that!(
        tuple!(eq(1)).describe(MatcherResult::DoesNotMatch),
        eq(indoc!(
            "
            is a tuple whose values do not respectively match:
              is equal to 1,
            "
        ))
    )
}

#[google_test]
fn tuple_matcher_2_has_correct_description_for_match() -> Result<()> {
    verify_that!(
        tuple!(eq(1), eq(2)).describe(MatcherResult::Matches),
        eq(indoc!(
            "
            is a tuple whose values respectively match:
              is equal to 1,
              is equal to 2,
            "
        ))
    )
}

#[google_test]
fn tuple_matcher_2_has_correct_description_for_mismatch() -> Result<()> {
    verify_that!(
        tuple!(eq(1), eq(2)).describe(MatcherResult::DoesNotMatch),
        eq(indoc!(
            "
            is a tuple whose values do not respectively match:
              is equal to 1,
              is equal to 2,
            "
        ))
    )
}

#[google_test]
fn describe_match_shows_which_tuple_element_did_not_match() -> Result<()> {
    verify_that!(
        tuple!(eq(1), eq(2)).explain_match(&(1, 3)),
        displays_as(eq(indoc!(
            "
            which is a tuple whose values do not respectively match:
              is equal to 1,
              is equal to 2,
            Element #1 is 3, which isn't equal to 2
            "
        )))
    )
}

#[google_test]
fn describe_match_shows_which_two_tuple_elements_did_not_match() -> Result<()> {
    verify_that!(
        tuple!(eq(1), eq(2)).explain_match(&(2, 3)),
        displays_as(eq(indoc!(
            "
            which is a tuple whose values do not respectively match:
              is equal to 1,
              is equal to 2,
            Element #0 is 2, which isn't equal to 1
            Element #1 is 3, which isn't equal to 2
            "
        )))
    )
}
