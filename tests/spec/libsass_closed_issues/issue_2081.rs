//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2081.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: #{bar();};\
             \n"
        )
        .unwrap_err(),
        "Error: expected \"}\".\
         \n  ,\
         \n1 | $foo: #{bar();};\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
    );
}
