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
    use googletest::prelude::*;
    use indoc::indoc;
    use std::process::Command;

    #[test]
    fn should_pass() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(2))
    }

    #[test]
    fn verify_that_supports_trailing_comma() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(2),)
    }

    #[test]
    fn should_pass_with_omitted_elements_are() -> Result<()> {
        verify_that!(vec![1, 2], [eq(1), eq(2)])
    }

    #[test]
    fn should_pass_with_omitted_unordered_elements_are() -> Result<()> {
        verify_that!(vec![1, 2], {eq(2), eq(1)})
    }

    #[test]
    fn verify_that_with_short_elements_are_syntax_supports_trailing_comma() -> Result<()> {
        verify_that!(vec![1, 2], [eq(1), eq(2),])
    }

    #[test]
    fn verify_that_with_short_unordered_elements_are_syntax_supports_trailing_comma() -> Result<()>
    {
        verify_that!(vec![1, 2], {eq(2), eq(1),})
    }

    #[test]
    fn should_pass_with_assert_that() {
        let value = 2;
        assert_that!(value, eq(2));
    }

    #[test]
    fn assert_that_supports_trailing_comma() {
        let value = 2;
        assert_that!(value, eq(2),);
    }

    #[test]
    fn assert_that_with_custom_failure_message_supports_trailing_comma() {
        let value = 2;
        assert_that!(value, eq(2), "A custom error message",);
    }

    #[googletest::test]
    fn should_pass_with_expect_that() -> Result<()> {
        let value = 2;
        expect_that!(value, eq(2));
        Ok(())
    }

    #[googletest::test]
    fn should_pass_with_expect_that_returning_unit() {
        let value = 2;
        expect_that!(value, eq(2));
    }

    #[googletest::test]
    fn expect_that_supports_trailing_comma() {
        let value = 2;
        expect_that!(value, eq(2),);
    }

    #[googletest::test]
    fn expect_that_with_custom_failure_message_supports_trailing_comma() {
        let value = 2;
        expect_that!(value, eq(2), "A custom error message",);
    }

    #[test]
    fn should_fail_on_assertion_failure() -> Result<()> {
        let status = run_external_process("simple_assertion_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[test]
    fn should_fail_on_assertion_failure_with_assert_that() -> Result<()> {
        let status = run_external_process("simple_assertion_failure_with_assert_that").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[test]
    fn should_fail_on_assertion_failure_with_expect_that() -> Result<()> {
        let status = run_external_process("expect_that_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[test]
    fn should_output_failure_message_on_assertion_failure() -> Result<()> {
        let output = run_external_process_in_tests_directory("simple_assertion_failure")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Value of: value
                Expected: is equal to 3
                Actual: 2,
                  which isn't equal to 3
                  at .*integration_tests/src/simple_assertion_failure.rs:[0-9]+:9"})
        )
    }

    #[test]
    fn should_output_failure_message_on_assertion_failure_with_assert_that() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("simple_assertion_failure_with_assert_that")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Value of: value
                Expected: is equal to 3
                Actual: 2,
                  which isn't equal to 3
                  at .*integration_tests/src/simple_assertion_failure_with_assert_that.rs:[0-9]+:9
                "})
        )
    }

    #[test]
    fn should_output_failure_message_on_assertion_failure_with_expect_that() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_that_failure")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Value of: value
                Expected: is equal to 3
                Actual: 2,
                  which isn't equal to 3
                  at .*integration_tests/src/expect_that_failure.rs:[0-9]+:9
                "})
        )
    }

    #[test]
    fn should_output_both_failure_messages_when_two_expect_that_assertions_fail() -> Result<()> {
        let output = run_external_process_in_tests_directory("two_expect_that_failures")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Value of: value
                Expected: is equal to 3
                Actual: 2,
                  which isn't equal to 3
                  at .*integration_tests/src/two_expect_that_failures.rs:[0-9]+:9

                Value of: value
                Expected: is equal to 4
                Actual: 2,
                  which isn't equal to 4
                  at .*integration_tests/src/two_expect_that_failures.rs:[0-9]+:9
                "})
        )
    }

    #[googletest::test]
    fn should_output_failure_message_with_simple_structured_value() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "assertion_failures_with_short_structured_actual_values",
        )?;

        expect_that!(
            output,
            contains_substring(indoc! {"
                Value of: Some(1)
                Expected: has a value which is equal to 2
                Actual: Some(1),
                  which has a value
                    which isn't equal to 2
            "})
        );
        expect_that!(
            output,
            contains_substring(indoc! {"
                Value of: value
                Expected: is a success containing a value, which is equal to 2
                Actual: Ok(1),
                  which is a success
                    which isn't equal to 2
            "})
        );
        expect_that!(
            output,
            contains_substring(indoc! {"
                Value of: value
                Expected: is an error which is equal to 2
                Actual: Err(1),
                  which is an error
                    which isn't equal to 2
            "})
        );
        Ok(())
    }

    #[test]
    fn should_fail_due_to_assertion_failure_in_subroutine() -> Result<()> {
        let status = run_external_process("simple_assertion_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[test]
    fn should_fail_due_to_returned_error_in_subroutine() -> Result<()> {
        let status = run_external_process("failure_due_to_returned_error").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[test]
    fn should_fail_test_on_and_log_failure() -> Result<()> {
        let status = run_external_process("non_fatal_failure_in_subroutine").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[test]
    fn should_log_test_failures_to_stdout() -> Result<()> {
        let output = run_external_process_in_tests_directory("two_non_fatal_failures")?;

        verify_that!(
            output,
            all!(
                contains_substring(indoc! {"
                    Expected: is equal to 3
                    Actual: 2,
                      which isn't equal to 3
                    "}),
                contains_substring(indoc! {"
                    Expected: is equal to 4
                    Actual: 2,
                      which isn't equal to 4
                    "})
            )
        )
    }

    #[test]
    fn should_log_fatal_and_non_fatal_errors_to_stdout() -> Result<()> {
        let output = run_external_process_in_tests_directory("fatal_and_non_fatal_failure")?;

        verify_that!(
            output,
            all!(
                contains_substring(indoc! {"
                    Expected: is equal to 3
                    Actual: 2,
                      which isn't equal to 3
                    "}),
                contains_substring(indoc! {"
                    Expected: is equal to 4
                    Actual: 2,
                      which isn't equal to 4
                    "})
            )
        )
    }

    #[test]
    fn should_abort_after_first_failure() -> Result<()> {
        let output = run_external_process_in_tests_directory("first_failure_aborts")?;

        verify_that!(
            output,
            not(contains_substring(indoc! {"
                Expected: is equal to 4
                Actual: 2,
                  which isn't equal to 4
                "}))
        )
    }

    #[test]
    fn should_fail_with_assertion_in_a_subroutine() -> Result<()> {
        let output = run_external_process_in_tests_directory("non_fatal_failure_in_subroutine")?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                Expected: is equal to 3
                Actual: 2,
                  which isn't equal to 3
                "})
        )
    }

    #[test]
    fn should_include_custom_error_message_in_failure() -> Result<()> {
        let output = run_external_process_in_tests_directory("custom_error_message")?;

        verify_that!(output, contains_substring("A custom error message"))?;
        verify_that!(output, contains_substring("A custom error message in a String"))?;
        verify_that!(output, contains_substring("A custom error message from a closure"))?;
        verify_that!(
            output,
            contains_substring("assert_that: A custom error message for value 2")
        )?;
        verify_that!(
            output,
            contains_substring("assert_that: A custom error message for incremented value 3")
        )?;
        verify_that!(
            output,
            contains_substring("assert_that: A custom error message for twice incremented value 4")
        )?;
        verify_that!(
            output,
            contains_substring("expect_that: A custom error message for value 2")
        )?;
        verify_that!(
            output,
            contains_substring("expect_that: A custom error message for incremented value 3")
        )?;
        verify_that!(
            output,
            contains_substring("expect_that: A custom error message for twice incremented value 4")
        )
    }

    #[test]
    fn should_not_run_closure_with_custom_error_message_if_test_passes() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(2))
            .with_failure_message(|| panic!("This should not execute, since the assertion passes."))
    }

    #[test]
    fn should_verify_predicate_with_success() -> Result<()> {
        verify_pred!(eq_predicate(1, 1))
    }

    #[test]
    fn should_verify_predicate_with_trailing_comma() -> Result<()> {
        verify_pred!(eq_predicate(1, 1,))
    }

    fn eq_predicate(a: i32, b: i32) -> bool {
        a == b
    }

    #[googletest::test]
    fn should_verify_predicate_with_success_using_expect_pred() -> Result<()> {
        expect_pred!(eq_predicate(1, 1));
        Ok(())
    }

    #[test]
    fn verify_pred_should_fail_test_on_failure() -> Result<()> {
        let status = run_external_process("verify_predicate_with_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[test]
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

    #[test]
    fn assert_pred_should_fail_test_on_failure() -> Result<()> {
        let status = run_external_process("assert_predicate_with_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[test]
    fn expect_pred_should_fail_test_on_failure() -> Result<()> {
        let status = run_external_process("expect_pred_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[test]
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

    #[test]
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

    #[test]
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

    #[test]
    fn should_verify_predicate_in_a_submodule() -> Result<()> {
        verify_pred!(submodule::eq_predicate_in_submodule(1, 1))
    }

    mod submodule {
        pub(super) fn eq_predicate_in_submodule(a: i32, b: i32) -> bool {
            a == b
        }
    }

    #[test]
    fn should_verify_predicate_as_a_method() -> Result<()> {
        let a_struct = AStruct {};

        verify_pred!(a_struct.eq_predicate_as_method(1, 1))
    }

    #[test]
    fn should_verify_predicate_as_a_method_on_an_expresion_result() -> Result<()> {
        verify_pred!((AStruct {}).eq_predicate_as_method(1, 1))
    }

    struct AStruct {}

    impl AStruct {
        fn eq_predicate_as_method(&self, a: i32, b: i32) -> bool {
            a == b
        }
    }

    #[test]
    fn should_verify_predicate_as_a_method_in_submodule() -> Result<()> {
        verify_pred!(another_submodule::A_STRUCT_IN_SUBMODULE.eq_predicate_as_method(1, 1))
    }

    mod another_submodule {
        pub(super) static A_STRUCT_IN_SUBMODULE: super::AStruct = super::AStruct {};
    }

    #[test]
    fn should_verify_predicate_as_a_method_on_a_field() -> Result<()> {
        let another_struct = AnotherStruct { a_struct: AStruct {} };

        verify_pred!(another_struct.a_struct.eq_predicate_as_method(1, 1))
    }

    struct AnotherStruct {
        a_struct: AStruct,
    }

    #[test]
    #[rustversion::before(1.76)]
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

    #[test]
    #[rustversion::since(1.76)]
    fn verify_pred_should_show_correct_qualified_function_name_in_test_failure_output() -> Result<()>
    {
        let output = run_external_process_in_tests_directory(
            "verify_predicate_with_failure_as_method_in_submodule",
        )?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                a_submodule::A_STRUCT_IN_SUBMODULE.eq_predicate_as_method(a, b) was false with
                  a = 1,
                  b = 2
                "})
        )
    }

    #[test]
    fn fail_macro_causes_test_failure() -> Result<()> {
        let status = run_external_process("failure_due_to_fail_macro").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[test]
    fn fail_macro_outputs_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("failure_due_to_fail_macro")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected test failure
                  at .*integration_tests/src/failure_due_to_fail_macro.rs:[0-9]+:9
                "})
        )
    }

    #[test]
    fn fail_macro_allows_empty_message() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "failure_due_to_fail_macro_with_empty_message",
        )?;

        verify_that!(output, contains_substring("Test failed"))
    }

    #[test]
    fn fail_macro_allows_message_with_format_arguments() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "failure_due_to_fail_macro_with_format_arguments",
        )?;

        verify_that!(output, contains_substring("Failure message with argument: An argument"))
    }

    #[test]
    fn test_using_normal_test_attribute_macro_formats_failure_message_correctly() -> Result<()> {
        let result = should_display_error_correctly_without_google_test_macro();

        verify_that!(
            // We hereby assume that the Rust test harness uses std::fmt::Debug to output the Err
            // variant of a Result.
            format!("{:?}", result.unwrap_err()),
            contains_substring(indoc! {"
                Value of: 1
                Expected: is equal to 2
                Actual: 1,
                  which isn't equal to 2
                "})
        )
    }

    // This is not marked as a test since it deliberately fails.
    fn should_display_error_correctly_without_google_test_macro() -> Result<()> {
        verify_that!(1, eq(2))
    }

    #[test]
    fn failure_message_uses_pretty_print_for_actual_value_when_long_enough() -> Result<()> {
        #[derive(Debug)]
        #[allow(unused)]
        struct NontrivialStruct {
            a: &'static str,
            b: &'static str,
        }
        let value = NontrivialStruct { a: "A long enough string", b: "Another long enough string" };
        let failed_assertion_result = verify_that!(value, not(anything()));

        verify_that!(
            failed_assertion_result,
            err(displays_as(contains_substring(indoc! {r#"
                Actual: NontrivialStruct {
                    a: "A long enough string",
                    b: "Another long enough string",
                }"#})))
        )
    }

    #[googletest::test]
    fn test_with_google_test_and_rstest_runs_only_once() -> Result<()> {
        let output = run_external_process_in_tests_directory("google_test_with_rstest")?;

        expect_that!(
            output,
            contains_substring("tests::test_should_work_with_rstest_first").times(eq(1))
        );
        expect_that!(
            output,
            contains_substring("tests::test_should_work_with_rstest_second").times(eq(1))
        );
        expect_that!(
            output,
            contains_substring("tests::test_should_work_with_qualified_rstest_first").times(eq(1))
        );
        expect_that!(
            output,
            contains_substring("tests::test_should_work_with_qualified_rstest_second").times(eq(1))
        );
        expect_that!(
            output,
            contains_substring("tests::test_should_work_with_qualified_test_annotation")
                .times(eq(1))
        );
        verify_that!(
            output,
            contains_substring("tests::test_should_work_with_second_test_annotation").times(eq(1))
        )
    }

    #[googletest::test]
    fn async_test_with_google_test_runs_correctly() -> Result<()> {
        let output = run_external_process_in_tests_directory("async_test_with_expect_that")?;

        expect_that!(
            output,
            contains_substring("tests::async_test_failure_with_non_fatal_assertion ... FAILED")
                .times(eq(1))
        );
        expect_that!(
            output,
            contains_substring("tests::async_test_failure_with_fatal_assertion ... FAILED")
                .times(eq(1))
        );
        expect_that!(output, contains_substring("Expected: is equal to 3"));
        verify_that!(output, contains_substring("Expected: is equal to 4"))
    }

    #[test]
    fn test_can_return_anyhow_generated_error() -> Result<()> {
        let output = run_external_process_in_tests_directory("test_returning_anyhow_error")?;

        verify_that!(output, contains_substring("Error from Anyhow"))
    }

    #[::core::prelude::v1::test]
    #[should_panic]
    fn should_panic_when_expect_that_runs_without_attribute_macro() {
        expect_that!(123, eq(123));
    }

    #[::core::prelude::v1::test]
    #[should_panic]
    fn should_panic_when_and_log_failure_runs_without_attribute_macro() {
        verify_that!(123, eq(123)).and_log_failure();
    }

    #[googletest::test]
    fn should_just_pass() -> Result<()> {
        Ok(())
    }

    #[::core::prelude::v1::test]
    #[should_panic]
    fn should_panic_when_expect_that_runs_without_attribute_macro_after_another_test() {
        // The boilerplate in the attribute googletest::test should reset the test
        // context when the test has finished running. If it fails to do so, then the
        // expect_that! call will see a test context and *not* panic, causing the test
        // to fail.
        let _ = should_just_pass();
        expect_that!(123, eq(123));
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
