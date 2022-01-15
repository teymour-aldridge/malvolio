/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::fmt::Display;

use super::{body::Body, head::Head};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
/// Construct a HTML document. If you are trying to render to a string, this is what you want to use.
pub struct Html {
    head: Head,
    body: Body,
}

/// Creates a new `Html` tag – functionally equivalent to `Html::new()` (but easier to type.)
pub fn html() -> Html {
    Html::new()
}

impl Default for Html {
    fn default() -> Self {
        Self {
            head: Head::default(),
            body: Body::default(),
        }
    }
}

impl Display for Html {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<!DOCTYPE html>")?;
        f.write_str("<html>")?;
        self.head.fmt(f)?;
        self.body.fmt(f)?;
        f.write_str("</html>")?;
        Ok(())
    }
}

impl Html {
    /// Create a new `Html` tag. Note that this is exactly the same as `Html::default()`, but it is
    /// a few characters shorter, so is provided as a convenience method.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach a <head> tag to this `Html` instance.
    pub fn head(mut self, head: Head) -> Self {
        self.head = head;
        self
    }

    /// Attach a new <body> tag to this `Html` instance.
    pub fn body(mut self, body: Body) -> Self {
        self.body = body;
        self
    }
}
