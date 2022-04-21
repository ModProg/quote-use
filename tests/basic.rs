use quote::quote;
use quote_use::quote_use;

#[test]
fn r#use() {
    let quoted = quote! {
        ::smth::ho::Name(10)
    };

    let quote_used = quote_use! {
        # use ::smth::ho::Name;

        Name(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());

    let quote_used = quote_use! {
        # use smth::ho::Name;

        Name(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());

    let quote_used = quote_use! {
        # use smth::ho::Name as Something;

        Something(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[cfg(feature = "prelude_core")]
#[test]
fn prelude_core() {
    let quoted = quote! {
        ::core::prelude::v1::Some(10)
    };

    let quote_used = quote_use! {
        Some(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[cfg(all(feature = "prelude_2021"))]
#[test]
fn prelude_2021() {
    let quoted = quote! {
        ::core::prelude::rust_2021::FromIterator
    };

    let quote_used = quote_use! {
        FromIterator
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[cfg(feature = "prelude_std")]
#[test]
fn prelude_std() {
    let quoted = quote! {
        ::std::prelude::v1::String::new("hello")
    };

    let quote_used = quote_use! {
        String::new("hello")
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[cfg(any(feature = "prelude_core", feature = "prelude_std"))]
#[test]
fn prelude_override() {
    let quoted = quote! {
        ::anyhow::Result
    };

    let quote_used = quote_use! {
        # use anyhow::Result;

        Result
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
        # use ::smth::ho::Name;

        Name(10);
        other::Name(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[test]
fn ident_in_var() {
    let name = "";
    let quoted = quote! {
        ::smth::ho::name(10);
        #name
    };

    let quote_used = quote_use! {
        # use ::smth::ho::name;

        name(10);
        #name
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
        # use ::smth::ho;

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
        # use ::smth::ho::{Name, Ident, module::{something::anything, another::Strange}};

        Name(10);
        Strange;
        other::Name(10)
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());

    let quote_used = quote_use! {
        # use ::smth::ho::{Name as SomethingElse, Ident, module::{another::Strange, something::anything}};

        SomethingElse(10);
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
        # use ::smth::ho::{self, Ident};

        ho::Name(10);
    };

    assert_eq!(quote_used.to_string(), quoted.to_string());

    let quote_used = quote_use! {
        # use ::smth::ho::{self as test, Ident};

        test::Name(10);
    };

    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[test]
fn braces() {
    let quoted = quote! {
        {::smth::ho::Name(10)}
        [::smth::ho::Name(10)]
        (::smth::ho::Name(10))
    };

    let quote_used = quote_use! {
        # use ::smth::ho::Name;

        {Name(10)}
        [Name(10)]
        (Name(10))
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[test]
fn var_in_path() {
    let root = quote!(::smth::ho);
    let quoted = quote! {
        #root::Name(10);
    };

    let quote_used = quote_use! {
        # use #root::Name;

        Name(10);
    };

    assert_eq!(quote_used.to_string(), quoted.to_string());
    let root = quote!(::smth::ho::);
    let quote_used = quote_use! {
        # use #root Name;

        Name(10);
    };

    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[cfg(feature = "namespace_idents")]
#[test]
fn namespace_idents() {
    let quoted = quote! {
        __quote_use_ident;
    };

    let quote_used = quote_use! {
         $ident;
    };
    assert_eq!(quote_used.to_string(), quoted.to_string());
}

#[cfg(feature = "namespace_idents")]
#[test]
fn format_ident_namespaced() {
    use quote::format_ident;
    use quote_use::format_ident_namespaced;

    let quoted = format_ident!("__quote_use_ident_{}", 2usize);

    let quote_used = format_ident_namespaced!("$ident_{}", 2usize);

    assert_eq!(quote_used.to_string(), quoted.to_string());
}
