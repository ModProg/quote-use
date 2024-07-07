use trybuild2::TestCases;

macro_rules! test_case {
    ($test_case:expr, $name:literal, {$($code:tt)*}, $expected:literal) => {
        $test_case.compile_fail_inline_check_sub($name, stringify!(fn main() {$($code)*}), $expected);
    };
}

#[test]
fn ui() {
    let t = TestCases::new();
    test_case!(
        t,
        "missing ;",
        {
            quote_use::quote_use!(
                # use hello
                not a ;
            );
        },
        "expected one of: `;`, `as`, `::`"
    );
    test_case!(
        t,
        "missing ; for {}",
        {
            quote_use::quote_use!(
                # use hello::{Hello}
                not a ;
            );
        },
        "expected `;`"
    );
}
