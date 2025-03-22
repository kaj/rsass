//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2153.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2153")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  a: \"\\\"foo\\\"\" + \"\";\
             \n  b: \"\" + \"\\\"foo\\\"\";\
             \n  c: \"\" + \"\\\"foo\";\
             \n  d: \"\\\"foo\\\"\" + \"bar\";\
             \n}\n"),
        "foo {\
         \n  a: \'\"foo\"\';\
         \n  b: \'\"foo\"\';\
         \n  c: \'\"foo\';\
         \n  d: \'\"foo\"bar\';\
         \n}\n"
    );
}
