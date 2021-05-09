//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1257.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  color: invert(red...);\
             \n}"),
        ".foo {\
         \n  color: aqua;\
         \n}\n"
    );
}
