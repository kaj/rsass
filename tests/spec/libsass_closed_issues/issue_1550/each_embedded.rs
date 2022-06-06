//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1550/each_embedded.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("each_embedded")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@each $i in (1) {\
             \n  @function foo() {\
             \n    @return \'foo\';\
             \n  }\
             \n}\n"
        ),
        "Error: Functions may not be declared in control directives.\
         \n  ,\
         \n2 |   @function foo() {\
         \n  |   ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
