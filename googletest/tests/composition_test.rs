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

use googletest::prelude::*;

#[test]
fn all_matcher_works_as_inner_matcher() -> Result<()> {
    let value = vec![1];
    verify_that!(value, contains_each![all!(gt(&0), lt(&2))])
}

#[test]
fn matches_pattern_works_as_inner_matcher() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(i32);
    verify_that!(vec![AStruct(123)], contains_each![matches_pattern!(&AStruct(eq(123)))])
}

#[test]
fn matches_pattern_works_with_property_as_inner_matcher() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(i32);
    impl AStruct {
        fn get_value(&self) -> i32 {
            self.0
        }
    }
    verify_that!(
        vec![AStruct(123)],
        contains_each![matches_pattern!(&AStruct {
            get_value(): eq(123)
        })]
    )
}

#[test]
fn contains_each_works_as_inner_matcher() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(Vec<i32>);
    verify_that!(AStruct(vec![123]), matches_pattern!(&AStruct(ref contains_each![eq(&123)])))
}

#[test]
fn pointwise_works_as_inner_matcher() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(Vec<i32>);
    verify_that!(AStruct(vec![123]), matches_pattern!(&AStruct(ref pointwise!(eq, [&123]))))
}

#[test]
fn elements_are_works_as_inner_matcher() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(Vec<i32>);
    verify_that!(AStruct(vec![123]), matches_pattern!(&AStruct(ref elements_are![eq(&123)])))
}

#[test]
fn tuple_works_as_inner_matcher() -> Result<()> {
    verify_that!(vec![(123,)], elements_are![(eq(&123),)])
}
