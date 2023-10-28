extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, GenericParam, Index,
    PathArguments, Type, TypeParam,
};

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
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let field =
                        generate_map_from_type(&field.ty, &param, &quote!(self.#field_name));
                    quote!(#field_name: #field)
                });
                quote!(Self::Target{#(#fields),*})
            }
            Fields::Unnamed(s) => {
                let fields =
                    s.unnamed.iter().enumerate().map(|(i, field)| {
                        generate_map_from_type(&field.ty, &param, &quote!(self.#i))
                    });
                quote!(Self::Target(#(#fields),*))
            }
            Fields::Unit => unreachable!("Cannot derive `Functor` for Unit Structs."),
        },
        Data::Enum(data) => {
            let variants = data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Named(fields) => {
                        let names = fields.named.iter().map(|field| field.ident.as_ref().unwrap());
                        let fields = fields.named.iter().map(|field| {
                            let field_name = field.ident.as_ref().unwrap();
                            let field = generate_map_from_type(&field.ty, &param, &quote!(#field_name));
                            quote!(#field_name: #field)
                        });

                        quote!(Self::#variant_name { #(#names),* } => {
                            Self::Target::#variant_name {
                                #(#fields),*
                            }
                        })
                    }
                    Fields::Unnamed(fields) => {
                        let names = (0..).map(|i| format_ident!("v{i}")).take(fields.unnamed.len());
                        let fields = fields.unnamed.iter().zip(names.clone()).map(|(field, i)|  {
                            generate_map_from_type(&field.ty, &param, &quote!(#i))
                        });
                        quote!(Self::#variant_name(#(#names),*) => Self::Target::#variant_name(#(#fields),*))
                    }
                    Fields::Unit => quote!(Self::#variant_name => Self::Target::#variant_name)
                }
            });
            quote!(match self {#(#variants),*})
        }
        Data::Union(_) => panic!("Deriving Functor on unions is unsupported."),
    };

    let def_name = input.ident;

    quote!(
        impl<#param> functor_derive_lib::Functor<#param> for #def_name<#param> {
            type Target<__T> = #def_name<__T>;

            fn fmap<__B>(self, __f: impl Fn(#param) -> __B) -> Self::Target<__B> {
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
                    quote!(#field.fmap(&__f))
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
        Type::Array(array) => {
            if type_contains_param(typ, param) {
                let map = generate_map_from_type(&array.elem, param, &quote!(__v));
                quote!(#field.map(|__v| #map))
            } else {
                quote!(#field)
            }
        }
        Type::Paren(p) => generate_map_from_type(&p.elem, param, field),

        // We cannot possibly map these, but passing them through is fine.
        Type::BareFn(_)
        | Type::Reference(_)
        | Type::Ptr(_)
        | Type::Slice(_)
        | Type::Never(_)
        | Type::Macro(_)
        | Type::Infer(_)
        | Type::ImplTrait(_)
        | Type::TraitObject(_)
        | Type::Verbatim(_)
        | Type::Group(_) => {
            quote!(#field)
        }

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
        Type::Array(array) => type_contains_param(&array.elem, param),
        Type::Tuple(tuple) => tuple.elems.iter().any(|t| type_contains_param(t, param)),

        Type::Paren(p) => type_contains_param(&p.elem, param),

        Type::BareFn(_)
        | Type::Reference(_)
        | Type::Ptr(_)
        | Type::Slice(_)
        | Type::Never(_)
        | Type::Macro(_)
        | Type::Infer(_)
        | Type::ImplTrait(_)
        | Type::TraitObject(_)
        | Type::Verbatim(_)
        | Type::Group(_) => false,

        _ => panic!("Found unknown type"),
    }
}
