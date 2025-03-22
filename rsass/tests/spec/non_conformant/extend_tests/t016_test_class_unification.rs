//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/016_test_class_unification.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("016_test_class_unification")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a .foo.bar {a: b}\
             \n.baz {@extend .foo} -a {@extend %-a}\n"),
        "-a .foo.bar, -a .bar.baz {\
         \n  a: b;\
         \n}\n"
    );
}
