//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/assignment.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: #{};\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n1 | $foo: #{};\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
