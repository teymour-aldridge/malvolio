/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use crate::{
    attributes::IntoAttribute,
    into_attribute_for_grouping_enum, into_grouping_union,
    prelude::{Class, Id, Style},
    utility_enum,
};

#[cfg(feature = "with_yew")]
#[cfg(not(tarpaulin))]
use std::rc::Rc;
use std::{borrow::Cow, collections::HashMap, fmt::Display};

#[cfg(feature = "with_yew")]
#[cfg(not(tarpaulin))]
use yew::virtual_dom::Listener;

use super::body::body_node::BodyNode;

#[derive(Debug, Clone, Derivative)]
#[derivative(Default(new = "true"))]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "with_proptest", derive(Arbitrary))]
/// A form input.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
/// for further information.
pub struct Input {
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

/// Creates a new `Input` tag – functionally equivalent to `Input::new()` (but easier to type.)
pub fn input() -> Input {
    Input::new()
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<input")?;
        for attr in &self.attrs {
            f.write_str(" ")?;
            attr.0.fmt(f)?;
            f.write_str("=\"")?;
            attr.1.fmt(f)?;
            f.write_str("\"")?;
        }
        f.write_str("/>")
    }
}

into_grouping_union!(Input, BodyNode);

impl Input {
    #[inline(always)]
    /// Attach a new attribute to this type.
    pub fn attribute<C>(mut self, c: C) -> Self
    where
        C: Into<InputAttr>,
    {
        let (a, b) = c.into().into_attribute();
        self.attrs.insert(a, b);
        self
    }

    /// Read an attribute that has been set
    pub fn read_attribute(&self, attribute: &'static str) -> Option<&Cow<'static, str>> {
        self.attrs.get(attribute)
    }

    /// Apply a function to this tag.
    pub fn map<F>(self, mapping: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        mapping(self)
    }
}

utility_enum!(
    /// The possible attributes which can be attached to an input item.
    #[allow(missing_docs)]
    pub enum InputAttr {
        Type(Type),
        Name(Name),
        Placeholder(Placeholder),
        Id(Id),
        Class(Class),
        Value(Value),
        Style(Style),
    }
);

into_attribute_for_grouping_enum!(InputAttr, Type, Name, Placeholder, Id, Class, Value, Style);

into_grouping_union!(Id, InputAttr);
into_grouping_union!(Class, InputAttr);
into_grouping_union!(Value, InputAttr);
into_grouping_union!(Style, InputAttr);
into_grouping_union!(Name, InputAttr);
into_grouping_union!(Type, InputAttr);
into_grouping_union!(Placeholder, InputAttr);

/// The `type` attribute for an input.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-type)
/// for further information.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum Type {
    Text,
    Email,
    Password,
    Textarea,
    Submit,
    Hidden,
    DateTimeLocal,
    Checkbox,
}

impl IntoAttribute for Type {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        (
            "type".into(),
            match self {
                Type::Text => "text",
                Type::Email => "email",
                Type::Password => "password",
                Type::Submit => "submit",
                Type::Textarea => "textarea",
                Type::Hidden => "hidden",
                Type::DateTimeLocal => "datetime-local",
                Type::Checkbox => "checkbox",
            }
            .into(),
        )
    }
}

/// The `name` attribute for an input.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-name)
/// for further information.
#[derive(Debug, Clone)]

pub struct Name(Cow<'static, str>);

impl IntoAttribute for Name {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        ("name".into(), self.0)
    }
}

impl Name {
    /// Create a new instance of this attribute with the specified value.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self(s.into())
    }
}

/// The "placeholder" attribute for an input field.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-placeholder)
/// for further information.
#[derive(Debug, Clone)]
pub struct Placeholder(Cow<'static, str>);

impl IntoAttribute for Placeholder {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        ("placeholder".into(), self.0)
    }
}

impl Placeholder {
    /// Create a new instance of this attribute with the specified value.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self(s.into())
    }
}

/// The "value" attribute for an input field.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-value)
/// for further information.
#[derive(Debug, Clone)]
pub struct Value(Cow<'static, str>);

impl Value {
    /// Create a new instance of this attribute with the specified value.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self(s.into())
    }
}

impl IntoAttribute for Value {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        ("value".into(), self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    #[test]
    fn test_input() {
        let document = Input::default()
            .attribute(Id::new("some-id"))
            .attribute(Placeholder::new("some-placeholder"))
            .attribute(Value::new("some-value"))
            .to_string();
        let document = scraper::Html::parse_document(&document);
        let input = scraper::Selector::parse("input").unwrap();
        let input = document.select(&input).next().unwrap().value();
        assert_eq!(input.attr("id"), Some("some-id"));
        assert_eq!(input.attr("placeholder"), Some("some-placeholder"));
        assert_eq!(input.attr("value"), Some("some-value"));
    }
}
