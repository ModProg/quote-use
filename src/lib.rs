use proc_macro2::{Ident, Spacing, TokenStream};
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, ItemUse, Path, Token, UseGroup, UseName, UsePath, UseTree,
};
mod prelude;

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
        let mut prelude: Vec<_> = prelude::prelude().collect();
        prelude.extend_from_slice(uses);
        let uses = &prelude;

        let mut in_path = false;
        tokens.extend(tail.clone().into_iter().flat_map(|token| {
            match &token {
                proc_macro2::TokenTree::Ident(ident) if !in_path => {
                    if let Some(Use(path, _)) = uses.iter().find(|item| &item.1 == ident) {
                        return quote!(#path);
                    }
                }
                proc_macro2::TokenTree::Punct(punct)
                    if punct.spacing() == Spacing::Joint && punct.as_char() == ':' =>
                {
                    in_path = true
                }
                proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ':' => (),
                _ => in_path = false,
            };
            quote!(#token)
        }));
    }
}

#[derive(Clone)]
struct Use(Path, Ident);

struct UseNode {
    path: TokenStream,
    trailing_colon2: Token!(::),
    last_ident: Option<Ident>,
    tree: UseTree,
}

impl Use {
    fn from_item_use(input: ItemUse) -> Vec<Self> {
        let mut output = Vec::new();

        let mut nodes = vec![UseNode {
            path: quote!(),
            trailing_colon2: parse_quote!(::),
            last_ident: None,
            tree: input.tree,
        }];

        while let Some(UseNode {
            mut path,
            trailing_colon2,
            last_ident,
            tree,
        }) = nodes.pop()
        {
            match tree {
                UseTree::Path(UsePath {
                    ident,
                    colon2_token,
                    tree,
                }) => {
                    trailing_colon2.to_tokens(&mut path);
                    ident.to_tokens(&mut path);
                    nodes.push(UseNode {
                        path,
                        trailing_colon2: colon2_token,
                        last_ident: Some(ident),
                        tree: *tree,
                    })
                }
                UseTree::Name(UseName { ident }) => {
                    if ident == "self" {
                        if let Some(ident) = last_ident {
                            output.push(Use(parse_quote!(#path), ident))
                        } else {
                            abort!(ident, "self at root level is not supported")
                        }
                    } else {
                        output.push(Use(parse_quote!(#path #trailing_colon2 #ident), ident))
                    }
                }
                UseTree::Group(UseGroup { items, .. }) => {
                    for item in items {
                        nodes.push(UseNode {
                            path: path.clone(),
                            trailing_colon2,
                            last_ident: last_ident.clone(),
                            tree: item,
                        })
                    }
                }
                UseTree::Rename(_) => abort!(tree, "Renaming is not supported"),
                UseTree::Glob(_) => abort!(tree, "Globs are not supported"),
            }
        }
        output
    }

    fn parse(input: ParseStream) -> syn::Result<Vec<Self>> {
        Ok(Self::from_item_use(input.parse()?))
    }
}
