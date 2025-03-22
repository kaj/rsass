//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/031_test_line_comment_in_script.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("031_test_line_comment_in_script")
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
