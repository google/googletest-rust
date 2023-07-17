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

//! Utilities to style matcher explanation message with ANSI formatting.

use std::fmt::Display;
use std::io::IsTerminal;


/// Environment variable controlling the usage of ansi color in difference
/// summary.
pub(crate) const NO_COLOR_VAR: &str = "GTEST_RUST_NO_COLOR";

// Use ANSI code to enable styling on the summary lines.
//
// See https://en.wikipedia.org/wiki/ANSI_escape_code.
pub(crate) struct LineStyle {
    ansi_prefix: &'static str,
    ansi_suffix: &'static str,
    highlighted_prefix: &'static str,
    header: char,
}

impl LineStyle {
    // Font in red and bold
    pub(crate) fn extra_actual_style() -> Self {
        Self {
            ansi_prefix: "\x1B[1;31m",
            ansi_suffix: "\x1B[0m",
            highlighted_prefix: "",
            header: '-',
        }
    }

    // Font in green and bold
    pub(crate) fn extra_expected_style() -> Self {
        Self {
            ansi_prefix: "\x1B[1;32m",
            ansi_suffix: "\x1B[0m",
            highlighted_prefix: "",
            header: '+',
        }
    }

    // Font in italic
    pub(crate) fn comment_style() -> Self {
        Self { ansi_prefix: "\x1B[3m", ansi_suffix: "\x1B[0m", highlighted_prefix: "", header: ' ' }
    }

    // No ansi styling
    pub(crate) fn unchanged_style() -> Self {
        Self { ansi_prefix: "", ansi_suffix: "", highlighted_prefix: "", header: ' ' }
    }

    pub(crate) fn style(self, line: &str) -> impl Display + '_ {
        StyledLine { style: self, line }
    }

    pub(crate) fn style_highlighted(self, line: HighlightedString) -> impl Display {
        StyledLineWithHighlight { style: self, line }
    }
}

struct StyledLine<'a> {
    style: LineStyle,
    line: &'a str,
}

impl<'a> Display for StyledLine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if stdout_supports_color() {
            write!(
                f,
                "{}{}{}{}",
                self.style.header, self.style.ansi_prefix, self.line, self.style.ansi_suffix
            )
        } else {
            write!(f, "{}{}", self.style.header, self.line)
        }
    }
}
struct StyledLineWithHighlight {
    style: LineStyle,
    line: HighlightedString,
}

impl Display for StyledLineWithHighlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.style.header)?;
        for (piece, highlight) in &self.line.0 {
            if ! stdout_supports_color() {
                write!(f, "{}", piece)?;
            } else if *highlight {
                write!(f, "{}{}{}", self.style.highlighted_prefix, piece, self.style.ansi_suffix)?;
            } else {
                write!(f, "{}{}{}", self.style.ansi_prefix, piece, self.style.ansi_suffix)?;
            }
        }
        Ok(())
    }
}

pub(crate) struct HighlightedString(Vec<(String, bool)>);

impl HighlightedString {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }

    pub(crate) fn push_highlighted(&mut self, c: char) {
        match self.0.last_mut() {
            Some((s, true)) => s.push(c),
            _ => self.0.push((c.to_string(), true)),
        }
    }

    pub(crate) fn push(&mut self, c: char) {
        match self.0.last_mut() {
            Some((s, false)) => s.push(c),
            _ => self.0.push((c.to_string(), false)),
        }
    }
}



#[rustversion::since(1.70)]
fn stdout_supports_color() -> bool {
    match (is_env_var_set("NO_COLOR"), is_env_var_set("FORCE_COLOR")) {
        (true, _) => false,
        (false, true) => true,
        (false, false) => std::io::stdout().is_terminal(),
    }
}

#[rustversion::not(since(1.70))]
fn stdout_supports_color() -> bool {
    is_env_var_set("FORCE_COLOR")
}

fn is_env_var_set(var: &'static str) -> bool {
    std::env::var(var).map(|s| !s.is_empty()).unwrap_or(false)
}
