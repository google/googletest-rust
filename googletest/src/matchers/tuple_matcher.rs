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

/// Functions for use only by the declarative macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::matcher::{Matcher, MatcherResult};
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

    // This implementation is provided for completeness, but is completely trivial.
    // The only actual value which can be supplied is (), which must match.
    impl Matcher for () {
        type ActualT = ();

        fn matches(&self, _: &Self::ActualT) -> MatcherResult {
            MatcherResult::Matches
        }

        fn describe(&self, matcher_result: MatcherResult) -> String {
            match matcher_result {
                MatcherResult::Matches => "is the empty tuple".into(),
                MatcherResult::DoesNotMatch => "is not the empty tuple".into(),
            }
        }
    }

    /// Generates a tuple matcher for tuples of a specific length.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    macro_rules! tuple_matcher_n {
        ($([$field_number:tt, $matcher_type:ident, $field_type:ident]),*) => {
            impl<$($field_type: Debug, $matcher_type: Matcher<ActualT = $field_type>),*>
                Matcher for ($($matcher_type,)*)
            {
                type ActualT = ($($field_type,)*);

                fn matches(&self, actual: &($($field_type,)*)) -> MatcherResult {
                    $(match self.$field_number.matches(&actual.$field_number) {
                        MatcherResult::Matches => {},
                        MatcherResult::DoesNotMatch => {
                            return MatcherResult::DoesNotMatch;
                        }
                    })*
                    MatcherResult::Matches
                }

                fn explain_match(&self, actual: &($($field_type,)*)) -> String {
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
                    (explanation)
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

    tuple_matcher_n!([0, I0, T0]);

    tuple_matcher_n!([0, I0, T0], [1, I1, T1]);

    tuple_matcher_n!([0, I0, T0], [1, I1, T1], [2, I2, T2]);

    tuple_matcher_n!([0, I0, T0], [1, I1, T1], [2, I2, T2], [3, I3, T3]);

    tuple_matcher_n!([0, I0, T0], [1, I1, T1], [2, I2, T2], [3, I3, T3], [4, I4, T4]);

    tuple_matcher_n!([0, I0, T0], [1, I1, T1], [2, I2, T2], [3, I3, T3], [4, I4, T4], [5, I5, T5]);

    tuple_matcher_n!(
        [0, I0, T0],
        [1, I1, T1],
        [2, I2, T2],
        [3, I3, T3],
        [4, I4, T4],
        [5, I5, T5],
        [6, I6, T6]
    );

    tuple_matcher_n!(
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
