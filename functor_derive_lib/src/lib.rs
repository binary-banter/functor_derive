#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse, parse_macro_input, DeriveInput, Expr, ExprPath, GenericArgument, GenericParam, Index, Meta, Path, PathArguments, PathSegment, Type, TypePath, Fields, Data};

#[proc_macro_derive(Functor, attributes(functor))]
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Name of the Struct or Enum we are implementing the `Functor` trait for.
    let def_name = input.ident.clone();

    // Get the parameter to be mapped. First looks in attributes and falls back to the first generic type parameter.
    let functor_param =
        functor_param_from_attrs(&input).unwrap_or_else(|| functor_param_first(&input));

    // Find the position of the generic parameter. Aborts if absent.
    let functor_param_idx = input.generics.params.iter().position(
        |param| matches!(param, GenericParam::Type(param) if param.ident == functor_param),
    );

    let Some(functor_param_idx) = functor_param_idx else {
        abort_call_site!(
            "The generic parameter `{}` could not be found in the definition of `{}`.",
            functor_param,
            def_name
        )
    };

    // Get the generic parameter.
    let functor_param_type = match &input.generics.params[functor_param_idx] {
        GenericParam::Type(typ) => typ,
        _ => unreachable!(),
    };

    // Maps the generic parameters to generic arguments for the source.
    let source_args = input
        .generics
        .params
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

    // Create generic arguments for the target. We use `__B` for the mapped generic.
    let mut target_args = source_args.clone();
    target_args[functor_param_idx] = GenericArgument::Type(Type::Path(TypePath {
        qself: None,
        path: Path::from(PathSegment::from(format_ident!("__B"))),
    }));

    let tokens = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let field =
                        generate_map_from_type(&field.ty, &functor_param, &quote!(self.#field_name));
                    quote!(#field_name: #field)
                });
                quote!(#def_name{#(#fields),*})
            }
            Fields::Unnamed(s) => {
                let fields = s.unnamed.iter().enumerate().map(|(i, field)| {
                    generate_map_from_type(&field.ty, &functor_param, &quote!(self.#i))
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
                            let field = generate_map_from_type(&field.ty, &functor_param, &quote!(#field_name));
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
                            generate_map_from_type(&field.ty, &functor_param, &quote!(#i))
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

    let gen_params = input.generics.params.iter().cloned().collect::<Vec<_>>();
    if functor_param_type.bounds.is_empty() {
        quote!(
            impl<#(#gen_params),*> ::functor_derive::Functor<#functor_param> for #def_name<#(#source_args),*> {
                type Target<__B> = #def_name<#(#target_args),*>;

                fn fmap<__B>(self, __f: impl Fn(#functor_param) -> __B) -> #def_name<#(#target_args),*> {
                    #tokens
                }
            }
        )
    } else {
        let bounds = &functor_param_type.bounds;
        quote!(
            impl<#(#gen_params),*> #def_name<#(#source_args),*> {
                pub fn fmap<__B: #bounds>(self, __f: impl Fn(#functor_param) -> __B) -> #def_name<#(#target_args),*> {
                    #tokens
                }
            }
        )
    }.into()
}

/// Optionally returns an identifier for the parameter to be mapped by the functor.
/// Aborts if multiple parameters are given in the attributes.
fn functor_param_from_attrs(input: &DeriveInput) -> Option<Ident> {
    let mut functor_param: Option<(Ident, Span)> = None;

    // Find upto one `functor` attribute.
    for attribute in &input.attrs {
        match &attribute.meta {
            Meta::List(list) => {
                // If there is more than one segment, it cannot be `functor`.
                if list.path.segments.len() > 1 {
                    continue;
                }
                let Some(segment) = list.path.segments.first() else {
                    continue;
                };
                if segment.ident != format_ident!("functor") {
                    continue;
                }
                // We already found a `functor` attribute!
                if let Some((functor_param, span)) = functor_param {
                    abort!(
                        span,
                        "Already found a previous attribute with parameter `{}`",
                        functor_param
                    )
                }
                let span = list.tokens.span();
                let param = parse::<Ident>(list.tokens.clone().into()).unwrap();
                functor_param = Some((param, span));
            }
            _ => {}
        }
    }

    functor_param.map(|(param, _)| param)
}

/// Finds the first generic type parameter. Aborts if none are found.
fn functor_param_first(input: &DeriveInput) -> Ident {
    input
        .generics
        .params
        .iter()
        .find_map(|param| {
            if let GenericParam::Type(typ) = param {
                Some(typ.ident.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| abort_call_site!("Could not find a generic to map!"))
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
                    let PathArguments::AngleBracketed(args) = &path.path.segments[0].arguments
                    else {
                        unreachable!()
                    };
                    let first_type_arg = args
                        .args
                        .iter()
                        .filter_map(|arg| {
                            if let GenericArgument::Type(typ) = arg {
                                Some(typ)
                            } else {
                                None
                            }
                        })
                        .find(|typ| type_contains_param(typ, param))
                        .expect("Expected a type param");
                    let map = generate_map_from_type(first_type_arg, param, &quote!(v));

                    quote!(#field.fmap(|v| { #map }))
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
