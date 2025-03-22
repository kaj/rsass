//! Tests auto-converted from "sass-spec/spec/directives/at_root/keyframes.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("keyframes")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok("@keyframes a {\
             \n  @at-root (without: all) {\
             \n    b {c: d}\
             \n  }\
             \n}\n"),
        "@keyframes a {}\
         \nb {\
         \n  c: d;\
         \n}\n"
    );
}
