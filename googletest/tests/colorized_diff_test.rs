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
use std::fmt::{Display, Write};

// Make a long text with each element of the iterator on one line.
// `collection` must contains at least one element.
fn build_text<T: Display>(mut collection: impl Iterator<Item = T>) -> String {
    let mut text = String::new();
    write!(&mut text, "{}", collection.next().expect("Provided collection without elements"))
        .unwrap();
    for item in collection {
        write!(&mut text, "\n{}", item).unwrap();
    }
    text
}

#[test]
fn colors_appear_when_no_color_is_no_set_and_force_color_is_set() -> Result<()> {
    std::env::remove_var("NO_COLOR");
    std::env::set_var("FORCE_COLOR", "1");

    let result = verify_that!(build_text(1..50), eq(build_text(1..51)));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "
  Difference(-\x1B[1;31mactual\x1B[0m / +\x1B[1;32mexpected\x1B[0m):
   1
   2
   \x1B[3m<---- 45 common lines omitted ---->\x1B[0m
   48
   49
  +\x1B[1;32m50\x1B[0m"
        )))
    )
}
