//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/not-into-not-not.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("not-into-not-not")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("// Regression test for dart-sass#191.\
             \n:not(:not(.x)) {a: b}\
             \n:not(.y) {@extend .x}\n"),
        ":not(:not(.x)) {\
         \n  a: b;\
         \n}\n"
    );
}
