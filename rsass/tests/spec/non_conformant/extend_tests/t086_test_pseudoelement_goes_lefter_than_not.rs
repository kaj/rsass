//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/086_test_pseudoelement_goes_lefter_than_not.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("086_test_pseudoelement_goes_lefter_than_not")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo:not(.bang) {a: b}\
             \n.baz::bar {@extend .foo}\n"),
        ".foo:not(.bang), .baz:not(.bang)::bar {\
         \n  a: b;\
         \n}\n"
    );
}
