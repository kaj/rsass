//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_590.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  foo: 1/2;\
            \n  foo: 0.5;\
            \n  foo: (1/2);\
            \n  foo: 1/2 == 0.5;\
            \n  foo: (1/2) == 0.5;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: 1/2;\
        \n  foo: 0.5;\
        \n  foo: 0.5;\
        \n  foo: true;\
        \n  foo: true;\
        \n}\
        \n"
    );
}
