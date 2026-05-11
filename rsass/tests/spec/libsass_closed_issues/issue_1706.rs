//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1706.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1706")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@function calc($e) { @return custom; }\
             \n@function -foo-calc($e) { @return custom; }\n\
             \n.test {\
             \n    a: calc(1px * 1%);\
             \n    b: -foo-calc(2px * 2%);\
             \n    c: call(calc, 3px * 3%);\
             \n}\n"),
        ".test {\
         \n  a: custom;\
         \n  b: -foo-calc(2px * 2%);\
         \n  c: custom;\
         \n}\n"
    );
}
