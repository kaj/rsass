//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/053_test_element_unification_with_namespaced_universal_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "053_test_element_unification_with_namespaced_universal_target",
    )
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a ns|*.foo {a: b}\
             \na {@extend .foo} -a {@extend %-a}\n"),
        "-a ns|*.foo {\
         \n  a: b;\
         \n}\n"
    );
}
