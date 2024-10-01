use proc_macro::{self, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(WithIgnoredFields, attributes(ignore_field))]
pub fn my_derive_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: class_name,
        data,
        ..
    } = parse_macro_input!(input);

    let class_with_ignored_fields = format_ident!("{}WithIgnoredFields", &class_name);

    let struct_data = match data {
        Data::Struct(struct_data) => struct_data,
        _ => panic!("Not supported"),
    };

    let fields_for_class_with_ignored_fields = struct_data
        .fields
        .iter()
        .filter(|field| {
            !field
                .attrs
                .iter()
                .any(|attr| attr.to_token_stream().to_string() == "#[ignore_field]")
        })
        .collect::<Vec<_>>();

    let output = quote! {
        struct #class_with_ignored_fields {
            #( #fields_for_class_with_ignored_fields ),*
        }
    };

    output.into()
}
