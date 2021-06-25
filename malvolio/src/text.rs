/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::{borrow::Cow, collections::HashMap, fmt::Display};

use crate::{impl_of_heading_new_fn, into_grouping_union, tags::body::body_node::BodyNode};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]

/// A text node.
pub struct Text {
    text: Cow<'static, str>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl_of_heading_new_fn!(Text, text);

into_grouping_union!(Text, BodyNode);

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.text.fmt(f)
    }
}

#[cfg(test)]
mod test_sanitize {
    use super::Text;

    #[test]
    fn test_unsanitized() {
        let text = Text::new("<script>alert(\"hello\")</script>");
        assert_ne!(&text.to_string(), "<script>alert(\"hello\")</script>");

        let text = Text::new_unchecked("<script>alert(\"hello\")</script>");
        assert_eq!(&text.to_string(), "<script>alert(\"hello\")</script>");
    }
}
