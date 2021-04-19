//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1941/function_function.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function parent() {\
             \n  @function nested() {\
             \n    @return foo;\
             \n  }\
             \n\
             \n  @return nested();\
             \n}\
             \n\
             \n\
             \ntest {\
             \n  foo: parent();\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @function nested() {\
         \n  |   ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet\
         \n",
    );
}
