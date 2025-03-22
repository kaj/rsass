//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1121.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1121")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: \"foo\";\
             \n$bar: \"bar\";\
             \n$baz: \"baz\";\
             \n/*\
             \n * <div class=\"foo #{$foo}\" bar=\"#{$bar}\" baz=\"#{$baz}\">\
             \n */\n"),
        "/*\
         \n * <div class=\"foo foo\" bar=\"bar\" baz=\"baz\">\
         \n */\n"
    );
}
