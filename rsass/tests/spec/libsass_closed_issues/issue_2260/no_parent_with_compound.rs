//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2260/no-parent-with-compound.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no-parent-with-compound")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@mixin test() {\
             \n  @at-root {\
             \n    @content;\
             \n  }\
             \n}\n\
             \n@include test {\
             \n  .test & {\
             \n    property: value;\
             \n   }\
             \n}\n"),
        ".test & {\
         \n  property: value;\
         \n}\n"
    );
}
