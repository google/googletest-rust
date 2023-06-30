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

#![cfg(feature = "proptest")]

use googletest::prelude::*;
use proptest::test_runner::{Config, TestRunner};

#[test]
fn numbers_are_greater_than_zero() -> Result<()> {
    let mut runner = TestRunner::new(Config::default());
    runner.run(&(1..100i32), |v| Ok(verify_that!(v, gt(0))?)).into_test_result()
}

#[test]
fn strings_are_nonempty() -> Result<()> {
    let mut runner = TestRunner::new(Config::default());
    runner.run(&"[a-zA-Z0-9]+", |v| Ok(verify_that!(v, not(eq("")))?)).into_test_result()
}
