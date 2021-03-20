//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/006_test_multiple_extendees.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b}\
            \n.bar {c: d}\
            \n.baz {@extend .foo; @extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo, .baz {\
        \n  a: b;\
        \n}\
        \n.bar, .baz {\
        \n  c: d;\
        \n}\
        \n"
    );
}
