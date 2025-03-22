//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1106.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1106")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:string\";\
             \n@function foo() { @return null; }\
             \n$foo: null;\
             \na {\
             \n    foo: bar;\
             \n    variable: $foo;\
             \n    function: foo();\
             \n    unquote: string.unquote($foo);\
             \n}\n\
             \nb {\
             \n    variable: $foo;\
             \n    function: foo();\
             \n    unquote: string.unquote($foo);\
             \n}\n"
        ),
        "Error: $string: null is not a string.\
         \n  ,\
         \n8 |     unquote: string.unquote($foo);\
         \n  |              ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 8:14  root stylesheet",
    );
}
