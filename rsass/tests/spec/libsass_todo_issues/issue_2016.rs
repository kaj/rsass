//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2016.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2016")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "$_: ___((Classes and IDs must follow a specific grammar. And this thing here doesn’t.));"
        ),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | $_: ___((Classes and IDs must follow a specific grammar. And this thing here doesn’t.));\
         \n  |                                                         ^\
         \n  \'\
         \n  input.scss 1:57  root stylesheet",
    );
}
