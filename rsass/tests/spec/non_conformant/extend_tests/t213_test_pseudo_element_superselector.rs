//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/213_test_pseudo_element_superselector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("213_test_pseudo_element_superselector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%x#bar {a: b}\
             \n%y, %y:before {@extend %x}\
             \na {@extend %y}\n"),
        "a#bar, a#bar:before {\
         \n  a: b;\
         \n}\n"
    );
}
