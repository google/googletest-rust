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

fn main() {}

#[cfg(test)]
mod tests {
    #[cfg(not(google3))]
    use googletest::{all, matchers, matchers::str_matcher};
    use googletest::{
        assert_that, expect_pred, expect_that, google_test, verify_pred, verify_that,
        GoogleTestSupport, Result,
    };
    use indoc::indoc;
    #[cfg(google3)]
    use matchers::all;
    use matchers::{anything, contains_regex, contains_substring, displays_as, eq, err, not};
    use std::process::Command;
    use str_matcher::StrMatcherConfigurator;

    #[google_test]
    fn should_pass() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(2))
    }

    #[google_test]
    fn should_pass_with_assert_that() -> Result<()> {
        let value = 2;
        assert_that!(value, eq(2));
        Ok(())
    }

    #[google_test]
    fn should_pass_with_expect_that() -> Result<()> {
        let value = 2;
        expect_that!(value, eq(2));
        Ok(())
    }

    #[google_test]
    fn should_fail_on_assertion_failure() -> Result<()> {
        let status = run_external_process("simple_assertion_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[google_test]
    fn should_fail_on_assertion_failure_with_assert_that() -> Result<()> {
        let status = run_external_process("simple_assertion_failure_with_assert_that").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[google_test]
    fn should_fail_on_assertion_failure_with_expect_that() -> Result<()> {
        let status = run_external_process("expect_that_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[google_test]
    fn should_output_failure_message_on_assertion_failure() -> Result<()> {
        let output = run_external_process_in_tests_directory("simple_assertion_failure")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Value of: value
                Expected: is equal to 3
                Actual: 2, which isn't equal to 3
                  at .*googletest/integration_tests/simple_assertion_failure.rs:[0-9]+:9"})
        )
    }

    #[google_test]
    fn should_output_failure_message_on_assertion_failure_with_assert_that() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("simple_assertion_failure_with_assert_that")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Value of: value
                Expected: is equal to 3
                Actual: 2, which isn't equal to 3
                  at .*googletest/integration_tests/simple_assertion_failure_with_assert_that.rs:[0-9]+:9
                "})
        )
    }

    #[google_test]
    fn should_output_failure_message_on_assertion_failure_with_expect_that() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_that_failure")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Value of: value
                Expected: is equal to 3
                Actual: 2, which isn't equal to 3
                  at .*googletest/integration_tests/expect_that_failure.rs:[0-9]+:9
                "})
        )
    }

    #[google_test]
    fn should_output_second_failure_message_on_second_assertion_failure_with_expect_that()
    -> Result<()> {
        let output = run_external_process_in_tests_directory("two_expect_that_failures")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Value of: value
                Expected: is equal to 4
                Actual: 2, which isn't equal to 4
                  at .*googletest/integration_tests/two_expect_that_failures.rs:[0-9]+:9
                "})
        )
    }

    #[google_test]
    fn should_fail_due_to_assertion_failure_in_subroutine() -> Result<()> {
        let status = run_external_process("simple_assertion_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[google_test]
    fn should_fail_due_to_returned_error_in_subroutine() -> Result<()> {
        let status = run_external_process("failure_due_to_returned_error").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[google_test]
    fn should_fail_test_on_and_log_failure() -> Result<()> {
        let status = run_external_process("non_fatal_failure_in_subroutine").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[google_test]
    fn should_log_test_failures_to_stdout() -> Result<()> {
        let output = run_external_process_in_tests_directory("two_non_fatal_failures")?;

        verify_that!(
            output,
            all!(
                contains_substring(indoc! {"
                    Expected: is equal to 3
                    Actual: 2, which isn't equal to 3
                    "}),
                contains_substring(indoc! {"
                    Expected: is equal to 4
                    Actual: 2, which isn't equal to 4
                    "})
            )
        )
    }

    #[google_test]
    fn should_abort_after_first_failure() -> Result<()> {
        let output = run_external_process_in_tests_directory("first_failure_aborts")?;

        verify_that!(
            output,
            not(contains_substring(indoc! {"
                Expected: is equal to 4
                Actual: 2, which isn't equal to 4
                "}))
        )
    }

    #[google_test]
    fn should_fail_with_assertion_in_a_subroutine() -> Result<()> {
        let output = run_external_process_in_tests_directory("non_fatal_failure_in_subroutine")?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                Expected: is equal to 3
                Actual: 2, which isn't equal to 3
                "})
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
        let value = 2;
        verify_that!(value, eq(2))
            .with_failure_message(|| panic!("This should not execute, since the assertion passes."))
    }

    #[google_test]
    fn should_verify_predicate_with_success() -> Result<()> {
        verify_pred!(eq_predicate(1, 1))
    }

    #[google_test]
    fn should_verify_predicate_with_trailing_comma() -> Result<()> {
        verify_pred!(eq_predicate(1, 1,))
    }

    fn eq_predicate(a: i32, b: i32) -> bool {
        a == b
    }

    #[google_test]
    fn should_verify_predicate_with_success_using_expect_pred() -> Result<()> {
        expect_pred!(eq_predicate(1, 1));
        Ok(())
    }

    #[google_test]
    fn verify_pred_should_fail_test_on_failure() -> Result<()> {
        let status = run_external_process("verify_predicate_with_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[google_test]
    fn verify_pred_should_output_correct_failure_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("verify_predicate_with_failure")?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                eq_predicate(a, b) was false with
                  a = 1,
                  b = 2
                "})
        )
    }

    #[google_test]
    fn assert_pred_should_fail_test_on_failure() -> Result<()> {
        let status = run_external_process("assert_predicate_with_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[google_test]
    fn expect_pred_should_fail_test_on_failure() -> Result<()> {
        let status = run_external_process("expect_pred_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[google_test]
    fn assert_pred_should_output_correct_failure_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("assert_predicate_with_failure")?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                eq_predicate(a, b) was false with
                  a = 1,
                  b = 2
                "})
        )
    }

    #[google_test]
    fn expect_pred_should_output_correct_failure_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_pred_failure")?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                eq_predicate(a, b) was false with
                  a = 1,
                  b = 2
                "})
        )
    }

    #[google_test]
    fn expect_pred_should_output_failure_message_for_second_failure() -> Result<()> {
        let output = run_external_process_in_tests_directory("two_expect_pred_failures")?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                eq_predicate(a, b) was false with
                  a = 3,
                  b = 4
                "})
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
    fn verify_pred_should_show_correct_qualified_function_name_in_test_failure_output() -> Result<()>
    {
        let output = run_external_process_in_tests_directory(
            "verify_predicate_with_failure_as_method_in_submodule",
        )?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                a_submodule :: A_STRUCT_IN_SUBMODULE.eq_predicate_as_method(a, b) was false with
                  a = 1,
                  b = 2
                "})
        )
    }

    #[google_test]
    fn fail_macro_causes_test_failure() -> Result<()> {
        let status = run_external_process("failure_due_to_fail_macro").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[google_test]
    fn fail_macro_outputs_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("failure_due_to_fail_macro")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected test failure
                  at .*googletest/integration_tests/failure_due_to_fail_macro.rs:[0-9]+:9
                "})
        )
    }

    #[google_test]
    fn fail_macro_allows_empty_message() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "failure_due_to_fail_macro_with_empty_message",
        )?;

        verify_that!(output, contains_substring("Test failed"))
    }

    #[google_test]
    fn fail_macro_allows_message_with_format_arguments() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "failure_due_to_fail_macro_with_format_arguments",
        )?;

        verify_that!(output, contains_substring("Failure message with argument: An argument"))
    }

    #[google_test]
    fn test_using_normal_test_attribute_macro_formats_failure_message_correctly() -> Result<()> {
        let result = should_display_error_correctly_without_google_test_macro();

        verify_that!(
            // We hereby assume that the Rust test harness uses std::fmt::Debug to output the Err
            // variant of a Result.
            format!("{:?}", result.unwrap_err()),
            contains_substring(indoc! {"
                Value of: 1
                Expected: is equal to 2
                Actual: 1, which isn't equal to 2
                "})
        )
    }

    // This is not marked as a test since it deliberately fails.
    fn should_display_error_correctly_without_google_test_macro() -> Result<()> {
        verify_that!(1, eq(2))
    }

    #[google_test]
    fn failure_message_uses_pretty_print_for_actual_value() -> Result<()> {
        #[derive(Debug)]
        #[allow(unused)]
        struct NontrivialStruct {
            a: i32,
            b: i32,
        }
        let value = NontrivialStruct { a: 1, b: 2 };
        let failed_assertion_result = verify_that!(value, not(anything()));

        verify_that!(
            failed_assertion_result,
            err(displays_as(contains_substring(indoc! {"
                Actual: NontrivialStruct {
                    a: 1,
                    b: 2,
                }"})))
        )
    }

    #[google_test]
    fn test_with_google_test_and_rstest_runs_only_once() -> Result<()> {
        let output = run_external_process_in_tests_directory("google_test_with_rstest")?;

        expect_that!(
            output,
            contains_substring("tests::test_should_work_with_rstest_second").times(eq(1))
        );
        verify_that!(
            output,
            contains_substring("tests::test_should_work_with_qualified_rstest_second").times(eq(1))
        )
    }

    fn run_external_process_in_tests_directory(name: &'static str) -> Result<String> {
        let mut command = run_external_process(name);
        let std::process::Output { stdout, .. } = command.output()?;
        Ok(String::from_utf8(stdout)?)
    }

    fn run_external_process(name: &'static str) -> Command {
        let command_path = format!(
            "./{}/debug/{name}",
            std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into())
        );
        Command::new(command_path)
    }
}
