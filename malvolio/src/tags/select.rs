/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/

use std::{borrow::Cow, collections::HashMap, fmt::Display};

use crate::{
    into_attribute_for_grouping_enum, into_grouping_union,
    prelude::{Class, Id},
    utility_enum,
    utils::write_attributes,
};

use crate::attributes::IntoAttribute;

use super::{body::body_node::BodyNode, input::Name, option::SelectOption};

#[derive(Derivative, Debug, Clone)]
#[derivative(Default(new = "true"))]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "with_proptest", derive(Arbitrary))]
/// The `select` tag.
///
/// See [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select) for
/// further information.
pub struct Select {
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
    children: Vec<SelectOption>,
}

/// Creates a new `Select` tag – functionally equivalent to `Select::new()` (but easier to type.)
pub fn select() -> Select {
    Select::new()
}

impl Select {
    /// Add a number of children to a <select> tag.
    pub fn children<I, C>(mut self, children: I) -> Self
    where
        C: Into<SelectOption>,
        I: IntoIterator<Item = C>,
    {
        self.children
            .extend(children.into_iter().map(Into::into).collect::<Vec<_>>());
        self
    }

    /// Add a single child to a <select> tag.
    pub fn child<C>(mut self, child: C) -> Self
    where
        C: Into<SelectOption>,
    {
        self.children.push(child.into());
        self
    }

    /// Add an attribute to the select in question.
    pub fn attribute<A>(mut self, attr: A) -> Self
    where
        A: Into<SelectAttr>,
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

into_grouping_union!(Select, BodyNode);

impl Display for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<select ")?;
        write_attributes(&self.attrs, f)?;
        f.write_str(">")?;
        for child in &self.children {
            child.fmt(f)?;
        }
        f.write_str("</select>")
    }
}

utility_enum!(
    #[allow(missing_docs)]
    pub enum SelectAttr {
        Name(Name),
        Class(Class),
        Id(Id),
    }
);

into_attribute_for_grouping_enum!(SelectAttr, Name, Class, Id);

into_grouping_union!(Name, SelectAttr);
into_grouping_union!(Class, SelectAttr);
into_grouping_union!(Id, SelectAttr);
