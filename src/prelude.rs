use syn::{parse::Parser, punctuated::Punctuated, Token};

use crate::{use_parser::UseItem, Use};

#[cfg(all(feature = "prelude_2021", not(feature = "prelude_core")))]
compile_error!("prelude_2021 only works when prelude_core is enabled");

#[cfg(feature = "prelude_core")]
pub(crate) fn prelude() -> impl Iterator<Item = Use> {
    #[cfg(feature = "prelude_core")]
    let prelude = parse_prelude(include_str!("prelude/core.rs"));
    #[cfg(feature = "prelude_std")]
    let prelude = prelude.chain(parse_prelude(include_str!("prelude/std.rs")));
    #[cfg(feature = "prelude_2021")]
    let prelude = prelude.chain(parse_prelude(include_str!("prelude/2021.rs")));

    prelude
}

#[cfg(not(feature = "prelude_core"))]
pub(crate) fn prelude() -> impl Iterator<Item = Use> {
    Vec::new().into_iter()
}

fn parse_prelude(file: &str) -> impl Iterator<Item = Use> {
    Punctuated::<UseItem, Token![;]>::parse_terminated
        .parse_str(file)
        .expect("prelude should be valid")
        .into_iter()
        .map(|u| u.0.into_iter())
        .flatten()
}
