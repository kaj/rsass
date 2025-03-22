//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1996.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1996")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: \"bar\";\n\
             \n%class //#{$foo}\
             \n{\
             \n  &_baz {\
             \n    color: red;\
             \n  }\
             \n}"),
        ""
    );
}
