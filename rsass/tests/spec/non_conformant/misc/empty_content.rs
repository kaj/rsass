//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/empty_content.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("empty_content")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo { @content }\
             \na { b: c; @include foo {} }\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
