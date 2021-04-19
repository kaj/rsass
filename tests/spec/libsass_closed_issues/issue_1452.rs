//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1452.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
             \n  foo: foo(());\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: () isn\'t a valid CSS value.\
         \n  ,\
         \n2 |   foo: foo(());\
         \n  |            ^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
    );
}
