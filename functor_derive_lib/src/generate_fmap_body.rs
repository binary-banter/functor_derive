use crate::generate_map::generate_map_from_type;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, Fields};

pub fn generate_fmap_body(
    data: &Data,
    def_name: &Ident,
    functor_param: &Ident,
    is_try: bool,
) -> TokenStream {
    match data {
        Data::Struct(d) => generate_fmap_body_struct(d, functor_param, def_name, is_try),
        Data::Enum(d) => generate_fmap_body_enum(d, functor_param, def_name, is_try),
        Data::Union(_) => abort_call_site!("Deriving Functor on unions is unsupported."),
    }
}

fn generate_fmap_body_enum(
    data: &DataEnum,
    functor_param: &Ident,
    def_name: &Ident,
    is_try: bool,
) -> TokenStream {
    let variants = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let names = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                let fields = fields.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let field = generate_map_from_type(
                        &field.ty,
                        functor_param,
                        &quote!(#field_name),
                        is_try,
                    )
                    .0;
                    quote!(#field_name: #field)
                });

                quote!(Self::#variant_name { #(#names),* } => {
                    #def_name::#variant_name {
                        #(#fields),*
                    }
                })
            }
            Fields::Unnamed(fields) => {
                let names = (0..)
                    .map(|i| format_ident!("v{i}"))
                    .take(fields.unnamed.len());
                let fields = fields.unnamed.iter().zip(names.clone()).map(|(field, i)| {
                    generate_map_from_type(&field.ty, functor_param, &quote!(#i), is_try).0
                });
                quote!(Self::#variant_name(#(#names),*) => #def_name::#variant_name(#(#fields),*))
            }
            Fields::Unit => quote!(Self::#variant_name => #def_name::#variant_name),
        }
    });
    quote!(match self {#(#variants),*})
}

fn generate_fmap_body_struct(
    s: &DataStruct,
    functor_param: &Ident,
    def_name: &Ident,
    is_try: bool,
) -> TokenStream {
    match &s.fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field = generate_map_from_type(
                    &field.ty,
                    functor_param,
                    &quote!(self.#field_name),
                    is_try,
                )
                .0;
                quote!(#field_name: #field)
            });
            quote!(#def_name{#(#fields),*})
        }
        Fields::Unnamed(s) => {
            let fields = s.unnamed.iter().enumerate().map(|(i, field)| {
                generate_map_from_type(&field.ty, functor_param, &quote!(self.#i), is_try).0
            });
            quote!(#def_name(#(#fields),*))
        }
        Fields::Unit => abort_call_site!("Cannot derive `Functor` for Unit Structs."),
    }
}
