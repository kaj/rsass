//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1399.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1399")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  foo: 3 - \"bar\";\
             \n  foo: (3 - \"bar\");\
             \n  foo: 3 / \"bar\";\
             \n  foo: (3 / \"bar\");\
             \n}\n"),
        "foo {\
         \n  foo: 3-\"bar\";\
         \n  foo: 3-\"bar\";\
         \n  foo: 3/\"bar\";\
         \n  foo: 3/\"bar\";\
         \n}\n"
    );
}
