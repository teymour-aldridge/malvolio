use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FieldsAccessibleVariant)]
pub fn fields_accessible_variant(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let r#struct = match derive_input.data.clone() {
        syn::Data::Struct(s) => s,
        _ => {
            return syn::Error::new_spanned(
                derive_input.ident,
                "The macro #[fields_accessible_variant] can only be used on structs.",
            )
            .into_compile_error()
            .into();
        }
    };

    let struct_name = derive_input.ident.clone();

    let vis = derive_input.vis.clone();

    let name = format_ident!("{}Pub", derive_input.ident);

    let fields = r#struct
        .fields
        .iter()
        .map(|field| {
            let ty = field.ty.clone();

            if let Some(name) = field.ident.clone() {
                quote! {
                    pub #name : #ty ,
                }
            } else {
                quote! {
                    #ty,
                }
            }
        })
        .fold(quote! {}, |a, b| {
            quote! { #a #b }
        });

    let conversion = r#struct
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            if let Some(name) = field.ident.clone() {
                quote! {
                    #name : self . #name ,
                }
            } else {
                quote! {
                    self . #index,
                }
            }
        })
        .fold(quote! {}, |a, b| quote! {#a #b});

    let conversion = quote! {
        #name {
            #conversion
        }
    };

    let new_struct = quote! {
        /// A variant of the struct in question which exposes all its fields for consumption.
        #[allow(missing_docs)]
        #vis struct #name {
            #fields
        }
    };

    From::from(quote! {
        #new_struct

        impl #struct_name {
            /// Convert the item in question into an identical struct, but with `pub` fields.
            ///
            /// Only really useful for library authors. To use this struct, you need to activate the
            /// `pub_fields` feature.
            pub fn into_pub_fields(self) -> #name {
                #conversion
            }
        }
    })
}
