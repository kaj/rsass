//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1550/while_embedded.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("while_embedded")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "$i: 1;\
             \n@while $i == 1 {\
             \n  @function foo() {\
             \n    @return \'foo\';\
             \n  }\
             \n  $i: $i + 1;\
             \n}\n"
        ),
        "Error: Functions may not be declared in control directives.\
         \n  ,\
         \n3 |   @function foo() {\
         \n  |   ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:3  root stylesheet",
    );
}
