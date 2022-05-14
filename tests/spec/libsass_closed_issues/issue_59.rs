//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_59.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_59")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin apply-to-ie6-only {\r\
             \n  * html {\r\
             \n    @content;\r\
             \n  }\r\
             \n}\r\
             \n@include apply-to-ie6-only {\r\
             \n  #logo {\r\
             \n    background-image: url(/logo.gif);\r\
             \n  }\r\
             \n}"),
        "* html #logo {\
         \n  background-image: url(/logo.gif);\
         \n}\n"
    );
}
