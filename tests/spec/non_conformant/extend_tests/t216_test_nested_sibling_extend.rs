//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/216_test_nested_sibling_extend.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {@extend .bar}\
            \n\
            \n.parent {\
            \n.bar {\
            \n  width: 2000px;\
            \n}\
            \n.foo {\
            \n  @extend .bar\
            \n}\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent .bar, .parent .foo {\
        \n  width: 2000px;\
        \n}\
        \n"
    );
}
