//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/014_test_comment_after_if_directive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
