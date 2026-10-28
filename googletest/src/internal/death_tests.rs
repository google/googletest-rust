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

use crate::internal::test_outcome::TestAssertionFailure;
use crate::matcher::Matcher;
use std::cell::{Cell, RefCell};
use std::process::{Command, ExitStatus, Stdio};

thread_local! {
    static CURRENT_TEST_NAME: RefCell<Option<String>> = const { RefCell::new(None) };
    static DEATH_TEST_COUNT: Cell<usize> = const { Cell::new(0) };
}

/// A drop guard returned by [`set_current_test_name`].
///
/// When dropped, it restores the current test name and the death test count
/// to the values they had before [`set_current_test_name`] was called. This
/// makes the guard safe to nest (e.g. when a test runner is itself exercised
/// from within a test) and ensures the state is cleaned up even if the test
/// panics.
///
/// The guard manipulates thread-local state, so it should be dropped on the
/// thread it was created on. It is nevertheless `Send`, like
/// `ScopedTraceGuard`, because `#[gtest]` holds it across the `.await` in
/// async tests.
#[must_use = "the test name is restored when the guard is dropped"]
pub struct CurrentTestNameGuard {
    /// The test name that was active before this guard was created.
    previous_name: Option<String>,
    /// The death test count that was active before this guard was created.
    previous_count: usize,
}

impl Drop for CurrentTestNameGuard {
    fn drop(&mut self) {
        CURRENT_TEST_NAME.with(|n| *n.borrow_mut() = self.previous_name.take());
        DEATH_TEST_COUNT.with(|c| c.set(self.previous_count));
    }
}

/// Returns the name of the current test, if one is currently running.
pub fn current_test_name() -> Option<String> {
    CURRENT_TEST_NAME.with(|n| n.borrow().clone())
}

/// Stores the name of the current test and resets the death test count.
/// Called by `#[gtest]` and test runners before running each test.
///
/// Returns a [`CurrentTestNameGuard`] that restores the previous test name and
/// death test count when dropped. The guard must be kept alive for the whole
/// duration of the test.
pub fn set_current_test_name(name: &str) -> CurrentTestNameGuard {
    let previous_name = CURRENT_TEST_NAME.with(|n| n.borrow_mut().replace(name.to_string()));
    let previous_count = DEATH_TEST_COUNT.with(|c| c.replace(0));
    CurrentTestNameGuard { previous_name, previous_count }
}

fn increment_death_test_count() -> usize {
    DEATH_TEST_COUNT.with(|c| {
        let count = c.get();
        c.set(count + 1);
        count
    })
}

/// Environment variable used to communicate to the child process which death
/// test it should execute.
///
/// When a death test is triggered, the parent process spawns a child process
/// (re-running the test binary). To ensure the child only runs the specific
/// death test that triggered it, the parent sets this variable to the index of
/// that death test. The child reads this variable to determine whether to
/// execute or skip a death test expression.
const INTERNAL_DEATH_TEST_INDEX_VAR: &str = "GOOGLETEST_INTERNAL_DEATH_TEST_INDEX";

/// Returns whether the current process is a death test child, i.e. a process
/// spawned by [`oversee_death_test`] to execute a single death test.
///
/// The parent matches the child's stderr against the death test's output
/// matcher. Test runners should therefore not write their own console output
/// (progress, summaries, filter notes) to stderr in the child, and should not
/// write reports such as the XML report, which belong to the parent.
pub fn is_death_test_child() -> bool {
    std::env::var_os(INTERNAL_DEATH_TEST_INDEX_VAR).is_some()
}

/// Marker printed by the child process to stderr if it survives the death test.
///
/// If the expression passed to the death test does NOT terminate the process,
/// this marker is printed to stderr by `DeathTestSentinel`. The parent process
/// looks for this marker to determine if the death test failed to die.
///
/// The `7dfb4a88e2` suffix is a random string to make the marker unique and
/// avoid collisions with user output.
const DEATH_TEST_LIVED_MARKER: &str = "___GOOGLETEST_DEATH_TEST_LIVED_7dfb4a88e2___";

/// The role a death test invocation should take.
pub enum DeathTestRole {
    /// We are in the parent process, and we should spawn a child.
    Oversee,
    /// We are in the child process, and this is the target death test.
    Execute,
    /// We are in the child process, but this is NOT the target death test. Skip
    /// it.
    Skip,
}

/// Determines which role to take when entering a death test macro.
pub fn should_run_death_test_expr() -> DeathTestRole {
    let index = increment_death_test_count();
    if let Ok(val) = std::env::var(INTERNAL_DEATH_TEST_INDEX_VAR) {
        if let Ok(target_index) = val.parse::<usize>() {
            if index == target_index {
                DeathTestRole::Execute
            } else {
                DeathTestRole::Skip
            }
        } else {
            DeathTestRole::Skip
        }
    } else {
        DeathTestRole::Oversee
    }
}

/// A sentinel that detects if the expression did not terminate the process.
pub struct DeathTestSentinel;

impl Drop for DeathTestSentinel {
    fn drop(&mut self) {
        // If control leaves the expression without exiting the process, the
        // Sentinel is dropped. This indicates the death test failed to
        // die. We ignore panics, as they will cause the child to exit
        // non-zero via the harness.
        if !std::thread::panicking() {
            use std::io::Write;
            let mut stderr = std::io::stderr();
            let _ = write!(stderr, "{}", DEATH_TEST_LIVED_MARKER);
            let _ = stderr.flush();
            // Exit to prevent further code execution in the child.
            std::process::exit(0);
        }
    }
}

/// Logic for the parent process to oversee the execution of the death test
/// child.
pub fn oversee_death_test<StatusM, OutputM>(
    expr_str: &str,
    status_matcher: &StatusM,
    output_matcher: &OutputM,
) -> crate::Result<()>
where
    StatusM: Matcher<ExitStatus>,
    for<'a> OutputM: Matcher<&'a str>,
{
    let test_name = current_test_name().ok_or_else(|| {
        TestAssertionFailure::create(
            "Could not determine current test name. Did you annotate the test with `#[gtest]`?"
                .to_string(),
        )
    })?;

    // The count has been incremented in should_run_death_test_expr(),
    // so current value - 1 is the index of THIS specific death test.
    let index = DEATH_TEST_COUNT.with(|c| c.get()) - 1;

    let exe = std::env::current_exe().map_err(|e| {
        TestAssertionFailure::create(format!("Failed to get current executable: {e}"))
    })?;

    // Spawn child process to re-execute only the current test. Like C++
    // GoogleTest, forward the parent's command-line arguments so that flags
    // (e.g. absl flags under the google3 testrunner, or libtest's `--ignored`)
    // have the same effect on setup code in the child as in the parent.
    let child = Command::new(exe)
        .args(std::env::args_os().skip(1))
        .env("TESTBRIDGE_TEST_ONLY", &test_name)
        .env(INTERNAL_DEATH_TEST_INDEX_VAR, index.to_string())
        .env("RUST_TEST_NOCAPTURE", "1")
        // Only the parent writes the XML test report.
        .env_remove("XML_OUTPUT_FILE")
        .stdin(Stdio::null())
        .stdout(Stdio::null()) // Suppress harness output
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            TestAssertionFailure::create(format!("Failed to spawn death test child process: {e}"))
        })?;

    let output = child.wait_with_output().map_err(|e| {
        TestAssertionFailure::create(format!("Failed to read death test child process output: {e}"))
    })?;

    let raw_stderr = String::from_utf8_lossy(&output.stderr);

    // Check for the lived marker
    if raw_stderr.contains(DEATH_TEST_LIVED_MARKER) {
        let cleaned_stderr = raw_stderr.replace(DEATH_TEST_LIVED_MARKER, "");
        let mut description = format!("Death test failed to die: {expr_str}\n");
        if !cleaned_stderr.trim().is_empty() {
            description.push_str("Actual output (stderr):\n");
            description.push_str(&cleaned_stderr);
        }
        return Err(TestAssertionFailure::create(description));
    }

    // Verify the exit status
    match status_matcher.matches(output.status) {
        crate::matcher::MatcherResult::Match => {
            // Status matched. Now check the output.
            match output_matcher.matches(&raw_stderr) {
                crate::matcher::MatcherResult::Match => Ok(()),
                crate::matcher::MatcherResult::NoMatch => {
                    let mut msg = "Death test died but output did not match.\n".to_string();
                    msg.push_str(&format!(
                        "Expected: {}\n",
                        output_matcher.describe(crate::matcher::MatcherResult::Match)
                    ));
                    msg.push_str("Actual output (stderr):\n");
                    msg.push_str(&raw_stderr);
                    Err(TestAssertionFailure::create(msg))
                }
            }
        }
        crate::matcher::MatcherResult::NoMatch => {
            let mut msg = "Death test died but exit status did not match.\n".to_string();
            msg.push_str(&format!(
                "Expected: {}\n",
                status_matcher.describe(crate::matcher::MatcherResult::Match)
            ));
            msg.push_str(&format!("Actual status: {:?}\n", output.status));
            if !raw_stderr.trim().is_empty() {
                msg.push_str("Actual output (stderr):\n");
                msg.push_str(&raw_stderr);
            }
            Err(TestAssertionFailure::create(msg))
        }
    }
}

// These tests are in `mod tests` rather than in a separate test crate because
// they use non-public items such as `DEATH_TEST_COUNT`.
#[cfg(test)]
mod tests {
    use super::*;

    fn death_test_count() -> usize {
        DEATH_TEST_COUNT.with(|c| c.get())
    }

    #[test]
    fn test_name_guard_clears_on_drop() {
        assert_eq!(current_test_name(), None);
        {
            let _guard = set_current_test_name("test_name_guard_clears_on_drop");
            assert_eq!(current_test_name().as_deref(), Some("test_name_guard_clears_on_drop"));
        }
        assert_eq!(current_test_name(), None);
    }

    #[test]
    fn test_name_guard_clears_on_panic() {
        assert_eq!(current_test_name(), None);
        let _ = std::panic::catch_unwind(|| {
            let _guard = set_current_test_name("panicking_test");
            assert_eq!(current_test_name().as_deref(), Some("panicking_test"));
            panic!("test panic");
        });
        assert_eq!(current_test_name(), None);
    }

    #[test]
    fn test_name_guard_resets_death_test_count() {
        let initial_count = 3;
        DEATH_TEST_COUNT.with(|c| c.set(initial_count));
        {
            let _guard = set_current_test_name("test_name_guard_resets_death_test_count");
            assert_eq!(death_test_count(), 0);
            increment_death_test_count();
            increment_death_test_count();
            assert_eq!(death_test_count(), 2);
        }
        assert_eq!(death_test_count(), initial_count);
    }

    #[test]
    fn test_name_guard_restores_previous_state_when_nested() {
        assert_eq!(current_test_name(), None);
        let _outer_guard = set_current_test_name("outer_test");
        // Simulate two death tests having already run in the outer test.
        increment_death_test_count();
        increment_death_test_count();
        assert_eq!(current_test_name().as_deref(), Some("outer_test"));
        assert_eq!(death_test_count(), 2);
        {
            let _inner_guard = set_current_test_name("inner_test");
            // The inner test sees its own name and a fresh death test count.
            assert_eq!(current_test_name().as_deref(), Some("inner_test"));
            assert_eq!(death_test_count(), 0);
            increment_death_test_count();
            assert_eq!(death_test_count(), 1);
        }
        // Dropping the inner guard restores the outer test's state rather than
        // clearing it.
        assert_eq!(current_test_name().as_deref(), Some("outer_test"));
        assert_eq!(death_test_count(), 2);
    }

    #[test]
    fn test_name_guard_is_send() {
        // `#[gtest]` holds the guard across the `.await` in async tests, so it
        // must be `Send` for async test futures to remain `Send`.
        fn assert_send<T: Send>() {}
        assert_send::<CurrentTestNameGuard>();
    }

    #[test]
    fn test_name_guard_restores_previous_state_when_nested_and_panicking() {
        assert_eq!(current_test_name(), None);
        let _outer_guard = set_current_test_name("outer_test");
        increment_death_test_count();
        let _ = std::panic::catch_unwind(|| {
            let _inner_guard = set_current_test_name("panicking_inner_test");
            assert_eq!(current_test_name().as_deref(), Some("panicking_inner_test"));
            panic!("test panic");
        });
        assert_eq!(current_test_name().as_deref(), Some("outer_test"));
        assert_eq!(death_test_count(), 1);
    }
}
