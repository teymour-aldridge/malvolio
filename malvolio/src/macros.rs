/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
//! Important: these are not intended for general consumption (only use them internally).
//!
//! A set of macros which are used to reduce the number of times one has to type out the same thing
//! over and over again, which I assure you is very boring (repeated typing of the same thing over
//! and over again tends to lead to asking existential questions as a way to pass the time – I'm
//! rambling here, aren't I :)

#[macro_export]
#[doc(hidden)]
/// For internal use only.
macro_rules! heading_display {
    ($name:ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("<")?;
                f.write_str(stringify!($name))?;
                f.write_str(" ")?;
                crate::utils::write_attributes(&self.attrs, f)?;
                f.write_str(">")?;
                self.text.fmt(f)?;
                f.write_str("</")?;
                f.write_str(stringify!($name))?;
                f.write_str(">")
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// For internal use only.
///
/// Generates new code to construct a heading.
macro_rules! impl_of_heading_new_fn {
    ($name:ident, $lowercase_name:ident) => {
        /// Create a new instance of the tag in question. Equivalent to `<tag name>::new(<text>)`,
        /// but easier to type (and therefore hopefully more ergonomic.)
        pub fn $lowercase_name(text: impl ToString) -> $name {
            $name::new(text)
        }
        impl $name {
            /// Create a new item of this type, given an item which can be converted into a
            /// `Cow<'static, str>` (for example a `&str` or a `String`).
            pub fn new<S>(from: S) -> Self
            where
                S: ToString,
            {
                Self {
                    text: From::from(::ammonia::clean(&from.to_string())),
                    attrs: std::collections::HashMap::new(),
                }
            }
            /// Create a new item of this type **without first sanitizing the text**. You only want
            /// this if you are certain that the text in question is safe (i.e. will not execute
            /// malicious Javascript when run.)
            pub fn new_unchecked<S>(from: S) -> Self
            where
                S: Into<Cow<'static, str>>,
            {
                Self {
                    text: from.into(),
                    attrs: std::collections::HashMap::new(),
                }
            }
            /// Attach a new attribute to this node.
            pub fn attribute<A>(mut self, a: A) -> Self
            where
                A: Into<$crate::tags::headings::HeadingAttr>,
            {
                use $crate::attributes::IntoAttribute;
                let (a, b) = a.into().into_attribute();
                self.attrs.insert(a, b);
                self
            }

            crate::define_raw_attribute_fn!();

            /// Read an attribute that has been set.
            pub fn read_attribute(
                &self,
                a: impl Into<Cow<'static, str>>,
            ) -> Option<&Cow<'static, str>> {
                self.attrs.get(&a.into())
            }
            /// Applies the provided function to this item.
            pub fn map<F>(mut self, mapping: F) -> Self
            where
                F: FnOnce(Self) -> Self,
            {
                self = mapping(self);
                self
            }
        }

        impl From<&'static str> for $name {
            fn from(string: &'static str) -> Self {
                $name::new(string)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// For internal use only.
macro_rules! enum_display {
    ($on:ident, $($variant:ident),*) => {
        impl std::fmt::Display for $on {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(x) => <$variant as std::fmt::Display>::fmt(&x.clone(), f)),*,
                    #[allow(unreachable_patterns)]
                    _ => panic!("Virtual components are not supported.")
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// For internal use only.
macro_rules! into_grouping_union {
    ($name:ident, $enum_name:ident) => {
        impl From<$name> for $enum_name {
            fn from(t: $name) -> $enum_name {
                $enum_name::$name(t)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// For intenal use only.
macro_rules! into_grouping_union_without_lifetimes {
    ($name:ident, $enum_name:ident) => {
        impl From<$name> for $enum_name {
            fn from(t: $name) -> $enum_name {
                $enum_name::$name(t)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_raw_attribute_fn {
    () => {
        /// Attach an attribute to this tag from the provided raw data.
        ///
        /// Note that if you can, you should use the `attribute` method, because it takes better
        /// advantage of Rust's type system.
        pub fn raw_attribute(
            mut self,
            key: impl Into<Cow<'static, str>>,
            value: impl Into<Cow<'static, str>>,
        ) -> Self {
            self.attrs.insert(key.into(), value.into());
            self
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// For internal use only.
macro_rules! add_single_attribute {
    ($lifetime:tt) => {
        #[inline(always)]
        pub fn attribute(mut self, k: & $lifetime str, v: & $lifetime str) -> Self {
            self.attrs.push((k, v));
            self
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// Generates code to convert an attribute into a grouping enum.
///
/// For internal use only.
macro_rules! into_attribute_for_grouping_enum {
    ($name:ident, $($variant:ident),*) => {
        impl $crate::attributes::IntoAttribute for $name {
            fn into_attribute(self) -> (std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>) {
                match self {
                    $(
                        Self::$variant(x) => {$crate::attributes::IntoAttribute::into_attribute(x)}
                    ),*

                }
            }
        }
    };
}

#[cfg(test)]
#[macro_export]
#[doc(hidden)]
/// For internal use only
macro_rules! component_named_app_with_html {
    ($($html:tt)*) => {
        struct App {}
        impl Component for App {
            type Properties = ();
            type Message = ();
            fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
                Self {}
            }
            fn update(&mut self, _msg: Self::Message) -> bool {
                false
            }
            fn change(&mut self, _props: Self::Properties) -> bool {
                false
            }
            fn view(&self) -> ::yew::virtual_dom::VNode {
                $($html)*
            }
        }
    }
}
