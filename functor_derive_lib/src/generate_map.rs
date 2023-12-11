use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote};
use syn::{GenericArgument, Index, PathArguments, ReturnType, Type, TypeParamBound, TypePath};

pub fn generate_map_from_type(
    typ: &Type,
    param: &Ident,
    field: &TokenStream,
    is_try: bool,
) -> Option<(TokenStream, bool)> {
    let stream = match typ {
        Type::Path(path) => return generate_map_from_path(path, param, field, is_try),
        Type::Tuple(tuple) => {
            let positions = tuple
                .elems
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let i = Index::from(i);
                    let field = generate_map_from_type(x, param, &quote!(#field.#i), is_try)?.0;
                    Some(quote!(#field,))
                })
                .collect::<Option<Vec<_>>>()?;
            quote!((#(#positions)*))
        }
        Type::Array(array) => {
            if type_contains_param(typ, param) {
                let map = generate_map_from_type(&array.elem, param, &quote!(__v), is_try)?.0;
                if is_try {
                    quote!(#field.try_fmap(|__v| Ok(#map))?)
                } else {
                    quote!(#field.map(|__v| #map))
                }
            } else {
                quote!(#field)
            }
        }
        Type::Paren(p) => generate_map_from_type(&p.elem, param, field, is_try)?.0,
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
            if type_contains_param(typ, param) {
                return None;
            } else {
                quote!(#field)
            }
        }
        _ => panic!("Found unknown type"),
    };

    return Some((stream, false));
}

fn generate_map_from_path(
    path: &TypePath,
    param: &Ident,
    field: &TokenStream,
    is_try: bool,
) -> Option<(TokenStream, bool)> {
    // Simply return the field if it does not contain the parameter `param`.
    if !type_contains_param(&Type::Path(path.clone()), param) {
        return Some((quote!(#field), false));
    }

    // If the path consists of exactly one segment, then it must be the param.
    match path.path.segments.iter().exactly_one() {
        Ok(segment) if &segment.ident == param => {
            if is_try {
                return Some((quote!(__f(#field)?), true));
            } else {
                return Some((quote!(__f(#field)), true));
            }
        }
        _ => {}
    }

    let Some(last_segment) = path.path.segments.last() else {
        unreachable!()
    };

    let PathArguments::AngleBracketed(args) = &last_segment.arguments else {
        unreachable!()
    };

    let mut tokens = quote!(#field);

    let enumerated_type_params = args
        .args
        .iter()
        .enumerate()
        .filter_map(|(idx, arg)| {
            if let GenericArgument::Type(typ) = arg {
                Some((idx, typ))
            } else {
                None
            }
        })
        .filter(|(_, typ)| type_contains_param(typ, param));

    // Loop over all arguments that contain `param`
    for (type_arg_idx, type_arg) in enumerated_type_params {
        let (map, is_end) = generate_map_from_type(type_arg, param, &quote!(v), is_try)?;

        if is_try {
            let map_ident = format_ident!("__try_fmap_{type_arg_idx}_ref");
            if is_end {
                tokens.extend(quote!(.#map_ident(__f)?))
            } else {
                tokens.extend(quote!(.#map_ident(&|v| { Ok(#map) })?))
            }
        } else {
            let map_ident = format_ident!("__fmap_{type_arg_idx}_ref");
            if is_end {
                tokens.extend(quote!(.#map_ident(__f)));
            } else {
                tokens.extend(quote!(.#map_ident(&|v| { #map })));
            }
        }
    }

    Some((tokens, false))
}

/// Returns whether or not the given type `typ` contains the parameter `param`.
fn type_contains_param(typ: &Type, param: &Ident) -> bool {
    match typ {
        Type::Path(path) => {
            // If the path consists of exactly one segment, then it must be the param.
            match path.path.segments.iter().exactly_one() {
                Ok(segment) if &segment.ident == param => return true,
                _ => {}
            }

            // If the path is empty, something weird happened.
            let Some(last_segment) = path.path.segments.last() else {
                return false;
            };

            let PathArguments::AngleBracketed(bracketed_params) = &last_segment.arguments else {
                return false;
            };

            bracketed_params.args.iter().any(|bracketed_param| {
                matches!(bracketed_param, GenericArgument::Type(typ) if type_contains_param(typ, param))
            })
        }
        Type::Array(array) => type_contains_param(&array.elem, param),
        Type::Tuple(tuple) => tuple.elems.iter().any(|t| type_contains_param(t, param)),
        Type::Paren(paren) => type_contains_param(&paren.elem, param),
        Type::BareFn(bare_fn) => {
            if bare_fn
                .inputs
                .iter()
                .any(|arg| type_contains_param(&arg.ty, param))
            {
                return true;
            }
            match &bare_fn.output {
                ReturnType::Default => false,
                ReturnType::Type(_, typ) => type_contains_param(&typ, param),
            }
        }
        Type::Reference(reference) => type_contains_param(&reference.elem, param),
        Type::Ptr(ptr) => type_contains_param(&ptr.elem, param),
        Type::Slice(slice) => type_contains_param(&slice.elem, param),
        Type::Never(_) => false,
        // Approximation, we'd rather generate wrong code than crash when not needed
        Type::Macro(_) | Type::Infer(_) => false,
        Type::ImplTrait(_) => unreachable!(),
        Type::TraitObject(obj) => obj.bounds.iter().any(|bound| match bound {
            TypeParamBound::Trait(t) => type_contains_param(
                &Type::Path(TypePath {
                    qself: None,
                    path: t.path.clone(),
                }),
                param,
            ),
            _ => false,
        }),
        Type::Verbatim(_) => false,
        Type::Group(g) => type_contains_param(&g.elem, param),
        _ => abort_call_site!("Found unknown type."),
    }
}
