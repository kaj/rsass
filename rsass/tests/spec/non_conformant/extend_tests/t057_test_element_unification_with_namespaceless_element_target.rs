//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/057_test_element_unification_with_namespaceless_element_target.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "057_test_element_unification_with_namespaceless_element_target",
    )
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a a.foo {a: b}\
             \n*|a {@extend .foo} -a {@extend %-a}\n"),
        "-a a {\
         \n  a: b;\
         \n}\n"
    );
}
