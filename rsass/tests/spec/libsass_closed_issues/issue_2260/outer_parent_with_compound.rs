//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2260/outer-parent-with-compound.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("outer-parent-with-compound")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin test() {\r\
             \n  .outer {\r\
             \n    @at-root {\r\
             \n      @content;\r\
             \n    }\r\
             \n  }\r\
             \n}\r\
             \n\r\
             \n@include test {\r\
             \n  .test & {\r\
             \n    property: value;\r\
             \n   }\r\
             \n }"),
        ".test .outer {\
         \n  property: value;\
         \n}\n"
    );
}
