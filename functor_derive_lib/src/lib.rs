#![doc = include_str!("../README.md")]
use crate::generate_fmap_body::generate_fmap_body;
use crate::parse_attribute::parse_attribute;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{abort_call_site, proc_macro_error};
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
    tokens.extend(generate_refs_impl(
        &input.data,
        &def_name,
        &source_params,
        &source_args,
    ));
    tokens.into()
}

/// Index in Vec, Index in Type Params
/// 'a, T, S -> (1, 0)
fn find_index(source_params: &Vec<GenericParam>, ident: &Ident) -> (usize, usize) {
    let mut total = 0;
    let mut types = 0;
    for param in source_params {
        match param {
            GenericParam::Type(t) if &t.ident == ident => return (total, types),
            GenericParam::Type(t) => types += 1,
            _ => {}
        };
        total += 1;
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
            let fmap_ident = format_ident!("fmap_{param_idx_types}_ref");
            let try_fmap_ident = format_ident!("try_fmap_{param_idx_types}_ref");

            // Generate body of the `fmap` implementation.
            let fmap_ref_body = generate_fmap_body(&data, &def_name, &param_ident, false);
            let try_fmap_ref_body = generate_fmap_body(&data, &def_name, &param_ident, true);

            let mut target_args = source_args.clone();
            target_args[param_idx_total] = GenericArgument::Type(Type::Path(TypePath {
                qself: None,
                path: Path::from(PathSegment::from(format_ident!("__B"))),
            }));

            tokens.extend(quote!(
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
        }
    }
    tokens
}

fn generate_default_impl(
    default: &Ident,
    def_name: &Ident,
    source_params: &Vec<GenericParam>,
    source_args: &Vec<GenericArgument>,
) -> TokenStream {
    let (default_idx_total, default_idx_types) = find_index(source_params, default);

    // Create generic arguments for the target. We use `__B` for the mapped generic.
    let mut target_args = source_args.clone();
    target_args[default_idx_total] = GenericArgument::Type(Type::Path(TypePath {
        qself: None,
        path: Path::from(PathSegment::from(format_ident!("__B"))),
    }));

    let default_map = format_ident!("fmap_{default_idx_types}_ref");
    let default_try_map = format_ident!("try_fmap_{default_idx_types}_ref");

    quote!(
        impl<#(#source_params),*> ::functor_derive::Functor<#default> for #def_name<#(#source_args),*> {
            type Target<__B> = #def_name<#(#target_args),*>;

            fn fmap<__B>(self, __f: impl Fn(#default) -> __B) -> #def_name<#(#target_args),*> {
                use ::functor_derive::*;
                self.#default_map(&__f)
            }

            fn try_fmap<__B, __E>(self, __f: impl Fn(#default) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> {
                use ::functor_derive::*;
                self.#default_try_map(&__f)
            }
        }
    )
}

// fn generate_impl(input: &DeriveInput, def_name: &Ident, functor_param: Ident, name_suffix: Option<Ident>) -> TokenStream {
// // Get the generic parameters *including* bounds and other attributes.
//     let gen_params = input.generics.params.iter().cloned().collect::<Vec<_>>();
//
//     // Find the position and type of the generic parameter. Aborts if absent.
//     let (functor_param_idx, functor_param_type) = gen_params
//         .iter()
//         .enumerate()
//         .find_map(|(idx, param)| match param {
//             GenericParam::Type(typ) if typ.ident == functor_param => Some((idx, typ)),
//             _ => None,
//         })
//         .unwrap_or_else(|| {
//             abort_call_site!(
//                 "The generic parameter `{}` could not be found in the definition of `{}`.",
//                 functor_param,
//                 def_name
//             )
//         });
//
//     // Maps the generic parameters to generic arguments for the source.
//     let source_args = gen_params
//         .iter()
//         .map(|param| match param {
//             GenericParam::Lifetime(l) => GenericArgument::Lifetime(l.lifetime.clone()),
//             GenericParam::Type(t) => GenericArgument::Type(Type::Path(TypePath {
//                 qself: None,
//                 path: Path::from(PathSegment::from(t.ident.clone())),
//             })),
//             GenericParam::Const(c) => GenericArgument::Const(Expr::Path(ExprPath {
//                 attrs: vec![],
//                 qself: None,
//                 path: Path::from(PathSegment::from(c.ident.clone())),
//             })),
//         })
//         .collect::<Vec<_>>();
//
//     // Create generic arguments for the target. We use `__B` for the mapped generic.
//     let mut target_args = source_args.clone();
//     target_args[functor_param_idx] = GenericArgument::Type(Type::Path(TypePath {
//         qself: None,
//         path: Path::from(PathSegment::from(format_ident!("__B"))),
//     }));
//
//     todo!()
//     // Generate body of the `fmap` implementation.
//     let fmap_body =
//         generate_fmap_body::generate_fmap_body(&input.data, &def_name, &functor_param, false);
//     let try_fmap_body =
//         generate_fmap_body::generate_fmap_body(&input.data, &def_name, &functor_param, true);
//     //
//     // // If there are no bounds on the generics, generate tokens for `Functor` trait impl for the given definition.
//     // // Otherwise, generate `fmap` impl for the given definition.
//     // if functor_param_type.bounds.is_empty() && name_suffix.is_none() {
//     //     quote!(
//     //         impl<#(#gen_params),*> ::functor_derive::Functor<#functor_param> for #def_name<#(#source_args),*> {
//     //             type Target<__B> = #def_name<#(#target_args),*>;
//     //
//     //             fn fmap_ref<__B>(self, __f: &impl Fn(#functor_param) -> __B) -> #def_name<#(#target_args),*> {
//     //                 #fmap_body
//     //             }
//     //
//     //             fn try_fmap_ref<__B, __E>(self, __f: &impl Fn(#functor_param) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> {
//     //                 Ok(#try_fmap_body)
//     //             }
//     //         }
//     //     )
//     // } else {
//     //     let bounds = &functor_param_type.bounds;
//     //
//     //     let suffix = name_suffix.map(|name_suffix| format!("_{name_suffix}")).unwrap_or_default();
//     //     let fmap = format_ident!("fmap{suffix}");
//     //     let fmap_ref = format_ident!("fmap_ref{suffix}");
//     //     let try_fmap = format_ident!("try_fmap{suffix}");
//     //     let try_fmap_ref = format_ident!("try_fmap_ref{suffix}");
//     //
//     //     quote!(
//     //         impl<#(#gen_params),*> #def_name<#(#source_args),*> {
//     //             pub fn #fmap<__B: #bounds>(self, __f: impl Fn(#functor_param) -> __B) -> #def_name<#(#target_args),*> {
//     //                 self.fmap_ref(&__f)
//     //             }
//     //
//     //             pub fn #fmap_ref<__B: #bounds>(self, __f: &impl Fn(#functor_param) -> __B) -> #def_name<#(#target_args),*> {
//     //                 #fmap_body
//     //             }
//     //
//     //             pub fn #try_fmap<__B: #bounds, __E>(self, __f: impl Fn(#functor_param) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> {
//     //                 self.try_fmap_ref(&__f)
//     //             }
//     //
//     //             pub fn #try_fmap_ref<__B: #bounds, __E>(self, __f: &impl Fn(#functor_param) -> Result<__B, __E>) -> Result<#def_name<#(#target_args),*>, __E> {
//     //                 Ok(#try_fmap_body)
//     //             }
//     //         }
//     //     )
//     // }.into()
// }
