//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2056.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2056")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(":foobar(.baz) {\
             \n    color: red;\
             \n}\n"),
        ":foobar(.baz) {\
         \n  color: red;\
         \n}\n"
    );
}
