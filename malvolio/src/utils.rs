/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::{borrow::Cow, collections::HashMap};

pub fn write_attributes(
    attrs: &HashMap<Cow<'static, str>, Cow<'static, str>>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    for (key, value) in attrs {
        f.write_str(key)?;
        f.write_str("=\"")?;
        match value {
            Cow::Borrowed(b) => {
                f.write_str(b)?;
            }
            Cow::Owned(string) => {
                f.write_str(&string)?;
            }
        }
        f.write_str("\"")?;
    }
    Ok(())
}
