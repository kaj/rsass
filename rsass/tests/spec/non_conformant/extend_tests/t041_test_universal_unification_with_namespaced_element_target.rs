//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/041_test_universal_unification_with_namespaced_element_target.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "041_test_universal_unification_with_namespaced_element_target",
    )
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a ns|a.foo {a: b}\
             \n*|* {@extend .foo} -a {@extend %-a}\n"),
        "-a ns|a {\
         \n  a: b;\
         \n}\n"
    );
}
