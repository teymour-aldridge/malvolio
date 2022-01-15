/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::{borrow::Cow, collections::HashMap, fmt::Display};

use crate::{
    attributes::{common::Class, IntoAttribute},
    prelude::{Style, H1, H2, H3, H4, H5, H6},
    to_html,
};

use crate::{into_attribute_for_grouping_enum, into_grouping_union, prelude::Id, utility_enum};

use super::body::body_node::BodyNode;

#[derive(Debug, Derivative, Clone)]
#[derivative(Default(new = "true"))]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
/// A `<div>` tag.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
/// for further information.
pub struct Div {
    children: Vec<BodyNode>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

/// Creates a new `<div>` tag – functionally equivalent to `Div::new()` (but easier to type.)
pub fn div() -> Div {
    Div::new()
}

impl Div {
    /// Attaches zero or more children to a `Div`. This method accepts anything which can be turned
    /// into an iterator of `BodyNode`'s (i.e. most tags that you would expect to be able to use).
    ///
    /// ```
    /// # use malvolio::prelude::*;
    /// Div::new()
    ///     .children(vec![1, 2, 3, 4, 5].into_iter().map(|item| {
    ///         H1::new(format!("Item {}", item))
    ///     }));
    /// ```
    ///
    /// ```
    /// # use malvolio::prelude::*;
    /// div()
    ///    .children(vec![1, 2, 3, 4, 5].into_iter().map(|item| {
    ///         if item % 2 == 0 {
    ///             BodyNode::from(h1(format!("Item {}, as h1", item)))
    ///         } else {
    ///             BodyNode::from(h2(format!("Item {}, as h2", item)))
    ///         }
    ///    }));
    ///
    pub fn children<C, D>(mut self, children: C) -> Self
    where
        C: IntoIterator<Item = D>,
        D: Into<BodyNode>,
    {
        self.children
            .extend(children.into_iter().map(Into::into).collect::<Vec<_>>());
        self
    }

    /// Add a single child to the `Div` in question.
    pub fn child<C>(mut self, child: C) -> Self
    where
        C: Into<BodyNode>,
    {
        self.children.push(child.into());
        self
    }

    /// Allows you to apply a custom function to this `Div`. This function is useful if you want to
    /// modify this tag according to some state captured from the environment.
    ///
    /// For example:
    ///
    /// ```
    /// # use malvolio::prelude::*;
    /// let a_items: Vec<H1> = vec![];
    /// let b_items = vec![H1::new("1"), H1::new("2"), H1::new("3")];
    /// Div::new()
    ///     .map(|div| if a_items.is_empty() {
    ///         div
    ///     } else {
    ///         div.child(H1::new("Some heading for a items")).children(a_items)
    ///     })
    ///     .map(|div| if b_items.is_empty() {
    ///         div
    ///     } else {
    ///         div.child(H1::new("Some heading for b items")).children(b_items)
    ///     });
    /// ```
    pub fn map<F>(mut self, mapping: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        self = mapping(self);
        self
    }

    /// Attach a single attribute to a `Div`. This will overwrite the existing attribute, if it has
    /// already been defined.
    pub fn attribute<A>(mut self, attribute: A) -> Self
    where
        A: Into<DivAttr>,
    {
        let (a, b) = attribute.into().into_attribute();
        self.attrs.insert(a, b);
        self
    }

    crate::define_raw_attribute_fn!();

    /// Read an attribute set on this function.
    pub fn read_attribute(&self, attribute: &'static str) -> Option<&Cow<'static, str>> {
        self.attrs.get(attribute)
    }

    /// Attach a new `H1` instance to this class. Note that this method only allows you to provide
    /// text (you cannot pass extra attributes to the `<h1>` tag). If you want to specify additional
    /// attributes, you should instead use the "child" method (see the documentation of that method
    /// for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h1("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H1::new("Hello World!"));
    /// ```
    pub fn h1<C>(self, c: C) -> Self
    where
        C: Into<H1>,
    {
        self.child(c.into())
    }

    /// Attach a new `H2` instance to this class. Note that this method only allows you to provide
    /// text (you cannot pass extra attributes to the `<h1>` tag). If you want to specify additional
    /// attributes, you should instead use the "child" method (see the documentation of that method
    /// for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h2("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H2::new("Hello World!"));
    /// ```
    pub fn h2<C>(self, c: C) -> Self
    where
        C: Into<H2>,
    {
        self.child(c.into())
    }

    /// Attach a new `H3` instance to this class. Note that this method only allows you to provide
    /// text (you cannot pass extra attributes to the `<h1>` tag). If you want to specify additional
    /// attributes, you should instead use the "child" method (see the documentation of that method
    /// for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h3("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H3::new("Hello World!"));
    /// ```
    pub fn h3<C>(self, c: C) -> Self
    where
        C: Into<H3>,
    {
        self.child(c.into())
    }
    /// Attach a new `H4` instance to this class. Note that this method only allows you to provide
    /// text (you cannot pass extra attributes to the `<h1>` tag). If you want to specify additional
    /// attributes, you should instead use the "child" method (see the documentation of that method
    /// for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h4("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H4::new("Hello World!"));
    /// ```
    pub fn h4<C>(self, c: C) -> Self
    where
        C: Into<H4>,
    {
        self.child(c.into())
    }
    /// Attach a new `H5` instance to this class. Note that this method only allows you to provide
    /// text (you cannot pass extra attributes to the `<h1>` tag). If you want to specify additional
    /// attributes, you should instead use the "child" method (see the documentation of that method
    /// for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h5("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H6::new("Hello World!"));
    /// ```
    pub fn h5<C>(self, c: C) -> Self
    where
        C: Into<H5>,
    {
        self.child(c.into())
    }
    /// Attach a new `H6` instance to this class. Note that this method only allows you to provide
    /// text (you cannot pass extra attributes to the `<h1>` tag). If you want to specify additional
    /// attributes, you should instead use the "child" method (see the documentation of that method
    /// for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h6("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H6::new("Hello World!"));
    /// ```
    pub fn h6<C>(self, c: C) -> Self
    where
        C: Into<H6>,
    {
        self.child(c.into())
    }

    to_html!();
}

impl Display for Div {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<div")?;
        for attr in &self.attrs {
            f.write_str(" ")?;
            attr.0.fmt(f)?;
            f.write_str("=\"")?;
            attr.1.fmt(f)?;
            f.write_str("\"")?;
        }
        f.write_str(">")?;
        for node in &self.children {
            node.fmt(f)?;
        }
        f.write_str("</div>")
    }
}

into_grouping_union!(Div, BodyNode);

utility_enum!(
    #[allow(missing_docs)]
    pub enum DivAttr {
        Id(Id),
        Class(Class),
        Style(Style),
    }
);

into_attribute_for_grouping_enum!(DivAttr, Id, Class, Style);

into_grouping_union!(Id, DivAttr);

into_grouping_union!(Class, DivAttr);

into_grouping_union!(Style, DivAttr);

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::prelude::*;
    #[test]
    fn test_div_attributes() {
        let document = Div::default()
            .attribute(crate::prelude::Class::from(Cow::Borrowed("some-class")))
            .attribute(crate::prelude::Style::new("font-family: Arial;"))
            .to_string();
        let document = scraper::Html::parse_document(&document);
        let div_selector = scraper::Selector::parse("div").unwrap();
        assert_eq!(document.select(&div_selector).count(), 1);
        let div = document.select(&div_selector).next().unwrap();
        assert_eq!(div.value().attr("class").unwrap(), "some-class");
        assert_eq!(div.value().attr("style").unwrap(), "font-family: Arial;");
    }
    #[test]
    fn test_div_children() {
        let document = Div::default()
            .children(vec!["1", "2", "3"].into_iter().map(P::with_text))
            .to_string();
        let document = scraper::Html::parse_document(&document);
        let div_selector = scraper::Selector::parse("div").unwrap();
        let div = document.select(&div_selector).next().unwrap();
        let children = div.children().collect::<Vec<_>>();
        assert_eq!(
            children[0]
                .children()
                .next()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .to_string()
                .as_str(),
            "1"
        );
        assert_eq!(
            children[1]
                .children()
                .next()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .to_string()
                .as_str(),
            "2"
        );
        assert_eq!(
            children[2]
                .children()
                .next()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .to_string()
                .as_str(),
            "3"
        );
    }
}
