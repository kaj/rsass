//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1303.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".simple {\
            \n  a: selector-replace(\'foo.bar\', \'foo\', \'foo[baz]\');\
            \n}\
            \n"
        )
        .unwrap(),
        ".simple {\
        \n  a: foo.bar[baz];\
        \n}\
        \n"
    );
}
