/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
//! # Start here
//!
//! Malvolio is a library for writing HTML. While there are many approaches that
//! have developed for producing HTML code inside of Rust programs, most of
//! these use macros; Malvolio's API avoids trying to escape confines of the
//! language by jumping out the macro "escape hatch".
//!
//! Although in early stages, Malvolio works and is suitable for general use.
//! There are likely to be quite a few API breakages, however.
//!
//! # Crate structure (for consumers of the crate)
//!
//! Most of the important things are re-exported from `prelude`. If you're
//! burning to try Malvolio out, adding `use malvolio::prelude::*;` will import
//! all the HTML tags that Malvolio supports – after that you should be good to
//! go!
//!
//! Note that although Malvolio *should* support all of the HTML specification,
//! it might be missing a few pieces – if you spot somethng missing please
//! [file an issue](https://github.com/puck-rs/malvolio/issues/new)! Malvolio
//! intends to support the full specification.
//!
//! Internally, most things are in the `tag` module, although some common
//! attributes are in the `attributes` module.

#![deny(missing_docs, missing_debug_implementations)]

/// Attributes which can be attached to multiple nodes.
pub mod attributes;
/// A list of types which are useful for using the library. Unless you have name
/// conflicts, we recommend just inserting a `use malvolio::prelude::*;` in
/// files where you're using Malvolio.
pub mod prelude;
/// The different HTML tags which Malvolio supports.
pub mod tags;
/// A text node.
pub mod text;

#[macro_use]
#[doc(hidden)]
pub(crate) mod macros;
#[doc(hidden)]
pub(crate) mod utils;
#[macro_use]
#[doc(hidden)]
pub(crate) mod docs;
