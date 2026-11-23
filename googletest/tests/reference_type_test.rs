// Copyright 2025 Google LLC
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

#[derive(Debug)]
struct ArenaHolder<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T: PartialEq + ?Sized> PartialEq<T> for ArenaHolder<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.value == other
    }
}

struct Strukt {
    a_field: String,
}

impl<'a> ArenaHolder<'a, Strukt> {
    fn get_a_field(&self) -> ArenaHolder<'_, str> {
        ArenaHolder { value: &self.value.a_field }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test() {
        let arena = [Strukt { a_field: "hello".into() }];

        let holder = ArenaHolder { value: &arena[0] };

        let inner_holder = holder.get_a_field();

        expect_that!(inner_holder, eq("hello"));
    }

    #[gtest]
    fn consistent_value_and_ref_matchers_across_copy_and_non_copy_results() {
        let r1: std::result::Result<(), ()> = Err(());
        let r2: std::result::Result<String, ()> = Err(());

        // Consistently without `&`:
        expect_that!(r1, err(eq(())));
        expect_that!(r2, err(eq(())));

        // Consistently with `&`:
        expect_that!(r1, err(eq(&())));
        expect_that!(r2, err(eq(&())));

        // Consistently with `&` on both subject and expected:
        expect_that!(&r1, err(eq(&())));
        expect_that!(&r2, err(eq(&())));
    }

    #[gtest]
    fn option_of_mut_ref_and_shared_ref_match_consistently() {
        fn return_as_some_mut(n: &mut i32) -> Option<&mut i32> {
            Some(n)
        }
        fn return_as_some_ref(n: &i32) -> Option<&i32> {
            Some(n)
        }

        let mut n = 17;
        expect_that!(return_as_some_mut(&mut n), some(eq(&17)));
        expect_that!(return_as_some_mut(&mut n), some(eq(&mut 17)));
        expect_that!(return_as_some_mut(&mut n), some(eq(&&17)));
        expect_that!(return_as_some_ref(&n), some(eq(&17)));
        expect_that!(return_as_some_ref(&n), some(eq(&&17)));
    }

    #[gtest]
    fn packed_struct_field_matches_without_unaligned_reference() {
        #[repr(C, packed)]
        struct PackedDescriptor {
            size: u32,
            selector: u16,
            reserved: u16,
        }

        let desc = PackedDescriptor { size: 4, selector: 7, reserved: 0 };
        expect_that!(desc.size, eq(4));
        expect_that!(desc.selector, eq(7));
        expect_that!(desc.reserved, eq(0));
    }

    #[gtest]
    fn option_ref_matches_container_matchers() {
        #[derive(Debug, PartialEq)]
        struct Node(i32);

        let node = Node(42);
        let opt = Some(Node(42));
        expect_that!(opt.as_ref(), unordered_elements_are![eq(&node)]);
        expect_that!(opt.as_ref(), elements_are![eq(&node)]);
        expect_that!(opt.as_ref(), contains(eq(&node)));
        expect_that!(opt.as_ref(), each(eq(&node)));
        expect_that!(opt.as_ref(), len(eq(1)));
        expect_that!(None::<&Node>, is_empty());
    }

    #[gtest]
    fn copy_into_iterator_view_in_property_matcher() {
        #[derive(Clone, Copy, Debug)]
        struct SliceView<'a, T>(&'a [T]);

        impl<'a, T> IntoIterator for SliceView<'a, T> {
            type Item = &'a T;
            type IntoIter = std::slice::Iter<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a, T> IntoIterator for &SliceView<'a, T> {
            type Item = &'a T;
            type IntoIter = std::slice::Iter<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        #[derive(Debug)]
        struct Request {
            names: Vec<String>,
        }

        impl Request {
            fn names(&self) -> SliceView<'_, String> {
                SliceView(&self.names)
            }
        }

        let requests = vec![Request { names: vec!["srv1:3400".into(), "srv2:9001".into()] }];
        expect_that!(
            requests,
            elements_are![property!(
                &Request.names(),
                unordered_elements_are!["srv1:3400", "srv2:9001"]
            )]
        );
        expect_that!(
            &requests,
            elements_are![property!(
                &Request.names(),
                unordered_elements_are!["srv1:3400", "srv2:9001"]
            )]
        );
    }

    #[gtest]
    fn closure_parameter_vec_type_inference_with_ref_element_matcher() {
        #[derive(Debug, PartialEq)]
        struct NonCopyMsg {
            field: i32,
        }

        #[derive(MatcherBase)]
        struct RefOnlyMsgMatcher(NonCopyMsg);

        impl Matcher<&NonCopyMsg> for RefOnlyMsgMatcher {
            fn matches(&self, actual: &&NonCopyMsg) -> googletest::matcher::MatcherResult {
                (*actual == &self.0).into()
            }

            fn describe(
                &self,
                matcher_result: googletest::matcher::MatcherResult,
            ) -> googletest::description::Description {
                match matcher_result {
                    googletest::matcher::MatcherResult::Match => {
                        format!("is equal to {:?}", self.0).into()
                    }
                    googletest::matcher::MatcherResult::NoMatch => {
                        format!("isn't equal to {:?}", self.0).into()
                    }
                }
            }
        }

        fn call_with_vec<T>(items: Vec<T>, f: impl FnOnce(Vec<T>)) {
            f(items);
        }

        call_with_vec(vec![NonCopyMsg { field: 17 }, NonCopyMsg { field: 19 }], |requests| {
            expect_that!(
                requests,
                elements_are![
                    RefOnlyMsgMatcher(NonCopyMsg { field: 17 }),
                    RefOnlyMsgMatcher(NonCopyMsg { field: 19 }),
                ]
            );
        });
    }

    // Mirrors the example in the section "Matching both values and references"
    // of crate_docs.md. Keep the two in sync.
    #[gtest]
    fn custom_wrapping_matcher_implemented_for_value_and_ref() {
        use googletest::description::Description;
        use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
        use std::fmt::Debug;

        #[derive(Debug)]
        struct Wrapper<T>(T);

        #[derive(MatcherBase)]
        struct WrapsMatcher<InnerMatcherT> {
            inner: InnerMatcherT,
        }

        impl<T: Debug, InnerMatcherT: Matcher<T>> Matcher<Wrapper<T>> for WrapsMatcher<InnerMatcherT> {
            fn matches(&self, actual: &Wrapper<T>) -> MatcherResult {
                self.inner.matches(&actual.0)
            }

            fn describe(&self, matcher_result: MatcherResult) -> Description {
                format!("wraps a value which {}", self.inner.describe(matcher_result)).into()
            }
        }

        impl<'a, T: Debug, InnerMatcherT: Matcher<&'a T>> Matcher<&'a Wrapper<T>>
            for WrapsMatcher<InnerMatcherT>
        {
            fn matches(&self, actual: &&'a Wrapper<T>) -> MatcherResult {
                self.inner.matches(&&actual.0)
            }

            fn describe(&self, matcher_result: MatcherResult) -> Description {
                format!("wraps a value which {}", self.inner.describe(matcher_result)).into()
            }
        }

        fn wraps<InnerMatcherT>(inner: InnerMatcherT) -> WrapsMatcher<InnerMatcherT> {
            WrapsMatcher { inner }
        }

        #[derive(Debug)]
        struct Holder {
            wrapper: Wrapper<String>,
        }

        // Uses the impl for `Wrapper<T>`.
        expect_that!(Wrapper(1), wraps(eq(1)));
        // Both of these use the impl for `&Wrapper<T>`: the first is given a
        // reference directly, the second is handed one by `matches_pattern!`
        // because `Holder` is not `Copy`.
        expect_that!(&Wrapper(1), wraps(eq(&1)));
        expect_that!(
            Holder { wrapper: Wrapper("a".to_string()) },
            matches_pattern!(Holder { wrapper: wraps(eq("a")) })
        );
    }
}
