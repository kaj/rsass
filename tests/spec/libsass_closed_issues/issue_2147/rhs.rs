//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2147/rhs.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: 1 + (a:b,c:d);\r\
             \n"
        )
        .unwrap_err(),
        "Error: (a: b, c: d) isn\'t a valid CSS value.\
         \n  ,\
         \n1 | $map: 1 + (a:b,c:d);\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
