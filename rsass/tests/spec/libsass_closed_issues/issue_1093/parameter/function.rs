//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/parameter/function.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@function foo($bar) {\
             \n  @return $bar;\
             \n}\n\
             \n$foo: foo(#{});\n"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n5 | $foo: foo(#{});\
         \n  |           ^^\
         \n  \'\
         \n  input.scss 5:11  root stylesheet",
    );
}
