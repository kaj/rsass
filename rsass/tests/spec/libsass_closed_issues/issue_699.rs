//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_699.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_699")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".selector {\
             \n  color: invert(rebeccapurple);\
             \n}"),
        ".selector {\
         \n  color: #99cc66;\
         \n}\n"
    );
}
