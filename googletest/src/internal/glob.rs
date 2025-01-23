//! Pattern matches using glob wildcards `*` and `?` with the
//! linear-time algorithm <https://research.swtch.com/glob>.

use std::iter::Peekable;
use std::str::Chars;

pub struct Pattern(String);

impl Pattern {
    /// Creates a new pattern matcher.  Each pattern consists of
    /// regular characters, single-character wildcards `'?'`, and
    /// multi-character wildcards `'*'`.
    pub fn new(pattern: &str) -> Self {
        Self(pattern.to_string())
    }

    /// Returns true if and only if the wildcard pattern matches the
    /// string.
    pub fn matches(&self, string: &str) -> bool {
        let processor = Processor {
            pattern: self.0.chars().peekable(),
            string: string.chars().peekable(),
            restart: None,
        };

        processor.process()
    }
}

type PeekableChars<'a> = Peekable<Chars<'a>>;

/// Represents the state we need to restart search from a star wildcard (`*`).
struct Restart<'a> {
    pattern_next: PeekableChars<'a>,
    string_next: PeekableChars<'a>,
}

/// The runtime state for glob matching.
struct Processor<'a> {
    pattern: PeekableChars<'a>,
    string: PeekableChars<'a>,
    restart: Option<Restart<'a>>,
}

/// Represents what to do after any step through the processor.
enum StepResult {
    Proceed,
    TryRestart,
}

impl Processor<'_> {
    /// Runs the pattern matching until we find an unrecoverable
    /// mismatch, or the input is consumed.
    fn process(mut self) -> bool {
        while self.not_empty() {
            match self.step() {
                StepResult::Proceed => {}
                StepResult::TryRestart => {
                    let restarted = self.try_restart();
                    if !restarted {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Whether the processor can still proceed.
    fn not_empty(&mut self) -> bool {
        self.pattern.peek().is_some() || self.string.peek().is_some()
    }

    /// Takes a single step forward, and returns whether to proceed or try to
    /// restart.
    fn step(&mut self) -> StepResult {
        match self.pattern.peek() {
            Some('?') => self.step_question_wildcard(),
            Some('*') => self.step_star_wildcard(),
            Some(pattern_ch) => {
                let pattern_ch = *pattern_ch;
                self.step_ordinary_character(pattern_ch)
            }
            None => StepResult::TryRestart,
        }
    }

    /// Match any single character.
    fn step_question_wildcard(&mut self) -> StepResult {
        if self.string.peek().is_some() {
            _ = self.pattern.next();
            _ = self.string.next();
            StepResult::Proceed
        } else {
            StepResult::TryRestart
        }
    }

    /// Match zero or more characters. Start by skipping over the
    /// wildcard and matching zero characters from string. If that
    /// fails, restart and match one more character than the last
    /// attempt.
    fn step_star_wildcard(&mut self) -> StepResult {
        self.restart = if self.string.peek().is_none() {
            // Subtle: if the string is already exhausted, we mark
            // that we can't restart.
            None
        } else {
            let pattern_next = self.pattern.clone();
            let mut string_next = self.string.clone();
            string_next.next();
            Some(Restart { pattern_next, string_next })
        };

        _ = self.pattern.next();
        StepResult::Proceed
    }

    /// Match an ordinary (non-wildcard) character.
    fn step_ordinary_character(&mut self, pattern_ch: char) -> StepResult {
        if self.string.peek() == Some(&pattern_ch) {
            _ = self.pattern.next();
            _ = self.string.next();
            StepResult::Proceed
        } else {
            StepResult::TryRestart
        }
    }

    /// Try to restart from failing to match a character.  If true, the
    /// matching can restart.
    fn try_restart(&mut self) -> bool {
        match &self.restart {
            Some(Restart { pattern_next, string_next }) => {
                self.pattern = pattern_next.clone();
                self.string = string_next.clone();

                true
            }
            None => false,
        }
    }
}

/// Returns true if `s` contains glob wildcards.
pub fn is_glob_pattern(s: &str) -> bool {
    s.contains(['?', '*'])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn simple_character_match() -> Result<()> {
        verify_that!(Pattern::new("a").matches("a"), is_true())
    }

    #[test]
    fn simple_character_mismatch() -> Result<()> {
        verify_that!(Pattern::new("b").matches("a"), is_false())
    }

    #[test]
    fn simple_word_mismatch() -> Result<()> {
        verify_that!(Pattern::new("judgement").matches("judgment"), is_false())
    }

    #[test]
    fn question_match() -> Result<()> {
        verify_that!(Pattern::new("?").matches("a"), is_true())
    }

    #[test]
    fn simple_word_question_match() -> Result<()> {
        let pattern = Pattern::new("judg?ment");
        verify_that!(pattern.matches("judgment"), is_false())?;
        verify_that!(pattern.matches("judgement"), is_true())?;
        verify_that!(pattern.matches("judge ment"), is_false())?;
        Ok(())
    }

    #[test]
    fn question_mismatch() -> Result<()> {
        verify_that!(Pattern::new("?").matches(""), is_false())
    }

    #[test]
    fn glob_on_empty() -> Result<()> {
        verify_that!(Pattern::new("*").matches(""), is_true())?;
        verify_that!(Pattern::new("**").matches(""), is_true())?;
        Ok(())
    }

    #[test]
    fn glob_simple() -> Result<()> {
        let star_a = Pattern::new("*a");
        verify_that!(star_a.matches("a"), is_true())?;
        verify_that!(star_a.matches("ba"), is_true())?;
        verify_that!(star_a.matches("bba"), is_true())?;
        verify_that!(star_a.matches("bbab"), is_false())?;

        let b_star_a = Pattern::new("b*a");
        verify_that!(b_star_a.matches("b"), is_false())?;
        verify_that!(b_star_a.matches("bb"), is_false())?;
        verify_that!(b_star_a.matches("ba"), is_true())?;
        verify_that!(b_star_a.matches("bbbba"), is_true())?;
        verify_that!(b_star_a.matches("baa"), is_true())?;
        Ok(())
    }

    #[test]
    fn glob_word() -> Result<()> {
        let word = Pattern::new("*word*");
        verify_that!(word.matches("bird"), is_false())?;
        verify_that!(word.matches("This is a wordy sentence"), is_true())?;
        verify_that!(word.matches("word soup"), is_true())?;
        verify_that!(word.matches("bird is the word"), is_true())?;
        verify_that!(word.matches("word"), is_true())?;
        Ok(())
    }

    #[test]
    fn degenerate_glob() -> Result<()> {
        verify_that!(
            Pattern::new("************************************************.*")
                .matches("this is a test.com"),
            is_true()
        )
    }

    #[test]
    fn degenerate_glob_2() -> Result<()> {
        // The first example from https://research.swtch.com/glob, with N=1000.
        const N: usize = 1000;
        let long_pattern = ["a*"; N].into_iter().collect::<String>() + "b";
        let long_string = ["a"; N].into_iter().collect::<String>();
        verify_that!(Pattern::new(&long_pattern).matches(&long_string), is_false())
    }
}
