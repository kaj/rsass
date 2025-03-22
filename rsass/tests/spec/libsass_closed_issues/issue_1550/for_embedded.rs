//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1550/for_embedded.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("for_embedded")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@for $i from 1 through 2 {\
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
