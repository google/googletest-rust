#!/bin/bash
#
# Copyright 2022 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Shell script to build and run the library integration tests. These will not be
# run with "cargo test" due to limitations in Cargo.
#
# To use this, just run the script in the root directory of GoogleTest Rust. You
# must have the Rust toolchain available.

set -e

INTEGRATION_TEST_BINARIES=(
  "integration_tests"
  "assert_predicate_with_failure"
  "assertion_failure_in_subroutine"
  "assertion_failures_with_short_structured_actual_values"
  "async_test_with_expect_that"
  "custom_error_message"
  "expect_pred_failure"
  "expect_that_failure"
  "failure_due_to_fail_macro"
  "failure_due_to_fail_macro_with_empty_message"
  "failure_due_to_fail_macro_with_format_arguments"
  "failure_due_to_returned_error"
  "failure_due_to_returned_error_with_line_numbers"
  "success_with_succeed_macro"
  "success_with_succeed_macro_with_empty_message"
  "success_with_succeed_macro_with_format_arguments"
  "add_failure_macro_causes_failure_but_continues_execution"
  "add_failure_macro_allows_empty_message"
  "add_failure_macro_allows_formatted_arguments"
  "add_failure_macro_needs_googletest_attribute"
  "add_failure_at_macro_causes_failure_but_continues_execution"
  "add_failure_at_macro_allows_empty_message"
  "add_failure_at_macro_allows_formatted_arguments"
  "add_failure_at_macro_needs_googletest_attribute"
  "verify_true_macro_on_false_condition"
  "expect_true_macro_on_false_condition_fails_test_and_continues"
  "verify_false_macro_on_true_condition"
  "expect_false_macro_on_true_condition_fails_test_and_continues"
  "verify_eq_when_not_equal_returns_error"
  "verify_eq_with_ordered_elements_when_not_equal_returns_error"
  "verify_eq_with_unordered_elements_when_not_equal_returns_error"
  "expect_eq_when_not_equal_returns_error"
  "expect_eq_supports_custom_message"
  "expect_eq_with_ordered_elements_when_not_equal_returns_error"
  "expect_eq_with_ordered_elements_supports_custom_message"
  "expect_eq_with_unordered_elements_when_not_equal_returns_error"
  "expect_eq_with_unordered_elements_supports_custom_message"
  "verify_ne_when_equal_returns_error"
  "expect_ne_when_equal_marks_failed"
  "expect_ne_supports_custom_message"
  "verify_lt_when_not_less_returns_error"
  "expect_lt_when_not_less_marks_failed"
  "expect_lt_supports_custom_message"
  "verify_le_when_greater_returns_error"
  "expect_le_when_greater_marks_failed"
  "expect_le_supports_custom_message"
  "verify_gt_when_not_greater_returns_error"
  "expect_gt_when_not_greater_marks_failed"
  "expect_gt_supports_custom_message"
  "verify_ge_when_less_returns_error"
  "expect_ge_when_less_marks_failed"
  "expect_ge_supports_custom_message"
  "verify_float_eq_when_not_equal_returns_error"
  "expect_float_eq_when_not_equal_marks_failed"
  "expect_float_eq_supports_custom_message"
  "verify_near_when_not_near_returns_error"
  "expect_near_when_not_near_marks_failed"
  "expect_near_supports_custom_message"
  "fatal_and_non_fatal_failure"
  "first_failure_aborts"
  "google_test_with_rstest"
  "non_fatal_failure_in_subroutine"
  "passing_test_with_should_panic"
  "simple_assertion_failure"
  "simple_assertion_failure_with_assert_that"
  "test_returning_anyhow_error"
  "two_expect_pred_failures"
  "two_expect_that_failures"
  "two_non_fatal_failures"
  "verify_predicate_with_failure"
  "verify_predicate_with_failure_as_method_in_submodule"
)

cargo build
for binary in ${INTEGRATION_TEST_BINARIES[@]}; do
  cargo rustc -p integration_tests --bin $binary -- --test
done
./target/debug/integration_tests
