//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/061_test_element_unification_with_namespaceless_element_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "061_test_element_unification_with_namespaceless_element_target",
    )
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a *|a.foo {a: b}\
             \nns|a {@extend .foo} -a {@extend %-a}\n"),
        "-a *|a.foo, -a ns|a {\
         \n  a: b;\
         \n}\n"
    );
}
