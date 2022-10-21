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

use googletest::{verify_pred, Result};
use googletest_macro::google_test_wrapper;

fn main() -> std::result::Result<(), ()> {
    verify_predicate_with_failure()
}

#[google_test_wrapper]
fn verify_predicate_with_failure() -> Result<()> {
    let a = 1;
    let b = 2;
    verify_pred!(eq_predicate(a, b))
}

fn eq_predicate(a: i32, b: i32) -> bool {
    a == b
}
