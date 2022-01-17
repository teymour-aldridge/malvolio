/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
//! # Start here
//!
//! Malvolio is a library for writing declarative, strongly-typed HTML. If you've come accross the
//! "builder pattern" that's what, in essence, this library is. While there are many excellent
//! (and thoughtful) approaches people have developed for producing HTML code inside of Rust
//! programs, most of these use macros; Malvolio's API avoids trying to escape confines of the
//! language by jumping out the macro "escape hatch".
//!
//! Although in early stages, Malvolio works and is suitable for general use. There are likely to be
//! quite a few API breakages, however.
//!
//! # Safety
//!
//! Malvolio uses no unsafe code.
//!
//! # Crate structure (for consumers of the crate)
//!
//! Most of the important things are re-exported from `prelude`. If you're burning to try Malvolio
//! out, adding `use malvolio::prelude::*;` will import all the HTML tags that Malvolio supports
//! – after that you should be good to go!
//!
//! Note that although Malvolio *should* support all of the HTML specification, it might be missing
//! a few pieces – if you spot somethng missing please
//! [file an issue](https://github.com/bailion/malvolio/issues/new)! Malvolio intends to support the
//! full specification.
//!
//! Internally, most things are in the `tag` module, although some common attributes are in the
//! `attributes` module.
//!
//! # CSS support
//!
//! Malvolio supports standard CSS things (classes, style attributes), but if you're looking for a
//! "rust in css" style solution (which allows you to write your CSS using Rust) have a look at a
//! library we maintain for doing this called [Mercutio](https://docs.rs/mercutio). Note that this
//! is not yet released.
//!
//! # UI library
//!
//! This may or may not work for you, but we also maintain a UI library called Portia (I know,
//! so many Shakespearian characters :D) that has a (we think, but if you don't think, please give
//! feedback :D) nice set of abstractions that make it easier to write user interfaces more easily.
//! This is also not yet released.
//!
//! # Contributing
//!
//! We welcome contributions, issues, concerns and suggestions (if you just want to chat, join us in
//! Malvolio's source code is located in our
//! [Github repository](https://github.com/bailion/malvolio).

#![deny(missing_docs, missing_debug_implementations)]
#![cfg_attr(feature = "fuzz", feature(type_alias_impl_trait, no_coverage))]

#[cfg(all(feature = "fuzz", feature = "pub_fields", test))]
mod fuzz;

#[macro_use]
extern crate derivative;

#[cfg(feature = "pub_fields")]
#[macro_use]
extern crate malvolio_codegen;

/// Attributes which can be attached to multiple nodes.
pub mod attributes;
/// A list of types which are useful for using the library. Unless you have name conflicts, we
/// recommend just inserting a `use malvolio::prelude::*;` in files where you're using Malvolio.
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
#[cfg(feature = "fuzz")]
#[cfg_attr(feature = "fuzz", no_coverage)]
pub(crate) mod mutators;
