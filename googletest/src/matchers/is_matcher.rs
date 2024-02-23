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
    matcher::{Matcher, MatcherExt, MatcherResult},
};
use std::fmt::Debug;

/// Matches precisely values matched by `inner`.
///
/// The returned matcher produces a description prefixed by the string
/// `description`. This is useful in contexts where the test assertion failure
/// output must include the additional description.
pub fn is<InnerMatcherT>(description: &str, inner: InnerMatcherT) -> IsMatcher<'_, InnerMatcherT> {
    IsMatcher { description, inner }
}

#[derive(MatcherExt)]
pub struct IsMatcher<'a, InnerMatcherT> {
    description: &'a str,
    inner: InnerMatcherT,
}

impl<'a, 'b, ActualT: Debug, InnerMatcherT: Matcher<'b, ActualT>> Matcher<'b, ActualT>
    for IsMatcher<'a, InnerMatcherT>
{
    fn matches<'c>(&self, actual: &'c ActualT) -> MatcherResult
    where
        'b: 'c,
    {
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

    fn explain_match<'c>(&self, actual: &'c ActualT) -> Description
    where
        'b: 'c,
    {
        self.inner.explain_match(actual)
    }
}
