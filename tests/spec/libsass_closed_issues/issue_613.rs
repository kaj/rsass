//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_613.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$var: 1;\n\
             \n@mixin test {\
             \n  $var: 2;\
             \n}\n\
             \n@function test() {\
             \n  $var: 3;\
             \n  @return \"dummy\";\
             \n}\n\
             \n.selector {\
             \n  $var: 4;\
             \n  @include test;\
             \n  $dummy: test();\
             \n  content: $var;\
             \n}\n\
             \n.other-selector {\
             \n    content: $var;\
             \n}\n"),
        ".selector {\
         \n  content: 4;\
         \n}\
         \n.other-selector {\
         \n  content: 1;\
         \n}\n"
    );
}
