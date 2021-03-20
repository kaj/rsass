//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_646.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo() {\
            \n  /* $bar: 1; */\
            \n @return true;\
            \n}\
            \n\
            \nfoo {\
            \n  foo: foo();\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: true;\
        \n}\
        \n"
    );
}
