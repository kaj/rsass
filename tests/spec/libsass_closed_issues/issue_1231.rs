//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1231.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1231")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div::before {\
             \n  content: #{\"\\\"\"+\\e600+\"\\\"\"};\
             \n}"),
        "div::before {\
         \n  content: \"\\e600\";\
         \n}\n"
    );
}
