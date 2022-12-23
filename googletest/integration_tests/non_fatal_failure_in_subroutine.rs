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
    use googletest::{google_test, verify_that, GoogleTestSupport, Result};
    use matchers::eq;

    #[google_test]
    fn calls_verify_that_in_subroutine() -> Result<()> {
        verify_that_things_are_okay(2);
        Ok(())
    }

    fn verify_that_things_are_okay(value: i32) {
        verify_that!(value, eq(3)).and_log_failure();
    }
}
