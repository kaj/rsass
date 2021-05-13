//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1208.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  &.bar, /* */\
             \n  &.baz {\
             \n    color: red;\
             \n  }\
             \n}\n"),
        "foo.bar, foo.baz {\
         \n  color: red;\
         \n}\n"
    );
}
