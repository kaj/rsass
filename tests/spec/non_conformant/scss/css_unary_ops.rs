//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_unary_ops.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  a: -0.5em;\
            \n  b: 0.5em;\
            \n  c: -foo(12px);\
            \n  d: +foo(12px); }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: -0.5em;\
        \n  b: 0.5em;\
        \n  c: -foo(12px);\
        \n  d: +foo(12px);\
        \n}\
        \n"
    );
}
