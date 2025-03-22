//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/function-in-each.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function-in-each")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@each $item in (a, b) {\r\
             \n  @function foo() {}\r\
             \n}"
        ),
        "Error: Functions may not be declared in control directives.\
         \n  ,\
         \n2 |   @function foo() {}\
         \n  |   ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
