use crate::into_grouping_union;
use crate::tags::head::head_node::HeadNode;
use std::{borrow::Cow, fmt::Display};

/// The `<style>` tag, useful for embedding CSS styling inside HTML documents.
///
/// See [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta) for
/// further information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
#[must_use]
pub struct StyleTag {
    text: Cow<'static, str>,
}

#[cfg(feature = "fuzz")]
#[cfg_attr(feature = "fuzz", no_coverage)]
mod style_mutator {
    use std::borrow::Cow;

    use fuzzcheck::{mutators::map::MapMutator, DefaultMutator, Mutator};

    use super::StyleTag;

    #[no_coverage]
    impl DefaultMutator for StyleTag {
        type Mutator = impl Mutator<Self>;

        fn default_mutator() -> Self::Mutator {
            // todo: maybe output valid CSS?
            MapMutator::new(
                crate::mutators::valid_attr_string_mutator::<0>(),
                |tag: &StyleTag| Some(tag.text.to_string()),
                |string| StyleTag {
                    text: Cow::Owned(string.clone()),
                },
                |_, c| c,
            )
        }
    }
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
