//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/096_test_long_extender_runs_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("096_test_long_extender_runs_unification")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("ns|*.foo.bar {a: b}\
             \na.baz {@extend .foo}\n"),
        "ns|*.foo.bar {\
         \n  a: b;\
         \n}\n"
    );
}
