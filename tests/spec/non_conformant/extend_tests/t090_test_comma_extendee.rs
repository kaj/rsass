//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/090_test_comma_extendee.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b}\
            \n.bar {c: d}\
            \n.baz {@extend .foo, .bar}\
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
