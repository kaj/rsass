//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1993.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
