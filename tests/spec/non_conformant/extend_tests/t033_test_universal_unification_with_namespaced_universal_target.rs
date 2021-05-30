//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/033_test_universal_unification_with_namespaced_universal_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a ns|*.foo {a: b}\
             \nns|* {@extend .foo} -a {@extend %-a}\n"),
        "-a ns|* {\
         \n  a: b;\
         \n}\n"
    );
}
