//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/078_test_pseudoelement_remains_at_end_of_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("078_test_pseudoelement_remains_at_end_of_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo::bar {a: b}\
             \n.baz {@extend .foo}\n"),
        ".foo::bar, .baz::bar {\
         \n  a: b;\
         \n}\n"
    );
}
