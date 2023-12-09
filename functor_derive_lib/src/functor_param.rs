use proc_macro2::{Ident, Span};
use proc_macro_error::{abort, abort_call_site};
use quote::format_ident;
use syn::spanned::Spanned;
use syn::{parse, DeriveInput, GenericParam, Meta};

/// Optionally returns an identifier for the parameter to be mapped by the functor.
/// Aborts if multiple parameters are given in the attributes.
pub fn functor_param_from_attrs(input: &DeriveInput) -> Option<Ident> {
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
pub fn functor_param_first(input: &DeriveInput) -> Ident {
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
