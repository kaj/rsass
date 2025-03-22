//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2260/inner-parent-with-compound.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("inner-parent-with-compound")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin test() {\r\
             \n  @at-root {\r\
             \n    .inner {\r\
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
        ".test .inner {\
         \n  property: value;\
         \n}\n"
    );
}
