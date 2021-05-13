//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_644.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  background-image: url(foo/#{\"bar\"}/baz.jpg);\
             \n}\n"),
        "foo {\
         \n  background-image: url(foo/bar/baz.jpg);\
         \n}\n"
    );
}
