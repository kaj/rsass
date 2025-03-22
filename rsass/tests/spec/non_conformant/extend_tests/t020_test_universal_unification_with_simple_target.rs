//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/020_test_universal_unification_with_simple_target.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("020_test_universal_unification_with_simple_target")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a .foo {a: b}\
             \n* {@extend .foo} -a {@extend %-a}\n"),
        "-a * {\
         \n  a: b;\
         \n}\n"
    );
}
