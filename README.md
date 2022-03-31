# Use statements in `quote!`

Until there is actual documentation these tests explain it best:
```rs
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

#[test]
fn ident_in_path() {
    let quoted = quote::quote! {
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
    let quoted = quote::quote! {
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
```
