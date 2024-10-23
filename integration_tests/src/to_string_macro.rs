// Copyright 2024 Google LLC
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
    use googletest::to_string;

    #[test]
    fn test_short_closure() -> Result<()> {
        let value = to_string!(|i: i32| i + 1);
        verify_eq!(value, "| i : i32 | i + 1")
    }

    #[test]
    fn test_short_closure_with_length_limit() -> Result<()> {
        let value = to_string!(|i: i32| i + 1, 20);
        verify_eq!(value, "| i : i32 | i + 1")
    }

    #[test]
    fn test_long_closure() -> Result<()> {
        let value = to_string!(|i: i32| i + 123456789101112131415161718192021222324252627282930);
        verify_eq!(value, "| i : i32 | i + 1234...")
    }

    #[test]
    fn test_long_closure_with_length_limit() -> Result<()> {
        let value =
            to_string!(|i: i32| i + 123456789101112131415161718192021222324252627282930, 22);
        verify_eq!(value, "| i : i32 | i + 123456...")
    }

    #[test]
    fn test_zero_limit() -> Result<()> {
        let value = to_string!(|i: i32| i + 1, 0);
        verify_eq!(value, "...")
    }
}
