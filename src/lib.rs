//! # Description
//!
//! Macro to simplify using Types in the [`quote!`] macro.
//!
//! # Usage
//!
//! The [`quote_use!`] macro can be used just like [`quote!`], but with the
//! added functionality of adding use statements at the top:
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
//! When you want to use your own type instead of the prelude type this can be
//! achieved by simply importing it like so
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
//! By default [`quote_use!`] uses the [std prelude](std::prelude), [core
//! prelude](core::prelude) and [2021 edition prelude](std::prelude::rust_2021),
//! but this can be configured via features, and also completely disabled.
//!
//! - **`prelude_core`**: Enables [`core::prelude::v1`]
//! - **`prelude_std`**: Enables [`std::prelude::v1`]  (Adds only those missing
//!   in core and enables
//! also `prelude_core`)
//! - **`prelude_2021`**: Enables [`core::prelude::rust_2021`] (enables also
//!   `prelude_core`)
//!
//! ## Other quote macros
//!
//! There are also variants for other quote macros from [syn] and [mod@quote]:
//!
//! - [`quote_use!`] and [`quote_spanned_use!`] as replacement for [`quote!`]
//!   and
//! [`quote_spanned!`](quote::quote_spanned!) respectively
//! - [`parse_quote_use!`] and [`parse_quote_spanned_use!`] for
//!   [`parse_quote!`](syn::parse_quote!)
//! and [`parse_quote_spanned!`](syn::parse_quote_spanned!)
//!
//! ## Auto namespacing idents
//!
//! Until [`Span::def_site`](proc_macro::Span::def_site) is stabilized,
//! identifiers in e.g. let bindings in proc-macro expansions can collide with
//! e.g. constants.
//!
//! To circumvent this you can enable the feature `namespace_idents` which will
//! replace all identifiers and lifetimes prefixed with `$` with autonamespaced
//! ones using the pattern `"__{crate_name}_{ident}"`. A `$` can be escaped by
//! doubling it `$$`.
//!
//! ```text
//! $ident      ->  __crate_name_ident
//! $'lifetime  ->  '__crate_name_lifetime
//! $$ident     ->  $ident
//! ```

use proc_macro2::{Punct, Spacing, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
#[cfg(feature = "namespace_idents")]
use syn::LitStr;
use syn::{parse_macro_input, Expr, Result, Token};
use use_parser::{Use, UseItem};

mod prelude;

mod use_parser;

/// [`quote!`] replacement that allows [using](https://doc.rust-lang.org/std/keyword.use.html) paths to be
/// automaticly replaced.
///
/// It supports both the explicit use via `use some::path::Type;` and the use of
/// the rust prelude:
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
#[proc_macro]
pub fn quote_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let uses = parse_macro_input!(input as QuoteUse);

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
#[proc_macro]
pub fn parse_quote_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let uses = parse_macro_input!(input as QuoteUse);

    quote! {
        ::syn::parse_quote!{
            #uses
        }
    }
    .into()
}

/// Like [`quote_spanned_use!`] but using
/// [`parse_quote_spanned!`](syn::parse_quote_spanned)
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

/// [`format_ident!`](quote::format_ident) replacement that allows the auto
/// namespacing matching the `quote!` macros of this crate.
/// ```
/// # use quote_use::format_ident_namespaced;
/// format_ident_namespaced!("$ident_{}", 2usize)
/// # ;
/// ```
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

struct UsesSpanned(TokenStream, QuoteUse);
impl Parse for UsesSpanned {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = Expr::parse(input)?;
        let arrow = <Token!(=>)>::parse(input)?;

        Ok(Self(quote!(#expr #arrow), QuoteUse::parse(input)?))
    }
}

struct QuoteUse(Vec<Use>, TokenStream);
impl Parse for QuoteUse {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut uses = Vec::new();
        while input.peek(Token![#]) && input.peek2(Token![use]) {
            input.parse::<Token![#]>().expect("# was peeked before");
            uses.extend_from_slice(&UseItem::parse(input)?.0);
        }

        Ok(QuoteUse(uses, input.parse()?))
    }
}

impl ToTokens for QuoteUse {
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
    use State::*;
    #[derive(Clone, Copy)]
    enum State {
        Path,
        Pound,
        Dollar,
        DollarQuote,
        Normal,
    }
    impl ToTokens for State {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                Path | Normal | Pound => {}
                Dollar => quote!($).to_tokens(tokens),
                DollarQuote => unreachable!("lifetime `'` must be followed by ident"),
            }
        }
    }
    let mut state = Normal;

    tokens
        .into_iter()
        .flat_map(|token| {
            let mut prev = Normal;
            match (&token, state) {
                (TokenTree::Ident(ident), Dollar) if ident != "crate" => {
                    state = Normal;
                    return format_ident!("{}{ident}", ident_prefix.expect("ident prefix is set"))
                        .into_token_stream();
                }
                (TokenTree::Ident(ident), DollarQuote) => {
                    state = Normal;
                    return [
                        TokenTree::from(Punct::new('\'', Spacing::Joint)),
                        format_ident!("{}{ident}", ident_prefix.expect("ident prefix is set"))
                            .into(),
                    ]
                    .into_iter()
                    .collect();
                }
                (TokenTree::Ident(ident), Normal) => {
                    if let Some(Use(path, _)) = uses.iter().find(|item| &item.1 == ident) {
                        return quote!(#path);
                    }
                }
                // first colon
                (TokenTree::Punct(punct), _)
                    if punct.spacing() == Spacing::Joint && punct.as_char() == ':' =>
                {
                    prev = state;
                    state = Path;
                }
                // second colon
                (TokenTree::Punct(punct), _) if punct.as_char() == ':' => (),
                // quote var `#ident`
                (TokenTree::Punct(punct), _) if punct.as_char() == '#' => {
                    prev = state;
                    state = Pound;
                }
                // $$ escapes to just $
                (TokenTree::Punct(punct), Dollar) if punct.as_char() == '$' => {
                    state = Normal;
                }
                (TokenTree::Punct(punct), Normal | Pound | Path)
                    if punct.as_char() == '$' && ident_prefix.is_some() =>
                {
                    state = Dollar;
                    return quote!();
                }
                (TokenTree::Punct(punct), Dollar)
                    if punct.as_char() == '\'' && ident_prefix.is_some() =>
                {
                    state = DollarQuote;
                    return quote!();
                }
                (TokenTree::Group(group), _) => {
                    let tokens = replace_in_group(uses, group.stream(), ident_prefix);
                    return match group.delimiter() {
                        proc_macro2::Delimiter::Parenthesis => quote!((#tokens)),
                        proc_macro2::Delimiter::Brace => quote!({#tokens}),
                        proc_macro2::Delimiter::Bracket => quote!([#tokens]),
                        proc_macro2::Delimiter::None => tokens,
                    };
                }
                _ => {
                    prev = state;
                    state = Normal;
                }
            };
            quote!(#prev #token)
        })
        .collect()
}
