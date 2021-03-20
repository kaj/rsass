//! Tests auto-converted from "sass-spec/spec/core_functions/string/to_upper_case.hrx"

#[test]
fn alphabet() {
    assert_eq!(
        crate::rsass(
            "a {b: to-upper-case(\"abcdefghijklmnopqrstuvqxyz\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"ABCDEFGHIJKLMNOPQRSTUVQXYZ\";\
        \n}\
        \n"
    );
}
#[test]
fn empty() {
    assert_eq!(
        crate::rsass(
            "a {b: to-upper-case(\"\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"\";\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: to-upper-case($string: abcDEF)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: ABCDEF;\
        \n}\
        \n"
    );
}
#[test]
fn non_ascii() {
    assert_eq!(
        crate::rsass(
            "// Only ASCII characters have their case changed.\
            \na {b: to-upper-case(\"äçðøþ\")}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \na {\
        \n  b: \"äçðøþ\";\
        \n}\
        \n"
    );
}
#[test]
fn number() {
    assert_eq!(
        crate::rsass(
            "a {b: to-upper-case(\"1234567890\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"1234567890\";\
        \n}\
        \n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        crate::rsass(
            "a {b: to-upper-case(aBcDeF)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: ABCDEF;\
        \n}\
        \n"
    );
}
