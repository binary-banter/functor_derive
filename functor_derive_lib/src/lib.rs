#![doc = include_str!("../README.md")]
use crate::generate_fmap_body::generate_fmap_body;
use crate::parse_attribute::parse_attribute;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Expr, ExprPath, GenericArgument, GenericParam, Path,
    PathSegment, Type, TypePath,
};

mod generate_fmap_body;
mod generate_map;
mod parse_attribute;

#[proc_macro_derive(Functor, attributes(functor))]
#[proc_macro_error]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Name of the Struct or Enum we are implementing the `Functor` trait for.
    let def_name = input.ident.clone();

    // Get the attributes for this invocation. If no attributes are given, the first generic is used as default.
    let attribute = parse_attribute(&input);

    // Get the generic parameters *including* bounds and other attributes.
    let source_params = input.generics.params.iter().cloned().collect::<Vec<_>>();

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
    if let Some(default) = attribute.default {
        tokens.extend(generate_default_impl(
            &default,
            &def_name,
            &source_params,
            &source_args,
        ));
    }
    for (param, name) in attribute.name_map {
        tokens.extend(generate_named_impl(
            &param,
            &name,
            &def_name,
            &source_params,
            &source_args,
        ));
    }
    tokens.extend(generate_refs_impl(
        &input.data,
        &def_name,
        &source_params,
        &source_args,
    ));
    tokens.into()
}

fn find_index(source_params: &[GenericParam], ident: &Ident) -> (usize, usize) {
    let mut types = 0;
    for (total, param) in source_params.iter().enumerate() {
        match param {
            GenericParam::Type(t) if &t.ident == ident => return (total, types),
            GenericParam::Type(_) => types += 1,
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
) -> TokenStream {
    let mut tokens = TokenStream::new();
    for param in source_params {
        if let GenericParam::Type(t) = param {
            let param_ident = t.ident.clone();
            let (param_idx_total, param_idx_types) = find_index(source_params, &t.ident);

            let functor_trait_ident = format_ident!("Functor{param_idx_types}");
            let fmap_ident = format_ident!("__fmap_{param_idx_types}_ref");
            let try_fmap_ident = format_ident!("__try_fmap_{param_idx_types}_ref");

            // Generate body of the `fmap` implementation.
            let fmap_ref_body = generate_fmap_body(data, def_name, &param_ident, false);
            let try_fmap_ref_body = generate_fmap_body(data, def_name, &param_ident, true);

            let mut target_args = source_args.clone();
            target_args[param_idx_total] = GenericArgument::Type(Type::Path(TypePath {
                qself: None,
                path: Path::from(PathSegment::from(format_ident!("__B"))),
            }));

            let GenericParam::Type(t) = &source_params[param_idx_total] else {
                unreachable!()
            };
            let bounds_colon = &t.colon_token;
            let bounds = &t.bounds;

            if bounds.is_empty() {
                tokens.extend(quote!(
                    #[allow(clippy::all)]
                    impl<#(#source_params),*> ::functor_derive::#functor_trait_ident<#param_ident> for #def_name<#(#source_args),*> {
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
            } else {
                tokens.extend(quote!(
                    #[allow(clippy::all)]
                    impl<#(#source_params),*> #def_name<#(#source_args),*> {
                        fn #fmap_ident<__B #bounds_colon #bounds>(self, __f: &impl Fn(#param_ident) -> __B) -> #def_name<#(#target_args),*> {
                            use ::functor_derive::*;
                            #fmap_ref_body
                        }

                        fn #try_fmap_ident<__B #bounds_colon #bounds, __E>(self, __f: &impl Fn(#param_ident) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> {
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
) -> TokenStream {
    let (default_idx_total, default_idx_types) = find_index(source_params, param);

    // Create generic arguments for the target. We use `__B` for the mapped generic.
    let mut target_args = source_args.clone();
    target_args[default_idx_total] = GenericArgument::Type(Type::Path(TypePath {
        qself: None,
        path: Path::from(PathSegment::from(format_ident!("__B"))),
    }));

    let default_map = format_ident!("__fmap_{default_idx_types}_ref");
    let default_try_map = format_ident!("__try_fmap_{default_idx_types}_ref");

    let GenericParam::Type(t) = &source_params[default_idx_total] else {
        unreachable!()
    };
    let bounds_colon = &t.colon_token;
    let bounds = &t.bounds;

    if bounds.is_empty() {
        quote!(
            #[allow(clippy::all)]
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
    } else {
        quote!(
            #[allow(clippy::all)]
            impl<#(#source_params),*> #def_name<#(#source_args),*> {
                fn fmap<__B #bounds_colon #bounds>(self, __f: impl Fn(#param) -> __B) -> #def_name<#(#target_args),*> {
                    use ::functor_derive::*;
                    self.#default_map(&__f)
                }

                fn try_fmap<__B #bounds_colon #bounds, __E>(self, __f: impl Fn(#param) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> {
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
) -> TokenStream {
    let (default_idx_total, default_idx_types) = find_index(source_params, param);

    // Create generic arguments for the target. We use `__B` for the mapped generic.
    let mut target_args = source_args.clone();
    target_args[default_idx_total] = GenericArgument::Type(Type::Path(TypePath {
        qself: None,
        path: Path::from(PathSegment::from(format_ident!("__B"))),
    }));

    let fmap_name = format_ident!("fmap_{name}");
    let try_fmap_name = format_ident!("try_fmap_{name}");

    let fmap = format_ident!("__fmap_{default_idx_types}_ref");
    let fmap_try = format_ident!("__try_fmap_{default_idx_types}_ref");

    let GenericParam::Type(t) = &source_params[default_idx_total] else {
        unreachable!()
    };
    let bounds_colon = &t.colon_token;
    let bounds = &t.bounds;

    quote!(
        #[allow(clippy::all)]
        impl<#(#source_params),*> #def_name<#(#source_args),*> {
            fn #fmap_name<__B #bounds_colon #bounds>(self, __f: impl Fn(#param) -> __B) -> #def_name<#(#target_args),*> {
                use ::functor_derive::*;
                self.#fmap(&__f)
            }

            fn #try_fmap_name<__B #bounds_colon #bounds, __E>(self, __f: impl Fn(#param) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> {
                use ::functor_derive::*;
                self.#fmap_try(&__f)
            }
        }
    )
}
