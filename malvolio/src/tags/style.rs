use crate::into_grouping_union;
use crate::tags::head::head_node::HeadNode;
use std::{borrow::Cow, fmt::Display};

/// The `<style>` tag, useful for embedding CSS styling inside HTML documents.
///
/// See [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta) for
/// further information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "with_proptest", derive(Arbitrary))]
pub struct StyleTag {
    text: Cow<'static, str>,
}

/// Creates a new `Style` tag – functionally equivalent to `Style::new()` (but easier to type.)
pub fn style(text: impl Into<Cow<'static, str>>) -> StyleTag {
    StyleTag::new(text)
}

impl StyleTag {
    /// Create a new style tag.
    pub fn new<C>(c: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self { text: c.into() }
    }
}

impl Display for StyleTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<style>")?;
        f.write_str(&self.text)?;
        f.write_str("</style>")
    }
}

into_grouping_union!(StyleTag, HeadNode);
