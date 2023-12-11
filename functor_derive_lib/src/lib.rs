#![doc = include_str!("../README.md")]

use crate::generate_fmap_body::generate_fmap_body;
use crate::map::{map_path, map_where};
use crate::parse_attribute::parse_attribute;
use once_cell::unsync::Lazy;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprPath, GenericArgument, GenericParam, Path, PathSegment, Type, TypePath, WhereClause, WherePredicate, TypeParamBound, PredicateType};
use syn::token::Colon;

mod generate_fmap_body;
mod generate_map;
mod map;
mod parse_attribute;

// Lints to disable inside of the generated code. This list may not be exhaustive.
const LINTS: Lazy<TokenStream> = Lazy::new(|| {
    quote! {
        #[allow(absolute_paths_not_starting_with_crate)]
        #[allow(bare_trait_objects)]
        #[allow(deprecated)]
        #[allow(drop_bounds)]
        #[allow(dyn_drop)]
        #[allow(non_camel_case_types)]
        #[allow(trivial_bounds)]
        #[allow(unused_qualifications)]
        #[allow(clippy::allow)]
        #[automatically_derived]
    }
});

#[proc_macro_derive(Functor, attributes(functor))]
#[proc_macro_error]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Name of the Struct or Enum we are implementing the `Functor` trait for.
    let def_name = input.ident.clone();

    // Get the attributes for this invocation. If no attributes are given, the first generic is used as default.
    let attribute = parse_attribute(&input);

    // Get the generic parameters leaving only the bounds and attributes.
    let source_params = input
        .generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Type(param) => {
                let mut param = param.clone();
                param.eq_token = None;
                param.default = None;
                GenericParam::Type(param)
            }
            GenericParam::Const(param) => {
                let mut param = param.clone();
                param.eq_token = None;
                param.default = None;
                GenericParam::Const(param)
            }
            param => param.clone(),
        })
        .collect::<Vec<_>>();

    // Maps the generic parameters to generic arguments for the source.
    let source_args = source_params
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

    let mut tokens = TokenStream::new();

    // Include default Functor implementation.
    if let Some(default) = attribute.default {
        tokens.extend(generate_default_impl(
            &default,
            &def_name,
            &source_params,
            &source_args,
            &input.generics.where_clause,
        ));
    }

    // Include all named implementations.
    for (param, name) in attribute.name_map {
        tokens.extend(generate_named_impl(
            &param,
            &name,
            &def_name,
            &source_params,
            &source_args,
            &input.generics.where_clause,
        ));
    }

    // Include internal implementations.
    tokens.extend(generate_refs_impl(
        &input.data,
        &def_name,
        &source_params,
        &source_args,
        &input.generics.where_clause,
    ));

    tokens.into()
}

fn find_index(source_params: &[GenericParam], ident: &Ident) -> usize {
    for (total, param) in source_params.iter().enumerate() {
        match param {
            GenericParam::Type(t) if &t.ident == ident => return total,
            _ => {}
        }
    }
    unreachable!()
}

fn generate_refs_impl(
    data: &Data,
    def_name: &Ident,
    source_params: &Vec<GenericParam>,
    source_args: &Vec<GenericArgument>,
    where_clause: &Option<WhereClause>,
) -> TokenStream {
    let mut tokens = TokenStream::new();
    for param in source_params {
        if let GenericParam::Type(t) = param {
            let param_ident = t.ident.clone();
            let param_idx = find_index(source_params, &t.ident);

            let functor_trait_ident = format_ident!("Functor{param_idx}");
            let fmap_ident = format_ident!("__fmap_{param_idx}_ref");
            let try_fmap_ident = format_ident!("__try_fmap_{param_idx}_ref");

            // Generate body of the `fmap` implementation.
            let Some(fmap_ref_body) = generate_fmap_body(data, def_name, &param_ident, false)
            else {
                continue;
            };
            let Some(try_fmap_ref_body) = generate_fmap_body(data, def_name, &param_ident, true)
            else {
                continue;
            };

            let mut target_args = source_args.clone();
            target_args[param_idx] = GenericArgument::Type(Type::Path(TypePath {
                qself: None,
                path: Path::from(PathSegment::from(format_ident!("__B"))),
            }));

            let lints = &*LINTS;

            if let Some(fn_where_clause) = create_fn_where_clause(where_clause, source_params, &param_ident) {
                tokens.extend(quote!(
                    #lints
                    impl<#(#source_params),*> #def_name<#(#source_args),*> #where_clause {
                        pub fn #fmap_ident<__B>(self, __f: &impl Fn(#param_ident) -> __B) -> #def_name<#(#target_args),*> #fn_where_clause {
                            use ::functor_derive::*;
                            #fmap_ref_body
                        }

                        pub fn #try_fmap_ident<__B, __E>(self, __f: &impl Fn(#param_ident) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> #fn_where_clause {
                            use ::functor_derive::*;
                            Ok(#try_fmap_ref_body)
                        }
                    }
                ))
            } else {
                tokens.extend(quote!(
                    #lints
                    impl<#(#source_params),*> ::functor_derive::#functor_trait_ident<#param_ident> for #def_name<#(#source_args),*> #where_clause {
                        type Target<__B> = #def_name<#(#target_args),*>;

                        fn #fmap_ident<__B>(self, __f: &impl Fn(#param_ident) -> __B) -> #def_name<#(#target_args),*> {
                            use ::functor_derive::*;
                            #fmap_ref_body
                        }

                        fn #try_fmap_ident<__B, __E>(self, __f: &impl Fn(#param_ident) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> {
                            use ::functor_derive::*;
                            Ok(#try_fmap_ref_body)
                        }
                    }
                ))
            }
        }
    }
    tokens
}

fn generate_default_impl(
    param: &Ident,
    def_name: &Ident,
    source_params: &Vec<GenericParam>,
    source_args: &Vec<GenericArgument>,
    where_clause: &Option<WhereClause>,
) -> TokenStream {
    let default_idx= find_index(source_params, param);

    // Create generic arguments for the target. We use `__B` for the mapped generic.
    let mut target_args = source_args.clone();
    target_args[default_idx] = GenericArgument::Type(Type::Path(TypePath {
        qself: None,
        path: Path::from(PathSegment::from(format_ident!("__B"))),
    }));

    let default_map = format_ident!("__fmap_{default_idx}_ref");
    let default_try_map = format_ident!("__try_fmap_{default_idx}_ref");

    let lints = &*LINTS;

    if let Some(fn_where_clause) = create_fn_where_clause(where_clause, source_params, &param) {
        quote!(
            #lints
            impl<#(#source_params),*> #def_name<#(#source_args),*> #where_clause {
                pub fn fmap<__B>(self, __f: impl Fn(#param) -> __B) -> #def_name<#(#target_args),*> #fn_where_clause {
                    use ::functor_derive::*;
                    self.#default_map(&__f)
                }

                pub fn try_fmap<__B, __E>(self, __f: impl Fn(#param) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> #fn_where_clause {
                    use ::functor_derive::*;
                    self.#default_try_map(&__f)
                }
            }
        )
    } else {
        quote!(
            #lints
            impl<#(#source_params),*> ::functor_derive::Functor<#param> for #def_name<#(#source_args),*> {
                type Target<__B> = #def_name<#(#target_args),*>;

                fn fmap<__B>(self, __f: impl Fn(#param) -> __B) -> #def_name<#(#target_args),*> {
                    use ::functor_derive::*;
                    self.#default_map(&__f)
                }

                fn try_fmap<__B, __E>(self, __f: impl Fn(#param) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> {
                    use ::functor_derive::*;
                    self.#default_try_map(&__f)
                }
            }
        )
    }
}

fn generate_named_impl(
    param: &Ident,
    name: &Ident,
    def_name: &Ident,
    source_params: &Vec<GenericParam>,
    source_args: &Vec<GenericArgument>,
    where_clause: &Option<WhereClause>,
) -> TokenStream {
    let default_idx = find_index(source_params, param);

    // Create generic arguments for the target. We use `__B` for the mapped generic.
    let mut target_args = source_args.clone();
    target_args[default_idx] = GenericArgument::Type(Type::Path(TypePath {
        qself: None,
        path: Path::from(PathSegment::from(format_ident!("__B"))),
    }));

    let fmap_name = format_ident!("fmap_{name}");
    let try_fmap_name = format_ident!("try_fmap_{name}");

    let fmap = format_ident!("__fmap_{default_idx}_ref");
    let fmap_try = format_ident!("__try_fmap_{default_idx}_ref");

    let lints = &*LINTS;

    let fn_where_clause = create_fn_where_clause(where_clause, source_params, param);

    quote!(
        #lints
        impl<#(#source_params),*> #def_name<#(#source_args),*> #where_clause {
            pub fn #fmap_name<__B>(self, __f: impl Fn(#param) -> __B) -> #def_name<#(#target_args),*> #fn_where_clause {
                use ::functor_derive::*;
                self.#fmap(&__f)
            }

            pub fn #try_fmap_name<__B, __E>(self, __f: impl Fn(#param) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> #fn_where_clause {
                use ::functor_derive::*;
                self.#fmap_try(&__f)
            }
        }
    )
}

fn create_fn_where_clause(where_clause: &Option<WhereClause>, source_params: &Vec<GenericParam>, param: &Ident) -> Option<WhereClause> {
    let mut predicates = where_clause
        .iter()
        .flat_map(|where_clause| map_where(&where_clause, &param))
        .flat_map(|where_clause| where_clause.predicates)
        .collect::<Vec<_>>();

    for source_param in source_params {
        if let GenericParam::Type(typ) = source_param {
            let mut bounds = typ.bounds.clone();
            if bounds.is_empty() { continue };
            for bound in bounds.iter_mut() {
                if let TypeParamBound::Trait(trt) = bound {
                    map_path(&mut trt.path, &param, &mut false);
                }
            }

            predicates.push(WherePredicate::Type(PredicateType {
                lifetimes: None,
                bounded_ty: Type::Path(TypePath {
                    qself: None,
                    path: Path { leading_colon: None, segments: [
                        PathSegment {
                            ident: if &typ.ident == param {
                                format_ident!("__B")
                            } else {
                                typ.ident.clone()
                            },
                            arguments: Default::default(),
                        }
                    ].into_iter().collect() },
                }),
                colon_token: Colon::default(),
                bounds,
            }))
        }
    }

    if predicates.is_empty() {
        None
    } else {
        Some(WhereClause {
            where_token: Default::default(),
            predicates: predicates.into_iter().collect(),
        })
    }
}