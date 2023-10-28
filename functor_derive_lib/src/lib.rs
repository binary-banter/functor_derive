extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Expr, ExprPath, Fields, GenericArgument, GenericParam,
    Index, Path, PathArguments, PathSegment, Type, TypePath,
};

#[proc_macro_derive(Functor)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let def_name = input.ident;
    let params = input.generics.params.iter().cloned().collect::<Vec<_>>();
    let first = params
        .iter()
        .enumerate()
        .find(|(_, param)| matches!(param, GenericParam::Type(_)))
        .map(|(i, _)| i)
        .expect("Expected type to have a generic parameter");
    let GenericParam::Type(type_param) = &params[first] else {
        unreachable!()
    };
    let param_name = &type_param.ident;

    let source_args = params
        .iter()
        .map(|param| match param {
            GenericParam::Lifetime(l) => GenericArgument::Lifetime(l.lifetime.clone()),
            GenericParam::Type(t) => GenericArgument::Type(Type::Path(TypePath {
                qself: None,
                path: Path::from(PathSegment::from(t.ident.clone())),
            })),
            GenericParam::Const(c) => GenericArgument::Const(Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: Path::from(PathSegment::from(c.ident.clone())),
            })),
        })
        .collect::<Vec<_>>();

    let mut target_args = source_args.clone();
    target_args[first] = GenericArgument::Type(Type::Path(TypePath {
        qself: None,
        path: Path::from(PathSegment::from(format_ident!("__B"))),
    }));

    let tokens = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let field =
                        generate_map_from_type(&field.ty, &param_name, &quote!(self.#field_name));
                    quote!(#field_name: #field)
                });
                quote!(#def_name{#(#fields),*})
            }
            Fields::Unnamed(s) => {
                let fields = s.unnamed.iter().enumerate().map(|(i, field)| {
                    generate_map_from_type(&field.ty, &param_name, &quote!(self.#i))
                });
                quote!(#def_name(#(#fields),*))
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
                            let field = generate_map_from_type(&field.ty, &param_name, &quote!(#field_name));
                            quote!(#field_name: #field)
                        });

                        quote!(Self::#variant_name { #(#names),* } => {
                            #def_name::#variant_name {
                                #(#fields),*
                            }
                        })
                    }
                    Fields::Unnamed(fields) => {
                        let names = (0..).map(|i| format_ident!("v{i}")).take(fields.unnamed.len());
                        let fields = fields.unnamed.iter().zip(names.clone()).map(|(field, i)|  {
                            generate_map_from_type(&field.ty, &param_name, &quote!(#i))
                        });
                        quote!(Self::#variant_name(#(#names),*) => #def_name::#variant_name(#(#fields),*))
                    }
                    Fields::Unit => quote!(Self::#variant_name => #def_name::#variant_name)
                }
            });
            quote!(match self {#(#variants),*})
        }
        Data::Union(_) => panic!("Deriving Functor on unions is unsupported."),
    };

    if type_param.bounds.is_empty() {
        quote!(
            impl<#(#params),*> ::functor_derive::Functor<#param_name> for #def_name<#(#source_args),*> {
                type Target<__B> = #def_name<#(#target_args),*>;

                fn fmap<__B>(self, __f: impl Fn(#param_name) -> __B) -> #def_name<#(#target_args),*> {
                    #tokens
                }
            }
        )
    } else {
        let bounds = &type_param.bounds;
        quote!(
            impl<#(#params),*> #def_name<#(#source_args),*> {
                pub fn fmap<__B: #bounds>(self, __f: impl Fn(#param_name) -> __B) -> #def_name<#(#target_args),*> {
                    #tokens
                }
            }
        )
    }.into()
}

fn generate_map_from_type(
    typ: &Type,
    param: &Ident,
    field: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match typ {
        typ @ Type::Path(path) => {
            if type_contains_param(typ, param) {
                if path.path.segments.len() == 1 && &path.path.segments[0].ident == param {
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

fn type_contains_param(typ: &Type, param: &Ident) -> bool {
    match typ {
        Type::Path(path) => {
            if path.path.segments.len() == 1 && &path.path.segments[0].ident == param {
                return true;
            }

            let PathArguments::AngleBracketed(bs) =
                &path.path.segments[path.path.segments.len() - 1].arguments
            else {
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
