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
#[doc(hidden)]
pub mod __private {
    pub use quote;
    pub use syn;
}

pub use quote_use_macros::*;
