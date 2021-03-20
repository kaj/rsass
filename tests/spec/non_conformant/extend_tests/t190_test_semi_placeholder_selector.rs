//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/190_test_semi_placeholder_selector.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "#context %foo, .bar .baz {color: blue}\
            \n\
            \n.bat {\
            \n  @extend %foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "#context .bat, .bar .baz {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
