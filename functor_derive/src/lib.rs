extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, GenericParam, PathArguments, Type, TypeParam, Index};

/// Example of user-defined [functor_derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Functor)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let param = input
        .generics
        .params
        .into_iter()
        .filter_map(|param| {
            if let GenericParam::Type(param) = param {
                Some(param)
            } else {
                None
            }
        })
        .next()
        .expect("Expected the type to have a generic type parameter.");

    let tokens = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(s) => {
                let mut fields = quote!();
                for field in s.named {
                    let name = field.ident.unwrap();
                    let map = generate_map_from_type(&field.ty, &param, &quote!(self.#name));
                    fields.extend(quote!(#name: #map,));
                }
                quote!(Self::Target{#fields})
            }
            Fields::Unnamed(s) => {
                let mut fields = quote!();
                for (i, field) in s.unnamed.iter().enumerate() {
                    let map = generate_map_from_type(&field.ty, &param, &quote!(self.#i));
                    fields.extend(quote!(#map,));
                }
                quote!(Self::Target(#fields))
            }
            Fields::Unit => unreachable!("Params should be used"),
        },
        Data::Enum(_) => todo!("Deriving Functor on enums is unsupported."),
        Data::Union(_) => panic!("Deriving Functor on unions is unsupported."),
    };

    let def_name = input.ident;

    quote!(
        impl<#param> functor_derive_lib::Functor<#param> for #def_name<#param> {
            type Target<__T> = #def_name<__T>;

            fn fmap<__B>(self, __f: &mut impl FnMut(#param) -> __B) -> Self::Target<__B> {
                #tokens
            }
        }
    )
    .into()
}

fn generate_map_from_type(
    typ: &Type,
    param: &TypeParam,
    field: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match typ {
        typ @ Type::Path(path) => {
            let segments: Vec<_> = path.path.segments.iter().collect();

            if type_contains_param(typ, param) {
                if segments.len() == 1 && segments[0].ident == param.ident {
                    quote!(__f(#field))
                } else {
                    quote!(#field.fmap(__f))
                }
            } else {
                quote!(#field)
            }
        }
        Type::Tuple(tuple) => {
            let positions = tuple.elems.iter().enumerate().map(|(i, x)| {
                let i = Index::from(i);
                let field = generate_map_from_type(x, param, &quote!(#field.#i));
                quote!(#field,)
            });
            quote!((#(#positions)*))
        }
        Type::Array(_) => todo!(),
        Type::BareFn(_) => todo!(),
        Type::Group(_) => todo!(),
        Type::ImplTrait(_) => todo!(),
        Type::Infer(_) => todo!(),
        Type::Macro(_) => todo!(),
        Type::Never(_) => todo!(),
        Type::Paren(_) => todo!(),
        Type::Ptr(_) => todo!(),
        Type::Reference(_) => todo!(),
        Type::Slice(_) => todo!(),
        Type::TraitObject(_) => todo!(),
        Type::Verbatim(_) => todo!(),
        _ => panic!("Found unknown type"),
    }
}

fn type_contains_param(typ: &Type, param: &TypeParam) -> bool {
    match typ {
        Type::Path(path) => {
            let segments: Vec<_> = path.path.segments.iter().collect();
            if segments.len() == 1 && segments[0].ident == param.ident {
                return true;
            }

            let PathArguments::AngleBracketed(bs) = &segments.last().unwrap().arguments else {
                return false;
            };

            bs.args.iter().any(|x| {
                if let GenericArgument::Type(typ) = x {
                    type_contains_param(typ, param)
                } else {
                    false
                }
            })
        }
        Type::Array(_) => todo!(),
        Type::BareFn(_) => todo!(),
        Type::Group(_) => todo!(),
        Type::ImplTrait(_) => todo!(),
        Type::Infer(_) => todo!(),
        Type::Macro(_) => todo!(),
        Type::Never(_) => todo!(),
        Type::Paren(_) => todo!(),
        Type::Ptr(_) => todo!(),
        Type::Reference(_) => todo!(),
        Type::Slice(_) => todo!(),
        Type::TraitObject(_) => todo!(),
        Type::Tuple(_) => todo!(),
        Type::Verbatim(_) => todo!(),
        _ => panic!("Found unknown type"),
    }
}
