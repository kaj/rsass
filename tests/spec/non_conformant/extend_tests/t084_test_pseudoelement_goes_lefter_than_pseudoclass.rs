//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/084_test_pseudoelement_goes_lefter_than_pseudoclass.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo:bar {a: b}\
             \n.baz::bang {@extend .foo}\n"),
        ".foo:bar, .baz:bar::bang {\
         \n  a: b;\
         \n}\n"
    );
}
