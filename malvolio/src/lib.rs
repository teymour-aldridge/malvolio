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
//! Malvolio is "isomorphic" (you can use it both on servers and in browsers – Malvolio integrates
//! with Yew in order to work inside browsers). If you'd like to use Yew, you'll need to activate
//! the "with_yew" feature. If you activate the "with_rocket" feature, Rocket's `Responder` trait is
//! implemented for `Html`, so you can return the `Html` type from routes.
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
//! – after that you should be good to go! If you run into problems feel free to reach out for help.
//! The best place to do this at the moment is the Yew Discord server (a link to this can be found
//! on the [Yew website](https://yew.rs))
//!
//! Note that although Malvolio *should* support all of the HTML specification, it might be missing
//! a few pieces – if you spot somethng missing please
//! [file an issue](https://github.com/bailion/malvolio/issues/new)! Malvolio intends to support the
//! full specification.
//!
//! Internally, most things are in the `tag` module, although some common attributes are in the
//! `attributes` module. Some extra items which are only available when the "with_yew" feature is
//! enabled can be found in the `vnode` modules, but these are not exported publically.
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
//! the #malvolio channel on the [Yew Discord server](https://discord.gg/VQck8X4))! Malvolio's
//! source code is located in our [Github repository](https://github.com/bailion/malvolio).

#![deny(missing_docs, missing_debug_implementations)]

#[macro_use]
extern crate derivative;

#[cfg(feature = "pub_fields")]
#[macro_use]
extern crate malvolio_codegen;

#[cfg(feature = "with_proptest")]
#[macro_use]
extern crate proptest_derive;

#[cfg(feature = "with_proptest")]
#[macro_use]
pub(crate) mod strategies;

/// Attributes which can be attached to multiple nodes.
pub mod attributes;
/// A list of types which are useful for using the library. Unless you have name conflicts, we
/// recommend just inserting a `use malvolio::prelude::*;` in files where you're using Malvolio.
pub mod prelude;
/// The different HTML tags which Malvolio supports.
pub mod tags;
/// A text node.
pub mod text;

#[cfg(all(feature = "with_yew", not(feature = "strategies")))]
pub(crate) mod vnode;

#[macro_use]
#[doc(hidden)]
pub(crate) mod macros;
#[doc(hidden)]
pub(crate) mod utils;
#[macro_use]
#[doc(hidden)]
pub(crate) mod docs;
