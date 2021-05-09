//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1768.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@debug(());\
             \n@debug(foo, (), bar);\
             \n@debug(foo () bar);\
             \n@debug((foo: (), bar: baz));"),
        ""
    );
}
