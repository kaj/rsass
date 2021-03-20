//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1277.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: foo;\
            \n$bar: bar;\
            \n\
            \n.foo {\
            \n  foo: foo #{$foo}, bar #{$bar};\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  foo: foo foo, bar bar;\
        \n}\
        \n"
    );
}
