//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2569.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2569")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@function test() {\
             \n  @if (false) {\
             \n    @return 0;\
             \n  } @else {\
             \n    opacity: 1;\
             \n  }\
             \n}\n\
             \n.my-module {\
             \n  opacity: test();\
             \n}"
        ),
        "Error: @function rules may not contain declarations.\
         \n  ,\
         \n5 |     opacity: 1;\
         \n  |     ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 5:5  root stylesheet",
    );
}
