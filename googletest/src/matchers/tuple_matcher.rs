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
/// This takes as arguments sequence of [`Matcher`][crate::matcher::Matcher]
/// corresponding to the tuple against which it matches. Each matcher is
/// applied to the corresponding tuple element.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!((123, 456), tuple!(eq(123), eq(456)))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!((123, 456), tuple!(eq(123), eq(0)))?; // Fails: second matcher does not match
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// Matchers must correspond to the actual tuple in count and type. Otherwise
/// the test will fail to compile.
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// # fn should_not_compile() -> Result<()> {
/// verify_that!((123, 456), tuple!(eq(123)))?; // Does not compile: wrong tuple size
/// verify_that!((123, "A string"), tuple!(eq(123), eq(456)))?; // Does not compile: wrong type
/// #     Ok(())
/// # }
/// ```
///
/// All fields must be covered by matchers. Use
/// [`anything`][crate::matchers::anything] for fields which are not relevant
/// for the test.
///
/// ```
/// # use googletest::prelude::*;
/// verify_that!((123, 456), tuple!(eq(123), anything()))
/// #     .unwrap();
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
        use $crate::matchers::tuple_matcher::internal::TupleMatcher12;
        TupleMatcher12(
            $matcher0,
            $matcher1,
            $matcher2,
            $matcher3,
            $matcher4,
            $matcher5,
            $matcher6,
            $matcher7,
            $matcher8,
            $matcher9,
            $matcher10,
            $matcher11,
            Default::default(),
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
        use $crate::matchers::tuple_matcher::internal::TupleMatcher11;
        TupleMatcher11(
            $matcher0,
            $matcher1,
            $matcher2,
            $matcher3,
            $matcher4,
            $matcher5,
            $matcher6,
            $matcher7,
            $matcher8,
            $matcher9,
            $matcher10,
            Default::default(),
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
        use $crate::matchers::tuple_matcher::internal::TupleMatcher10;
        TupleMatcher10(
            $matcher0,
            $matcher1,
            $matcher2,
            $matcher3,
            $matcher4,
            $matcher5,
            $matcher6,
            $matcher7,
            $matcher8,
            $matcher9,
            Default::default(),
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
        use $crate::matchers::tuple_matcher::internal::TupleMatcher9;
        TupleMatcher9(
            $matcher0,
            $matcher1,
            $matcher2,
            $matcher3,
            $matcher4,
            $matcher5,
            $matcher6,
            $matcher7,
            $matcher8,
            Default::default(),
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
        use $crate::matchers::tuple_matcher::internal::TupleMatcher8;
        TupleMatcher8(
            $matcher0,
            $matcher1,
            $matcher2,
            $matcher3,
            $matcher4,
            $matcher5,
            $matcher6,
            $matcher7,
            Default::default(),
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
        use $crate::matchers::tuple_matcher::internal::TupleMatcher7;
        TupleMatcher7(
            $matcher0,
            $matcher1,
            $matcher2,
            $matcher3,
            $matcher4,
            $matcher5,
            $matcher6,
            Default::default(),
        )
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr,
        $matcher4:expr,
        $matcher5:expr $(,)?
    ) => {{
        use $crate::matchers::tuple_matcher::internal::TupleMatcher6;
        TupleMatcher6(
            $matcher0,
            $matcher1,
            $matcher2,
            $matcher3,
            $matcher4,
            $matcher5,
            Default::default(),
        )
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr,
        $matcher4:expr $(,)?
    ) => {{
        use $crate::matchers::tuple_matcher::internal::TupleMatcher5;
        TupleMatcher5($matcher0, $matcher1, $matcher2, $matcher3, $matcher4, Default::default())
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr,
        $matcher3:expr $(,)?
    ) => {{
        use $crate::matchers::tuple_matcher::internal::TupleMatcher4;
        TupleMatcher4($matcher0, $matcher1, $matcher2, $matcher3, Default::default())
    }};

    (
        $matcher0:expr,
        $matcher1:expr,
        $matcher2:expr $(,)?
    ) => {{
        use $crate::matchers::tuple_matcher::internal::TupleMatcher3;
        TupleMatcher3($matcher0, $matcher1, $matcher2, Default::default())
    }};

    (
        $matcher0:expr,
        $matcher1:expr $(,)?
    ) => {{
        use $crate::matchers::tuple_matcher::internal::TupleMatcher2;
        TupleMatcher2($matcher0, $matcher1, Default::default())
    }};

    (
        $matcher0:expr $(,)?
    ) => {{
        use $crate::matchers::tuple_matcher::internal::TupleMatcher1;
        TupleMatcher1($matcher0, Default::default())
    }};

    () => {{
        use $crate::matchers::eq;
        eq(())
    }};
}

/// Functions for use only by the declarative macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::matcher::{Matcher, MatcherResult};
    use std::{
        fmt::{Debug, Write},
        marker::PhantomData,
    };

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
            pub struct $name<$($field_type, $matcher_type),*>(
                $(pub $matcher_type),*,
                pub PhantomData<($($field_type,)*)>,
            );

            impl<$($field_type: Debug, $matcher_type: Matcher<ActualT = $field_type>),*>
                Matcher for $name<$($field_type, $matcher_type),*>
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
