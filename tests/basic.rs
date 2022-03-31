use quote::quote;
use quote_use::quote_use;
use std::result::{Result, self};

#[test]
fn r#use() {
    let quoted = quote! {
        ::smth::ho::Name(10)
    };

    let quote_used = quote_use! {
        use ::smth::ho::Name;

        Name(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());

    let quote_used = quote_use! {
        use smth::ho::Name;

        Name(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[test]
fn prelude() {
    let quoted = quote! {
        ::core::option::Option::Some(10)
    };

    let quote_used = quote_use! {
        Some(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[test]
fn ident_in_path() {
    let quoted = quote! {
        ::smth::ho::Name(10);
        other::Name(10)
    };

    let quote_used = quote_use! {
        use ::smth::ho::Name;

        Name(10);
        other::Name(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[test]
fn module() {
    let quoted = quote! {
        ::smth::ho::Name(10);
        other::Name(10)
    };

    let quote_used = quote_use! {
        use ::smth::ho;

        ho::Name(10);
        other::Name(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[test]
fn group() {
    let quoted = quote! {
        ::smth::ho::Name(10);
        ::smth::ho::module::another::Strange;
        other::Name(10)
    };

    let quote_used = quote_use! {
        use ::smth::ho::{Name, Ident, module::{another::Strange, something::anything}};

        Name(10);
        Strange;
        other::Name(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[test]
fn self_in_group() {
    let quoted = quote! {
        ::smth::ho::Name(10);
    };

    let quote_used = quote_use! {
        use ::smth::ho::{self, Ident};

        ho::Name(10);
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}
