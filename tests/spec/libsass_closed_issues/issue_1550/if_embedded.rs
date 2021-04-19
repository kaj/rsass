//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1550/if_embedded.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@if (true) {\
             \n  @function foo() {\
             \n    @return \'foo\';\
             \n  }\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Functions may not be declared in control directives.\
         \n  ,\
         \n2 |   @function foo() {\
         \n  |   ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
