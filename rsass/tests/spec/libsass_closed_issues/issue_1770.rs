//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1770.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1770")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function returns-string() {\
             \n  @return \"selector\";\
             \n}\n\
             \n#{\"selector\"} {\
             \n  color: red;\
             \n}\n\
             \n#{returns-string()} {\
             \n  color: red;\
             \n}\n\
             \n#{\"selector\"} selector2 {\
             \n  color: red;\
             \n}\n\
             \n#{returns-string()} selector2 {\
             \n  color: red;\
             \n}"),
        "selector {\
         \n  color: red;\
         \n}\
         \nselector {\
         \n  color: red;\
         \n}\
         \nselector selector2 {\
         \n  color: red;\
         \n}\
         \nselector selector2 {\
         \n  color: red;\
         \n}\n"
    );
}
