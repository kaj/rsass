//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_231.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_231")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("// test.scss:\r\
             \na {\r\
             \n  background-image: url(fn(\"s\"));\r\
             \n}"),
        "a {\
         \n  background-image: url(fn(\"s\"));\
         \n}\n"
    );
}
