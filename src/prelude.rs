use proc_macro2::Ident;
use syn::{parse_quote, parse_str, File, Item, ItemUse, UsePath, UseTree};

use crate::Use;

#[cfg(all(
    feature = "prelude_2021",
    not(any(feature = "prelude_std", feature = "prelude_core"))
))]
compile_error!("prelude_2021 only works when either prelude_std or prelude_core are enabled");

#[cfg(all(feature = "prelude_std", feature = "prelude_core"))]
compile_error!("prelude_core and prelude_std are mutually exclusive");

#[cfg(feature = "prelude_std")]
pub(crate) fn prelude() -> impl Iterator<Item = Use> {
    let prelude = parse_prelude(include_str!("prelude/std.rs"), parse_quote!(std));
    #[cfg(feature = "prelude_2021")]
    return prelude.chain(parse_prelude(
        include_str!("prelude/2021.rs"),
        parse_quote!(std),
    ));
    #[cfg(not(feature = "prelude_2021"))]
    prelude
}

#[cfg(feature = "prelude_core")]
pub(crate) fn prelude() -> impl Iterator<Item = Use> {
    let prelude = parse_prelude(include_str!("prelude/core.rs"), parse_quote!(core));
    #[cfg(feature = "prelude_2021")]
    return prelude.chain(parse_prelude(
        include_str!("prelude/2021.rs"),
        parse_quote!(core),
    ));
    #[cfg(not(feature = "prelude_2021"))]
    prelude
}

#[cfg(not(any(feature = "prelude_core", feature = "prelude_std")))]
pub(crate) fn prelude() -> impl Iterator<Item = Use> {
    Vec::new().into_iter()
}

fn parse_prelude(file: &str, crate_: Ident) -> impl Iterator<Item = Use> {
    let statements: File = parse_str(file).unwrap();

    statements
        .items
        .into_iter()
        .flat_map(move |expr| match expr {
            Item::Use(item_use) => Use::from_item_use(match item_use {
                ItemUse {
                    attrs,
                    vis,
                    use_token,
                    leading_colon,
                    tree:
                        UseTree::Path(UsePath {
                            ident,
                            colon2_token,
                            tree,
                        }),
                    semi_token,
                } if ident == "crate" => ItemUse {
                    attrs,
                    vis,
                    use_token,
                    leading_colon,
                    tree: UseTree::Path(UsePath {
                        ident: crate_.clone(),
                        colon2_token,
                        tree,
                    }),
                    semi_token,
                },
                any => any,
            }),
            _ => Vec::new(),
        })
}
