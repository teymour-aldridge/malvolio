#[cfg(all(feature = "with_yew", not(feature = "strategies")))]
use yew::virtual_dom::VNode;

/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use crate::{
    enum_display,
    tags::{
        a::A,
        br::Br,
        div::Div,
        form::Form,
        headings::{H1, H2, H3, H4, H5, H6},
        img::Img,
        input::Input,
        label::Label,
        noscript::NoScript,
        p::P,
        select::Select,
    },
    text::Text,
    utility_enum,
};

utility_enum!(
    #[allow(missing_docs)]
    /// A node which can be mounted to the <body> tag (or any of its children).
    pub enum BodyNode {
        H1(H1),
        H2(H2),
        H3(H3),
        H4(H4),
        H5(H5),
        H6(H6),
        P(P),
        Text(Text),
        Form(Form),
        Br(Br),
        Div(Div),
        A(A),
        Input(Input),
        Label(Label),
        Select(Select),
        NoScript(NoScript),
        Img(Img),
        #[cfg(all(feature = "with_yew", not(feature = "strategies")))]
        VNode(VNode),
    }
);

enum_display!(
    BodyNode, H1, H2, H3, H4, H5, H6, P, Br, Text, Form, Div, A, Input, Select, NoScript, Img,
    Label
);

#[cfg(all(feature = "with_yew", not(feature = "strategies")))]
mod vnode_impls {
    use crate::vnode::IntoVNode;

    use super::*;

    impl IntoVNode for BodyNode {
        fn into_vnode(self) -> VNode {
            match self {
                BodyNode::H1(h) => h.into_vnode(),
                BodyNode::H2(h) => h.into_vnode(),
                BodyNode::H3(h) => h.into_vnode(),
                BodyNode::H4(h) => h.into_vnode(),
                BodyNode::H5(h) => h.into_vnode(),
                BodyNode::H6(h) => h.into_vnode(),
                BodyNode::P(p) => p.into_vnode(),
                BodyNode::Text(t) => t.into_vnode(),
                BodyNode::Form(f) => f.into_vnode(),
                BodyNode::Br(b) => b.into_vnode(),
                BodyNode::Div(d) => d.into_vnode(),
                BodyNode::A(a) => a.into_vnode(),
                BodyNode::Input(i) => i.into_vnode(),
                BodyNode::Label(l) => l.into_vnode(),
                BodyNode::Select(s) => s.into_vnode(),
                BodyNode::NoScript(n) => n.into_vnode(),
                BodyNode::Img(i) => i.into_vnode(),
                BodyNode::VNode(v) => v,
            }
        }
    }
}
