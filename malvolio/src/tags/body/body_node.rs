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
