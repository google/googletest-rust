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

use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct CustomError {
    message: String,
    source: Option<Box<CustomError>>,
}
impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[test]
    fn should_fail_due_to_error_in_subroutine() -> Result<()> {
        // do not use `or_fail` here as it would need specialization (currently an unstable rust
        // feature) to implement support for it there
        returns_custom_error()?;
        verify_false!(false)
    }

    fn returns_custom_error() -> std::result::Result<(), CustomError> {
        let source1 = CustomError { message: "test1".to_string(), source: None };
        let source2 = CustomError { message: "test2".to_string(), source: Some(source1.into()) };
        let error = CustomError { message: "test3".to_string(), source: Some(source2.into()) };
        Err(error)
    }
}
