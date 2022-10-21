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

#[cfg(not(google3))]
use googletest::{all, matchers};
use googletest::{
    google_test, verify_pred, verify_that, GoogleTestSupport, MapErrorToTestFailure, Result,
};
use googletest_macro::google_test_wrapper;
#[cfg(google3)]
use matchers::all;
use matchers::{contains_substring, eq, matches_regex, not};
use std::process::Command;

#[google_test]
fn should_pass() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(2))
}

#[google_test]
fn should_fail_on_assertion_failure() -> Result<()> {
    let status = run_external_process("simple_assertion_failure").status().err_to_test_failure()?;

    verify_that!(status.success(), eq(false))
}

#[google_test]
fn should_output_failure_message_on_assertion_failure() -> Result<()> {
    let output = run_external_process_in_tests_directory("simple_assertion_failure")?;

    verify_that!(
        output,
        matches_regex(
            "Value of: value\n\
            Expected: is equal to 3\n\
            Actual: 2, which isn't equal to 3\n  \
            at .*googletest/tests/simple_assertion_failure.rs:[0-9]+:5\n"
        )
    )
}

#[google_test]
fn should_fail_due_to_assertion_failure_in_subroutine() -> Result<()> {
    let status = run_external_process("simple_assertion_failure").status().err_to_test_failure()?;

    verify_that!(status.success(), eq(false))
}

#[google_test]
fn should_fail_due_to_returned_error_in_subroutine() -> Result<()> {
    let status =
        run_external_process("failure_due_to_returned_error").status().err_to_test_failure()?;

    verify_that!(status.success(), eq(false))
}

#[google_test]
fn should_fail_test_on_and_log_failure() -> Result<()> {
    let status =
        run_external_process("non_fatal_failure_in_subroutine").status().err_to_test_failure()?;

    verify_that!(status.success(), eq(false))
}

// Using google_test_wrapper rather than google_test prevents this from being
// run as a real test, which would of course fail.
#[google_test_wrapper]
fn fails_but_continues() -> Result<()> {
    verify_that!(2, eq(3)).and_log_failure();
    Ok(())
}

#[google_test]
fn should_log_test_failures_to_stdout() -> Result<()> {
    let output = run_external_process_in_tests_directory("two_non_fatal_failures")?;

    verify_that!(
        output,
        all!(
            contains_substring(
                "\
Expected: is equal to 3
Actual: 2, which isn't equal to 3
"
            ),
            contains_substring(
                "\
Expected: is equal to 4
Actual: 2, which isn't equal to 4
"
            )
        )
    )
}

#[google_test]
fn should_abort_after_first_failure() -> Result<()> {
    let output = run_external_process_in_tests_directory("first_failure_aborts")?;

    verify_that!(
        output,
        not(contains_substring(
            "\
Expected: is equal to 4
Actual: 2, which isn't equal to 4
"
        ))
    )
}

#[google_test]
fn should_fail_with_assertion_in_a_subroutine() -> Result<()> {
    let output = run_external_process_in_tests_directory("non_fatal_failure_in_subroutine")?;

    verify_that!(
        output,
        contains_substring(
            "\
Expected: is equal to 3
Actual: 2, which isn't equal to 3
"
        )
    )
}

#[google_test]
fn should_include_custom_error_message_in_failure() -> Result<()> {
    let output = run_external_process_in_tests_directory("custom_error_message")?;

    verify_that!(output, contains_substring("A custom error message"))?;
    verify_that!(output, contains_substring("A custom error message in a String"))?;
    verify_that!(output, contains_substring("A custom error message from a closure"))
}

#[google_test]
fn should_not_run_closure_with_custom_error_message_if_test_passes() -> Result<()> {
    let result = should_pass_with_custom_error_message_in_closure();

    verify_that!(result, eq(Ok(())))
}

// Using google_test_wrapper rather than google_test prevents this from being
// run as a real test.
#[google_test_wrapper]
fn should_pass_with_custom_error_message_in_closure() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(2))
        .with_failure_message(|| panic!("This should not execute, since the assertion passes."))
}

#[google_test]
fn should_verify_predicate_with_success() -> Result<()> {
    let result = verify_predicate_with_success();

    verify_that!(result, eq(Ok(())))
}

#[google_test_wrapper]
fn verify_predicate_with_success() -> Result<()> {
    verify_pred!(eq_predicate(1, 1))
}

#[google_test]
fn should_verify_predicate_with_trailing_comma() -> Result<()> {
    let result = verify_predicate_with_trailing_comma();

    verify_that!(result, eq(Ok(())))
}

#[google_test_wrapper]
fn verify_predicate_with_trailing_comma() -> Result<()> {
    verify_pred!(eq_predicate(1, 1,))
}

fn eq_predicate(a: i32, b: i32) -> bool {
    a == b
}

#[google_test]
fn verify_pred_should_fail_test_on_failure() -> Result<()> {
    let status =
        run_external_process("verify_predicate_with_failure").status().err_to_test_failure()?;

    verify_that!(status.success(), eq(false))
}

#[google_test]
fn verify_pred_should_output_correct_failure_message() -> Result<()> {
    let output = run_external_process_in_tests_directory("verify_predicate_with_failure")?;

    verify_that!(
        output,
        contains_substring(
            "\
eq_predicate(a, b) was false with
  a = 1,
  b = 2
"
        )
    )
}

#[google_test]
fn should_verify_predicate_in_a_submodule() -> Result<()> {
    verify_pred!(submodule::eq_predicate_in_submodule(1, 1))
}

mod submodule {
    pub(super) fn eq_predicate_in_submodule(a: i32, b: i32) -> bool {
        a == b
    }
}

#[google_test]
fn should_verify_predicate_as_a_method() -> Result<()> {
    let a_struct = AStruct {};

    verify_pred!(a_struct.eq_predicate_as_method(1, 1))
}

#[google_test]
fn should_verify_predicate_as_a_method_on_an_expresion_result() -> Result<()> {
    verify_pred!((AStruct {}).eq_predicate_as_method(1, 1))
}

struct AStruct {}

impl AStruct {
    fn eq_predicate_as_method(&self, a: i32, b: i32) -> bool {
        a == b
    }
}

#[google_test]
fn should_verify_predicate_as_a_method_in_submodule() -> Result<()> {
    verify_pred!(another_submodule::A_STRUCT_IN_SUBMODULE.eq_predicate_as_method(1, 1))
}

mod another_submodule {
    pub(super) static A_STRUCT_IN_SUBMODULE: super::AStruct = super::AStruct {};
}

#[google_test]
fn should_verify_predicate_as_a_method_on_a_field() -> Result<()> {
    let another_struct = AnotherStruct { a_struct: AStruct {} };

    verify_pred!(another_struct.a_struct.eq_predicate_as_method(1, 1))
}

struct AnotherStruct {
    a_struct: AStruct,
}

#[google_test]
fn verify_pred_should_show_correct_qualified_function_name_in_test_failure_output() -> Result<()> {
    let output = run_external_process_in_tests_directory(
        "verify_predicate_with_failure_as_method_in_submodule",
    )?;

    verify_that!(
        output,
        contains_substring(
            "\
a_submodule :: A_STRUCT_IN_SUBMODULE.eq_predicate_as_method(a, b) was false with
  a = 1,
  b = 2
"
        )
    )
}

#[google_test]
fn fail_macro_causes_test_failure() -> Result<()> {
    let status =
        run_external_process("failure_due_to_fail_macro").status().err_to_test_failure()?;

    verify_that!(status.success(), eq(false))
}

#[google_test]
fn fail_macro_outputs_message() -> Result<()> {
    let output = run_external_process_in_tests_directory("failure_due_to_fail_macro")?;

    verify_that!(
        output,
        matches_regex(
            "\
Expected test failure
  at .*googletest/tests/failure_due_to_fail_macro.rs:[0-9]+:5
"
        )
    )
}

#[google_test]
fn fail_macro_allows_empty_message() -> Result<()> {
    let output =
        run_external_process_in_tests_directory("failure_due_to_fail_macro_with_empty_message")?;

    verify_that!(output, contains_substring("Test failed"))
}

#[google_test]
fn fail_macro_allows_message_with_format_arguments() -> Result<()> {
    let output =
        run_external_process_in_tests_directory("failure_due_to_fail_macro_with_format_arguments")?;

    verify_that!(output, contains_substring("Failure message with argument: An argument"))
}

fn run_external_process_in_tests_directory(name: &'static str) -> Result<String> {
    let mut command = run_external_process(name);
    let std::process::Output { stdout, .. } = command.output().err_to_test_failure()?;
    String::from_utf8(stdout).err_to_test_failure()
}

fn run_external_process(name: &'static str) -> Command {
    let command_path = format!(
        "../{}/debug/{name}",
        std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into())
    );
    Command::new(command_path)
}
