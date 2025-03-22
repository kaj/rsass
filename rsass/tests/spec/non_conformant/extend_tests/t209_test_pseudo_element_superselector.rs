//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/209_test_pseudo_element_superselector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("209_test_pseudo_element_superselector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "%x#bar {a: b} // Add an id to make the results have high specificity\
             \n%y, %y::fblthp {@extend %x}\
             \na {@extend %y}\n"
        ),
        "a#bar, a#bar::fblthp {\
         \n  a: b;\
         \n}\n"
    );
}
