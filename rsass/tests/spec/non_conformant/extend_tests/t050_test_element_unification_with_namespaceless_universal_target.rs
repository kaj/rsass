//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/050_test_element_unification_with_namespaceless_universal_target.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "050_test_element_unification_with_namespaceless_universal_target",
    )
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a *|*.foo {a: b}\
             \n*|a {@extend .foo} -a {@extend %-a}\n"),
        "-a *|*.foo, -a *|a {\
         \n  a: b;\
         \n}\n"
    );
}
