//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1732/invalid/ruleset.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "color: green;\
             \n"
        )
        .unwrap_err(),
        "Error: expected \"{\".\
         \n  ,\
         \n1 | color: green;\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
