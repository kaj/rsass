//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/054_test_element_unification_with_namespaced_universal_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a ns|*.foo {a: b}\
             \n*|a {@extend .foo} -a {@extend %-a}\n"),
        "-a ns|*.foo, -a ns|a {\
         \n  a: b;\
         \n}\n"
    );
}
