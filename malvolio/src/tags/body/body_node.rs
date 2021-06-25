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
    }
);

enum_display!(
    BodyNode, H1, H2, H3, H4, H5, H6, P, Br, Text, Form, Div, A, Input, Select, NoScript, Img,
    Label
);

#[cfg(feature = "with_proptest")]
pub(crate) mod body_proptest {
    use super::*;

    use proptest::prelude::*;

    pub(crate) fn body_node() -> BoxedStrategy<BodyNode> {
        let leaf = prop_oneof![
            any::<H1>().prop_map(BodyNode::H1),
            any::<H2>().prop_map(BodyNode::H2),
            any::<H3>().prop_map(BodyNode::H3),
            any::<H4>().prop_map(BodyNode::H4),
            any::<H5>().prop_map(BodyNode::H5),
            any::<H6>().prop_map(BodyNode::H6),
            any::<Br>().prop_map(BodyNode::Br),
            any::<A>().prop_map(BodyNode::A),
            any::<Input>().prop_map(BodyNode::Input),
            any::<Label>().prop_map(BodyNode::Label),
            any::<Select>().prop_map(BodyNode::Select),
            any::<NoScript>().prop_map(BodyNode::NoScript),
            any::<Img>().prop_map(BodyNode::Img)
        ];

        leaf.prop_recursive(2, 16, 1, |inner| {
            prop_oneof![
                prop::collection::vec(inner.clone(), 0..10)
                    .prop_map(|prop| { BodyNode::Div(Div::default().children(prop)) }),
                prop::collection::vec(inner.clone(), 0..10)
                    .prop_map(|item| { BodyNode::P(P::default().children(item)) }),
                prop::collection::vec(inner.clone(), 0..10)
                    .prop_map(|item| { BodyNode::Form(Form::default().children(item)) })
            ]
        })
        .boxed()
    }

    impl Arbitrary for BodyNode {
        type Parameters = ();

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            body_node()
        }

        type Strategy = BoxedStrategy<BodyNode>;
    }
}
