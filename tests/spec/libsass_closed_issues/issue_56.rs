//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_56.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media (min-width: 980px) {\r\
             \n    a {\r\
             \n        color: red;\r\
             \n    }\r\
             \n}"),
        "@media (min-width: 980px) {\
         \n  a {\
         \n    color: red;\
         \n  }\
         \n}\n"
    );
}
