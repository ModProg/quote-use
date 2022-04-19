//! # Description
//!
//! Macro to simplify using Types in the [`quote!`] macro.
//!
//! # Usage
//!
//! The [`quote_use!`] macro can be used just like [`quote!`], but with the added functionality of
//! adding use statements at the top:
//!
//! ```
//! # use quote_use::quote_use;
//! quote_use! {
//!     ## use std::fs::read;
//!
//!     read("src/main.rs")
//! }
//! # ;
//! ```
//!
//! This will expand to the equivalent statement using [`quote!`]:
//!
//! ```
//! # use quote::quote;
//! quote! {
//!     ::std::fs::read::read("src/main.rs")
//! }
//! # ;
//! ```
//!
//! ## Prelude
//!
//! This also allows to use contents of the rust prelude directly:
//!
//! ```
//! # use quote_use::quote_use;
//! quote_use! {
//!     Some("src/main.rs")
//! }
//! # ;
//! ```
//! ### Overriding prelude
//! When you want to use your own type instead of the prelude type this can be achieved by simply
//! importing it like so
//!
//! ```
//! # use quote_use::quote_use;
//! quote_use! {
//!     ## use anyhow::Result;
//!
//!     Result
//! }
//! # ;
//! ```
//! ### Different preludes
//!
//! By default [`quote_use!`] uses the [std prelude](std::prelude) for [2021 edition](std::prelude::rust_2021),
//! but this can be configured via features, and also completely disabled.
//!
//! - **`prelude_std`**: Enables [`std::prelude::v1`]  (incompatible with `prelude_core`)
//! - `prelude_core`: Enables [`core::prelude::v1`] (incompatible with `prelude_std`)
//! - **`prelude_2021`**: Enables [`core::prelude::rust_2021`] (requires either `prelude_std` or `prelude_core`)
//!
//! ## Other quote macros
//!
//! There are also variants for other quote macros from [syn] and [mod@quote]:
//!
//! - [`quote_use!`] and [`quote_spanned_use!`] as replacement for [`quote!`] and
//! [`quote_spanned!`](quote::quote_spanned!) respectively
//! - [`parse_quote_use!`] and [`parse_quote_spanned_use!`] for [`parse_quote!`](syn::parse_quote!)
//! and [`parse_quote_spanned!`](syn::parse_quote_spanned!)
//!
//! ## Auto namespacing idents
//!
//! Until [`Span::def_site`](proc_macro::Span::def_site) is stabilized, identifiers in e.g. let
//! bindings in proc-macro expansions can collide with e.g. constants.
//!
//! To circumvent this you can enable the feature `namespace_idents` which will replace all
//! identifiers with autonamespaced ones using the pattern `"__{crate_name}_{ident}"`.

use proc_macro2::{Ident, Spacing, TokenStream};
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::{format_ident, quote, ToTokens};
#[cfg(feature = "namespace_idents")]
use syn::LitStr;
use syn::{
    ext::IdentExt,
    group::{parse_braces, Braces},
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::Brace,
    Expr, ItemUse, Result, Token, UseGroup, UseName, UsePath, UseTree,
};

mod prelude;

/// [`quote!`] replacement that allows [using](https://doc.rust-lang.org/std/keyword.use.html) paths to be
/// automaticly replaced.
///
/// It supports both the explicit use via `use some::path::Type;` and the use of the rust prelude:
/// ```
/// # use quote_use::quote_use;
/// quote_use! {
///     ## use std::fs::read;
///
///     read("src/main.rs")
///
///     Some(20)
/// }
/// # ;
/// ```
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

/// Like [`quote_use!`] but using [`quote_spanned!`](quote::quote_spanned)
/// ```
/// # use quote_use::quote_spanned_use;
/// # use proc_macro2::Span;
/// #
/// # let span = Span::call_site();
/// quote_spanned_use! {span=>
///     ## use std::fs::read;
///
///     read("src/main.rs")
///
///     Some(20)
/// }
/// # ;
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn quote_spanned_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let UsesSpanned(spanned, uses) = parse_macro_input!(input as UsesSpanned);

    quote! {
        ::quote::quote_spanned!{
            #spanned
            #uses
        }
    }
    .into()
}

/// Like [`quote_use!`] but using [`parse_quote!`](syn::parse_quote!)
/// ```
/// # use quote_use::parse_quote_use;
/// # use syn::{Expr, parse_quote};
/// # use quote::ToTokens;
/// #
/// let expr: Expr = parse_quote_use!{
///     ## use std::fs::read;
///     
///     read("src/main.rs")
/// }
/// # ;
/// # let expected: Expr = parse_quote!(::std::fs::read("src/main.rs"));
/// # assert_eq!{
/// #   expr.to_token_stream().to_string(),
/// #   expected.to_token_stream().to_string()
/// # };
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn parse_quote_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let uses = parse_macro_input!(input as Uses);

    quote! {
        ::syn::parse_quote!{
            #uses
        }
    }
    .into()
}

/// Like [`quote_spanned_use!`] but using [`parse_quote_spanned!`](syn::parse_quote_spanned)
/// ```
/// # use quote_use::parse_quote_spanned_use;
/// # use syn::{parse_quote_spanned, Expr, spanned::Spanned};
/// # use proc_macro2::Span;
/// # use quote::ToTokens;
/// #
/// # let span = Span::call_site();
/// let expr: Expr = parse_quote_spanned_use!{span=>
///     ## use std::fs::read;
///     
///     read("src/main.rs")
/// }
/// # ;
/// # let expected: Expr = parse_quote_spanned!(span=> ::std::fs::read("src/main.rs"));
/// # assert_eq!{
/// #   expr.to_token_stream().to_string(),
/// #   expected.to_token_stream().to_string()
/// # };
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn parse_quote_spanned_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let UsesSpanned(spanned, uses) = parse_macro_input!(input as UsesSpanned);

    quote! {
        ::syn::parse_quote_spanned!{
            #spanned
            #uses
        }
    }
    .into()
}

/// [`format_ident!`](quote::format_ident) replacement that allows the auto namespacing matching the
/// `quote!` macros of this crate.
/// ```
/// # use quote_use::format_ident_namespaced;
/// format_ident_namespaced!("$ident_{}", 2usize)
/// # ;
/// ```
#[proc_macro_error]
#[proc_macro]
#[cfg(feature = "namespace_idents")]
pub fn format_ident_namespaced(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let format_input = parse_macro_input!(input as FormatInput);

    quote! {
        ::quote::format_ident!(#format_input)
    }
    .into()
}

#[cfg(feature = "namespace_idents")]
struct FormatInput(LitStr, TokenStream);
#[cfg(feature = "namespace_idents")]
impl Parse for FormatInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse()?;

        Ok(Self(lit, input.parse()?))
    }
}
#[cfg(feature = "namespace_idents")]
impl ToTokens for FormatInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(lit, tail) = self;

        let lit_span = lit.span();

        if let Some(lit) = lit.value().strip_prefix('$') {
            LitStr::new(&format!("{}{}", ident_prefix(), lit), lit_span).to_tokens(tokens);
        } else {
            lit.to_tokens(tokens)
        };

        tail.to_tokens(tokens);
    }
}

#[cfg(feature = "namespace_idents")]
fn ident_prefix() -> String {
    if let Ok(crate_name) = std::env::var("CARGO_PKG_NAME") {
        format!("__{}_", crate_name.replace('-', "_"))
    } else {
        String::from("___procmacro_")
    }
}

struct UsesSpanned(TokenStream, Uses);
impl Parse for UsesSpanned {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = Expr::parse(input)?;
        let arrow = <Token!(=>)>::parse(input)?;

        Ok(Self(quote!(#expr #arrow), Uses::parse(input)?))
    }
}

struct Uses(Vec<Use>, TokenStream);
impl Parse for Uses {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut uses = Vec::new();
        while input.peek(Token![#]) && input.peek2(Token![use]) {
            input.parse::<Token![#]>().expect("# was peeked before");
            uses.extend(Use::parse(input)?.into_iter());
        }

        Ok(Uses(uses, input.parse()?))
    }
}

impl ToTokens for Uses {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(uses, tail) = self;
        let mut uses = uses.to_vec();
        uses.extend(prelude::prelude());

        #[cfg(feature = "namespace_idents")]
        let ident_prefix = Some(ident_prefix());

        #[cfg(not(feature = "namespace_idents"))]
        let ident_prefix: Option<String> = None;

        tokens.extend(replace_in_group(
            &uses,
            tail.clone(),
            ident_prefix.as_deref(),
        ));
    }
}

fn replace_in_group(uses: &[Use], tokens: TokenStream, ident_prefix: Option<&str>) -> TokenStream {
    let mut in_path = false;
    let mut in_var = false;
    let mut namespaced_ident = false;
    tokens
        .into_iter()
        .flat_map(|token| {
            match &token {
                proc_macro2::TokenTree::Ident(ident)
                    if !in_path && namespaced_ident && ident != "crate" =>
                {
                    namespaced_ident = false;
                    return format_ident!("{}{ident}", ident_prefix.expect("ident prefix is set"))
                        .into_token_stream();
                }
                proc_macro2::TokenTree::Ident(ident) if !(in_path || in_var) => {
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
                proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '#' => in_var = true,
                proc_macro2::TokenTree::Punct(punct)
                    if punct.as_char() == '$' && ident_prefix.is_some() =>
                {
                    namespaced_ident = true;
                    return quote!();
                }
                proc_macro2::TokenTree::Group(group) => {
                    let tokens = replace_in_group(uses, group.stream(), ident_prefix);
                    return match group.delimiter() {
                        proc_macro2::Delimiter::Parenthesis => quote!((#tokens)),
                        proc_macro2::Delimiter::Brace => quote!({#tokens}),
                        proc_macro2::Delimiter::Bracket => quote!([#tokens]),
                        proc_macro2::Delimiter::None => tokens,
                    };
                }
                _ => {
                    in_path = false;
                    in_var = false;
                }
            };
            if namespaced_ident {
                namespaced_ident = false;
                quote!($#token)
            } else {
                quote!(#token)
            }
        })
        .collect()
}

#[derive(Clone)]
struct Use(TokenStream, Ident);

struct InnerUse(Option<TokenStream>, Option<Ident>);
impl InnerUse {
    fn some(path: TokenStream, ident: Ident) -> Self {
        Self(Some(path), Some(ident))
    }
}

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

    fn parse(input: ParseStream) -> Result<Vec<Self>> {
        input.parse::<Token![use]>()?;
        fn parse_inner(input: ParseStream) -> Result<Vec<InnerUse>> {
            let mut path = TokenStream::new();
            let mut uses: Vec<InnerUse> = Vec::new();

            let mut tail: Tail = Tail::None;

            while !input.is_empty() {
                if let Ok(star) = input.parse::<Token![*]>() {
                    if input.peek(Token![;]) || input.peek(Token![,]) || input.is_empty() {
                        abort!(star, "wildcards are not supported")
                    } else {
                        star.to_tokens(&mut path);
                        continue;
                    }
                }
                if let Ok(ident) = Ident::parse_any(input) {
                    if ident == "crate" {
                        abort!(ident, "crate is not supported");
                    }
                    let alias = if input.peek(Token![as])
                        && input.peek2(syn::Ident)
                        && (input.peek3(Token![;]) || input.peek3(Token![,]))
                    {
                        input.parse::<Token![as]>().unwrap();
                        Some(input.parse::<Ident>().unwrap())
                    } else {
                        None
                    };

                    if input.parse::<Token![;]>().is_ok()
                        || input.peek(Token![,])
                        || input.is_empty()
                    {
                        if ident == "self" {
                            if !path.is_empty() {
                                abort!(ident, "The `self` keyword is only allowed as the first segment of a path")
                            }
                            uses.push(InnerUse(None, alias));
                        } else {
                            uses.push(InnerUse::some(
                                quote!(#path #tail #ident),
                                alias.unwrap_or(ident),
                            ));
                        }
                        break;
                    } else if let Ok(colon2) = input.parse() {
                        tail.take().to_tokens(&mut path);
                        tail = Tail::Some(ident, colon2);
                    } else {
                        tail.take().to_tokens(&mut path);
                        ident.to_tokens(&mut path);
                    }
                    continue;
                }
                if let Ok(colon2) = input.parse::<Token![::]>() {
                    tail.take().to_tokens(&mut path);
                    colon2.to_tokens(&mut path);
                    continue;
                }
                if let Ok(Braces { content, .. }) = parse_braces(input) {
                    let inner: Punctuated<Vec<InnerUse>, Token![,]> =
                        Punctuated::parse_terminated_with(&content, parse_inner)?;
                    uses.extend(inner.into_iter().flatten().map(|InnerUse(inner, ident)| {
                        match (inner.to_owned(), ident.to_owned(), &tail) {
                            (None, None, Tail::Some(ident, _)) => {
                                InnerUse::some(quote!(#path #ident), ident.to_owned())
                            }
                            (None, Some(ident), Tail::Some(last_ident, _)) => {
                                InnerUse::some(quote!(#path #last_ident), ident)
                            }
                            (Some(inner), Some(ident), tail) => {
                                InnerUse::some(quote!(#path #tail #inner), ident)
                            }
                            // TODO correct span
                            // TODO allow `#smth::{self as name}`?
                            (None, _, Tail::None) => {
                                abort_call_site!("unable to detect module name for `self`")
                            }
                            (..) => unreachable!("{:?} {:?}", &inner, &ident),
                        }
                    }));
                    if input.parse::<Token![;]>().is_ok() || input.peek(Token![,]) {
                        break;
                    }
                    continue;
                }
                // parse `#` so that variables expanded by quote like `#smth` are consumed in on go
                input.parse::<Token![#]>().ok().to_tokens(&mut path);
                input
                    .parse::<proc_macro2::TokenTree>()?
                    .to_tokens(&mut path);
            }
            Ok(uses)
        }

        let mut leading: Option<Token![::]> = input.parse()?;

        if leading.is_none() && (input.peek(Ident::peek_any) || input.peek(Brace)) {
            leading = Some(<Token![::]>::default());
        }

        let inner = parse_inner(input)?;
        Ok(if let Some(leading) = leading {
            inner
                .into_iter()
                .map(|InnerUse(path, ident)| Use(quote!(#leading #path), ident.unwrap()))
                .collect()
        } else {
            inner
                .into_iter()
                .map(|InnerUse(path, ident)| Use(path.unwrap(), ident.unwrap()))
                .collect()
        })
    }
}

enum Tail {
    None,
    Some(Ident, Token![::]),
}

impl Tail {
    fn take(&mut self) -> Self {
        std::mem::replace(self, Tail::None)
    }
}

impl ToTokens for Tail {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Tail::Some(ident, colon2) = self {
            ident.to_tokens(tokens);
            colon2.to_tokens(tokens);
        }
    }
}
