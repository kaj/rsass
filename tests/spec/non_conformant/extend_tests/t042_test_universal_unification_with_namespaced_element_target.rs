//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/042_test_universal_unification_with_namespaced_element_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a ns|a.foo {a: b}\
             \nns|* {@extend .foo} -a {@extend %-a}\n"),
        "-a ns|a {\
         \n  a: b;\
         \n}\n"
    );
}
