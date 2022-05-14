//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/051_test_element_unification_with_namespaceless_universal_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "051_test_element_unification_with_namespaceless_universal_target",
    )
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a *.foo {a: b}\
             \nns|a {@extend .foo} -a {@extend %-a}\n"),
        "-a *.foo {\
         \n  a: b;\
         \n}\n"
    );
}
