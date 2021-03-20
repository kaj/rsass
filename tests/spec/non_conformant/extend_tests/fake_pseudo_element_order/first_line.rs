//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/first-line.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%a:first-line {x: y}\
            \nb:c {@extend %a}\
            \n"
        )
        .unwrap(),
        "b:c:first-line {\
        \n  x: y;\
        \n}\
        \n"
    );
}
