//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/030_test_universal_unification_with_namespaceless_universal_target.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "030_test_universal_unification_with_namespaceless_universal_target",
    )
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a *|*.foo {a: b}\
             \nns|* {@extend .foo} -a {@extend %-a}\n"),
        "-a *|*.foo, -a ns|* {\
         \n  a: b;\
         \n}\n"
    );
}
