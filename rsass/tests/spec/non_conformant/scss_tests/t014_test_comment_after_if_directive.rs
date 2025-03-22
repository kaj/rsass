//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/014_test_comment_after_if_directive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("014_test_comment_after_if_directive")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  @if true {a: b}\
             \n  /* This is a comment */\
             \n  c: d }\n"),
        "foo {\
         \n  a: b;\
         \n  /* This is a comment */\
         \n  c: d;\
         \n}\n"
    );
}
