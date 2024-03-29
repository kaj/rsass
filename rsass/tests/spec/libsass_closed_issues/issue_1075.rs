//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1075.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1075")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$name: \"lighten\";\
             \n$args: (\"color\": #ff0000, \"amount\": 10%);\
             \nfoo {\
             \n  bar: call($name, $args...);\
             \n}\n"),
        "foo {\
         \n  bar: #ff3333;\
         \n}\n"
    );
}
