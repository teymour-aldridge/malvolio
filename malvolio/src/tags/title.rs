/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::{borrow::Cow, collections::HashMap};

use crate::{heading_display, impl_of_heading_new_fn, into_grouping_union};

use super::head::head_node::HeadNode;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "with_proptest", derive(Arbitrary))]
/// The <title> tag.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title) for more
/// info.
pub struct Title {
    text: Cow<'static, str>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl_of_heading_new_fn!(Title, title);

heading_display!(Title);

into_grouping_union!(Title, HeadNode);

#[cfg(all(feature = "with_yew", not(feature = "strategies")))]
mod vnode_impls {
    use yew::virtual_dom::{VTag, VText};

    use crate::vnode::IntoVNode;

    use super::*;

    impl IntoVNode for Title {
        fn into_vnode(self) -> yew::virtual_dom::VNode {
            let mut tag = VTag::new("title");
            for (k, v) in self.attrs {
                if let ::std::borrow::Cow::Borrowed(string) = k {
                    tag.add_attribute(string, v);
                } else {
                    panic!("Dynamic keys for Yew are not yet supported.")
                }
            }
            tag.add_child(VText::new(self.text).into());
            tag.into()
        }
    }
}
