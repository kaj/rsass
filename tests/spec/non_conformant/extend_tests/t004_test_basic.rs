//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/004_test_basic.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b}\
            \n.bar {@extend .foo; c: d}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n.bar {\
        \n  c: d;\
        \n}\
        \n"
    );
}
