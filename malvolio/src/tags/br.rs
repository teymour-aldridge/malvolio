/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::fmt::Display;

use crate::into_grouping_union_without_lifetimes;

use super::body::body_node::BodyNode;

#[derive(Debug, Clone)]

/// A new line.
///
/// ```
/// # use malvolio::prelude::*;
/// Div::new().child(Br);
/// ```
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/br) for more
/// info.
#[cfg_attr(feature = "with_proptest", derive(Arbitrary))]
pub struct Br;

impl Display for Br {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<br/>")
    }
}

into_grouping_union_without_lifetimes!(Br, BodyNode);

#[cfg(test)]
mod test {
    use crate::prelude::*;
    #[test]
    fn test_br() {
        let document = Br.to_string();
        let document = scraper::Html::parse_document(&document);
        let br = scraper::Selector::parse("br").unwrap();
        document.select(&br).next().unwrap();
    }
}
