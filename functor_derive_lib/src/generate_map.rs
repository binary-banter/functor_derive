use proc_macro2::Ident;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{GenericArgument, Index, PathArguments, Type};

pub fn generate_map_from_type(
    typ: &Type,
    param: &Ident,
    field: &proc_macro2::TokenStream,
    ref_name: &Ident,
    is_try: bool,
) -> (proc_macro2::TokenStream, bool) {
    (
        match typ {
            typ @ Type::Path(path) => {
                if type_contains_param(typ, param) {
                    if path.path.segments.len() == 1 && &path.path.segments[0].ident == param {
                        if is_try {
                            return (quote!(__f(#field)?), true);
                        } else {
                            return (quote!(__f(#field)), true);
                        }
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
                        let (map, is_end) = generate_map_from_type(
                            first_type_arg,
                            param,
                            &quote!(v),
                            ref_name,
                            is_try,
                        );

                        //TODO for tomorrow, should use #ref_name but only if self-recursive
                        match (is_try, is_end) {
                            (true, true) => quote!(#field.try_fmap_ref(__f)?),
                            (true, false) => quote!(#field.try_fmap_ref(&|v| { Ok(#map) })?),
                            (false, true) => quote!(#field.fmap_ref(__f)),
                            (false, false) => quote!(#field.fmap_ref(&|v| { #map })),
                        }
                    }
                } else {
                    quote!(#field)
                }
            }
            Type::Tuple(tuple) => {
                let positions = tuple.elems.iter().enumerate().map(|(i, x)| {
                    let i = Index::from(i);
                    let field =
                        generate_map_from_type(x, param, &quote!(#field.#i), ref_name, is_try).0;
                    quote!(#field,)
                });
                quote!((#(#positions)*)) // todo: do we need an ok in front of this tuple?
            }
            Type::Array(array) => {
                if type_contains_param(typ, param) {
                    let map =
                        generate_map_from_type(&array.elem, param, &quote!(__v), ref_name, is_try)
                            .0;
                    if is_try {
                        quote!(#field.try_fmap(|__v| Ok(#map))?)
                    } else {
                        quote!(#field.map(|__v| #map))
                    }
                } else {
                    quote!(#field)
                }
            }
            Type::Paren(p) => generate_map_from_type(&p.elem, param, field, ref_name, is_try).0,
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
        },
        false,
    )
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
        _ => abort_call_site!("Found unknown type."),
    }
}
