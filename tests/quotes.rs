use proc_macro2::Span;
use quote::{quote_spanned, ToTokens};
use quote_use::{parse_quote_spanned_use, parse_quote_use, quote_spanned_use};
use syn::{parse_quote, parse_quote_spanned, Expr};

#[test]
fn quote_spanned() {
    let quoted = quote_spanned! {Span::call_site()=>
        ::smth::ho::Name(10)
    };

    let quote_used = quote_spanned_use! {Span::call_site()=>
        use smth::ho::Name;

        Name(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[test]
fn quote_spanned_empty() {
    let quoted = quote_spanned! (Span::call_site()=>);
    let quote_used = quote_spanned_use! (Span::call_site()=>);
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[test]
fn parse_quote_spanned() {
    let quoted: Expr = parse_quote_spanned! {Span::call_site()=>
        ::smth::ho::Name(10)
    };

    let quote_used: Expr = parse_quote_spanned_use! {Span::call_site()=>
        use smth::ho::Name;

        Name(10)
    };
    assert_eq!(
        quote_used.to_token_stream().to_string(),
        quoted.to_token_stream().to_string()
    );
}

#[test]
fn parse_quote() {
    let quoted: Expr = parse_quote! {
        ::smth::ho::Name(10)
    };

    let quote_used: Expr = parse_quote_use! {
        use smth::ho::Name;

        Name(10)
    };
    assert_eq!(
        quote_used.to_token_stream().to_string(),
        quoted.to_token_stream().to_string()
    );
}
