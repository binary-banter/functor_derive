use proc_macro2::{Ident, Span};
use proc_macro_error::{abort, abort_call_site};
use quote::format_ident;
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse, DeriveInput, GenericParam, Meta, Token};

pub fn parse_attribute(input: &DeriveInput) -> Attribute {
    functor_param_from_attrs(&input).unwrap_or_else(|| functor_param_first(&input))
}

fn functor_param_first(input: &DeriveInput) -> Attribute {
    input
        .generics
        .params
        .iter()
        .find_map(|param| {
            if let GenericParam::Type(typ) = param {
                Some(Attribute {
                    default: Some(typ.ident.clone()),
                    name_map: vec![],
                })
            } else {
                None
            }
        })
        .unwrap_or_else(|| abort_call_site!("Could not find a generic to map!"))
}

fn functor_param_from_attrs(input: &DeriveInput) -> Option<Attribute> {
    let mut functor_attribute = None::<(Attribute, Span)>;

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
            if let Some((_, span)) = functor_attribute {
                abort!(span, "Found two functor attributes",)
            }
            let span = list.tokens.span();
            let param = parse(list.tokens.clone().into()).unwrap();
            functor_attribute = Some((param, span));
        }
    }

    functor_attribute.map(|(param, _)| param)
}

#[derive(Debug)]
pub struct Attribute {
    pub default: Option<Ident>,
    pub name_map: Vec<(Ident, Ident)>,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut default = None;
        let mut name_map = Vec::new();
        let mut seen_names = HashSet::new();

        for sub_attr in Punctuated::<SubAttribute, Token![,]>::parse_separated_nonempty(&input)? {
            match sub_attr {
                SubAttribute::Default(param) => {
                    if default.replace(param).is_some() {
                        abort_call_site!("Two defaults were provided.")
                    }
                }
                SubAttribute::NameMap(param, name) => {
                    if !seen_names.insert(name.to_string()) {
                        abort_call_site!("Two identical fmap names were provided.")
                    }
                    name_map.push((param, name));
                }
            }
        }

        Ok(Attribute { default, name_map })
    }
}

enum SubAttribute {
    Default(Ident),
    NameMap(Ident, Ident),
}

impl Parse for SubAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let param = input.parse::<Ident>()?;

        let sub_attr = if input.peek(Token![as]) {
            input.parse::<Token![as]>()?;
            let name = input.parse::<Ident>()?;
            SubAttribute::NameMap(param, name)
        } else {
            SubAttribute::Default(param)
        };

        Ok(sub_attr)
    }
}
