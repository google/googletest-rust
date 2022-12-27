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
    use googletest::matchers;
    use googletest::{verify_that, GoogleTestSupport, Result};
    use matchers::eq;

    #[test]
    fn should_fail_with_custom_error_message() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(3)).failure_message("A custom error message")
    }

    #[test]
    fn should_fail_with_custom_error_message_in_string() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(3)).failure_message("A custom error message in a String".to_string())
    }

    #[test]
    fn should_fail_with_custom_error_message_in_closure() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(3))
            .with_failure_message(|| "A custom error message from a closure".to_string())
    }
}
