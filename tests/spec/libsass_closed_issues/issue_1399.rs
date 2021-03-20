//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1399.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  foo: 3 - \"bar\";\
            \n  foo: (3 - \"bar\");\
            \n  foo: 3 / \"bar\";\
            \n  foo: (3 / \"bar\");\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: 3-\"bar\";\
        \n  foo: 3-\"bar\";\
        \n  foo: 3/\"bar\";\
        \n  foo: 3/\"bar\";\
        \n}\
        \n"
    );
}
