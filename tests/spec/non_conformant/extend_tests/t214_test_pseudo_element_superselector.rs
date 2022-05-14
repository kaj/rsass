//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/214_test_pseudo_element_superselector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("214_test_pseudo_element_superselector")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%x#bar {a: b}\
             \n%y, %y:after {@extend %x}\
             \na {@extend %y}\n"),
        "a#bar, a#bar:after {\
         \n  a: b;\
         \n}\n"
    );
}
