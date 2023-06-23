//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/098_test_nested_extender_runs_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("098_test_nested_extender_runs_unification")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo.bar {a: b}\
             \nfoo bar {@extend .foo}\n"),
        ".foo.bar, foo bar.bar {\
         \n  a: b;\
         \n}\n"
    );
}
