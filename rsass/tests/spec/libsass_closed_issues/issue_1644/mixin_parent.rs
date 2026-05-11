//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1644/mixin-parent.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin-parent")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@mixin parent {\
             \n  @content;\
             \n}\n\
             \n@include parent() {\
             \n  body.immobile & {\
             \n    margin-bottom: 0;\
             \n  }\
             \n}\n"),
        "body.immobile & {\
         \n  margin-bottom: 0;\
         \n}\n"
    );
}
