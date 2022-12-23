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
  "simple_assertion_failure"
  "simple_assertion_failure_with_assert_that"
)

cargo build
for binary in ${INTEGRATION_TEST_BINARIES[@]}; do
  cargo rustc -p googletest --bin $binary -- --test
done
./target/debug/integration_tests
