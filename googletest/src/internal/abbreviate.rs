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

#![doc(hidden)]

use std::borrow::Cow;

/// A single-line actual value which has a debug output longer than this
/// character length will be abbreviated by showing the first and last
/// [`ABBREVIATION_MARGIN`] - 1 characters of the string, separated by an
/// ellipsis.
const ABBREVIATION_LENGTH_THRESHOLD: usize = 61;

/// The number of characters at the beginning and end of a string to show when
/// abbreviating it.
const ABBREVIATION_MARGIN: usize = (ABBREVIATION_LENGTH_THRESHOLD - 1) / 2 + 1;

/// Returns an abbreviated version of `value`.
///
/// If `value` is a single line of character length greater than
/// [`ABBREVIATION_LENGTH_THRESHOLD`] and containing an escaped newline (`\n`),
/// the returned string is owned consisting of the first and last
/// [`ABBREVIATION_MARGIN`] characters of `value` separated by an ellipsis.
/// Otherwise, this returns `value` itself.
///
/// This implementation does not split multibyte characters, but it may split
/// grapheme clusters, resulting in incorrect rendering around the ellipsis.
pub(crate) fn abbreviate(value: &str) -> Cow<str> {
    if value.lines().count() > 1 || !value.contains("\\n") {
        return value.into();
    }

    let value_chars: Vec<_> = value.chars().collect();
    if value_chars.len() > ABBREVIATION_LENGTH_THRESHOLD {
        let mut result: String = value_chars[..ABBREVIATION_MARGIN].iter().collect();
        result.push('…');
        result.extend(&value_chars[value_chars.len() - ABBREVIATION_MARGIN..]);
        result.into()
    } else {
        value.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn abbreviate_does_not_affect_string_shorter_than_threshold_characters() -> Result<()> {
        let input = format!("{}\\n", ".".repeat(ABBREVIATION_LENGTH_THRESHOLD - 2));

        let result = abbreviate(&input);

        verify_that!(result, eq(input.as_str()))
    }

    #[test]
    fn abbreviate_does_not_affect_string_with_more_than_one_line() -> Result<()> {
        let input = format!("{}\n.", ".".repeat(ABBREVIATION_LENGTH_THRESHOLD));

        let result = abbreviate(&input);

        verify_that!(result, eq(input.as_str()))
    }

    #[test]
    fn abbreviate_does_not_affect_string_lacking_an_esacped_newline() -> Result<()> {
        let input = ".".repeat(ABBREVIATION_LENGTH_THRESHOLD + 1);

        let result = abbreviate(&input);

        verify_that!(result, eq(input.as_str()))
    }

    #[test]
    fn abbreviate_shortens_string_longer_than_threshold_characters() -> Result<()> {
        let input = format!("{}\\n", ".".repeat(ABBREVIATION_LENGTH_THRESHOLD - 1));
        let result_expected = format!(
            "{}…{}\\n",
            ".".repeat(ABBREVIATION_MARGIN),
            ".".repeat(ABBREVIATION_MARGIN - 2)
        );

        let result = abbreviate(&input);

        verify_that!(result, eq(result_expected))
    }

    #[test]
    fn abbreviate_does_not_short_string_of_threshold_multi_byte_chars() -> Result<()> {
        let input = format!("{}\\n", "ä".repeat(ABBREVIATION_LENGTH_THRESHOLD - 2));

        let result = abbreviate(&input);

        verify_that!(result, eq(input.as_str()))
    }

    #[test]
    fn abbreviate_shortens_string_based_on_character_count_with_multi_byte_chars() -> Result<()> {
        let input = format!("{}\\n", "ä".repeat(ABBREVIATION_LENGTH_THRESHOLD - 1));
        let result_expected = format!(
            "{}…{}\\n",
            "ä".repeat(ABBREVIATION_MARGIN),
            "ä".repeat(ABBREVIATION_MARGIN - 2)
        );

        let result = abbreviate(&input);

        verify_that!(result, eq(result_expected))
    }
}
