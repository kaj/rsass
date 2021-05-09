//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/189_test_empty_content.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
