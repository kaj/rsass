//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1419/quoted.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
