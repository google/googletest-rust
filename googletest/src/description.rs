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
    fmt::{Display, Formatter, Result},
};

use crate::internal::description_renderer::{List, INDENTATION_SIZE};

/// A structured description, either of a (composed) matcher or of an
/// assertion failure.
///
/// One can compose blocks of text into a `Description`. Each one appears on a
/// new line. For example:
///
/// ```
/// # use googletest::prelude::*;
/// # use googletest::description::Description;
/// let description = Description::new()
///     .text("A block")
///     .text("Another block");
/// verify_that!(description, displays_as(eq("A block\nAnother block")))
/// # .unwrap();
/// ```
///
/// One can embed nested descriptions into a `Description`. The resulting
/// nested description is then rendered with an additional level of
/// indentation. For example:
///
/// ```
/// # use googletest::prelude::*;
/// # use googletest::description::Description;
/// let inner_description = Description::new()
///     .text("A block")
///     .text("Another block");
/// let outer_description = Description::new()
///     .text("Header")
///     .nested(inner_description);
/// verify_that!(outer_description, displays_as(eq("\
/// Header
///   A block
///   Another block")))
/// # .unwrap();
/// ```
///
/// One can also enumerate or bullet list the elements of a `Description`:
///
/// ```
/// # use googletest::prelude::*;
/// # use googletest::description::Description;
/// let description = Description::new()
///     .text("First item")
///     .text("Second item")
///     .bullet_list();
/// verify_that!(description, displays_as(eq("\
/// * First item
/// * Second item")))
/// # .unwrap();
/// ```
///
/// One can construct a `Description` from a [`String`] or a string slice, an
/// iterator thereof, or from an iterator over other `Description`s:
///
/// ```
/// # use googletest::description::Description;
/// let single_element_description: Description =
///     "A single block description".into();
/// let two_element_description: Description =
///     ["First item", "Second item"].into_iter().collect();
/// let two_element_description_from_strings: Description =
///     ["First item".to_string(), "Second item".to_string()].into_iter().collect();
/// ```
///
/// No newline is added after the last element during rendering. This makes it
/// easier to support single-line matcher descriptions and match explanations.
#[derive(Debug, Default)]
pub struct Description {
    elements: List,
    initial_indentation: usize,
}

impl Description {
    /// Returns a new empty [`Description`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Appends a block of text to this instance.
    ///
    /// The block is indented uniformly when this instance is rendered.
    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.elements.push_literal(text.into());
        self
    }

    /// Appends a nested [`Description`] to this instance.
    ///
    /// The nested [`Description`] `inner` is indented uniformly at the next
    /// level of indentation when this instance is rendered.
    pub fn nested(mut self, inner: Description) -> Self {
        self.elements.push_nested(inner.elements);
        self
    }

    /// Appends all [`Description`] in the given sequence `inner` to this
    /// instance.
    ///
    /// Each element is treated as a nested [`Description`] in the sense of
    /// [`Self::nested`].
    pub fn collect(self, inner: impl IntoIterator<Item = Description>) -> Self {
        inner.into_iter().fold(self, |outer, inner| outer.nested(inner))
    }

    /// Indents the lines in elements of this description.
    ///
    /// This operation will be performed lazily when [`self`] is displayed.
    ///
    /// This will indent every line inside each element.
    ///
    /// For example:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # use googletest::description::Description;
    /// let description = std::iter::once("A B C\nD E F".to_string()).collect::<Description>();
    /// verify_that!(description.indent(), displays_as(eq("  A B C\n  D E F")))
    /// # .unwrap();
    /// ```
    pub fn indent(self) -> Self {
        Self { initial_indentation: INDENTATION_SIZE, ..self }
    }

    /// Instructs this instance to render its elements as a bullet list.
    ///
    /// Each element (from either [`Description::text`] or
    /// [`Description::nested`]) is rendered as a bullet point. If an element
    /// contains multiple lines, the following lines are aligned with the first
    /// one in the block.
    ///
    /// For instance:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # use googletest::description::Description;
    /// let description = Description::new()
    ///     .text("First line\nsecond line")
    ///     .bullet_list();
    /// verify_that!(description, displays_as(eq("\
    /// * First line
    ///   second line")))
    /// # .unwrap();
    /// ```
    pub fn bullet_list(self) -> Self {
        Self { elements: self.elements.bullet_list(), ..self }
    }

    /// Instructs this instance to render its elements as an enumerated list.
    ///
    /// Each element (from either [`Description::text`] or
    /// [`Description::nested`]) is rendered with its zero-based index. If an
    /// element contains multiple lines, the following lines are aligned with
    /// the first one in the block.
    ///
    /// For instance:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # use googletest::description::Description;
    /// let description = Description::new()
    ///     .text("First line\nsecond line")
    ///     .enumerate();
    /// verify_that!(description, displays_as(eq("\
    /// 0. First line
    ///    second line")))
    /// # .unwrap();
    /// ```
    pub fn enumerate(self) -> Self {
        Self { elements: self.elements.enumerate(), ..self }
    }

    /// Returns the length of elements.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the set of elements is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.elements.render(f, self.initial_indentation)
    }
}

impl<ElementT: Into<Cow<'static, str>>> FromIterator<ElementT> for Description {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = ElementT>,
    {
        Self { elements: iter.into_iter().map(ElementT::into).collect(), ..Default::default() }
    }
}

impl FromIterator<Description> for Description {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Description>,
    {
        Self { elements: iter.into_iter().map(|s| s.elements).collect(), ..Default::default() }
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for Description {
    fn from(value: T) -> Self {
        let mut elements = List::default();
        elements.push_literal(value.into());
        Self { elements, ..Default::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::Description;
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn renders_single_fragment() -> Result<()> {
        let description: Description = "A B C".into();
        verify_that!(description, displays_as(eq("A B C")))
    }

    #[test]
    fn renders_two_fragments() -> Result<()> {
        let description =
            ["A B C".to_string(), "D E F".to_string()].into_iter().collect::<Description>();
        verify_that!(description, displays_as(eq("A B C\nD E F")))
    }

    #[test]
    fn nested_description_is_indented() -> Result<()> {
        let description = Description::new()
            .text("Header")
            .nested(["A B C".to_string()].into_iter().collect::<Description>());
        verify_that!(description, displays_as(eq("Header\n  A B C")))
    }

    #[test]
    fn nested_description_indents_two_elements() -> Result<()> {
        let description = Description::new().text("Header").nested(
            ["A B C".to_string(), "D E F".to_string()].into_iter().collect::<Description>(),
        );
        verify_that!(description, displays_as(eq("Header\n  A B C\n  D E F")))
    }

    #[test]
    fn nested_description_indents_one_element_on_two_lines() -> Result<()> {
        let description = Description::new().text("Header").nested("A B C\nD E F".into());
        verify_that!(description, displays_as(eq("Header\n  A B C\n  D E F")))
    }

    #[test]
    fn single_fragment_renders_with_bullet_when_bullet_list_enabled() -> Result<()> {
        let description = Description::new().text("A B C").bullet_list();
        verify_that!(description, displays_as(eq("* A B C")))
    }

    #[test]
    fn single_nested_fragment_renders_with_bullet_when_bullet_list_enabled() -> Result<()> {
        let description = Description::new().nested("A B C".into()).bullet_list();
        verify_that!(description, displays_as(eq("* A B C")))
    }

    #[test]
    fn two_fragments_render_with_bullet_when_bullet_list_enabled() -> Result<()> {
        let description = Description::new().text("A B C").text("D E F").bullet_list();
        verify_that!(description, displays_as(eq("* A B C\n* D E F")))
    }

    #[test]
    fn two_nested_fragments_render_with_bullet_when_bullet_list_enabled() -> Result<()> {
        let description =
            Description::new().nested("A B C".into()).nested("D E F".into()).bullet_list();
        verify_that!(description, displays_as(eq("* A B C\n* D E F")))
    }

    #[test]
    fn single_fragment_with_more_than_one_line_renders_with_one_bullet() -> Result<()> {
        let description = Description::new().text("A B C\nD E F").bullet_list();
        verify_that!(description, displays_as(eq("* A B C\n  D E F")))
    }

    #[test]
    fn single_fragment_renders_with_enumeration_when_enumerate_enabled() -> Result<()> {
        let description = Description::new().text("A B C").enumerate();
        verify_that!(description, displays_as(eq("0. A B C")))
    }

    #[test]
    fn two_fragments_render_with_enumeration_when_enumerate_enabled() -> Result<()> {
        let description = Description::new().text("A B C").text("D E F").enumerate();
        verify_that!(description, displays_as(eq("0. A B C\n1. D E F")))
    }

    #[test]
    fn single_fragment_with_two_lines_renders_with_one_enumeration_label() -> Result<()> {
        let description = Description::new().text("A B C\nD E F").enumerate();
        verify_that!(description, displays_as(eq("0. A B C\n   D E F")))
    }

    #[test]
    fn multi_digit_enumeration_renders_with_correct_offset() -> Result<()> {
        let description = ["A B C\nD E F"; 11]
            .into_iter()
            .map(str::to_string)
            .collect::<Description>()
            .enumerate();
        verify_that!(
            description,
            displays_as(eq(indoc!(
                "
                 0. A B C
                    D E F
                 1. A B C
                    D E F
                 2. A B C
                    D E F
                 3. A B C
                    D E F
                 4. A B C
                    D E F
                 5. A B C
                    D E F
                 6. A B C
                    D E F
                 7. A B C
                    D E F
                 8. A B C
                    D E F
                 9. A B C
                    D E F
                10. A B C
                    D E F"
            )))
        )
    }
}
