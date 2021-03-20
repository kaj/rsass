//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/209_test_pseudo_element_superselector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%x#bar {a: b} // Add an id to make the results have high specificity\
            \n%y, %y::fblthp {@extend %x}\
            \na {@extend %y}\
            \n"
        )
        .unwrap(),
        "a#bar, a#bar::fblthp {\
        \n  a: b;\
        \n}\
        \n"
    );
}
