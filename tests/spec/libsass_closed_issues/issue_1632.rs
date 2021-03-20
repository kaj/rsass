//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1632.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: \\/ !global;\
            \n.foo#{$foo}bar { a: b; }\
            \n"
        )
        .unwrap(),
        ".foo\\/bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
