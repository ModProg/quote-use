use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::Token;

use crate::use_parser::UseItem;
use crate::Use;

pub(crate) fn prelude() -> impl Iterator<Item = Use> {
    let prelude = parse_prelude(include_str!("prelude/core.rs"));
    let prelude = prelude.chain(parse_prelude(include_str!("prelude/std.rs")));
    let prelude = prelude.chain(parse_prelude(include_str!("prelude/2021.rs")));

    prelude
}

fn parse_prelude(file: &str) -> impl Iterator<Item = Use> {
    Punctuated::<UseItem, Token![;]>::parse_terminated
        .parse_str(file)
        .expect("prelude should be valid")
        .into_iter()
        .flat_map(|u| u.0.into_iter())
}
