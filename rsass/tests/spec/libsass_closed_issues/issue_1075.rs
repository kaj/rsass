//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1075.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1075")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$name: \"lighten\";\
             \n$args: (\"color\": #ff0000, \"amount\": 10%);\
             \nfoo {\
             \n  bar: meta.call($name, $args...);\
             \n}\n"),
        "foo {\
         \n  bar: #ff3333;\
         \n}\n"
    );
}
