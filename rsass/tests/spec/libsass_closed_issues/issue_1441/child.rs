//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1441/child.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("child")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".child {\
             \n    & > & {\
             \n        foo: bar;\
             \n    }\
             \n}\n"),
        ".child > .child {\
         \n  foo: bar;\
         \n}\n"
    );
}
