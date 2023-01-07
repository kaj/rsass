//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1441/adjacent.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("adjacent")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".adjacent {\
             \n    & + & {\
             \n        foo: bar;\
             \n    }\
             \n}\n"),
        ".adjacent + .adjacent {\
         \n  foo: bar;\
         \n}\n"
    );
}
