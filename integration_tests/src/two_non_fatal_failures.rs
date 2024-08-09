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

    #[gtest]
    fn more_than_one_failure() -> Result<()> {
        let value = 2;
        verify_that!(value, eq(3)).and_log_failure();
        verify_that!(value, eq(4)).and_log_failure();
        Ok(())
    }
}
