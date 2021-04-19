//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/parameter/function.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($bar) {\
             \n  @return $bar;\
             \n}\
             \n\
             \n$foo: foo(#{});\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n5 | $foo: foo(#{});\
         \n  |           ^^\
         \n  \'\
         \n  input.scss 5:11  root stylesheet",
    );
}
