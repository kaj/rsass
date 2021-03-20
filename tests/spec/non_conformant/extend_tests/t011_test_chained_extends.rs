//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/011_test_chained_extends.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b}\
            \n.bar {@extend .foo}\
            \n.baz {@extend .bar}\
            \n.bip {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo, .bar, .bip, .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
