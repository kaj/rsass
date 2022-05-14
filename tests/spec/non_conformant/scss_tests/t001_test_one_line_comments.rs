//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/001_test_one_line_comments.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("001_test_one_line_comments")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {// bar: baz;}\
             \n  baz: bang; //}\
             \n}\n"),
        ".foo {\
         \n  baz: bang;\
         \n}\n"
    );
}
