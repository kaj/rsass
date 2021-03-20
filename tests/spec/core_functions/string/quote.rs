//! Tests auto-converted from "sass-spec/spec/core_functions/string/quote.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn escape() {
    assert_eq!(
        crate::rsass(
            "a {b: quote(\\0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"\\\\0 \";\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: quote($string: c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"c\";\
        \n}\
        \n"
    );
}
mod quote_unquoted_quote {
    #[test]
    fn double() {
        assert_eq!(
            crate::rsass(
                "// See sass/libsass#2873\
            \na {b: quote(unquote(\'\"\') + unquote(\"\'\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"\\\"\'\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "// See sass/libsass#2873\
            \na {b: quote(unquote(\'\"\'))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \'\"\';\
        \n}\
        \n"
        );
    }
}
#[test]
fn quoted_double() {
    assert_eq!(
        crate::rsass(
            "a {b: quote(\"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"c\";\
        \n}\
        \n"
    );
}
#[test]
fn quoted_single() {
    assert_eq!(
        crate::rsass(
            "a {b: quote(\'c\')}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"c\";\
        \n}\
        \n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        crate::rsass(
            "a {b: quote(c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"c\";\
        \n}\
        \n"
    );
}
