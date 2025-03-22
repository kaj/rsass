//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/002_test_one_line_comments.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("002_test_one_line_comments")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo bar[val=\"//\"] {\
             \n  baz: bang; //}\
             \n}\n"),
        ".foo bar[val=\"//\"] {\
         \n  baz: bang;\
         \n}\n"
    );
}
