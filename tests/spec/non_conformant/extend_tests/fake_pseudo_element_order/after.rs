//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/after.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%a:after {x: y}\
            \nb:c {@extend %a}\
            \n"
        )
        .unwrap(),
        "b:c:after {\
        \n  x: y;\
        \n}\
        \n"
    );
}
