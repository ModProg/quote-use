use quote_use::quote_use;

#[test]
fn r#use() {
    let quoted = quote::quote! {
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
    let quoted = quote::quote! {
        ::core::option::Option::Some(10)
    };

    let quote_used = quote_use! {
        Some(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}
