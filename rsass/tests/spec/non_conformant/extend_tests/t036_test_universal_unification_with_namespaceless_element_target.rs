//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/036_test_universal_unification_with_namespaceless_element_target.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "036_test_universal_unification_with_namespaceless_element_target",
    )
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a *|a.foo {a: b}\
             \n* {@extend .foo} -a {@extend %-a}\n"),
        "-a *|a.foo, -a a {\
         \n  a: b;\
         \n}\n"
    );
}
