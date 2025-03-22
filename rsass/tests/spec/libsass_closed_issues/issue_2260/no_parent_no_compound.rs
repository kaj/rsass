//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2260/no-parent-no-compound.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no-parent-no-compound")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin test() {\r\
             \n  @at-root {\r\
             \n    @content;\r\
             \n  }\r\
             \n}\r\
             \n\r\
             \n@include test {\r\
             \n  .test {\r\
             \n    property: value;\r\
             \n   }\r\
             \n }"),
        ".test {\
         \n  property: value;\
         \n}\n"
    );
}
