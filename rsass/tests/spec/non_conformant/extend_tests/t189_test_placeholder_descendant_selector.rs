//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/189_test_placeholder_descendant_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("189_test_placeholder_descendant_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("#context %foo a {a: b}\
             \n.bar {@extend %foo}\n"),
        "#context .bar a {\
         \n  a: b;\
         \n}\n"
    );
}
