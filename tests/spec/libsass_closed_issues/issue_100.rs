//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_100.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$endColor: red;\r\
             \ntest {\r\
             \n  background-color: darken($endColor, 10%) \\9;\r\
             \n}"),
        "test {\
         \n  background-color: #cc0000 \\9 ;\
         \n}\n"
    );
}
