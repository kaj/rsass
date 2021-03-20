//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/212_test_pseudo_element_superselector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%x#bar {a: b}\
            \n%y, %y:first-letter {@extend %x}\
            \na {@extend %y}\
            \n"
        )
        .unwrap(),
        "a#bar, a#bar:first-letter {\
        \n  a: b;\
        \n}\
        \n"
    );
}
