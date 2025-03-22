//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/189_test_empty_content.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("189_test_empty_content")
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
