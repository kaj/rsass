//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1683/function.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($x, $y) { @return null }\
             \n\
             \na {\
             \n  b: foo(1 2 3...);\
             \n}"
        )
        .unwrap_err(),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n    ,\
         \n1   | @function foo($x, $y) { @return null }\
         \n    |           =========== declaration\
         \n... |\
         \n4   |   b: foo(1 2 3...);\
         \n    |      ^^^^^^^^^^^^^ invocation\
         \n    \'\
         \n  input.scss 4:6  foo()\
         \n  input.scss 4:6  root stylesheet",
    );
}
