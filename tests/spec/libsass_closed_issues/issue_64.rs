//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_64.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_64")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$var: 10px;\r\
             \np {\r\
             \n\twidth: -$var;\r\
             \n}"),
        "p {\
         \n  width: -10px;\
         \n}\n"
    );
}
