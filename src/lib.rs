use derive_where::derive_where;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, parse_str, File, Item, ItemUse, Path, Token, UseGroup, UseName,
    UsePath, UseTree,
};

#[proc_macro_error]
#[proc_macro]
pub fn quote_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let uses = parse_macro_input!(input as Uses);

    quote! {
        ::quote::quote!{
            #uses
        }
    }
    .into()
}

struct Uses(Vec<Use>, TokenStream);
impl Parse for Uses {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut uses = Vec::new();
        while input.peek(Token![use]) {
            uses.extend(Use::parse(input)?.into_iter());
        }

        Ok(Uses(uses, input.parse()?))
    }
}

impl ToTokens for Uses {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(uses, tail) = self;
        let mut prelude = prelude();
        prelude.extend_from_slice(uses);
        let uses = &prelude;

        tokens.extend(tail.clone().into_iter().flat_map(|token| {
            if let proc_macro2::TokenTree::Ident(ident) = &token {
                if let Some(Use(path, _)) = uses.iter().find(|item| &item.1 == ident) {
                    return quote!(#path);
                }
            };
            quote!(#token)
        }));
    }
}

#[derive(Clone)]
#[derive_where(Debug)]
struct Use(#[derive_where(skip)] Path, Ident);

impl Use {
    fn from_item_use(input: ItemUse) -> syn::Result<Vec<Self>> {
        let mut output = Vec::new();

        let mut nodes: Vec<(TokenStream, UseTree)> = vec![(quote!(::), input.tree)];

        while let Some((mut path, node)) = nodes.pop() {
            match node {
                UseTree::Path(UsePath {
                    ident,
                    colon2_token,
                    tree,
                }) => {
                    ident.to_tokens(&mut path);
                    colon2_token.to_tokens(&mut path);
                    nodes.push((path, *tree))
                }
                UseTree::Name(UseName { ident }) => {
                    output.push(Use(parse_quote!(#path #ident), ident))
                }
                UseTree::Group(UseGroup { items, .. }) => {
                    for item in items {
                        nodes.push((path.clone(), item))
                    }
                }
                UseTree::Rename(_) => abort!(node, "Renaming is not supported"),
                UseTree::Glob(_) => abort!(node, "Globs are not supported"),
            }
        }
        Ok(output)
    }

    fn parse(input: ParseStream) -> syn::Result<Vec<Self>> {
        Self::from_item_use(input.parse()?)
    }
}

fn prelude() -> Vec<Use> {
    let statements: File =
        parse_str(&include_str!("prelude.rs").replace("crate::", "core::")).unwrap();

    statements
        .items
        .into_iter()
        .flat_map(|expr| match expr {
            Item::Use(item_use) => Use::from_item_use(item_use).unwrap(),
            _ => Vec::new(),
        })
        .collect()
}
