use std::{borrow::Cow, collections::HashSet};

use super::IntoAttribute;

#[derive(Debug, Derivative, Clone)]
#[derivative(Default(new = "true"))]

/// A builder for constructing values for the `class` attribute.
pub struct Class(HashSet<Cow<'static, str>>);

impl From<Cow<'static, str>> for Class {
    fn from(str: Cow<'static, str>) -> Self {
        let mut set = HashSet::new();
        set.insert(str);
        Self(set)
    }
}

impl From<&'static str> for Class {
    fn from(str: &'static str) -> Self {
        let mut set = HashSet::new();
        set.insert(str.into());
        Self(set)
    }
}

impl Class {
    /// Add a new class to this `Class` attribute.
    pub fn class(mut self, class: Cow<'static, str>) -> Self {
        self.0.insert(class);
        self
    }
}

impl IntoAttribute for Class {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        (
            "class".into(),
            self.0.into_iter().collect::<Vec<_>>().join(" ").into(),
        )
    }
}

#[derive(Debug, Default, Clone)]

/// The "id" attribute.
pub struct Id(Cow<'static, str>);

impl Id {
    /// Create a new instance of this attribute with the specified value.
    pub fn new<C>(c: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self(c.into())
    }
}

impl IntoAttribute for Id {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        ("id".into(), self.0)
    }
}

#[derive(Debug, Default, Clone)]

/// The "style" attribute.
pub struct Style(Cow<'static, str>);

impl Style {
    /// Create a new instance of this attribute with the specified value.
    pub fn new<C>(string: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self(string.into())
    }
}

impl IntoAttribute for Style {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        ("style".into(), self.0)
    }
}
