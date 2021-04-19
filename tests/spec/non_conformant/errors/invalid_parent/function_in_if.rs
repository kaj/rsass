//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/function-in-if.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@if (true) {\r\
             \n  @function foo() {}\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Functions may not be declared in control directives.\
         \n  ,\
         \n2 |   @function foo() {}\
         \n  |   ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet\
         \n",
    );
}
