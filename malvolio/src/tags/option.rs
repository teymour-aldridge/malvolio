/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/

use crate::{
    into_attribute_for_grouping_enum, into_grouping_union, prelude::Id, utility_enum,
    utils::write_attributes,
};

use crate::attributes::IntoAttribute;
use ammonia::clean;
use std::{borrow::Cow, collections::HashMap, fmt::Display};

use super::input::{Name, Value};

#[derive(Derivative, Debug, Clone)]
#[derivative(Default(new = "true"))]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "with_proptest", derive(Arbitrary))]
/// The `option` tag.
///
/// See [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option) for
/// further information..
pub struct SelectOption {
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
    text: Cow<'static, str>,
}

/// Creates a new `SelectOption` tag – functionally equivalent to `SelectOption::new()` (but easier
/// to type.)
pub fn select_option() -> SelectOption {
    Default::default()
}

impl SelectOption {
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

    /// Attach a new attribute to this type. Note that this will overwrite existing values for the
    /// attribute, if one has been provided.
    pub fn attribute<A>(mut self, attr: A) -> Self
    where
        A: Into<SelectOptionAttr>,
    {
        let (a, b) = attr.into().into_attribute();
        self.attrs.insert(a, b);
        self
    }

    crate::define_raw_attribute_fn!();

    /// Read an attribute that has been set
    pub fn read_attribute(&self, attribute: &'static str) -> Option<&Cow<'static, str>> {
        self.attrs.get(attribute)
    }
}

impl Display for SelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<option ")?;
        write_attributes(&self.attrs, f)?;
        f.write_str(">")?;
        self.text.fmt(f)?;
        f.write_str("</option>")
    }
}

utility_enum!(
    /// An attribute for the <select> tag.
    #[allow(missing_docs)]
    pub enum SelectOptionAttr {
        Value(Value),
        Id(Id),
        Name(Name),
    }
);

into_attribute_for_grouping_enum!(SelectOptionAttr, Value, Id, Name);

into_grouping_union!(Value, SelectOptionAttr);
into_grouping_union!(Id, SelectOptionAttr);
into_grouping_union!(Name, SelectOptionAttr);
