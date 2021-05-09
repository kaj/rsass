//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2260/no-parent-no-compound.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
