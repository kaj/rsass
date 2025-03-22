//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/one_line_comments.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("one_line_comments")
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
