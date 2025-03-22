//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_817.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_817")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  foo: url(\'foo/bar.baz\');\
             \n  foo: url(\"foo/bar.baz\");\
             \n  foo: url(foo/bar.baz);\
             \n  foo: foo(\'foo/bar.baz\', \"bar\", 55);\
             \n  foo: foo(\"foo/bar.baz\", \'bar\', 55);\
             \n  foo: foo(\"foo/bar.baz\", bar, 55); }\n"),
        "foo {\
         \n  foo: url(\"foo/bar.baz\");\
         \n  foo: url(\"foo/bar.baz\");\
         \n  foo: url(foo/bar.baz);\
         \n  foo: foo(\"foo/bar.baz\", \"bar\", 55);\
         \n  foo: foo(\"foo/bar.baz\", \"bar\", 55);\
         \n  foo: foo(\"foo/bar.baz\", bar, 55);\
         \n}\n"
    );
}
