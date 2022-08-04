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

#[allow(missing_docs)]
impl BodyNode {
    pub fn as_h1(&self) -> Option<&H1> {
        if let Self::H1(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_h2(&self) -> Option<&H2> {
        if let Self::H2(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_h3(&self) -> Option<&H3> {
        if let Self::H3(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_h4(&self) -> Option<&H4> {
        if let Self::H4(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_h5(&self) -> Option<&H5> {
        if let Self::H5(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_h6(&self) -> Option<&H6> {
        if let Self::H6(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_p(&self) -> Option<&P> {
        if let Self::P(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_text(&self) -> Option<&Text> {
        if let Self::Text(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_form(&self) -> Option<&Form> {
        if let Self::Form(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_br(&self) -> Option<&Br> {
        if let Self::Br(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_div(&self) -> Option<&Div> {
        if let Self::Div(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_a(&self) -> Option<&A> {
        if let Self::A(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_input(&self) -> Option<&Input> {
        if let Self::Input(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_label(&self) -> Option<&Label> {
        if let Self::Label(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_select(&self) -> Option<&Select> {
        if let Self::Select(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_no_script(&self) -> Option<&NoScript> {
        if let Self::NoScript(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_img(&self) -> Option<&Img> {
        if let Self::Img(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
