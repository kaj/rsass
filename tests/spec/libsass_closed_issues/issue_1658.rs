//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1658.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@else{}\
             \n"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n1 | @else{}\
         \n  | ^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet\
         \n",
    );
}
