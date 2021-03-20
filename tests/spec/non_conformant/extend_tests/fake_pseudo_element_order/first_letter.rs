//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/first-letter.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%a:first-letter {x: y}\
            \nb:c {@extend %a}\
            \n"
        )
        .unwrap(),
        "b:c:first-letter {\
        \n  x: y;\
        \n}\
        \n"
    );
}
