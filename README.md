# Use statements in `quote!`
[![Crates.io Version](https://img.shields.io/crates/v/quote-use.svg)](https://crates.io/crates/quote-use)
[![Live Build Status](https://img.shields.io/github/workflow/status/ModProg/quote-use/Test/main)](https://github.com/ModProg/quote-use/actions/workflows/test.yml)
[![Docs.rs Documentation](https://img.shields.io/docsrs/quote-use)](https://docs.rs/crate/quote-use)

## Description
                                                                                                             
Macro to simplify using Types in the [`quote!`] macro.
                                                                                                             
## Usage
                                                                                                             
The [`quote_use!`] macro can be used just like [`quote!`], but with the added functionality of
adding use statements at the top:
                                                                                                             
```rust
## use quote_use::quote_use;
quote_use!{
    use std::fs::read;
    
    read("src/main.rs")
}
## ;
```
                                                                                                             
This will expand to the equivalent statement using [`quote!`]:
                                                                                                             
```rust
## use quote::quote;
quote!{
    ::std::fs::read::read("src/main.rs")
}
## ;
```
                                                                                                             
### Prelude
                                                                                                             
This also allows to use contents of the rust prelude directly:
                                                                                                             
```rust
## use quote_use::quote_use;
quote_use!{
    Some("src/main.rs")
}
## ;
```
#### Overriding prelude
When you want to use your own type instead of the prelude type this can be achieved by simply
importing it like so
                                                                                                             
```rust
## use quote_use::quote_use;
quote_use!{
    use anyhow::Result;
                                                                                                             
    Result
}
## ;
```
#### Different preludes
                                                                                                             
By default [`quote_use!`] uses the [std prelude](std::prelude) for [2021 edition](std::prelude::rust_2021), 
but this can be configured via features, and also completely disabled.
                                                                                                             
- **`prelude_std`**: Enables [`std::prelude::v1`]  (incompatible with `prelude_core`)
- `prelude_core`: Enables [`core::prelude::v1`] (incompatible with `prelude_std`)
- **`prelude_2021`**: Enables [`core::prelude::rust_2021`] (requires either `prelude_std` or `prelude_core`)
