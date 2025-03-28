//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/033_test_universal_unification_with_namespaced_universal_target.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "033_test_universal_unification_with_namespaced_universal_target",
    )
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a ns|*.foo {a: b}\
             \nns|* {@extend .foo} -a {@extend %-a}\n"),
        "-a ns|* {\
         \n  a: b;\
         \n}\n"
    );
}
