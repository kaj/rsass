//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1063.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1063")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%foo {\
             \n  & > x { display: block; }\
             \n}\n\
             \na {\
             \n  > b { @extend %foo; }\
             \n  > b > c { @extend %foo; }\
             \n}\n"),
        "a > b > c > x, a > b > x {\
         \n  display: block;\
         \n}\n"
    );
}
