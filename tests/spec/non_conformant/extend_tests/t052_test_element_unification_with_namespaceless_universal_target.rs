//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/052_test_element_unification_with_namespaceless_universal_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a *|*.foo {a: b}\
             \nns|a {@extend .foo} -a {@extend %-a}\n"),
        "-a *|*.foo, -a ns|a {\
         \n  a: b;\
         \n}\n"
    );
}
