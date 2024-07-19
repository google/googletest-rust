// Copyright 2024 Google LLC
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

/// Macro that wraps the expression with `eq(...)` if the expression is
/// not a matcher.
///
/// This is useful to let users pass expected value to macro matchers like
/// `field!` and `property!`.
///`
/// **For internal use only. API stablility is not guaranteed!**
/// If you are interested in using it in your matcher, please file an issue to
/// stabilize this.
#[macro_export]
macro_rules! __auto_eq {
    ($e:expr) => {{
        #[allow(unused_imports)]
        use $crate::matcher_support::__internal_unstable_do_not_depend_on_these::ExpectedKind;
        match $e {
            expected => {
                $crate::matcher_support::__internal_unstable_do_not_depend_on_these::Wrapper(
                    &expected,
                )
                .kind()
                .matcher(expected)
            }
        }
    }};
}

// This reimplements the pattern presented in
// https://github.com/dtolnay/case-studies/issues/14
pub mod internal {
    use crate::{
        matcher::MatcherBase,
        matchers::{eq, EqMatcher},
    };

    pub struct Wrapper<T>(pub T);

    impl<'a, T: MatcherBase> Wrapper<&'a T> {
        #[inline]
        pub fn kind(&self) -> MatcherTag {
            MatcherTag
        }
    }

    pub trait ExpectedKind {
        #[inline]
        fn kind(&self) -> ExpectedTag {
            ExpectedTag
        }
    }

    impl<T> ExpectedKind for Wrapper<T> {}

    pub struct MatcherTag;

    impl MatcherTag {
        #[inline]
        pub fn matcher<M>(self, matcher: M) -> M {
            matcher
        }
    }
    pub struct ExpectedTag;

    impl ExpectedTag {
        #[inline]
        pub fn matcher<T>(self, expected: T) -> EqMatcher<T> {
            eq(expected)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn auto_ref_matcher() -> Result<()> {
        verify_that!(123, __auto_eq!(ge(9)))
    }

    #[test]
    fn auto_ref_expected() -> Result<()> {
        verify_that!(123, __auto_eq!(123))
    }

    #[test]
    fn auto_ref_on_ref_matcher() -> Result<()> {
        let matcher = eq(123);
        verify_that!(123, __auto_eq!(&matcher))
    }
}
