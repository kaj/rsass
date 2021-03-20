//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/005_test_multiple_targets.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b}\
            \n.bar {@extend .foo}\
            \n.blip .foo {c: d}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n.blip .foo, .blip .bar {\
        \n  c: d;\
        \n}\
        \n"
    );
}
