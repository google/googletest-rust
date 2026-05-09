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

use crate::{
    description::Description,
    matcher::{Matcher, MatcherBase, MatcherResult},
};
use std::process::ExitStatus;

/// Matches an [`ExitStatus`] which corresponds to a process that exited normally with
/// the given exit code.
///
/// ```
/// # use googletest::prelude::*;
/// # use std::process::Command;
/// # fn run_command() -> std::process::ExitStatus { Command::new("true").status().unwrap() }
/// # fn should_pass() -> Result<()> {
/// verify_that!(run_command(), exited_with_code(0))?;
/// # Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn exited_with_code(expected: i32) -> ExitedWithCodeMatcher {
    ExitedWithCodeMatcher { expected }
}

/// A matcher for `ExitStatus` checking if it corresponds to normal exit with a code.
#[derive(MatcherBase)]
pub struct ExitedWithCodeMatcher {
    expected: i32,
}

impl Matcher<&ExitStatus> for ExitedWithCodeMatcher {
    fn matches(&self, actual: &ExitStatus) -> MatcherResult {
        match actual.code() {
            Some(c) if c == self.expected => MatcherResult::Match,
            _ => MatcherResult::NoMatch,
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("exited with code {}", self.expected).into(),
            MatcherResult::NoMatch => format!("did not exit with code {}", self.expected).into(),
        }
    }
}

/// Matches an [`ExitStatus`] which corresponds to a process that did not exit successfully
/// (either a non-zero exit code or terminated by a signal).
///
/// ```
/// # use googletest::prelude::*;
/// # use std::process::Command;
/// # fn run_command() -> std::process::ExitStatus { Command::new("false").status().unwrap() }
/// # fn should_pass() -> Result<()> {
/// verify_that!(run_command(), died())?;
/// # Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn died() -> DiedMatcher {
    DiedMatcher
}

/// A matcher for `ExitStatus` checking if it corresponds to a failed execution.
#[derive(MatcherBase)]
pub struct DiedMatcher;

impl Matcher<&ExitStatus> for DiedMatcher {
    fn matches(&self, actual: &ExitStatus) -> MatcherResult {
        if actual.success() {
            MatcherResult::NoMatch
        } else {
            MatcherResult::Match
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => "died (non-zero exit code or signaled)".into(),
            MatcherResult::NoMatch => "did not die (exited successfully with code 0)".into(),
        }
    }
}
