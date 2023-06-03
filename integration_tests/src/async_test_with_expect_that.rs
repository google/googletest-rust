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

#[deny(warnings)]
#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use std::time::Duration;
    use tokio::time::sleep;

    #[googletest::test]
    #[tokio::test]
    async fn async_test_failure_with_non_fatal_assertion() -> Result<()> {
        sleep(Duration::from_millis(1)).await;
        expect_that!(2, eq(3));
        Ok(())
    }

    #[googletest::test]
    #[tokio::test]
    async fn async_test_failure_with_fatal_assertion() -> Result<()> {
        sleep(Duration::from_millis(1)).await;
        verify_that!(3, eq(4))?;
        Ok(())
    }
}
