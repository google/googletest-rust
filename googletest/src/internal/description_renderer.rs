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

use std::{
    borrow::Cow,
    fmt::{Result, Write},
};

/// Number of space used to indent lines when no alignement is required.
pub(crate) const INDENTATION_SIZE: usize = 2;

/// A list of [`Block`] possibly rendered with a [`Decoration`].
///
/// This is the top-level renderable component, corresponding to the description
/// or match explanation of a single matcher.
///
/// The constituent [`Block`] of a `List` can be decorated with either bullets
/// (`* `) or enumeration (`0. `, `1. `, ...). This is controlled via the
/// methods [`List::bullet_list`] and [`List::enumerate`]. By default, there is
/// no decoration.
///
/// A `List` can be constructed as follows:
///
///   * [`Default::default()`] constructs an empty `List`.
///   * [`Iterator::collect()`] on an [`Iterator`] of [`Block`].
///   * [`Iterator::collect()`] on an [`Iterator`] of `String`, which produces a
///     [`Block::Literal`] for each `String`.
///   * [`Iterator::collect()`] on an [`Iterator`] of `List`, which produces a
///     [`Block::Nested`] for each `List`.
#[derive(Debug, Default)]
pub(crate) struct List(Vec<Block>, Decoration);

impl List {
    /// Render this instance using the formatter `f`.
    ///
    /// Indent each line of output by `indentation` spaces.
    pub(crate) fn render(&self, f: &mut dyn Write, indentation: usize) -> Result {
        self.render_with_prefix(f, indentation, "".into())
    }

    /// Append a new [`Block`] containing `literal`.
    ///
    /// The input `literal` is split into lines so that each line will be
    /// indented correctly.
    pub(crate) fn push_literal(&mut self, literal: Cow<'static, str>) {
        self.0.push(literal.into());
    }

    /// Append a new [`Block`] containing `inner` as a nested [`List`].
    pub(crate) fn push_nested(&mut self, inner: List) {
        self.0.push(Block::Nested(inner));
    }

    /// Render each [`Block`] of this instance preceded with a bullet "* ".
    pub(crate) fn bullet_list(self) -> Self {
        Self(self.0, Decoration::Bullet)
    }

    /// Render each [`Block`] of this instance preceded with its 0-based index.
    pub(crate) fn enumerate(self) -> Self {
        Self(self.0, Decoration::Enumerate)
    }

    /// Return the number of [`Block`] in this instance.
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    /// Return `true` if there are no [`Block`] in this instance, `false`
    /// otherwise.
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn render_with_prefix(
        &self,
        f: &mut dyn Write,
        indentation: usize,
        prefix: Cow<'static, str>,
    ) -> Result {
        if self.0.is_empty() {
            return Ok(());
        }

        let enumeration_padding = self.enumeration_padding();

        self.0[0].render(
            f,
            indentation,
            self.full_prefix(0, enumeration_padding, &prefix).into(),
        )?;
        for (index, block) in self.0[1..].iter().enumerate() {
            writeln!(f)?;
            block.render(
                f,
                indentation + prefix.len(),
                self.prefix(index + 1, enumeration_padding),
            )?;
        }
        Ok(())
    }

    fn full_prefix(&self, index: usize, enumeration_padding: usize, prior_prefix: &str) -> String {
        format!("{prior_prefix}{}", self.prefix(index, enumeration_padding))
    }

    fn prefix(&self, index: usize, enumeration_padding: usize) -> Cow<'static, str> {
        match self.1 {
            Decoration::None => "".into(),
            Decoration::Bullet => "* ".into(),
            Decoration::Enumerate => format!("{:>enumeration_padding$}. ", index).into(),
        }
    }

    fn enumeration_padding(&self) -> usize {
        match self.1 {
            Decoration::None => 0,
            Decoration::Bullet => 0,
            Decoration::Enumerate => {
                if self.0.len() > 1 {
                    ((self.0.len() - 1) as f64).log10().floor() as usize + 1
                } else {
                    // Avoid negative logarithm when there is only 0 or 1 element.
                    1
                }
            }
        }
    }
}

impl FromIterator<Block> for List {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Block>,
    {
        Self(iter.into_iter().collect(), Decoration::None)
    }
}

impl<ElementT: Into<Cow<'static, str>>> FromIterator<ElementT> for List {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = ElementT>,
    {
        Self(iter.into_iter().map(|b| b.into().into()).collect(), Decoration::None)
    }
}

impl FromIterator<List> for List {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = List>,
    {
        Self(iter.into_iter().map(Block::nested).collect(), Decoration::None)
    }
}

/// A sequence of [`Fragment`] or a nested [`List`].
///
/// This may be rendered with a prefix specified by the [`Decoration`] of the
/// containing [`List`]. In this case, all lines are indented to align with the
/// first character of the first line of the block.
#[derive(Debug)]
enum Block {
    /// A block of text.
    ///
    /// Each constituent [`Fragment`] contains one line of text. The lines are
    /// indented uniformly to the current indentation of this block when
    /// rendered.
    Literal(Vec<Fragment>),

    /// A nested [`List`].
    ///
    /// The [`List`] is rendered recursively at the next level of indentation.
    Nested(List),
}

impl Block {
    fn nested(inner: List) -> Self {
        Self::Nested(inner)
    }

    fn render(&self, f: &mut dyn Write, indentation: usize, prefix: Cow<'static, str>) -> Result {
        match self {
            Self::Literal(fragments) => {
                if fragments.is_empty() {
                    return Ok(());
                }

                write!(f, "{:indentation$}{prefix}", "")?;
                fragments[0].render(f)?;
                let block_indentation = indentation + prefix.as_ref().len();
                for fragment in &fragments[1..] {
                    writeln!(f)?;
                    write!(f, "{:block_indentation$}", "")?;
                    fragment.render(f)?;
                }
                Ok(())
            }
            Self::Nested(inner) => inner.render_with_prefix(
                f,
                indentation + INDENTATION_SIZE.saturating_sub(prefix.len()),
                prefix,
            ),
        }
    }
}

impl From<String> for Block {
    fn from(value: String) -> Self {
        Block::Literal(value.lines().map(|v| Fragment(v.to_string().into())).collect())
    }
}

impl From<&'static str> for Block {
    fn from(value: &'static str) -> Self {
        Block::Literal(value.lines().map(|v| Fragment(v.into())).collect())
    }
}

impl From<Cow<'static, str>> for Block {
    fn from(value: Cow<'static, str>) -> Self {
        match value {
            Cow::Borrowed(value) => value.into(),
            Cow::Owned(value) => value.into(),
        }
    }
}

/// A string representing one line of a description or match explanation.
#[derive(Debug)]
struct Fragment(Cow<'static, str>);

impl Fragment {
    fn render(&self, f: &mut dyn Write) -> Result {
        write!(f, "{}", self.0)
    }
}

/// The decoration which appears on [`Block`] of a [`List`] when rendered.
#[derive(Debug, Default)]
enum Decoration {
    /// No decoration on each [`Block`]. The default.
    #[default]
    None,

    /// Each [`Block`] is preceded by a bullet (`* `).
    Bullet,

    /// Each [`Block`] is preceded by its index in the [`List`] (`0. `, `1. `,
    /// ...).
    Enumerate,
}

#[cfg(test)]
mod tests {
    use super::{Block, Fragment, List};
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn renders_fragment() -> Result<()> {
        let fragment = Fragment("A fragment".into());
        let mut result = String::new();

        fragment.render(&mut result)?;

        verify_that!(result, eq("A fragment"))
    }

    #[test]
    fn renders_empty_block() -> Result<()> {
        let block = Block::Literal(vec![]);
        let mut result = String::new();

        block.render(&mut result, 0, "".into())?;

        verify_that!(result, eq(""))
    }

    #[test]
    fn renders_block_with_one_fragment() -> Result<()> {
        let block: Block = "A fragment".into();
        let mut result = String::new();

        block.render(&mut result, 0, "".into())?;

        verify_that!(result, eq("A fragment"))
    }

    #[test]
    fn renders_block_with_two_fragments() -> Result<()> {
        let block: Block = "A fragment\nAnother fragment".into();
        let mut result = String::new();

        block.render(&mut result, 0, "".into())?;

        verify_that!(result, eq("A fragment\nAnother fragment"))
    }

    #[test]
    fn renders_indented_block() -> Result<()> {
        let block: Block = "A fragment\nAnother fragment".into();
        let mut result = String::new();

        block.render(&mut result, 2, "".into())?;

        verify_that!(result, eq("  A fragment\n  Another fragment"))
    }

    #[test]
    fn renders_block_with_prefix() -> Result<()> {
        let block: Block = "A fragment\nAnother fragment".into();
        let mut result = String::new();

        block.render(&mut result, 0, "* ".into())?;

        verify_that!(result, eq("* A fragment\n  Another fragment"))
    }

    #[test]
    fn renders_indented_block_with_prefix() -> Result<()> {
        let block: Block = "A fragment\nAnother fragment".into();
        let mut result = String::new();

        block.render(&mut result, 2, "* ".into())?;

        verify_that!(result, eq("  * A fragment\n    Another fragment"))
    }

    #[test]
    fn renders_empty_list() -> Result<()> {
        let list = list(vec![]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq(""))
    }

    #[test]
    fn renders_plain_list_with_one_block() -> Result<()> {
        let list = list(vec!["A fragment".into()]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("A fragment"))
    }

    #[test]
    fn renders_plain_list_with_two_blocks() -> Result<()> {
        let list = list(vec!["A fragment".into(), "A fragment in a second block".into()]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("A fragment\nA fragment in a second block"))
    }

    #[test]
    fn renders_plain_list_with_one_block_with_two_fragments() -> Result<()> {
        let list = list(vec!["A fragment\nA second fragment".into()]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("A fragment\nA second fragment"))
    }

    #[test]
    fn renders_nested_plain_list_with_one_block() -> Result<()> {
        let list = list(vec![Block::nested(list(vec!["A fragment".into()]))]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("  A fragment"))
    }

    #[test]
    fn renders_nested_plain_list_with_two_blocks() -> Result<()> {
        let list = list(vec![Block::nested(list(vec![
            "A fragment".into(),
            "A fragment in a second block".into(),
        ]))]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("  A fragment\n  A fragment in a second block"))
    }

    #[test]
    fn renders_nested_plain_list_with_one_block_with_two_fragments() -> Result<()> {
        let list = list(vec![Block::nested(list(vec!["A fragment\nA second fragment".into()]))]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("  A fragment\n  A second fragment"))
    }

    #[test]
    fn renders_bulleted_list_with_one_block() -> Result<()> {
        let list = list(vec!["A fragment".into()]).bullet_list();
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("* A fragment"))
    }

    #[test]
    fn renders_bulleted_list_with_two_blocks() -> Result<()> {
        let list =
            list(vec!["A fragment".into(), "A fragment in a second block".into()]).bullet_list();
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("* A fragment\n* A fragment in a second block"))
    }

    #[test]
    fn renders_bulleted_list_with_one_block_with_two_fragments() -> Result<()> {
        let list = list(vec!["A fragment\nA second fragment".into()]).bullet_list();
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("* A fragment\n  A second fragment"))
    }

    #[test]
    fn renders_nested_bulleted_list_with_one_block() -> Result<()> {
        let list = list(vec![Block::nested(list(vec!["A fragment".into()]).bullet_list())]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("  * A fragment"))
    }

    #[test]
    fn renders_nested_bulleted_list_with_two_blocks() -> Result<()> {
        let list = list(vec![Block::nested(
            list(vec!["A fragment".into(), "A fragment in a second block".into()]).bullet_list(),
        )]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("  * A fragment\n  * A fragment in a second block"))
    }

    #[test]
    fn renders_nested_bulleted_list_with_one_block_with_two_fragments() -> Result<()> {
        let list = list(vec![Block::nested(
            list(vec!["A fragment\nA second fragment".into()]).bullet_list(),
        )]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("  * A fragment\n    A second fragment"))
    }

    #[test]
    fn renders_enumerated_list_with_one_block() -> Result<()> {
        let list = list(vec!["A fragment".into()]).enumerate();
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("0. A fragment"))
    }

    #[test]
    fn renders_enumerated_list_with_two_blocks() -> Result<()> {
        let list =
            list(vec!["A fragment".into(), "A fragment in a second block".into()]).enumerate();
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("0. A fragment\n1. A fragment in a second block"))
    }

    #[test]
    fn renders_enumerated_list_with_one_block_with_two_fragments() -> Result<()> {
        let list = list(vec!["A fragment\nA second fragment".into()]).enumerate();
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("0. A fragment\n   A second fragment"))
    }

    #[test]
    fn aligns_renders_enumerated_list_with_more_than_ten_blocks() -> Result<()> {
        let list =
            (0..11).map(|i| Block::from(format!("Fragment {i}"))).collect::<List>().enumerate();
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(
            result,
            eq(indoc! {"
                 0. Fragment 0
                 1. Fragment 1
                 2. Fragment 2
                 3. Fragment 3
                 4. Fragment 4
                 5. Fragment 5
                 6. Fragment 6
                 7. Fragment 7
                 8. Fragment 8
                 9. Fragment 9
                10. Fragment 10"})
        )
    }

    #[test]
    fn renders_fragment_plus_nested_plain_list_with_one_block() -> Result<()> {
        let list =
            list(vec!["A fragment".into(), Block::nested(list(vec!["Another fragment".into()]))]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("A fragment\n  Another fragment"))
    }

    #[test]
    fn renders_double_nested_plain_list_with_one_block() -> Result<()> {
        let list =
            list(vec![Block::nested(list(vec![Block::nested(list(vec!["A fragment".into()]))]))]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("    A fragment"))
    }

    #[test]
    fn renders_headers_plus_double_nested_plain_list() -> Result<()> {
        let list = list(vec![
            "First header".into(),
            Block::nested(list(vec![
                "Second header".into(),
                Block::nested(list(vec!["A fragment".into()])),
            ])),
        ]);
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("First header\n  Second header\n    A fragment"))
    }

    #[test]
    fn renders_double_nested_bulleted_list() -> Result<()> {
        let list =
            list(vec![Block::nested(list(vec!["A fragment".into()]).bullet_list())]).bullet_list();
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("* * A fragment"))
    }

    #[test]
    fn renders_nested_enumeration_with_two_blocks_inside_bulleted_list() -> Result<()> {
        let list =
            list(vec![Block::nested(list(vec!["Block 1".into(), "Block 2".into()]).enumerate())])
                .bullet_list();
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("* 0. Block 1\n  1. Block 2"))
    }

    #[test]
    fn renders_nested_enumeration_with_block_with_two_fragments_inside_bulleted_list() -> Result<()>
    {
        let list = list(vec![Block::nested(
            list(vec!["A fragment\nAnother fragment".into()]).enumerate(),
        )])
        .bullet_list();
        let mut result = String::new();

        list.render(&mut result, 0)?;

        verify_that!(result, eq("* 0. A fragment\n     Another fragment"))
    }

    fn list(blocks: Vec<Block>) -> List {
        List(blocks, super::Decoration::None)
    }
}
