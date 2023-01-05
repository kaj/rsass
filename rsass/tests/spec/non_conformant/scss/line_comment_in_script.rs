//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/line_comment_in_script.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("line_comment_in_script")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {a: 1 + // flang }\
             \n  blang }\n"),
        "foo {\
         \n  a: 1blang;\
         \n}\n"
    );
}
