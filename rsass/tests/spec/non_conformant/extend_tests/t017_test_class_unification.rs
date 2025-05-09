//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/017_test_class_unification.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("017_test_class_unification")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a .foo.baz {a: b}\
             \n.baz {@extend .foo} -a {@extend %-a}\n"),
        "-a .baz {\
         \n  a: b;\
         \n}\n"
    );
}
