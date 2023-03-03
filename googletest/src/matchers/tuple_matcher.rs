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

// There are no visible documentation elements in this module; the declarative
// macro is documented at the top level.
#![doc(hidden)]

/// Matches a tuple whose elements are matched by each of the given matchers.
///
/// ```
/// verify_that((123, 456), tuple!(eq(123), eq(456)))?; // Passes
/// verify_that((123, 456), tuple!(eq(123), eq(0)))?; // Fails: second matcher does not match
/// ```
///
/// Matchers must correspond to the actual tuple in count and type. Otherwise
/// the test will fail to compile.
///
/// ```
/// verify_that((123, 456), tuple!(eq(123)))?; // Does not compile: wrong tuple size
/// verify_that((123, "A string"), tuple!(eq(123), eq(456)))?; // Does not compile: wrong type
/// ```
///
/// All fields must be covered by matchers. Use
/// [`anything`][crate::matchers::anything] for fields which are not relevant
/// for the test.
///
/// ```
/// verify_that((123, 456), tuple!(eq(123), anything()))
/// ```
///
/// This supports tuples of up to 12 elements. Tuples longer than that do not
/// automatically inherit the `Debug` trait from their members, so are generally
/// not supported; see [Rust by Example](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html#tuples).
///
/// This macro is the analogue of [`matches_pattern`][crate::matches_pattern]
/// for tuples. To match on fields of tuple structs, structs, and enums, use
/// [`matches_pattern`][crate::matches_pattern].
#[macro_export]
macro_rules! tuple {
    ($($t:tt)*) => { $crate::tuple_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! tuple_internal {
    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr,
        $matcher4:expr,
        $matcher5:expr,
        $matcher6:expr,
        $matcher7:expr,
        $matcher8:expr,
        $matcher9:expr,
        $matcher10:expr,
        $matcher11:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher12;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher12;
        TupleMatcher12(
            $matcher0, $matcher1, $matcher2, $matcher3, $matcher4, $matcher5, $matcher6, $matcher7,
            $matcher8, $matcher9, $matcher10, $matcher11,
        )
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr,
        $matcher4:expr,
        $matcher5:expr,
        $matcher6:expr,
        $matcher7:expr,
        $matcher8:expr,
        $matcher9:expr,
        $matcher10:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher11;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher11;
        TupleMatcher11(
            $matcher0, $matcher1, $matcher2, $matcher3, $matcher4, $matcher5, $matcher6, $matcher7,
            $matcher8, $matcher9, $matcher10,
        )
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr,
        $matcher4:expr,
        $matcher5:expr,
        $matcher6:expr,
        $matcher7:expr,
        $matcher8:expr,
        $matcher9:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher10;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher10;
        TupleMatcher10(
            $matcher0, $matcher1, $matcher2, $matcher3, $matcher4, $matcher5, $matcher6, $matcher7,
            $matcher8, $matcher9,
        )
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr,
        $matcher4:expr,
        $matcher5:expr,
        $matcher6:expr,
        $matcher7:expr,
        $matcher8:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher9;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher9;
        TupleMatcher9(
            $matcher0, $matcher1, $matcher2, $matcher3, $matcher4, $matcher5, $matcher6, $matcher7,
            $matcher8,
        )
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr,
        $matcher4:expr,
        $matcher5:expr,
        $matcher6:expr,
        $matcher7:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher8;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher8;
        TupleMatcher8(
            $matcher0, $matcher1, $matcher2, $matcher3, $matcher4, $matcher5, $matcher6, $matcher7,
        )
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr,
        $matcher4:expr,
        $matcher5:expr,
        $matcher6:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher7;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher7;
        TupleMatcher7($matcher0, $matcher1, $matcher2, $matcher3, $matcher4, $matcher5, $matcher6)
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr,
        $matcher4:expr,
        $matcher5:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher6;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher6;
        TupleMatcher6($matcher0, $matcher1, $matcher2, $matcher3, $matcher4, $matcher5)
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr,
        $matcher4:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher5;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher5;
        TupleMatcher5($matcher0, $matcher1, $matcher2, $matcher3, $matcher4)
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher4;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher4;
        TupleMatcher4($matcher0, $matcher1, $matcher2, $matcher3)
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher3;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher3;
        TupleMatcher3($matcher0, $matcher1, $matcher2)
    }};

    (
        $matcher0:expr,
        $matcher1:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher2;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher2;
        TupleMatcher2($matcher0, $matcher1)
    }};

    (
        $matcher0:expr $(,)?
    ) => {{
        #[cfg(google3)]
        use $crate::internal::TupleMatcher1;
        #[cfg(not(google3))]
        use $crate::matchers::tuple_matcher::internal::TupleMatcher1;
        TupleMatcher1($matcher0)
    }};

    () => {{
        #[cfg(google3)]
        use eq_matcher::eq;
        #[cfg(not(google3))]
        use $crate::matchers::eq;
        eq(())
    }};
}

/// Functions for use only by the declarative macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    #[cfg(not(google3))]
    use crate as googletest;
    use googletest::matcher::{MatchExplanation, Matcher, MatcherResult};
    use std::fmt::{Debug, Write};

    /// Replaces the first expression with the second at compile time.
    ///
    /// This is used below in repetition sequences where the output must only
    /// include the same expression repeated the same number of times as the
    /// macro input.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    macro_rules! replace_expr {
        ($_ignored:tt, $replacement:expr) => {
            $replacement
        };
    }

    /// Generates a tuple matcher for tuples of a specific length.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    macro_rules! tuple_matcher_n {
        ($name:ident, $([$field_number:tt, $matcher_type:ident, $field_type:ident]),*) => {
            #[doc(hidden)]
            pub struct $name<$($matcher_type),*>($(pub $matcher_type),*);

            impl<$($field_type: Debug, $matcher_type: Matcher<$field_type>),*>
                Matcher<($($field_type,)*)> for $name<$($matcher_type),*>
            {
                fn matches(&self, actual: &($($field_type,)*)) -> MatcherResult {
                    $(match self.$field_number.matches(&actual.$field_number) {
                        MatcherResult::Matches => {},
                        MatcherResult::DoesNotMatch => {
                            return MatcherResult::DoesNotMatch;
                        }
                    })*
                    MatcherResult::Matches
                }

                fn explain_match(&self, actual: &($($field_type,)*)) -> MatchExplanation {
                    let mut explanation = format!("which {}", self.describe(self.matches(actual)));
                    $(match self.$field_number.matches(&actual.$field_number) {
                        MatcherResult::Matches => {},
                        MatcherResult::DoesNotMatch => {
                            writeln!(
                                &mut explanation,
                                concat!("Element #", $field_number, " is {:?}, {}"),
                                actual.$field_number,
                                self.$field_number.explain_match(&actual.$field_number)
                            ).unwrap();
                        }
                    })*
                    MatchExplanation::create(explanation)
                }

                fn describe(&self, matcher_result: MatcherResult) -> String {
                    match matcher_result {
                        MatcherResult::Matches => {
                            format!(
                                concat!(
                                    "is a tuple whose values respectively match:\n",
                                    $(replace_expr!($field_number, "  {},\n")),*
                                ),
                                $(self.$field_number.describe(matcher_result)),*
                            )
                        }
                        MatcherResult::DoesNotMatch => {
                            format!(
                                concat!(
                                    "is a tuple whose values do not respectively match:\n",
                                    $(replace_expr!($field_number, "  {},\n")),*
                                ),
                                $(self.$field_number.describe(MatcherResult::Matches)),*
                            )
                        }
                    }
                }
            }
        };
    }

    tuple_matcher_n!(TupleMatcher1, [0, I0, T0]);

    tuple_matcher_n!(TupleMatcher2, [0, I0, T0], [1, I1, T1]);

    tuple_matcher_n!(TupleMatcher3, [0, I0, T0], [1, I1, T1], [2, I2, T2]);

    tuple_matcher_n!(TupleMatcher4, [0, I0, T0], [1, I1, T1], [2, I2, T2], [3, I3, T3]);

    tuple_matcher_n!(
        TupleMatcher5,
        [0, I0, T0],
        [1, I1, T1],
        [2, I2, T2],
        [3, I3, T3],
        [4, I4, T4]
    );

    tuple_matcher_n!(
        TupleMatcher6,
        [0, I0, T0],
        [1, I1, T1],
        [2, I2, T2],
        [3, I3, T3],
        [4, I4, T4],
        [5, I5, T5]
    );

    tuple_matcher_n!(
        TupleMatcher7,
        [0, I0, T0],
        [1, I1, T1],
        [2, I2, T2],
        [3, I3, T3],
        [4, I4, T4],
        [5, I5, T5],
        [6, I6, T6]
    );

    tuple_matcher_n!(
        TupleMatcher8,
        [0, I0, T0],
        [1, I1, T1],
        [2, I2, T2],
        [3, I3, T3],
        [4, I4, T4],
        [5, I5, T5],
        [6, I6, T6],
        [7, I7, T7]
    );

    tuple_matcher_n!(
        TupleMatcher9,
        [0, I0, T0],
        [1, I1, T1],
        [2, I2, T2],
        [3, I3, T3],
        [4, I4, T4],
        [5, I5, T5],
        [6, I6, T6],
        [7, I7, T7],
        [8, I8, T8]
    );

    tuple_matcher_n!(
        TupleMatcher10,
        [0, I0, T0],
        [1, I1, T1],
        [2, I2, T2],
        [3, I3, T3],
        [4, I4, T4],
        [5, I5, T5],
        [6, I6, T6],
        [7, I7, T7],
        [8, I8, T8],
        [9, I9, T9]
    );

    tuple_matcher_n!(
        TupleMatcher11,
        [0, I0, T0],
        [1, I1, T1],
        [2, I2, T2],
        [3, I3, T3],
        [4, I4, T4],
        [5, I5, T5],
        [6, I6, T6],
        [7, I7, T7],
        [8, I8, T8],
        [9, I9, T9],
        [10, I10, T10]
    );

    tuple_matcher_n!(
        TupleMatcher12,
        [0, I0, T0],
        [1, I1, T1],
        [2, I2, T2],
        [3, I3, T3],
        [4, I4, T4],
        [5, I5, T5],
        [6, I6, T6],
        [7, I7, T7],
        [8, I8, T8],
        [9, I9, T9],
        [10, I10, T10],
        [11, I11, T11]
    );
}

#[cfg(test)]
mod tests {
    #[cfg(not(google3))]
    use crate as googletest;
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{
        google_test,
        matcher::{Matcher, MatcherResult},
        verify_that, Result,
    };
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
        verify_that!(
            (1, 2, 3, 4, 5, 6, 7),
            tuple!(eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7),)
        )
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
            eq("\
is a tuple whose values respectively match:
  is equal to 1,
")
        )
    }

    #[google_test]
    fn tuple_matcher_1_has_correct_description_for_mismatch() -> Result<()> {
        verify_that!(
            tuple!(eq(1)).describe(MatcherResult::DoesNotMatch),
            eq("\
is a tuple whose values do not respectively match:
  is equal to 1,
")
        )
    }

    #[google_test]
    fn tuple_matcher_2_has_correct_description_for_match() -> Result<()> {
        verify_that!(
            tuple!(eq(1), eq(2)).describe(MatcherResult::Matches),
            eq("\
is a tuple whose values respectively match:
  is equal to 1,
  is equal to 2,
")
        )
    }

    #[google_test]
    fn tuple_matcher_2_has_correct_description_for_mismatch() -> Result<()> {
        verify_that!(
            tuple!(eq(1), eq(2)).describe(MatcherResult::DoesNotMatch),
            eq("\
is a tuple whose values do not respectively match:
  is equal to 1,
  is equal to 2,
")
        )
    }

    #[google_test]
    fn describe_match_shows_which_tuple_element_did_not_match() -> Result<()> {
        verify_that!(
            tuple!(eq(1), eq(2)).explain_match(&(1, 3)),
            displays_as(eq("\
which is a tuple whose values do not respectively match:
  is equal to 1,
  is equal to 2,
Element #1 is 3, which isn't equal to 2
"))
        )
    }

    #[google_test]
    fn describe_match_shows_which_two_tuple_elements_did_not_match() -> Result<()> {
        verify_that!(
            tuple!(eq(1), eq(2)).explain_match(&(2, 3)),
            displays_as(eq("\
which is a tuple whose values do not respectively match:
  is equal to 1,
  is equal to 2,
Element #0 is 2, which isn't equal to 1
Element #1 is 3, which isn't equal to 2
"))
        )
    }
}
