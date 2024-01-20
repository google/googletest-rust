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

#![doc(hidden)]

use crate::{
    description::Description,
    matcher::{Matcher, MatcherResult},
};
use std::{fmt::Debug, marker::PhantomData};

/// Matches precisely values matched by `inner`.
///
/// The returned matcher produces a description prefixed by the string
/// `description`. This is useful in contexts where the test assertion failure
/// output must include the additional description.
pub fn is<'d, 'a, ActualT: Debug + 'a + 'd, InnerMatcherT: Matcher<'a, ActualT = ActualT> + 'd>(
    description: &'d str,
    inner: InnerMatcherT,
) -> impl Matcher<'a, ActualT = ActualT> + 'd {
    IsMatcher { description, inner, phantom: Default::default() }
}

struct IsMatcher<'d, ActualT, InnerMatcherT> {
    description: &'d str,
    inner: InnerMatcherT,
    phantom: PhantomData<ActualT>,
}

impl<'d, 'a, ActualT: Debug, InnerMatcherT: Matcher<'a, ActualT = ActualT>> Matcher<'a>
    for IsMatcher<'d, ActualT, InnerMatcherT>
{
    type ActualT = ActualT;

    fn matches(&self, actual: &'a Self::ActualT) -> MatcherResult {
        self.inner.matches(actual)
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!(
                "is {} which {}",
                self.description,
                self.inner.describe(MatcherResult::Match)
            )
            .into(),
            MatcherResult::NoMatch => format!(
                "is not {} which {}",
                self.description,
                self.inner.describe(MatcherResult::Match)
            )
            .into(),
        }
    }

    fn explain_match(&self, actual: &'a Self::ActualT) -> Description {
        self.inner.explain_match(actual)
    }
}
