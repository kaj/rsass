//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_884.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo() {\
            \n  @return 2;\
            \n}\
            \n\
            \n$foo: false;\
            \n@if foo() % 2 == 0 {\
            \n  $foo: true;\
            \n}\
            \n\
            \na {\
            \n  foo: $foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: true;\
        \n}\
        \n"
    );
}
