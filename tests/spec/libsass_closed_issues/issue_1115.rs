//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1115.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n    bar: \"x\\79\";\
             \n    baz: \"#{x}\\79\";\
             \n    bar: \"x\\a\";\
             \n    baz: \"#{x}\\a\";\
             \n}\n"),
        "foo {\
         \n  bar: \"xy\";\
         \n  baz: \"xy\";\
         \n  bar: \"x\\a\";\
         \n  baz: \"x\\a\";\
         \n}\n"
    );
}
