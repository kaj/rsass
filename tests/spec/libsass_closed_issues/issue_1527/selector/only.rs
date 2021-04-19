//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1527/selector/only.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "& {}\
             \n"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | & {}\
         \n  | ^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
