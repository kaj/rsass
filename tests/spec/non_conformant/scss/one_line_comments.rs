//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/one_line_comments.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
