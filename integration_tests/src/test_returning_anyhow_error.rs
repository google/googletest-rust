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
    fn should_fail_due_to_error_in_subroutine() -> Result<()> {
        returns_anyhow_error().or_fail()?;
        Ok(())
    }

    fn returns_anyhow_error() -> std::result::Result<(), anyhow::Error> {
        Err(anyhow::anyhow!("Error from Anyhow"))
    }
}
