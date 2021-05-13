//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2382.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".test {\r\
             \n  font: normal normal 400 16px/calc(16px * 1.4) Oxygen;\r\
             \n}"),
        ".test {\
         \n  font: normal normal 400 16px/calc(16px * 1.4) Oxygen;\
         \n}\n"
    );
}
