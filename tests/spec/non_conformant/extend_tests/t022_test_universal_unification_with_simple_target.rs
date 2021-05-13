//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/022_test_universal_unification_with_simple_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a .foo.bar {a: b}\
             \n* {@extend .foo} -a {@extend %-a}\n"),
        "-a .bar {\
         \n  a: b;\
         \n}\n"
    );
}
