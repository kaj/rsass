//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_invisible_comments.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_invisible_comments")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  a: d; }\n"),
        "foo {\
         \n  a: d;\
         \n}\n"
    );
}
