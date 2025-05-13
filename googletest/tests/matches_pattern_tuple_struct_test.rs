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
use googletest::Result;

#[test]
fn matches_tuple_struct_containing_single_field() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32);
    let actual = AStruct(123);

    verify_that!(actual, matches_pattern!(&AStruct(eq(123))))
}

#[test]
fn matches_tuple_struct_containing_two_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32);
    let actual = AStruct(123, 234);

    verify_that!(actual, matches_pattern!(&AStruct(eq(123), eq(234))))
}

#[test]
fn matches_tuple_struct_containing_three_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32);
    let actual = AStruct(123, 234, 345);

    verify_that!(actual, matches_pattern!(&AStruct(eq(123), eq(234), eq(345))))
}

#[test]
fn matches_tuple_struct_containing_four_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456);

    verify_that!(actual, matches_pattern!(&AStruct(eq(123), eq(234), eq(345), eq(456))))
}

#[test]
fn matches_tuple_struct_containing_five_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567);

    verify_that!(actual, matches_pattern!(&AStruct(eq(123), eq(234), eq(345), eq(456), eq(567))))
}

#[test]
fn matches_tuple_struct_containing_six_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678);

    verify_that!(
        actual,
        matches_pattern!(&AStruct(eq(123), eq(234), eq(345), eq(456), eq(567), eq(678)))
    )
}

#[test]
fn matches_tuple_struct_containing_seven_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789);

    verify_that!(
        actual,
        matches_pattern!(&AStruct(eq(123), eq(234), eq(345), eq(456), eq(567), eq(678), eq(789)))
    )
}

#[test]
fn matches_tuple_struct_containing_eight_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789, 890);

    verify_that!(
        actual,
        matches_pattern!(&AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678),
            eq(789),
            eq(890)
        ))
    )
}

#[test]
fn matches_tuple_struct_containing_nine_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789, 890, 901);

    verify_that!(
        actual,
        matches_pattern!(&AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678),
            eq(789),
            eq(890),
            eq(901)
        ))
    )
}

#[test]
fn matches_tuple_struct_containing_ten_fields() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789, 890, 901, 12);

    verify_that!(
        actual,
        matches_pattern!(&AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678),
            eq(789),
            eq(890),
            eq(901),
            eq(12)
        ))
    )
}

#[test]
fn matches_tuple_struct_containing_ten_fields_by_ref() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789, 890, 901, 12);

    verify_that!(
        actual,
        matches_pattern!(&AStruct(
            ref eq(&123),
            ref eq(&234),
            ref eq(&345),
            ref eq(&456),
            ref eq(&567),
            ref eq(&678),
            ref eq(&789),
            ref eq(&890),
            ref eq(&901),
            ref eq(&12)
        ))
    )
}

#[test]
fn matches_tuple_struct_with_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32);
    let actual = AStruct(123);

    verify_that!(
        actual,
        matches_pattern!(&AStruct(
            eq(123), // Keep the trailing comma, block reformatting
        ))
    )
}

#[test]
fn matches_tuple_struct_with_two_fields_and_trailing_comma() -> Result<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32);
    let actual = AStruct(123, 234);

    verify_that!(
        actual,
        matches_pattern!(&AStruct(
            eq(123),
            eq(234), // Keep the trailing comma, block reformatting
        ))
    )
}

#[test]
fn matches_tuple_struct_with_interleaved_underscore() -> Result<()> {
    #[derive(Debug)]
    struct NonCopy;
    #[derive(Debug)]
    struct AStruct(u32, NonCopy, u32);
    let actual = AStruct(1, NonCopy, 3);

    verify_that!(actual, matches_pattern!(&AStruct(eq(1), _, eq(3))))?;
    verify_that!(actual, matches_pattern!(AStruct(eq(&1), _, eq(&3))))
}

#[test]
fn matches_tuple_struct_non_exhaustive() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct AStruct(i32, u32);
    let actual = AStruct(1, 3);

    verify_that!(actual, matches_pattern!(&AStruct(_, ..)))?;
    verify_that!(actual, matches_pattern!(AStruct(_, ..)))
}

#[test]
fn matches_generic_tuple_struct_exhaustively() -> Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct AStruct<T>(T, u32);
    let actual = AStruct(1, 3);

    verify_that!(actual, matches_pattern!(&AStruct(_, _)))?;
    verify_that!(actual, matches_pattern!(AStruct(_, _)))
}
