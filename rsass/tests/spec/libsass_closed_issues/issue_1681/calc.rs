//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1681/calc.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("calc")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@function calc() {\
             \n  @return null;\
             \n}\n"),
        ""
    );
}
