#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, DeriveInput, Expr, ExprPath, GenericArgument, GenericParam, Path,
    PathSegment, Type, TypePath,
};

mod functor_param;
mod generate_fmap_body;
mod generate_map;

#[proc_macro_derive(Functor, attributes(functor))]
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Name of the Struct or Enum we are implementing the `Functor` trait for.
    let def_name = input.ident.clone();

    // Get the parameter to be mapped. First looks in attributes and falls back to the first generic type parameter.
    let functor_param = functor_param::functor_param_from_attrs(&input)
        .unwrap_or_else(|| functor_param::functor_param_first(&input));

    // Get the generic parameters *including* bounds and other attributes.
    let gen_params = input.generics.params.into_iter().collect::<Vec<_>>();

    // Find the position and type of the generic parameter. Aborts if absent.
    let (functor_param_idx, functor_param_type) = gen_params
        .iter()
        .enumerate()
        .find_map(|(idx, param)| match param {
            GenericParam::Type(typ) if typ.ident == functor_param => Some((idx, typ)),
            _ => None,
        })
        .unwrap_or_else(|| {
            abort_call_site!(
                "The generic parameter `{}` could not be found in the definition of `{}`.",
                functor_param,
                def_name
            )
        });

    // Maps the generic parameters to generic arguments for the source.
    let source_args = gen_params
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

    // Generate body of the `fmap` implementation.
    let fmap_body = generate_fmap_body::generate_fmap_body(input.data, &def_name, &functor_param);

    // If there are no bounds on the generics, generate tokens for `Functor` trait impl for the given definition.
    // Otherwise, generate `fmap` impl for the given definition.
    if functor_param_type.bounds.is_empty() {
        quote!(
            impl<#(#gen_params),*> ::functor_derive::Functor<#functor_param> for #def_name<#(#source_args),*> {
                type Target<__B> = #def_name<#(#target_args),*>;

                fn fmap_ref<__B>(self, __f: &impl Fn(#functor_param) -> __B) -> #def_name<#(#target_args),*> {
                    #fmap_body
                }
            }
        )
    } else {
        let bounds = &functor_param_type.bounds;
        quote!(
            impl<#(#gen_params),*> #def_name<#(#source_args),*> {
                fn fmap<__B: #bounds>(self, f: impl Fn(A) -> __B) -> #def_name<#(#target_args),*> {
                    self.fmap_ref(&f)
                }

                pub fn fmap_ref<__B: #bounds>(self, __f: &impl Fn(#functor_param) -> __B) -> #def_name<#(#target_args),*> {
                    #fmap_body
                }
            }
        )
    }.into()
}
