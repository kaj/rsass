//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2057.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2057")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(":not(.thing) {\r\
             \n    color: red;\r\
             \n}\r\
             \n:not(.bar) {\r\
             \n    @extend .thing;\r\
             \n    background: blue;\r\
             \n}"),
        ":not(.thing) {\
         \n  color: red;\
         \n}\
         \n:not(.bar) {\
         \n  background: blue;\
         \n}\n"
    );
}
