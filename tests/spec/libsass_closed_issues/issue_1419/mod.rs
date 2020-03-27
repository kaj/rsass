//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1419"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1419/quoted.hrx"
#[test]
fn quoted() {
    assert_eq!(
        rsass(
            "foo {\
            \n  foo: to-upper-case(\"ab\\63 d\");\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: \"ABCD\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1419/unquoted.hrx"
#[test]
fn unquoted() {
    assert_eq!(
        rsass(
            "foo {\
            \n  foo: to-upper-case(ab\\63 d);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: ABCD;\
        \n}\
        \n"
    );
}
