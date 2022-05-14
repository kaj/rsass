//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1993.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1993")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".test {\
             \n  @extend #{\'%test\'} !optional\
             \n}"),
        ""
    );
}
