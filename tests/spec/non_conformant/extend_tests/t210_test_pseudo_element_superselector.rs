//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/210_test_pseudo_element_superselector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%x#bar {a: b}\
            \n%y, %y:fblthp {@extend %x}\
            \na {@extend %y}\
            \n"
        )
        .unwrap(),
        "a#bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
