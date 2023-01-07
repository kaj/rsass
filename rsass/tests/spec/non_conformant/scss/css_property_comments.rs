//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_property_comments.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_property_comments")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  /* Foo\
             \n   * Bar */\
             \n  a: b; }\n"),
        ".foo {\
         \n  /* Foo\
         \n   * Bar */\
         \n  a: b;\
         \n}\n"
    );
}
