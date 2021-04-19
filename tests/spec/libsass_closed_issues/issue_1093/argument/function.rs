//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/argument/function.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($bar:#{}) {\
             \n  @return $bar;\
             \n}\
             \n\
             \n$foo: foo();\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @function foo($bar:#{}) {\
         \n  |                    ^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet\
         \n",
    );
}
