//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_713/not.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@function not() {\
             \n  @return \"not\";\
             \n}\n\
             \ntest {\
             \n  not: not();\
             \n}\n"
        ),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function not() {\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
