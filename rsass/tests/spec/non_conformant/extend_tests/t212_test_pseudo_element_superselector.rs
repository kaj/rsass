//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/212_test_pseudo_element_superselector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("212_test_pseudo_element_superselector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%x#bar {a: b}\
             \n%y, %y:first-letter {@extend %x}\
             \na {@extend %y}\n"),
        "a#bar, a#bar:first-letter {\
         \n  a: b;\
         \n}\n"
    );
}
