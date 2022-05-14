//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_rule_comments.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_rule_comments")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("/* Foo\
             \n * Bar */\
             \n.foo {\
             \n  a: b; }\n"),
        "/* Foo\
         \n * Bar */\
         \n.foo {\
         \n  a: b;\
         \n}\n"
    );
}
