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

use std::fmt::{Display, Error, Formatter};

/// Encapsulates a location in source code.
///
/// This is intended to report the location of an assertion which failed to
/// stdout.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub struct SourceLocation {
    file: &'static str,
    line: u32,
    column: u32,
}

impl SourceLocation {
    /// Constructs a new [`SourceLocation`].
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "  at {}:{}:{}", self.file, self.line, self.column)
    }
}
