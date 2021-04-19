//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1683/mixin.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($x, $y) { }\
             \n\
             \na {\
             \n  @include foo(1 2 3...);\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n    ,\
         \n1   | @mixin foo($x, $y) { }\
         \n    |        =========== declaration\
         \n... |\
         \n4   |   @include foo(1 2 3...);\
         \n    |   ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n    \'\
         \n  input.scss 4:3  foo()\
         \n  input.scss 4:3  root stylesheet",
    );
}
