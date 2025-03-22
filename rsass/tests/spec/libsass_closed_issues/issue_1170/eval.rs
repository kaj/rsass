//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1170/eval.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("eval")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("el {\
             \n  @if ((& + \'\') == \'el\') {\
             \n    content: \"It works!\";\
             \n  }\
             \n}"),
        "el {\
         \n  content: \"It works!\";\
         \n}\n"
    );
}
