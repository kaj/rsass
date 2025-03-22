//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1825.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1825")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  &-- {\
             \n    &baz {\
             \n      color: red;\
             \n    } \
             \n  } \
             \n} "),
        "foo--baz {\
         \n  color: red;\
         \n}\n"
    );
}
