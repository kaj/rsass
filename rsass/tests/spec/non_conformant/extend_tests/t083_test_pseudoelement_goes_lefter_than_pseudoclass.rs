//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/083_test_pseudoelement_goes_lefter_than_pseudoclass.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("083_test_pseudoelement_goes_lefter_than_pseudoclass")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo::bar {a: b}\
             \n.baz:bang {@extend .foo}\n"),
        ".foo::bar, .baz:bang::bar {\
         \n  a: b;\
         \n}\n"
    );
}
