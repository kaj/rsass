//! Tests auto-converted from "sass-spec/spec/libsass/eq.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  foo: center == \"center\";\
            \n  foo: (a b c) == (a b c);\
            \n  foo: a b c == a b c;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: a b false b c;\
        \n}\
        \n"
    );
}
