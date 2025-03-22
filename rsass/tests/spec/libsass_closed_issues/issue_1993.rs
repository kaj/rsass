//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1993.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1993")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".test {\
             \n  @extend #{\'%test\'} !optional\
             \n}"),
        ""
    );
}
