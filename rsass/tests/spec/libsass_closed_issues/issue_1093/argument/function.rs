//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/argument/function.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@function foo($bar:#{}) {\
             \n  @return $bar;\
             \n}\n\
             \n$foo: foo();\n"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @function foo($bar:#{}) {\
         \n  |                    ^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
