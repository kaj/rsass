//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2147/lhs.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (a:b,c:d) + 1;\r\
             \n"
        )
        .unwrap_err(),
        "Error: (a: b, c: d) isn\'t a valid CSS value.\
         \n  ,\
         \n1 | $map: (a:b,c:d) + 1;\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
