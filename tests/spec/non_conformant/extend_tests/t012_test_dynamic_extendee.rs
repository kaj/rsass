//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/012_test_dynamic_extendee.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b}\
            \n.bar {@extend #{\".foo\"}}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
