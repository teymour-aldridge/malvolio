/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
pub use crate::tags::{
    a::{a, Href, A},
    body::body_node::BodyNode,
    body::{body, Body},
    br::Br,
    div::{div, Div},
    form::{form, Action, Form, Method},
    head::{head, Head},
    headings::{h1, h2, h3, h4, h5, h6, H1, H2, H3, H4, H5, H6},
    html::{html, Html},
    img::{img, Alt, Img, Src},
    input::{input, Input, Name, Placeholder, Type, Value},
    label::{label, Label},
    meta::{meta, Content, Meta, MetaName},
    option::{select_option, SelectOption},
    p::{p, P},
    select::{select, Select},
    style::{style, StyleTag},
    title::{title, Title},
};

pub use crate::attributes::common::{Class, Id, Style};
