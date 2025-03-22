//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2333.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2333")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \ntest { test: meta.inspect((a:1,b:(foo,bar),c:3)); }"),
        "test {\
         \n  test: (a: 1, b: (foo, bar), c: 3);\
         \n}\n"
    );
}
