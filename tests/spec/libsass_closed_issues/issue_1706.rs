//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1706.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function calc($e) { @return custom; }\
             \n@function -foo-calc($e) { @return custom; }\
             \n\
             \n.test {\
             \n    a: calc(1px * 1%);\
             \n    b: -foo-calc(2px * 2%);\
             \n    c: call(calc, 3px * 3%);\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function calc($e) { @return custom; }\
         \n  | ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
