//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1569.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1569")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$common-border: \"foo\";\
             \n.nihilo & {\
             \n  .dijitMenu {\
             \n    border: $common-border;\
             \n    .dijitMenuItem {\
             \n      color: getColor(\'text-dark-main\');\
             \n    }\
             \n  }\
             \n}\n"),
        ".nihilo & .dijitMenu {\
         \n  border: \"foo\";\
         \n}\
         \n.nihilo & .dijitMenu .dijitMenuItem {\
         \n  color: getColor(\"text-dark-main\");\
         \n}\n"
    );
}
