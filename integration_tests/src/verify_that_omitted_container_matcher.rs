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

    #[googletest::test]
    fn verify_that_ordered_elements_omitted() -> Result<()> {
        verify_that!(vec![1, 2], [eq(1), eq(2)])
    }

    #[googletest::test]
    fn verify_that_unordered_elements_omitted() -> Result<()> {
        verify_that!(vec![1, 2], {eq(2), eq(1)})
    }
}
