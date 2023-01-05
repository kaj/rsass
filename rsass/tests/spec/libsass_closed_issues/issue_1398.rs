//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1398.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1398")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media screen and (hux: 3/4) {\
             \n  foo {\
             \n    bar: baz;\
             \n  }\
             \n}\n"),
        "@media screen and (hux: 3/4) {\
         \n  foo {\
         \n    bar: baz;\
         \n  }\
         \n}\n"
    );
}
