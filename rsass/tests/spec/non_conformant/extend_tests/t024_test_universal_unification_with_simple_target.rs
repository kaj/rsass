//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/024_test_universal_unification_with_simple_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("024_test_universal_unification_with_simple_target")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a .foo.bar {a: b}\
             \nns|* {@extend .foo} -a {@extend %-a}\n"),
        "-a .foo.bar, -a ns|*.bar {\
         \n  a: b;\
         \n}\n"
    );
}
