/// Stores a number of common attributes.
pub mod common;

use std::borrow::Cow;

/// Allows you to convert items into attributes.
pub trait IntoAttribute {
    /// Convert the current item into an attribute.
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>);
}
