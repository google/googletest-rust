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
    matcher_support::{edit_distance, summarize_diff::create_diff},
};
use std::{fmt::Debug, marker::PhantomData, ops::Deref};

/// Matches a value equal (in the sense of `==`) to the dereferenced value of
/// `expected`.
///
/// This is similar to [`eq`][crate::matchers::eq] but takes a reference or
/// smart pointer to the expected value rather than consuming it. This is useful
/// when:
///
///  * one has only a reference to the expected value, and
///  * the expected value cannot or should not be copied or cloned to create an
///    owned value from it.
///
/// ```
/// # use googletest::{matchers::eq_deref_of, verify_that};
/// #[derive(Debug, PartialEq)]
/// struct NonCloneableStruct(i32);
/// let expected = NonCloneableStruct(123);
/// verify_that!(NonCloneableStruct(123), eq_deref_of(&expected))
/// #    .unwrap()
/// ```
///
/// **Note**: while one can use `eq_deref_of` with the configuration methods of
/// [`StrMatcherConfigurator`][crate::matchers::str_matcher::StrMatcherConfigurator]
/// to configure string equality, it is not possible to do so when the input is
/// a smart pointer to a string.
///
/// ```compile_fail
/// # use googletest::{matchers::{eq_deref_of, str_matcher::StrMatcherConfigurator}, verify_that};
/// verify_that!("A string", eq_deref_of(Box::new("A STRING")).ignoring_ascii_case()) // Does not compile
/// #    .unwrap()
/// ```
///
/// Otherwise, this has the same behaviour as [`eq`][crate::matchers::eq].
pub fn eq_deref_of<ActualT: ?Sized, ExpectedRefT>(
    expected: ExpectedRefT,
) -> EqDerefOfMatcher<ActualT, ExpectedRefT> {
    EqDerefOfMatcher { expected, phantom: Default::default() }
}

/// A matcher which matches a value equal to the derefenced value of `expected`.
///
/// See [`eq_deref_of`].
pub struct EqDerefOfMatcher<ActualT: ?Sized, ExpectedRefT> {
    pub(crate) expected: ExpectedRefT,
    phantom: PhantomData<ActualT>,
}

impl<ActualT, ExpectedRefT, ExpectedT> Matcher for EqDerefOfMatcher<ActualT, ExpectedRefT>
where
    ActualT: Debug + ?Sized,
    ExpectedRefT: Deref<Target = ExpectedT> + Debug,
    ExpectedT: PartialEq<ActualT> + Debug,
{
    type ActualT = ActualT;

    fn matches(&self, actual: &ActualT) -> MatcherResult {
        (self.expected.deref() == actual).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("is equal to {:?}", self.expected).into(),
            MatcherResult::NoMatch => format!("isn't equal to {:?}", self.expected).into(),
        }
    }

    fn explain_match(&self, actual: &ActualT) -> Description {
        format!(
            "which {}{}",
            &self.describe(self.matches(actual)),
            create_diff(
                &format!("{:#?}", actual),
                &format!("{:#?}", self.expected.deref()),
                edit_distance::Mode::Exact,
            )
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::eq_deref_of;
    use crate::prelude::*;
    use indoc::indoc;

    #[derive(Debug, PartialEq)]
    struct NonCloneNonCopyStruct(i32);

    #[test]
    fn matches_value_with_ref_to_equal_value() -> Result<()> {
        verify_that!(NonCloneNonCopyStruct(123), eq_deref_of(&NonCloneNonCopyStruct(123)))
    }

    #[test]
    fn matches_value_with_box_of_equal_value() -> Result<()> {
        verify_that!(NonCloneNonCopyStruct(123), eq_deref_of(Box::new(NonCloneNonCopyStruct(123))))
    }

    #[test]
    fn does_not_match_value_with_non_equal_value() -> Result<()> {
        verify_that!(NonCloneNonCopyStruct(123), not(eq_deref_of(&NonCloneNonCopyStruct(234))))
    }

    #[test]
    fn shows_structured_diff() -> Result<()> {
        #[derive(Debug, PartialEq)]
        struct Strukt {
            int: i32,
            string: String,
        }

        let result = verify_that!(
            Strukt { int: 123, string: "something".into() },
            eq_deref_of(Box::new(Strukt { int: 321, string: "someone".into() }))
        );
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {
            "
            Actual: Strukt { int: 123, string: \"something\" },
              which isn't equal to Strukt { int: 321, string: \"someone\" }
              Difference(-actual / +expected):
               Strukt {
              -    int: 123,
              +    int: 321,
              -    string: \"something\",
              +    string: \"someone\",
               }
            "})))
        )
    }
}
