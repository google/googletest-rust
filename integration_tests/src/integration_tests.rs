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

    #[gtest]
    fn should_pass() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(2))
    }

    #[gtest]
    fn verify_that_supports_trailing_comma() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(2),)
    }

    #[gtest]
    fn should_pass_with_omitted_elements_are() -> Result<()> {
        verify_that!(vec![1, 2], [eq(&1), eq(&2)])
    }

    #[gtest]
    fn should_pass_with_omitted_unordered_elements_are() -> Result<()> {
        verify_that!(vec![1, 2], {eq(&2), eq(&1)})
    }

    #[gtest]
    fn verify_that_with_short_elements_are_syntax_supports_trailing_comma() -> Result<()> {
        verify_that!(vec![1, 2], [eq(&1), eq(&2),])
    }

    #[gtest]
    fn verify_that_with_short_unordered_elements_are_syntax_supports_trailing_comma() -> Result<()>
    {
        verify_that!(vec![1, 2], {eq(&2), eq(&1),})
    }

    #[gtest]
    fn should_pass_with_assert_that() {
        let value = 2;
        assert_that!(value, eq(2));
    }

    #[gtest]
    fn assert_that_supports_trailing_comma() {
        let value = 2;
        assert_that!(value, eq(2),);
    }

    #[gtest]
    fn assert_that_with_custom_failure_message_supports_trailing_comma() {
        let value = 2;
        assert_that!(value, eq(2), "A custom error message",);
    }

    #[gtest]
    fn assert_that_supports_element_sequences() {
        assert_that!(vec![1, 2], [eq(&1), eq(&2)]);
    }

    #[gtest]
    fn assert_that_supports_unordered_element_sequences() {
        assert_that!(vec![1, 2], {eq(&2), eq(&1)});
    }

    #[gtest]
    fn assert_that_supports_ordered_element_sequences_with_format_string() {
        assert_that!(vec![1, 2], [eq(&1), eq(&2)], "A custom error message");
    }

    #[gtest]
    fn assert_that_supports_unordered_element_sequences_with_format_string() {
        assert_that!(vec![1, 2], {eq(&2), eq(&1)}, "A custom error message");
    }

    #[gtest]
    fn should_pass_with_expect_that() -> Result<()> {
        let value = 2;
        expect_that!(value, eq(2));
        Ok(())
    }

    #[gtest]
    fn should_pass_with_expect_that_returning_unit() {
        let value = 2;
        expect_that!(value, eq(2));
    }

    #[gtest]
    fn expect_that_supports_trailing_comma() {
        let value = 2;
        expect_that!(value, eq(2),);
    }

    #[gtest]
    fn expect_that_with_custom_failure_message_supports_trailing_comma() {
        let value = 2;
        expect_that!(value, eq(2), "A custom error message",);
    }

    #[gtest]
    fn expect_that_with_omitted_elements_are() {
        expect_that!(vec![1, 2], [eq(&1), eq(&2)]);
    }

    #[gtest]
    fn expect_that_with_omitted_elements_supports_custom_error_msg() {
        expect_that!(vec![1, 2], [eq(&1), eq(&2)], "A custom error message");
    }

    #[gtest]
    fn expect_that_with_unordered_elements_supports_custom_error_msg() {
        expect_that!(vec![1, 2], {eq(&2), eq(&1)}, "A custom error message");
    }

    #[gtest]
    fn should_fail_on_assertion_failure() -> Result<()> {
        let status = run_external_process("simple_assertion_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[gtest]
    fn should_fail_on_assertion_failure_with_assert_that() -> Result<()> {
        let status = run_external_process("simple_assertion_failure_with_assert_that").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[gtest]
    fn should_fail_on_assertion_failure_with_expect_that() -> Result<()> {
        let status = run_external_process("expect_that_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[gtest]
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

    #[gtest]
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

    #[gtest]
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

    #[gtest]
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

    #[gtest]
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

    #[gtest]
    fn should_fail_due_to_assertion_failure_in_subroutine() -> Result<()> {
        let status = run_external_process("simple_assertion_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[gtest]
    fn should_fail_due_to_returned_error_in_subroutine() -> Result<()> {
        let status = run_external_process("failure_due_to_returned_error").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[gtest]
    fn should_log_error_location_in_returned_error() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "failure_due_to_returned_error_with_line_numbers",
        )?;

        verify_that!(
            output,
            contains_substring(
                "FakeError\n  at integration_tests/src/failure_due_to_returned_error_with_line_numbers.rs:38:9"
            )
        )
    }

    #[gtest]
    fn should_fail_test_on_and_log_failure() -> Result<()> {
        let status = run_external_process("non_fatal_failure_in_subroutine").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[gtest]
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

    #[gtest]
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

    #[gtest]
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

    #[gtest]
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

    #[gtest]
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

    #[gtest]
    fn should_not_run_closure_with_custom_error_message_if_test_passes() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(2))
            .with_failure_message(|| panic!("This should not execute, since the assertion passes."))
    }

    #[gtest]
    fn should_verify_predicate_with_success() -> Result<()> {
        verify_pred!(eq_predicate(1, 1))
    }

    #[gtest]
    fn should_verify_predicate_with_trailing_comma() -> Result<()> {
        verify_pred!(eq_predicate(1, 1,))
    }

    fn eq_predicate(a: i32, b: i32) -> bool {
        a == b
    }

    #[gtest]
    fn should_verify_predicate_with_success_using_expect_pred() -> Result<()> {
        expect_pred!(eq_predicate(1, 1));
        Ok(())
    }

    #[gtest]
    fn verify_pred_should_fail_test_on_failure() -> Result<()> {
        let status = run_external_process("verify_predicate_with_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[gtest]
    fn verify_pred_should_output_correct_failure_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("verify_predicate_with_failure")?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                eq_predicate(a, b) was false with
                  a = 1,
                  b = 2,
                  at"
            })
        )
    }

    #[gtest]
    fn assert_pred_should_fail_test_on_failure() -> Result<()> {
        let status = run_external_process("assert_predicate_with_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[gtest]
    fn expect_pred_should_fail_test_on_failure() -> Result<()> {
        let status = run_external_process("expect_pred_failure").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[gtest]
    fn assert_pred_should_output_correct_failure_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("assert_predicate_with_failure")?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                eq_predicate(a, b) was false with
                  a = 1,
                  b = 2,
                  at"
            })
        )
    }

    #[gtest]
    fn expect_pred_should_output_correct_failure_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_pred_failure")?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                eq_predicate(a, b) was false with
                  a = 1,
                  b = 2,
                  at"
            })
        )
    }

    #[gtest]
    fn expect_pred_should_output_failure_message_for_second_failure() -> Result<()> {
        let output = run_external_process_in_tests_directory("two_expect_pred_failures")?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                eq_predicate(a, b) was false with
                  a = 3,
                  b = 4,
                  at"
            })
        )
    }

    #[gtest]
    fn should_verify_predicate_in_a_submodule() -> Result<()> {
        verify_pred!(submodule::eq_predicate_in_submodule(1, 1))
    }

    mod submodule {
        pub(super) fn eq_predicate_in_submodule(a: i32, b: i32) -> bool {
            a == b
        }
    }

    #[gtest]
    fn should_verify_predicate_as_a_method() -> Result<()> {
        let a_struct = AStruct {};

        verify_pred!(a_struct.eq_predicate_as_method(1, 1))
    }

    #[gtest]
    fn should_verify_predicate_as_a_method_on_an_expresion_result() -> Result<()> {
        verify_pred!(AStruct {}.eq_predicate_as_method(1, 1))
    }

    struct AStruct {}

    impl AStruct {
        fn eq_predicate_as_method(&self, a: i32, b: i32) -> bool {
            a == b
        }
    }

    #[gtest]
    fn should_verify_predicate_as_a_method_in_submodule() -> Result<()> {
        verify_pred!(another_submodule::A_STRUCT_IN_SUBMODULE.eq_predicate_as_method(1, 1))
    }

    mod another_submodule {
        pub(super) static A_STRUCT_IN_SUBMODULE: super::AStruct = super::AStruct {};
    }

    #[gtest]
    fn should_verify_predicate_as_a_method_on_a_field() -> Result<()> {
        let another_struct = AnotherStruct { a_struct: AStruct {} };

        verify_pred!(another_struct.a_struct.eq_predicate_as_method(1, 1))
    }

    struct AnotherStruct {
        a_struct: AStruct,
    }

    #[gtest]
    fn verify_pred_should_show_correct_qualified_function_name_in_test_failure_output() -> Result<()>
    {
        let output = run_external_process_in_tests_directory(
            "verify_predicate_with_failure_as_method_in_submodule",
        )?;

        verify_that!(
            output,
            contains_substring(indoc! {"
                a_submodule :: A_STRUCT_IN_SUBMODULE.eq_predicate_as_method(a, b) was false with
                  a_submodule :: A_STRUCT_IN_SUBMODULE does not implement Debug,
                  a = 1,
                  b = 2,
                  at"
            })
        )
    }

    #[gtest]
    fn fail_macro_causes_test_failure() -> Result<()> {
        let status = run_external_process("failure_due_to_fail_macro").status()?;

        verify_that!(status.success(), eq(false))
    }

    #[gtest]
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

    #[gtest]
    fn fail_macro_allows_empty_message() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "failure_due_to_fail_macro_with_empty_message",
        )?;

        verify_that!(output, contains_substring("Test failed"))
    }

    #[gtest]
    fn fail_macro_allows_message_with_format_arguments() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "failure_due_to_fail_macro_with_format_arguments",
        )?;

        verify_that!(output, contains_substring("Failure message with argument: An argument"))
    }

    #[gtest]
    fn succeed_macro_test_completes() -> Result<()> {
        let output = run_external_process_in_tests_directory_with_args(
            "success_with_succeed_macro",
            &["--nocapture"],
        )?;

        verify_that!(output, contains_regex("Should do nothing"))
    }

    #[gtest]
    fn succeed_macro_allows_empty_message() -> Result<()> {
        let output = run_external_process_in_tests_directory_with_args(
            "success_with_succeed_macro_with_empty_message",
            &["--nocapture"],
        )?;

        verify_that!(output, contains_regex("Success"))
    }

    #[gtest]
    fn succeed_macro_with_format_arguments() -> Result<()> {
        let output = run_external_process_in_tests_directory_with_args(
            "success_with_succeed_macro_with_format_arguments",
            &["--nocapture"],
        )?;

        verify_that!(output, contains_regex("Success message with argument: An argument"))
    }

    #[gtest]
    fn add_failure_macro_causes_failure_but_continues_execution() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "add_failure_macro_causes_failure_but_continues_execution",
        )?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                First failure
                  at .*integration_tests/src/add_failure_macro_causes_failure_but_continues_execution.rs:[0-9]+:9
                "})
        );
        expect_that!(
            output,
            contains_regex(indoc! {"
                Second failure
                  at .*integration_tests/src/add_failure_macro_causes_failure_but_continues_execution.rs:[0-9]+:9
                "})
        );
        Ok(())
    }

    #[gtest]
    fn add_failure_macro_allows_empty_message() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("add_failure_macro_allows_empty_message")?;

        verify_that!(output, contains_regex("Failed"))
    }

    #[gtest]
    fn add_failure_macro_allows_formatted_arguments() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "add_failure_macro_allows_formatted_arguments",
        )?;

        verify_that!(output, contains_regex("Failure message with argument: An argument"))
    }

    #[gtest]
    fn add_failure_macro_needs_googletest_attribute() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "add_failure_macro_needs_googletest_attribute",
        )?;

        verify_that!(output, contains_regex("Did you annotate the test with gtest?"))
    }

    #[gtest]
    fn add_failure_at_macro_causes_failure_but_continues_execution() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "add_failure_at_macro_causes_failure_but_continues_execution",
        )?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                First failure
                  at .*first_file.rs:32:12
                "})
        );
        expect_that!(
            output,
            contains_regex(indoc! {"
                Second failure
                  at .*second_file.rs:32:12
                "})
        );
        Ok(())
    }

    #[gtest]
    fn add_failure_at_macro_allows_empty_message() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("add_failure_at_macro_allows_empty_message")?;

        verify_that!(output, contains_regex("Failed"))
    }

    #[gtest]
    fn add_failure_at_macro_allows_formatted_arguments() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "add_failure_at_macro_allows_formatted_arguments",
        )?;

        verify_that!(output, contains_regex("Failure message with argument: An argument"))
    }

    #[gtest]
    fn add_failure_at_macro_needs_googletest_attribute() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "add_failure_at_macro_needs_googletest_attribute",
        )?;

        verify_that!(output, contains_regex("Did you annotate the test with gtest?"))
    }

    #[gtest]
    fn verify_true_when_true_returns_ok() {
        assert!(verify_true!("test" == "test").is_ok())
    }

    #[gtest]
    fn verify_true_when_false_returns_err() {
        assert!(verify_true!(2 + 2 == 5).is_err())
    }

    #[gtest]
    fn verify_true_macro_on_false_condition_logs_error_when_handled() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("verify_true_macro_on_false_condition")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected: is equal to true
                Actual: false,
                  which isn't equal to true
                  at .*verify_true_macro_on_false_condition.rs:23:9
                "})
        )
    }

    #[gtest]
    fn expect_true_macro_on_true_condition_does_nothing() {
        expect_true!(2 + 2 == 4)
    }

    #[gtest]
    fn expect_true_macro_on_false_condition_fails_test_and_continues() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "expect_true_macro_on_false_condition_fails_test_and_continues",
        )?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is equal to true
                Actual: false,
                  which isn't equal to true
                  at .*expect_true_macro_on_false_condition_fails_test_and_continues.rs:23:9
                "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn verify_false_when_false_returns_ok() {
        assert!(verify_false!("test" == "not test").is_ok())
    }

    #[gtest]
    fn verify_false_when_true_returns_err() {
        assert!(verify_false!(2 + 2 == 4).is_err())
    }

    #[gtest]
    fn verify_false_macro_on_true_condition_logs_error_when_handled() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("verify_false_macro_on_true_condition")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected: is equal to false
                Actual: true,
                  which isn't equal to false
                  at .*verify_false_macro_on_true_condition.rs:23:9
                "})
        )
    }

    #[gtest]
    fn expect_false_macro_on_false_condition_does_nothing() {
        expect_false!(2 + 2 == 5)
    }

    #[gtest]
    fn expect_false_macro_on_true_condition_fails_test_and_continues() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "expect_false_macro_on_true_condition_fails_test_and_continues",
        )?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is equal to false
                Actual: true,
                  which isn't equal to false
                  at .*expect_false_macro_on_true_condition_fails_test_and_continues.rs:23:9
                "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn verify_eq_should_pass() -> Result<()> {
        let value = 2;
        verify_eq!(value, 2)
    }

    #[gtest]
    fn verify_eq_with_non_copyable_type() -> Result<()> {
        #[derive(Debug, PartialEq)]
        struct NonCopyable(i32);
        verify_eq!(NonCopyable(123), NonCopyable(123))
    }

    #[gtest]
    fn verify_eq_supports_trailing_comma() -> Result<()> {
        let value = 2;
        verify_eq!(value, 2,)
    }

    #[gtest]
    fn verify_eq_supports_tuples() -> Result<()> {
        verify_eq!((Some("a"), Some("b")), (Some("a"), Some("b")))
    }

    #[gtest]
    fn verify_eq_supports_ordered_elements() -> Result<()> {
        verify_eq!(vec![1, 2], [1, 2])
    }

    #[gtest]
    fn verify_eq_supports_ordered_elements_with_non_primitives() -> Result<()> {
        verify_eq!(vec![Some("a"), Some("b")], [Some("a"), Some("b")])
    }

    #[gtest]
    fn verify_eq_supports_unordered_elements() -> Result<()> {
        verify_eq!(vec![1, 2], {2, 1})
    }

    #[gtest]
    fn verify_eq_supports_unordered_elements_with_non_primitives() -> Result<()> {
        verify_eq!(vec![Some("a"), Some("b")], {Some("b"), Some("a")})
    }

    #[gtest]
    fn verify_eq_supports_ordered_elements_with_trailing_comma() -> Result<()> {
        verify_eq!(vec![1, 2], [1, 2,],)
    }

    #[gtest]
    fn verify_eq_supports_unordered_elements_with_trailing_comma() -> Result<()> {
        verify_eq!(vec![1, 2], {2, 1,},)
    }

    #[gtest]
    fn verify_eq_reports_diff_in_trailing_newline() -> Result<()> {
        let result = verify_eq!("hello\nworld\n", "hello\nworld");
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc! {r#"
                    Expected: is equal to "hello\nworld"
                    Actual: "hello\nworld\n",
                      which isn't equal to "hello\nworld"
                      
                      Actual includes a terminating newline that is absent from expected.
                "#})))
        )
    }

    #[gtest]
    fn verify_eq_when_not_equal_returns_error() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("verify_eq_when_not_equal_returns_error")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected: is equal to 1
                Actual: 2,
                  which isn't equal to 1
                  at .*verify_eq_when_not_equal_returns_error.rs:23:9
                "})
        )
    }

    #[gtest]
    fn verify_eq_with_ordered_elements_when_not_equal_returns_error() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "verify_eq_with_ordered_elements_when_not_equal_returns_error",
        )?;

        verify_that!(
            output,
            contains_regex(indoc! {r"
                Expected: has elements:
                  0. is equal to 1
                  1. is equal to 3
                Actual: \[1, 2\],
                  where element #1 is 2, which isn't equal to 3
                  at .*verify_eq_with_ordered_elements_when_not_equal_returns_error.rs:23:9
                "})
        )
    }

    #[gtest]
    fn verify_eq_with_unordered_elements_when_not_equal_returns_error() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "verify_eq_with_unordered_elements_when_not_equal_returns_error",
        )?;

        verify_that!(
            output,
            contains_regex(indoc! {r"
                Expected: contains elements matching in any order:
                  0. is equal to 3
                  1. is equal to 1
                Actual: \[1, 2\],
                  whose element #1 does not match any expected elements and no elements match the expected element #0
                  at .*verify_eq_with_unordered_elements_when_not_equal_returns_error.rs:23:9
                "})
        )
    }

    #[gtest]
    fn expect_eq_should_pass() {
        let value = 2;
        expect_eq!(value, 2);
    }

    #[gtest]
    fn expect_eq_with_non_copyable_type() {
        #[derive(Debug, PartialEq)]
        struct NonCopyable(i32);
        expect_eq!(NonCopyable(123), NonCopyable(123));
    }

    #[gtest]
    fn expect_eq_should_allow_multiple_calls() {
        expect_eq!(1, 1);
        expect_eq!(2, 2);
    }

    #[gtest]
    fn expect_eq_supports_trailing_comma() {
        let value = 2;
        expect_eq!(value, 2,);
    }

    #[gtest]
    fn expect_eq_supports_tuples() {
        expect_eq!((Some("a"), Some("b")), (Some("a"), Some("b")));
    }

    #[gtest]
    fn expect_eq_supports_ordered_elements() {
        expect_eq!(vec![1, 2], [1, 2]);
    }

    #[gtest]
    fn expect_eq_supports_ordered_elements_with_non_primitives() {
        expect_eq!(vec![Some("a"), Some("b")], [Some("a"), Some("b")]);
    }

    #[gtest]
    fn expect_eq_supports_ordered_elements_with_trailing_comma() {
        expect_eq!(vec![1, 2], [1, 2,],);
    }

    #[gtest]
    fn expect_eq_supports_unordered_elements() {
        expect_eq!(vec![1, 2], {2, 1});
    }

    #[gtest]
    fn expect_eq_supports_unordered_elements_with_trailing_comma() {
        expect_eq!(vec![1, 2], {2, 1,},);
    }

    #[gtest]
    fn expect_eq_when_not_equal_returns_error() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("expect_eq_when_not_equal_returns_error")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is equal to 1
                Actual: 2,
                  which isn't equal to 1
                  at .*expect_eq_when_not_equal_returns_error.rs:23:9
                "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_eq_supports_custom_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_eq_supports_custom_message")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is equal to 1
                Actual: 2,
                  which isn't equal to 1
                Failure message with argument: argument
                  at .*expect_eq_supports_custom_message.rs:[0-9]+:[0-9]
                "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_eq_with_ordered_elements_when_not_equal_returns_error() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "expect_eq_with_ordered_elements_when_not_equal_returns_error",
        )?;

        expect_that!(
            output,
            contains_regex(indoc! {r"
                Expected: has elements:
                  0. is equal to 1
                  1. is equal to 3
                Actual: \[1, 2\],
                  where element #1 is 2, which isn't equal to 3
                  at .*expect_eq_with_ordered_elements_when_not_equal_returns_error.rs:[0-9]+:[0-9]
                "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_eq_with_ordered_elements_supports_custom_message() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "expect_eq_with_ordered_elements_supports_custom_message",
        )?;

        expect_that!(
            output,
            contains_regex(indoc! {r"
                Expected: has elements:
                  0. is equal to 1
                  1. is equal to 3
                Actual: \[1, 2\],
                  where element #1 is 2, which isn't equal to 3
                Failure message with argument: argument
                  at .*expect_eq_with_ordered_elements_supports_custom_message.rs:[0-9]+:[0-9]
                "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_eq_with_unordered_elements_when_not_equal_returns_error() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "expect_eq_with_unordered_elements_when_not_equal_returns_error",
        )?;

        expect_that!(
            output,
            contains_regex(indoc! {r"
                Expected: contains elements matching in any order:
                  0. is equal to 3
                  1. is equal to 1
                Actual: \[1, 2\],
                  whose element #1 does not match any expected elements and no elements match the expected element #0
                  at .*expect_eq_with_unordered_elements_when_not_equal_returns_error.rs:[0-9]+:[0-9]
                "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_eq_with_unordered_elements_supports_custom_message() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "expect_eq_with_unordered_elements_supports_custom_message",
        )?;

        expect_that!(
            output,
            contains_regex(indoc! {r"
                Expected: contains elements matching in any order:
                  0. is equal to 3
                  1. is equal to 1
                Actual: \[1, 2\],
                  whose element #1 does not match any expected elements and no elements match the expected element #0
                Failure message with argument: argument
                  at .*expect_eq_with_unordered_elements_supports_custom_message.rs:[0-9]+:[0-9]
                "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn verify_ne_should_pass() -> Result<()> {
        let value = 2;
        verify_ne!(value, 3)
    }

    #[gtest]
    fn verify_ne_with_non_copyable_type() -> Result<()> {
        #[derive(Debug, PartialEq)]
        struct NonCopyable(i32);
        verify_ne!(NonCopyable(123), NonCopyable(321))
    }

    #[gtest]
    fn verify_ne_supports_trailing_comma() -> Result<()> {
        let value = 2;
        verify_ne!(value, 3,)
    }

    #[gtest]
    fn verify_ne_when_equal_returns_error() -> Result<()> {
        let output = run_external_process_in_tests_directory("verify_ne_when_equal_returns_error")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected: isn't equal to 1
                Actual: 1,
                  which is equal to 1
                  at .*verify_ne_when_equal_returns_error.rs:[0-9]+:[0-9]
            "})
        )
    }

    #[gtest]
    fn expect_ne_should_pass() {
        expect_ne!(1, 2);
    }

    #[gtest]
    fn expect_ne_with_non_copyable_type() {
        #[derive(Debug, PartialEq)]
        struct NonCopyable(i32);
        expect_ne!(NonCopyable(123), NonCopyable(321));
    }

    #[gtest]
    fn expect_ne_should_allow_multiple_calls() {
        expect_ne!(1, 2);
        expect_ne!(1, 3);
    }

    #[gtest]
    fn expect_ne_supports_trailing_comma() {
        expect_ne!(1, 2,);
    }

    #[gtest]
    fn expect_ne_when_equal_marks_failed() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_ne_when_equal_marks_failed")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: isn't equal to 1
                Actual: 1,
                  which is equal to 1
                  at .*expect_ne_when_equal_marks_failed.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_ne_supports_custom_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_ne_supports_custom_message")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: isn't equal to 1
                Actual: 1,
                  which is equal to 1
                Failure message with argument: argument
                  at .*expect_ne_supports_custom_message.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn verify_lt_should_pass() -> Result<()> {
        let value = 2;
        verify_lt!(value, 3)
    }

    #[gtest]
    fn verify_lt_supports_trailing_comma() -> Result<()> {
        let value = 2;
        verify_lt!(value, 3,)
    }

    #[gtest]
    fn verify_lt_when_not_less_returns_error() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("verify_lt_when_not_less_returns_error")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected: is less than 1
                Actual: 2,
                  which is greater than or equal to 1
                  at .*verify_lt_when_not_less_returns_error.rs:[0-9]+:[0-9]
            "})
        )
    }

    #[gtest]
    fn expect_lt_should_pass() {
        expect_lt!(1, 2);
    }

    #[gtest]
    fn expect_lt_should_allow_multuple_calls() {
        expect_lt!(1, 2);
        expect_lt!(1, 3);
    }

    #[gtest]
    fn expect_lt_supports_trailing_comma() {
        expect_lt!(1, 2,);
    }

    #[gtest]
    fn expect_lt_when_not_less_marks_failed() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("expect_lt_when_not_less_marks_failed")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is less than 1
                Actual: 1,
                  which is greater than or equal to 1
                  at .*expect_lt_when_not_less_marks_failed.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_lt_supports_custom_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_lt_supports_custom_message")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is less than 1
                Actual: 1,
                  which is greater than or equal to 1
                Failure message with argument: argument
                  at .*expect_lt_supports_custom_message.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn verify_le_should_pass() -> Result<()> {
        verify_le!(1, 1)
    }

    #[gtest]
    fn verify_le_supports_trailing_comma() -> Result<()> {
        verify_le!(1, 2,)
    }

    #[gtest]
    fn verify_le_when_greater_returns_error() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("verify_le_when_greater_returns_error")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected: is less than or equal to 1
                Actual: 2,
                  which is greater than 1
                  at .*verify_le_when_greater_returns_error.rs:[0-9]+:[0-9]
            "})
        )
    }

    #[gtest]
    fn expect_le_should_pass() {
        expect_le!(1, 1);
    }

    #[gtest]
    fn expect_le_should_allow_multiple_calls() {
        expect_le!(1, 1);
        expect_le!(1, 2);
    }

    #[gtest]
    fn expect_le_supports_trailing_comma() {
        expect_le!(1, 2,);
    }

    #[gtest]
    fn expect_le_when_greater_marks_failed() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("expect_le_when_greater_marks_failed")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is less than or equal to 1
                Actual: 2,
                  which is greater than 1
                  at .*expect_le_when_greater_marks_failed.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_le_supports_custom_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_le_supports_custom_message")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is less than or equal to 1
                Actual: 2,
                  which is greater than 1
                Failure message with argument: argument
                  at .*expect_le_supports_custom_message.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn verify_gt_should_pass() -> Result<()> {
        verify_gt!(2, 1)
    }

    #[gtest]
    fn verify_gt_supports_trailing_comma() -> Result<()> {
        verify_gt!(2, 1,)
    }

    #[gtest]
    fn verify_gt_when_not_greater_returns_error() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("verify_gt_when_not_greater_returns_error")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected: is greater than 2
                Actual: 1,
                  which is less than or equal to 2
                  at .*verify_gt_when_not_greater_returns_error.rs:[0-9]+:[0-9]
            "})
        )
    }

    #[gtest]
    fn expect_gt_should_pass() {
        expect_gt!(2, 1);
    }

    #[gtest]
    fn expect_gt_should_allow_multiple_calls() {
        expect_gt!(2, 1);
        expect_gt!(3, 1);
    }

    #[gtest]
    fn expect_gt_supports_trailing_comma() {
        expect_gt!(2, 1,);
    }

    #[gtest]
    fn expect_gt_when_not_greater_marks_failed() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("expect_gt_when_not_greater_marks_failed")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is greater than 2
                Actual: 1,
                  which is less than or equal to 2
                  at .*expect_gt_when_not_greater_marks_failed.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_gt_supports_custom_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_gt_supports_custom_message")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is greater than 2
                Actual: 1,
                  which is less than or equal to 2
                Failure message with argument: argument
                  at .*expect_gt_supports_custom_message.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn verify_ge_should_pass() -> Result<()> {
        verify_ge!(1, 1)
    }

    #[gtest]
    fn verify_ge_supports_trailing_comma() -> Result<()> {
        verify_ge!(2, 1,)
    }

    #[gtest]
    fn verify_ge_when_less_returns_error() -> Result<()> {
        let output = run_external_process_in_tests_directory("verify_ge_when_less_returns_error")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected: is greater than or equal to 2
                Actual: 1,
                  which is less than 2
                  at .*verify_ge_when_less_returns_error.rs:[0-9]+:[0-9]
            "})
        )
    }

    #[gtest]
    fn expect_ge_should_pass() {
        expect_ge!(1, 1);
    }

    #[gtest]
    fn expect_ge_should_allow_multiple_calls() {
        expect_ge!(1, 1);
        expect_ge!(2, 1);
    }

    #[gtest]
    fn expect_ge_supports_trailing_comma() {
        expect_ge!(2, 1,);
    }

    #[gtest]
    fn expect_ge_when_less_marks_failed() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_ge_when_less_marks_failed")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is greater than or equal to 2
                Actual: 1,
                  which is less than 2
                  at .*expect_ge_when_less_marks_failed.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_ge_supports_custom_message() -> Result<()> {
        let output = run_external_process_in_tests_directory("expect_ge_supports_custom_message")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is greater than or equal to 2
                Actual: 1,
                  which is less than 2
                Failure message with argument: argument
                  at .*expect_ge_supports_custom_message.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn verify_float_eq_should_pass() -> Result<()> {
        verify_float_eq!(1.0, 1.0)
    }

    #[gtest]
    fn verify_float_eq_supports_trailing_comma() -> Result<()> {
        verify_float_eq!(1.0, 1.0,)
    }

    #[gtest]
    fn verify_float_eq_when_not_equal_returns_error() -> Result<()> {
        let output = run_external_process_in_tests_directory(
            "verify_float_eq_when_not_equal_returns_error",
        )?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected: is within 1.[0-9]+e-[0-9]+ of 2.0
                Actual: 1.0,
                  which isn't within 1.[0-9]+e-[0-9]+ of 2.0
                  at .*verify_float_eq_when_not_equal_returns_error.rs:[0-9]+:[0-9]
            "})
        )
    }

    #[gtest]
    fn expect_float_eq_should_pass() {
        expect_float_eq!(1.0, 1.0);
    }

    #[gtest]
    fn expect_float_eq_supports_trailing_comma() {
        expect_float_eq!(1.0, 1.0,);
    }

    #[gtest]
    fn expect_float_eq_allows_multiple_invocations() {
        expect_float_eq!(1.0, 1.0);
        expect_float_eq!(2.0, 2.0);
    }

    #[gtest]
    fn expect_float_eq_when_not_equal_marks_failed() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("expect_float_eq_when_not_equal_marks_failed")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is within 1.[0-9]+e-[0-9]+ of 2.0
                Actual: 1.0,
                  which isn't within 1.[0-9]+e-[0-9]+ of 2.0
                  at .*expect_float_eq_when_not_equal_marks_failed.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_float_eq_supports_custom_message() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("expect_float_eq_supports_custom_message")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is within 1.[0-9]+e-[0-9]+ of 2.0
                Actual: 1.0,
                  which isn't within 1.[0-9]+e-[0-9]+ of 2.0
                Failure message with argument: argument
                  at .*expect_float_eq_supports_custom_message.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn verify_near_should_pass() -> Result<()> {
        verify_near!(1.12345, 1.12346, 1e-5)
    }

    #[gtest]
    fn verify_near_supports_trailing_comma() -> Result<()> {
        verify_near!(1.12345, 1.12346, 1e-5,)
    }

    #[gtest]
    fn verify_near_when_not_near_returns_error() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("verify_near_when_not_near_returns_error")?;

        verify_that!(
            output,
            contains_regex(indoc! {"
                Expected: is within 1e-6 of 1.12346
                Actual: 1.12345,
                  which isn't within 1e-6 of 1.12346
                  at .*verify_near_when_not_near_returns_error.rs:[0-9]+:[0-9]
            "})
        )
    }

    #[gtest]
    fn expect_near_should_pass() {
        expect_near!(1.12345, 1.12346, 1e-5);
    }

    #[gtest]
    fn expect_near_should_allow_trailing_comma() {
        expect_near!(1.12345, 1.12346, 1e-5,);
    }

    #[gtest]
    fn expect_near_should_allow_multiple_execution() {
        expect_near!(1.12345, 1.12346, 1e-5);
        expect_near!(1.123456, 1.123457, 1e-6);
    }

    #[gtest]
    fn expect_near_when_not_near_marks_failed() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("expect_near_when_not_near_marks_failed")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is within 1e-6 of 1.12346
                Actual: 1.12345,
                  which isn't within 1e-6 of 1.12346
                  at .*expect_near_when_not_near_marks_failed.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
    fn expect_near_supports_custom_message() -> Result<()> {
        let output =
            run_external_process_in_tests_directory("expect_near_supports_custom_message")?;

        expect_that!(
            output,
            contains_regex(indoc! {"
                Expected: is within 1e-6 of 1.12346
                Actual: 1.12345,
                  which isn't within 1e-6 of 1.12346
                Failure message with argument: argument
                  at .*expect_near_supports_custom_message.rs:[0-9]+:[0-9]
            "})
        );
        verify_that!(output, contains_regex("This will print"))
    }

    #[gtest]
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

    #[gtest]
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

    #[gtest]
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

    #[gtest]
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

    #[gtest]
    fn test_can_return_anyhow_generated_error() -> Result<()> {
        let output = run_external_process_in_tests_directory("test_returning_anyhow_error")?;

        verify_that!(output, contains_substring("Error from Anyhow"))
    }

    #[gtest]
    fn test_can_return_option_generated_error() -> Result<()> {
        let output = run_external_process_in_tests_directory("test_returning_option")?;

        verify_that!(
            output,
            all![
                contains_substring("called `Option::or_fail()` on a `Option::<()>::None` value"),
                contains_substring("test_returning_option.rs:23")
            ]
        )
    }

    #[gtest]
    fn test_can_return_string_error_generated_error() -> Result<()> {
        let output = run_external_process_in_tests_directory("test_returning_string_error")?;

        verify_that!(
            output,
            all![
                contains_substring("Error as a String"),
                contains_substring("test_returning_string_error.rs:23")
            ]
        )
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

    #[gtest]
    fn should_just_pass() -> Result<()> {
        Ok(())
    }

    #[gtest]
    #[should_panic]
    fn should_pass_with_should_panic() {
        expect_that!(2, eq(4));
    }

    #[gtest]
    #[should_panic(expected = "See failure output above")]
    fn should_pass_with_should_panic_with_expectation() {
        expect_that!(2, eq(4));
    }

    #[should_panic]
    #[gtest]
    fn should_pass_with_should_panic_in_first_position() {
        expect_that!(2, eq(4));
    }

    #[gtest]
    #[should_panic]
    fn should_pass_with_should_panic_and_verify_that() -> Result<()> {
        verify_that!(2, eq(4))?;
        verify_that!(3, eq(3))
    }

    #[gtest]
    fn should_fail_when_should_panic_is_present_and_no_panic_occurs() -> Result<()> {
        let output = run_external_process_in_tests_directory("passing_test_with_should_panic")?;

        verify_that!(output, contains_substring("should panic"))
    }

    #[::core::prelude::v1::test]
    #[should_panic]
    fn should_panic_when_expect_that_runs_without_attribute_macro_after_another_test() {
        // The boilerplate in the attribute gtest should reset the test
        // context when the test has finished running. If it fails to do so, then the
        // expect_that! call will see a test context and *not* panic, causing the test
        // to fail.
        let _ = should_just_pass();
        expect_that!(123, eq(123));
    }

    #[gtest]
    fn macros_are_hygenic() -> Result<()> {
        let output = run_external_process_in_tests_directory("macro_hygiene")?;

        verify_that!(
            output,
            all!(
                contains_substring("test tests::verify_that_works ... ok"),
                contains_substring("test tests::verify_pred_works ... ok"),
                contains_substring("test tests::fail_works ... FAILED"),
                contains_substring("test tests::succeed_works ... ok"),
                contains_substring("test tests::add_failure_works ... FAILED"),
                contains_substring("test tests::add_failure_at_works ... FAILED"),
                contains_substring("test tests::verify_true_works ... ok"),
                contains_substring("test tests::expect_true_works ... ok"),
                contains_substring("test tests::verify_false_works ... ok"),
                contains_substring("test tests::expect_false_works ... ok"),
                contains_substring("test tests::verify_eq_works ... ok"),
                contains_substring("test tests::expect_eq_works ... ok"),
                contains_substring("test tests::verify_ne_works ... ok"),
                contains_substring("test tests::expect_ne_works ... ok"),
                contains_substring("test tests::verify_lt_works ... ok"),
                contains_substring("test tests::expect_lt_works ... ok"),
                contains_substring("test tests::verify_le_works ... ok"),
                contains_substring("test tests::expect_le_works ... ok"),
                contains_substring("test tests::verify_gt_works ... ok"),
                contains_substring("test tests::expect_gt_works ... ok"),
                contains_substring("test tests::verify_ge_works ... ok"),
                contains_substring("test tests::expect_ge_works ... ok"),
                contains_substring("test tests::verify_float_eq_works ... ok"),
                contains_substring("test tests::expect_float_eq_works ... ok"),
                contains_substring("test tests::verify_near_works ... ok"),
                contains_substring("test tests::expect_near_works ... ok"),
                contains_substring("test tests::assert_that_works ... ok"),
                contains_substring("test tests::assert_pred_works ... ok"),
                contains_substring("test tests::expect_that_works ... ok"),
                contains_substring("test tests::expect_pred_works ... ok"),
                contains_substring("test result: FAILED. 27 passed; 3 failed;")
            )
        )
    }

    fn run_external_process_in_tests_directory_with_args(
        name: &'static str,
        args: &[&'static str],
    ) -> Result<String> {
        let mut command = run_external_process(name);
        let std::process::Output { stdout, .. } = command.args(args).output()?;
        Ok(String::from_utf8(stdout)?)
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
