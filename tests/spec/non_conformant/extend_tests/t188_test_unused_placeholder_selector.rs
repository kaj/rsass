//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/188_test_unused_placeholder_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%foo {color: blue}\
            \n%bar {color: red}\
            \n.baz {@extend %foo}\
            \n"
        )
        .unwrap(),
        ".baz {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
