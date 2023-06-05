// Copyright 2023 Google LLC
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

    #[test]
    fn should_fail_with_option_value() -> Result<()> {
        verify_that!(Some(1), some(eq(2)))
    }

    #[test]
    fn should_fail_with_result_ok_value() -> Result<()> {
        let value: std::result::Result<i32, ()> = Ok(1);
        verify_that!(value, ok(eq(2)))
    }

    #[test]
    fn should_fail_with_result_err_value() -> Result<()> {
        let value: std::result::Result<(), i32> = Err(1);
        verify_that!(value, err(eq(2)))
    }
}
