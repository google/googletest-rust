// Copyright 2025 Google LLC
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

#![no_std]

/// A simple, no_std function.
fn no_std_identity(value: u32) -> u32 {
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn no_std_verify() -> Result<()> {
        verify_eq!(no_std_identity(42), 42)?;
        Ok(())
    }

    #[gtest]
    fn no_std_expect() {
        expect_eq!(no_std_identity(214), 214)
    }
}
