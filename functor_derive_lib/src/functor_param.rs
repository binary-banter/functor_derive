use proc_macro2::{Ident, Span};
use proc_macro_error::{abort, abort_call_site};
use quote::format_ident;
use syn::spanned::Spanned;
use syn::{parse, DeriveInput, GenericParam, Meta, Token};
use syn::parse::{Parse, ParseStream};

pub enum Attribute {
    Single(Ident),
    Many(Vec<(Ident, Ident)>),
}

impl Parse for Attribute{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let x = input.parse::<Ident>()?;

        if input.is_empty() {
            return Ok(Attribute::Single(x))
        }

        input.parse::<Token![as]>()?;
        let y = input.parse::<Ident>()?;

        let mut list = vec![(x, y)];

        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            let x = input.parse::<Ident>()?;
            input.parse::<Token![as]>()?;
            let y = input.parse::<Ident>()?;
            list.push((x, y));
        }

        Ok(Attribute::Many(list))
    }
}

/// Optionally returns an identifier for the parameter to be mapped by the functor.
/// Aborts if multiple parameters are given in the attributes.
pub fn functor_param_from_attrs(input: &DeriveInput) -> Option<Attribute> {
    let mut functor_param: Option<(Attribute, Span)> = None;

    // Find upto one `functor` attribute.
    for attribute in &input.attrs {
        if let Meta::List(list) = &attribute.meta {
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
            if let Some((_, span)) = functor_param {
                abort!(
                    span,
                    "Found two functor attributes",
                )
            }
            let span = list.tokens.span();
            let param = parse::<Attribute>(list.tokens.clone().into()).unwrap();
            functor_param = Some((param, span));
        }
    }

    functor_param.map(|(param, _)| param)
}

/// Finds the first generic type parameter. Aborts if none are found.
pub fn functor_param_first(input: &DeriveInput) -> Attribute {
    input
        .generics
        .params
        .iter()
        .find_map(|param| {
            if let GenericParam::Type(typ) = param {
                Some(Attribute::Single(typ.ident.clone()))
            } else {
                None
            }
        })
        .unwrap_or_else(|| abort_call_site!("Could not find a generic to map!"))
}
