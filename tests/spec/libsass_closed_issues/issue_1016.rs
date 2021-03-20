//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1016.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  [baz=\"#{&}\"] {\
            \n    foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo [baz=\".foo\"] {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
