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

use googletest::prelude::*;
use std::process::exit;

#[gtest]
fn test_expect_exit_success() -> Result<()> {
    verify_exit!(exit(0), exited_with_code(0), contains_substring(""))
}

#[gtest]
fn test_expect_exit_macro_success() {
    expect_exit!(exit(0), exited_with_code(0), contains_substring(""));
}

#[gtest]
fn test_expect_exit_failure_code() -> Result<()> {
    verify_exit!(exit(42), exited_with_code(42), contains_substring(""))
}

#[gtest]
fn test_expect_exit_with_output() -> Result<()> {
    verify_exit!(
        {
            eprintln!("Fatal error encountered!");
            exit(1);
        },
        exited_with_code(1),
        contains_substring("Fatal error")
    )
}

#[gtest]
fn test_expect_exit_multiple() -> Result<()> {
    verify_exit!(exit(1), exited_with_code(1), anything())?;
    verify_exit!(exit(2), exited_with_code(2), anything())?;
    Ok(())
}

#[gtest]
fn test_expect_exit_fails_when_lived() -> Result<()> {
    // We test that verify_exit fails when the code doesn't die.
    // We can verify that verify_exit itself returns an Err!
    let res = verify_exit!(
        {
            // Does nothing, returns normally
        },
        anything(),
        anything()
    );
    verify_that!(res, err(displays_as(contains_substring("Death test failed to die"))))
}

#[gtest]
fn test_expect_exit_fails_wrong_code() -> Result<()> {
    let res = verify_exit!(exit(1), exited_with_code(0), anything());
    verify_that!(res, err(displays_as(contains_substring("exit status did not match"))))
}

#[gtest]
fn test_normal_test_that_should_not_run_in_death_test_child() {
    eprintln!("___DEATH_TEST_SPOILER_ALERT___");
}

#[gtest]
fn test_death_test_does_not_run_other_tests_in_child() -> Result<()> {
    verify_exit!(
        {
            eprintln!("Child is running");
            exit(0);
        },
        exited_with_code(0),
        all![
            contains_substring("Child is running"),
            not(contains_substring("___DEATH_TEST_SPOILER_ALERT___")),
            not(contains_substring("___DEATH_TEST_SPOILER_ALERT_2___"))
        ]
    )
}

#[gtest]
fn test_another_normal_test_that_should_not_run_in_death_test_child() {
    eprintln!("___DEATH_TEST_SPOILER_ALERT_2___");
}
