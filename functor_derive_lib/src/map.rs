use proc_macro2::Ident;
use quote::format_ident;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, ReturnType, Type,
    TypeParamBound, WhereClause, WherePredicate,
};

/// Maps the given parameter `param` in the where clause `where_clause` to `__B`.
pub fn map_where(where_clause: &WhereClause, param: &Ident) -> Option<WhereClause> {
    let mut clause = where_clause.clone();
    let mut contains_param = false;
    for pred in &mut clause.predicates {
        if let WherePredicate::Type(t) = pred {
            map_type(&mut t.bounded_ty, param, &mut contains_param);
            for bound in t.bounds.iter_mut() {
                if let TypeParamBound::Trait(trt) = bound {
                    map_path(&mut trt.path, param, &mut contains_param);
                }
            }
        }
    }
    if contains_param {
        Some(clause)
    } else {
        None
    }
}

/// Apologies for the long name, we blame the `Syn` crate =).
fn map_angle_bracketed_generic_arguments(
    args: &mut AngleBracketedGenericArguments,
    param: &Ident,
    contains_param: &mut bool,
) {
    for arg in &mut args.args {
        match arg {
            GenericArgument::Type(t) => map_type(t, param, contains_param),
            GenericArgument::AssocType(assoc) => {
                map_type(&mut assoc.ty, param, contains_param);
                if let Some(generics) = &mut assoc.generics {
                    map_angle_bracketed_generic_arguments(generics, param, contains_param);
                }
            }
            GenericArgument::Constraint(_) => {}
            _ => {}
        }
    }
}

/// Maps the given parameter `param` in the path `path` to `__B`.
fn map_path(path: &mut Path, param: &Ident, contains_param: &mut bool) {
    // Replace top-level ident
    if let Some(seg) = path.segments.first_mut() {
        if &seg.ident == param {
            seg.ident = format_ident!("__B");
            *contains_param = true;
        }
    }

    for seg in &mut path.segments {
        match &mut seg.arguments {
            PathArguments::AngleBracketed(args) => {
                map_angle_bracketed_generic_arguments(args, param, contains_param);
            }
            PathArguments::Parenthesized(args) => {
                for input in &mut args.inputs {
                    map_type(input, param, contains_param);
                }

                if let ReturnType::Type(_, t) = &mut args.output {
                    map_type(t, param, contains_param)
                }
            }
            _ => continue,
        }
    }
}

/// Maps the given parameter `param` in the type `typ` to `__B`.
fn map_type(typ: &mut Type, param: &Ident, contains_param: &mut bool) {
    match typ {
        Type::Array(array) => {
            map_type(&mut array.elem, param, contains_param);
        }
        Type::BareFn(fun) => {
            for input in &mut fun.inputs {
                map_type(&mut input.ty, param, contains_param);
            }

            match &mut fun.output {
                ReturnType::Default => {}
                ReturnType::Type(_, t) => map_type(t, param, contains_param),
            }
        }
        Type::Group(group) => map_type(&mut group.elem, param, contains_param),
        Type::ImplTrait(impl_trait) => {
            for bound in &mut impl_trait.bounds {
                if let TypeParamBound::Trait(trait_bound) = bound {
                    map_path(&mut trait_bound.path, param, contains_param);
                }
            }
        }
        Type::Paren(paren) => map_type(&mut paren.elem, param, contains_param),
        Type::Path(path) => map_path(&mut path.path, param, contains_param),
        Type::Ptr(ptr) => map_type(&mut ptr.elem, param, contains_param),
        Type::Reference(refer) => map_type(&mut refer.elem, param, contains_param),
        Type::Slice(slice) => map_type(&mut slice.elem, param, contains_param),
        Type::TraitObject(obj) => {
            for bound in &mut obj.bounds {
                if let TypeParamBound::Trait(trait_bound) = bound {
                    map_path(&mut trait_bound.path, param, contains_param);
                }
            }
        }
        Type::Tuple(tup) => {
            for elem in &mut tup.elems {
                map_type(elem, param, contains_param);
            }
        }
        _ => {}
    }
}
