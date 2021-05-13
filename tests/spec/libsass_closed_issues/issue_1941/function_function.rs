//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1941/function_function.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@function parent() {\
             \n  @function nested() {\
             \n    @return foo;\
             \n  }\n\
             \n  @return nested();\
             \n}\n\n\
             \ntest {\
             \n  foo: parent();\
             \n}\n"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @function nested() {\
         \n  |   ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
