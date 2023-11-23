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

/// Helper structure to build better output of
/// [`Matcher::describe`][crate::matcher::Matcher::describe] and
/// [`Matcher::explain_match`][crate::matcher::Matcher::explain_match]. This
/// is especially useful with composed matchers and matchers over containers.
///
/// It provides simple operations to lazily format lists of strings.
///
/// Usage:
/// ```ignore
/// let iter: impl Iterator<String> = ...
/// format!("{}", iter.collect::<Description>().indent().bullet_list())
/// ```
///
/// To construct a [`Description`], use `Iterator<Item=String>::collect()`.
/// Each element of the collected iterator will be separated by a
/// newline when displayed. The elements may be multi-line, but they will
/// nevertheless be indented consistently.
///
/// Note that a newline will only be added between each element, but not
/// after the last element. This makes it simpler to keep
/// [`Matcher::describe`][crate::matcher::Matcher::describe]
/// and [`Matcher::explain_match`][crate::matcher::Matcher::explain_match]
/// consistent with simpler [`Matchers`][crate::matcher::Matcher].
///
/// They can also be indented, enumerated and or
/// bullet listed if [`Description::indent`], [`Description::enumerate`], or
/// respectively [`Description::bullet_list`] has been called.
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
    /// The nested [`Description`] `inner` is indented uniformly at the next level of indentation
    /// when this instance is rendered.
    pub fn nested(mut self, inner: Description) -> Self {
        self.elements.push_nested(inner.elements);
        self
    }

    /// Appends all [`Description`] in the given sequence `inner` to this instance.
    ///
    /// Each element is treated as a nested [`Description`] in the sense of [`Self::nested`].
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

    /// Bullet lists the elements of [`self`].
    ///
    /// This operation will be performed lazily when [`self`] is displayed.
    ///
    /// Note that this will only bullet list each element, not each line
    /// in each element.
    ///
    /// For instance:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # use googletest::description::Description;
    /// let description = std::iter::once("A B C\nD E F".to_string()).collect::<Description>();
    /// verify_that!(description.bullet_list(), displays_as(eq("* A B C\n  D E F")))
    /// # .unwrap();
    /// ```
    pub fn bullet_list(self) -> Self {
        Self { elements: self.elements.bullet_list(), ..self }
    }

    /// Enumerates the elements of [`self`].
    ///
    /// This operation will be performed lazily when [`self`] is displayed.
    ///
    /// Note that this will only enumerate each element, not each line in
    /// each element.
    ///
    /// For instance:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # use googletest::description::Description;
    /// let description = std::iter::once("A B C\nD E F".to_string()).collect::<Description>();
    /// verify_that!(description.enumerate(), displays_as(eq("0. A B C\n   D E F")))
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

impl FromIterator<String> for Description {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        Self { elements: iter.into_iter().collect(), ..Default::default() }
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

impl<T: Into<String>> From<T> for Description {
    fn from(value: T) -> Self {
        let mut elements = List::default();
        elements.push_literal(value.into().into());
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
