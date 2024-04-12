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
  "success_with_succeed_macro"
  "success_with_succeed_macro_with_empty_message"
  "success_with_succeed_macro_with_format_arguments"
  "add_failure_macro_causes_failure_but_continues_execution"
  "add_failure_macro_allows_empty_message"
  "add_failure_macro_allows_formatted_arguments"
  "add_failure_macro_needs_googletest_attribute"
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
