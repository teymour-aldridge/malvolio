/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/

use crate::{
    attributes::IntoAttribute,
    into_attribute_for_grouping_enum, into_grouping_union,
    prelude::{Id, Style},
    utility_enum,
};
use ammonia::clean;
use std::{borrow::Cow, collections::HashMap, fmt::Display};

use super::body::body_node::BodyNode;

/// A link (anchor).
///
/// ```
/// # use malvolio::prelude::*;
/// A::default()
///     .attribute(Href::new("https://example.com/mark-read"))
///     .text("Mark as read");
/// ```
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-download)
/// for further information.
#[derive(Debug, Clone, Derivative)]
#[derivative(Default(new = "true"))]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "with_proptest", derive(Arbitrary))]
pub struct A {
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
    text: Cow<'static, str>,
}

/// Creates a new `A` tag – functionally equivalent to `A::new()` (but easier to type.)
pub fn a() -> A {
    A::new()
}

impl A {
    /// Adds the supplied text to this node, overwriting the previously existing text (if text has
    /// already been added to the node).
    ///
    /// This method sanitises the input (i.e. it escapes HTML);
    /// this might not be what you want – if you are *absolutely certain* that the text you are
    /// providing does not come from a potentially malicious source (e.g. user-supplied text can
    /// contain script tags which will execute unwanted code) you can use `text_unsanitized` which
    /// is identical to this method, except for that it does not sanitise the inputted text (and is
    /// thus slightly faster).
    pub fn text<S>(mut self, text: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.text = clean(&text.into()).into();
        self
    }

    /// Adds the supplied text to this node, overwriting the previously existing text (if text has
    /// already been added to the node).
    ///
    /// WARNING: Do not (under any circumstances) use this method with unescaped user-supplied text.
    /// It will be rendered and poses a major security threat to your application. If in doubt, use
    /// the `text` method instead of this one (the risk is much lower that way).
    pub fn text_unsanitized<S>(mut self, text: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.text = text.into();
        self
    }

    /// Adds an attribute to this node. This method takes one argument which must implement
    /// `Into<AAttr>`.
    pub fn attribute<I>(mut self, attribute: I) -> Self
    where
        I: Into<AAttr>,
    {
        let res = attribute.into().into_attribute();
        self.attrs.insert(res.0, res.1);
        self
    }

    /// Attach a new `href` attribute to this tag.
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// A::new().href("http://example.com");
    /// ```
    ///
    /// Note: this method is a shortcut for
    /// ```rust
    /// # use malvolio::prelude::*;
    /// A::new().attribute(Href::new("https://example.com"));
    /// ```
    pub fn href<C>(self, href: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        self.attribute(Href::new(href))
    }

    /// Attach a new `id` attribute to this tag.
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// A::new().id("some-id");
    /// ```
    ///
    /// Note: this method is a shortcut for
    /// ```rust
    /// # use malvolio::prelude::*;
    /// A::new().attribute(Id::new("some-id"));
    /// ```
    pub fn id<C>(self, id: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        self.attribute(Id::new(id))
    }

    /// Read an attribute that has been set.
    pub fn read_attribute(&self, attribute: &'static str) -> Option<&Cow<'static, str>> {
        self.attrs.get(attribute)
    }
}

impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<a")?;
        for attr in &self.attrs {
            f.write_str(" ")?;
            attr.0.fmt(f)?;
            f.write_str("=\"")?;
            attr.1.fmt(f)?;
            f.write_str("\"")?;
        }
        f.write_str("\"")?;
        f.write_str(">")?;
        self.text.fmt(f)?;
        f.write_str("</a>")
    }
}

into_grouping_union!(A, BodyNode);

utility_enum!(
    /// An attribute for the A tag.
    #[allow(missing_docs)]
    pub enum AAttr {
        Href(Href),
        Download(Download),
        Target(Target),
        Id(Id),
        Style(Style),
    }
);

into_grouping_union!(Id, AAttr);

into_grouping_union!(Style, AAttr);

into_attribute_for_grouping_enum!(AAttr, Href, Download, Target, Id, Style);

/// The "href" attribute (currently only usable with the `<a>` tags, but support for other tags is
/// planned – if you need support now, feel free – and welcome/encouraged – to submit a pull
/// request).
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-href)
/// for further information.
#[derive(Debug, Clone)]
pub struct Href(Cow<'static, str>);

impl Href {
    /// Create a new `Href` attribute. This method accepts any item that implements
    /// `Into<Cow<'static, str>>` (which includes `String` and `&str`).
    pub fn new<C>(value: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self(value.into())
    }
}

into_grouping_union!(Href, AAttr);

/// The download attribute.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-download)
/// for further information.
#[derive(Debug, Clone)]
pub struct Download(Cow<'static, str>);

impl Download {
    /// Create a new attribute from the provided value.
    pub fn new<C>(value: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self(value.into())
    }
}

impl IntoAttribute for Download {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        ("download".into(), self.0)
    }
}

into_grouping_union!(Download, AAttr);

impl IntoAttribute for Href {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        ("href".into(), self.0)
    }
}

/// The "target" attribute for a link.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-target)
/// for further information.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum Target {
    Blank,
}

into_grouping_union!(Target, AAttr);

impl IntoAttribute for Target {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        (
            "target".into(),
            match self {
                Target::Blank => "_blank".into(),
            },
        )
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    #[test]
    fn test_a_with_attributes() {
        let document = A::default()
            .attribute(super::Href::new("https://example.com"))
            .attribute(super::Target::Blank)
            .attribute(super::Download::new("some-download"))
            .to_string();
        let document = scraper::Html::parse_document(&document);
        let a = scraper::Selector::parse("a").unwrap();
        let a = document.select(&a).next().unwrap().value();
        assert_eq!(a.attr("href").unwrap(), "https://example.com");
        assert_eq!(a.attr("target").unwrap(), "_blank");
        assert_eq!(a.attr("download").unwrap(), "some-download");
    }
}
